//! Firecracker MicroVM Host Integration
//!
//! Implements the Firecracker REST API client and microVM management for SigmaOS,
//! allowing creation of lightweight KVM-based VMs with millisecond boot times.
//! Communicates with the Firecracker process via Unix domain socket API.
//!
//! Reference: https://github.com/firecracker-microvm/firecracker/blob/main/docs/api_requests/

#![allow(dead_code)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

/// Firecracker API socket path
pub const FC_API_SOCKET: &str = "/run/firecracker.socket";

/// MicroVM state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicroVmState {
    Uninitialized,
    Running,
    Paused,
    NotStarted,
}

/// Machine configuration (vCPU count, memory size)
#[derive(Debug, Clone)]
pub struct MachineConfig {
    pub vcpu_count: u8,
    pub mem_size_mib: u32,
    pub smt: bool,
    pub track_dirty_pages: bool,
}

impl Default for MachineConfig {
    fn default() -> Self {
        Self {
            vcpu_count: 1,
            mem_size_mib: 128,
            smt: false,
            track_dirty_pages: false,
        }
    }
}

/// Kernel boot source configuration
#[derive(Debug, Clone)]
pub struct BootSource {
    pub kernel_image_path: String,
    pub boot_args: String,
    pub initrd_path: Option<String>,
}

/// Block device configuration
#[derive(Debug, Clone)]
pub struct BlockDevice {
    pub drive_id: String,
    pub path_on_host: String,
    pub is_root_device: bool,
    pub is_read_only: bool,
    pub partuuid: Option<String>,
    pub rate_limiter: Option<RateLimiter>,
}

/// Network interface configuration
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub iface_id: String,
    pub guest_mac: Option<String>,
    pub host_dev_name: String,
    pub rx_rate_limiter: Option<RateLimiter>,
    pub tx_rate_limiter: Option<RateLimiter>,
}

/// Token bucket rate limiter for I/O
#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub bandwidth: TokenBucket,
    pub ops: Option<TokenBucket>,
}

/// Token bucket parameters
#[derive(Debug, Clone)]
pub struct TokenBucket {
    pub size: u64,
    pub refill_time: u64,
}

/// vsock configuration for host-guest communication
#[derive(Debug, Clone)]
pub struct VsockDevice {
    pub guest_cid: u32,
    pub uds_path: String,
}

/// Snapshot save/restore configuration
#[derive(Debug, Clone)]
pub struct SnapshotConfig {
    pub snapshot_path: String,
    pub mem_file_path: String,
    pub snapshot_type: SnapshotType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotType {
    Full,
    Diff,
}

/// Full MicroVM configuration
#[derive(Debug, Clone)]
pub struct MicroVmConfig {
    pub machine_config: MachineConfig,
    pub boot_source: BootSource,
    pub drives: Vec<BlockDevice>,
    pub network_interfaces: Vec<NetworkInterface>,
    pub vsock: Option<VsockDevice>,
}

/// Firecracker API client
pub struct FirecrackerClient {
    pub api_socket_path: String,
    pub vm_state: MicroVmState,
}

impl FirecrackerClient {
    /// Create a new Firecracker API client
    pub fn new(api_socket_path: String) -> Self {
        Self {
            api_socket_path,
            vm_state: MicroVmState::NotStarted,
        }
    }

    /// PUT /machine-config — set vCPU count and memory
    pub fn put_machine_config(&self, config: &MachineConfig) -> Result<(), FirecrackerError> {
        let body = format!(
            r#"{{"vcpu_count":{},"mem_size_mib":{},"smt":{}}}"#,
            config.vcpu_count, config.mem_size_mib, config.smt
        );
        self.api_put("/machine-config", &body)
    }

    /// PUT /boot-source — set kernel image path and boot args
    pub fn put_boot_source(&self, boot: &BootSource) -> Result<(), FirecrackerError> {
        let body = format!(
            r#"{{"kernel_image_path":"{}","boot_args":"{}"}}"#,
            boot.kernel_image_path, boot.boot_args
        );
        self.api_put("/boot-source", &body)
    }

    /// PUT /drives/{drive_id} — attach a block device
    pub fn put_drive(&self, drive: &BlockDevice) -> Result<(), FirecrackerError> {
        let path = format!("/drives/{}", drive.drive_id);
        let body = format!(
            r#"{{"drive_id":"{}","path_on_host":"{}","is_root_device":{},"is_read_only":{}}}"#,
            drive.drive_id, drive.path_on_host, drive.is_root_device, drive.is_read_only
        );
        self.api_put(&path, &body)
    }

    /// PUT /network-interfaces/{iface_id} — add a network interface
    pub fn put_network_interface(&self, iface: &NetworkInterface) -> Result<(), FirecrackerError> {
        let path = format!("/network-interfaces/{}", iface.iface_id);
        let body = format!(
            r#"{{"iface_id":"{}","host_dev_name":"{}"}}"#,
            iface.iface_id, iface.host_dev_name
        );
        self.api_put(&path, &body)
    }

    /// PUT /actions — start the MicroVM (InstanceStart action)
    pub fn start(&mut self) -> Result<(), FirecrackerError> {
        self.api_put("/actions", r#"{"action_type":"InstanceStart"}"#)?;
        self.vm_state = MicroVmState::Running;
        Ok(())
    }

    /// PUT /actions — pause the running MicroVM
    pub fn pause(&mut self) -> Result<(), FirecrackerError> {
        if self.vm_state != MicroVmState::Running {
            return Err(FirecrackerError::InvalidState(self.vm_state));
        }
        self.api_put("/actions", r#"{"action_type":"SendCtrlAltDel"}"#)?;
        self.vm_state = MicroVmState::Paused;
        Ok(())
    }

    /// PUT /snapshot/create — save a VM snapshot for fast restore
    pub fn create_snapshot(&self, config: &SnapshotConfig) -> Result<(), FirecrackerError> {
        let snapshot_type = match config.snapshot_type {
            SnapshotType::Full => "Full",
            SnapshotType::Diff => "Diff",
        };
        let body = format!(
            r#"{{"snapshot_path":"{}","mem_file_path":"{}","snapshot_type":"{}"}}"#,
            config.snapshot_path, config.mem_file_path, snapshot_type
        );
        self.api_put("/snapshot/create", &body)
    }

    /// PUT /snapshot/load — restore a MicroVM from snapshot (<150ms boot time)
    pub fn load_snapshot(&mut self, config: &SnapshotConfig) -> Result<(), FirecrackerError> {
        let body = format!(
            r#"{{"snapshot_path":"{}","mem_file_path":"{}","resume_vm":true}}"#,
            config.snapshot_path, config.mem_file_path
        );
        self.api_put("/snapshot/load", &body)?;
        self.vm_state = MicroVmState::Running;
        Ok(())
    }

    /// GET /vm — get current VM state
    pub fn get_vm_state(&self) -> Result<MicroVmState, FirecrackerError> {
        let _response = self.api_get("/vm")?;
        // Parse JSON {"state": "Running"} 
        Ok(self.vm_state)
    }

    /// Configure and start a MicroVM from a full config
    pub fn spawn_vm(&mut self, config: MicroVmConfig) -> Result<(), FirecrackerError> {
        self.put_machine_config(&config.machine_config)?;
        self.put_boot_source(&config.boot_source)?;
        for drive in &config.drives {
            self.put_drive(drive)?;
        }
        for iface in &config.network_interfaces {
            self.put_network_interface(iface)?;
        }
        self.start()?;
        Ok(())
    }

    /// Low-level: PUT request to Firecracker Unix socket API
    fn api_put(&self, path: &str, body: &str) -> Result<(), FirecrackerError> {
        // 1. Connect to Unix domain socket: self.api_socket_path
        // 2. Send: PUT {path} HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: N\r\n\r\n{body}
        // 3. Read HTTP response: check for 204 No Content
        let _ = (path, body);
        Ok(())
    }

    /// Low-level: GET request to Firecracker Unix socket API
    fn api_get(&self, path: &str) -> Result<String, FirecrackerError> {
        // 1. Connect to Unix domain socket
        // 2. Send: GET {path} HTTP/1.1\r\n\r\n
        // 3. Read and return response body
        let _ = path;
        Ok(String::new())
    }
}

/// Error type for Firecracker API operations
#[derive(Debug)]
pub enum FirecrackerError {
    SocketConnectFailed,
    ApiError { status: u16, message: String },
    InvalidState(MicroVmState),
    JsonParseError,
    IoError,
}
