
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

#[cfg(not(feature = "standalone_test"))]
use crate::klib::collections::HashMap;

#[cfg(feature = "standalone_test")]
use std::collections::BTreeMap as HashMap;

/// WSL Execution Architecture Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WslArchitectureMode {
    /// WSL1 Mode: Direct POSIX syscall to SigmaOS microkernel capability translation layer (Pico processes)
    Wsl1PicoTranslation,
    /// WSL2 Mode: Lightweight micro-VM hypervisor container with native Linux kernel payload
    Wsl2MicroVmContainer,
    /// BSD Hybrid Mode: FreeBSD Linuxulator / NetBSD Rump kernel hybrid translation engine
    BsdLinuxulatorHybrid,
}

/// Dynamic File System Mount Types for DrvFs / VolFs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WslMountType {
    /// Windows/Sigma host drive translation (/mnt/c, /mnt/d)
    DrvFsHostDrive,
    /// Native Linux Virtual File System with metadata support (chmod, chown, xattr)
    VolFsNativeLinux,
    /// Virtio-FS / 9P network protocol zero-copy host-guest file sharing
    VirtioFs9PProtocol,
}

/// DrvFs File System Mount Entry
#[derive(Debug, Clone)]
pub struct DrvFsMountPoint {
    pub mount_point: String,
    pub source_path: String,
    pub mount_type: WslMountType,
    pub case_sensitive: bool,
    pub metadata_enabled: bool,
    pub is_mounted: bool,
}

/// Mount Manager for DrvFs and host-guest VFS sharing
#[derive(Debug, Clone)]
pub struct DrvFsMountManager {
    pub mounts: HashMap<String, DrvFsMountPoint>,
}

impl DrvFsMountManager {
    pub fn new() -> Self {
        let mut manager = Self {
            mounts: HashMap::new(),
        };

        // Pre-register standard drive mounts (/mnt/c, /mnt/d)
        manager
            .mount_drive("C:", "/mnt/c", WslMountType::DrvFsHostDrive, true, true)
            .ok();
        manager
            .mount_drive("D:", "/mnt/d", WslMountType::DrvFsHostDrive, false, false)
            .ok();

        manager
    }

    pub fn mount_drive(
        &mut self,
        source: &str,
        mount_point: &str,
        mount_type: WslMountType,
        case_sensitive: bool,
        metadata_enabled: bool,
    ) -> Result<(), &'static str> {
        let entry = DrvFsMountPoint {
            mount_point: mount_point.to_string(),
            source_path: source.to_string(),
            mount_type,
            case_sensitive,
            metadata_enabled,
            is_mounted: true,
        };

        self.mounts.insert(mount_point.to_string(), entry);
        Ok(())
    }

    pub fn unmount_drive(&mut self, mount_point: &str) -> Result<(), &'static str> {
        if let Some(entry) = self.mounts.get_mut(mount_point) {
            entry.is_mounted = false;
            Ok(())
        } else {
            Err("WSL DrvFs: Mount point not found")
        }
    }

    pub fn resolve_linux_path(&self, host_path: &str) -> String {
        let clean = host_path.replace('\\', "/");
        if clean.len() >= 2 && &clean[1..2] == ":" {
            let drive_letter = clean[..1].to_lowercase();
            let path_tail = &clean[2..];
            format!("/mnt/{}{}", drive_letter, path_tail)
        } else {
            host_path.to_string()
        }
    }

    pub fn resolve_host_path(&self, linux_path: &str) -> String {
        if linux_path.starts_with("/mnt/") && linux_path.len() >= 6 {
            let drive_letter = &linux_path[5..6].to_uppercase();
            let path_tail = &linux_path[6..].replace('/', "\\");
            format!("{}:{}", drive_letter, path_tail)
        } else {
            linux_path.to_string()
        }
    }
}

impl Default for DrvFsMountManager {
    fn default() -> Self {
        Self::new()
    }
}

/// WSLg GUI Application Forwarding Display Protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WslgDisplayProtocol {
    WaylandWestonRdp,
    X11Xwayland,
    PulseAudioPipeWire,
}

/// WSLg GUI App Forwarding Engine
#[derive(Debug, Clone)]
pub struct WslgGuiForwardingEngine {
    pub display_id: String,
    pub wayland_socket_path: String,
    pub pulse_server_endpoint: String,
    pub rdp_virtual_channel_active: bool,
    pub active_windows_count: u32,
}

impl WslgGuiForwardingEngine {
    pub fn new() -> Self {
        Self {
            display_id: String::from(":0"),
            wayland_socket_path: String::from("/tmp/wayland-0"),
            pulse_server_endpoint: String::from("unix:/tmp/pulse-socket"),
            rdp_virtual_channel_active: true,
            active_windows_count: 0,
        }
    }

    pub fn create_gui_window(&mut self, app_title: &str, protocol: WslgDisplayProtocol) -> String {
        self.active_windows_count += 1;
        format!(
            "WSLg Window Created: [{}] via {:?} on DISPLAY={}",
            app_title, protocol, self.display_id
        )
    }

    pub fn close_gui_window(&mut self) {
        if self.active_windows_count > 0 {
            self.active_windows_count -= 1;
        }
    }
}

impl Default for WslgGuiForwardingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// WSL Interoperability Bridge for Cross-Environment Execution
#[derive(Debug, Clone)]
pub struct WslInteropBridge {
    pub path_append_enabled: bool,
    pub wslenv_rules: HashMap<String, String>,
}

impl WslInteropBridge {
    pub fn new() -> Self {
        let mut bridge = Self {
            path_append_enabled: true,
            wslenv_rules: HashMap::new(),
        };

        bridge
            .wslenv_rules
            .insert(String::from("PATH"), String::from("l:p"));
        bridge
            .wslenv_rules
            .insert(String::from("TMPDIR"), String::from("p"));

        bridge
    }

    pub fn execute_host_command(&self, cmd: &str, args: &[&str]) -> Result<String, &'static str> {
        if cmd.is_empty() {
            return Err("Interop: Command name cannot be empty");
        }
        let full_args = args.join(" ");
        Ok(format!(
            "Host Command Output (via Interop): {} {}",
            cmd, full_args
        ))
    }

    pub fn translate_env_var(&self, var_name: &str, val: &str) -> String {
        if let Some(rule) = self.wslenv_rules.get(var_name) {
            let contains_p: bool = rule.contains('p');
            if contains_p {
                val.replace('\\', "/").replace("C:", "/mnt/c")
            } else {
                val.to_string()
            }
        } else {
            val.to_string()
        }
    }
}

impl Default for WslInteropBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Registered Guest Distribution Spec in WSL
#[derive(Debug, Clone)]
pub struct WslDistroSpec {
    pub name: String,
    pub version: u32, // 1 for WSL1, 2 for WSL2
    pub mode: WslArchitectureMode,
    pub default_user: String,
    pub kernel_version: String,
    pub is_default: bool,
    pub is_running: bool,
}

/// WSL Distribution Registry & Instance Manager
#[derive(Debug, Clone)]
pub struct DistroRegistryManager {
    pub distros: HashMap<String, WslDistroSpec>,
    pub default_distro_name: Option<String>,
    pub mount_manager: DrvFsMountManager,
    pub gui_engine: WslgGuiForwardingEngine,
    pub interop_bridge: WslInteropBridge,
}

impl DistroRegistryManager {
    pub fn new() -> Self {
        let mut manager = Self {
            distros: HashMap::new(),
            default_distro_name: None,
            mount_manager: DrvFsMountManager::new(),
            gui_engine: WslgGuiForwardingEngine::new(),
            interop_bridge: WslInteropBridge::new(),
        };

        // Register default distro targets inspired by popular Linux & BSD distros
        manager
            .register_distro(
                "Ubuntu",
                2,
                WslArchitectureMode::Wsl2MicroVmContainer,
                "ubuntu",
                "5.15.90.1-sigmaos",
                true,
            )
            .ok();

        manager
            .register_distro(
                "Arch",
                2,
                WslArchitectureMode::Wsl2MicroVmContainer,
                "arch",
                "6.5.6-sigmaos",
                false,
            )
            .ok();

        manager
            .register_distro(
                "Alpine",
                1,
                WslArchitectureMode::Wsl1PicoTranslation,
                "root",
                "5.10.0-sigmaos",
                false,
            )
            .ok();

        manager
            .register_distro(
                "FreeBSD-Hybrid",
                1,
                WslArchitectureMode::BsdLinuxulatorHybrid,
                "freebsd",
                "14.0-RELEASE-sigmaos",
                false,
            )
            .ok();

        manager
    }

    pub fn register_distro(
        &mut self,
        name: &str,
        version: u32,
        mode: WslArchitectureMode,
        default_user: &str,
        kernel_version: &str,
        set_default: bool,
    ) -> Result<(), &'static str> {
        let spec = WslDistroSpec {
            name: name.to_string(),
            version,
            mode,
            default_user: default_user.to_string(),
            kernel_version: kernel_version.to_string(),
            is_default: set_default,
            is_running: false,
        };

        if set_default || self.default_distro_name.is_none() {
            self.default_distro_name = Some(name.to_string());
        }

        self.distros.insert(name.to_string(), spec);
        Ok(())
    }

    pub fn set_distro_version(&mut self, name: &str, version: u32) -> Result<(), &'static str> {
        let distro = self.distros.get_mut(name).ok_or("Distro not registered")?;
        distro.version = version;
        distro.mode = match version {
            1 => WslArchitectureMode::Wsl1PicoTranslation,
            2 => WslArchitectureMode::Wsl2MicroVmContainer,
            _ => WslArchitectureMode::BsdLinuxulatorHybrid,
        };
        Ok(())
    }

    pub fn launch_distro(&mut self, name: &str) -> Result<String, &'static str> {
        let distro = self.distros.get_mut(name).ok_or("Distro not registered")?;
        distro.is_running = true;

        Ok(format!(
            "Launched WSL Distro [{}] (WSL{}, Mode: {:?}, Kernel: {})",
            distro.name, distro.version, distro.mode, distro.kernel_version
        ))
    }

    pub fn terminate_distro(&mut self, name: &str) -> Result<(), &'static str> {
        let distro = self.distros.get_mut(name).ok_or("Distro not registered")?;
        distro.is_running = false;
        Ok(())
    }

    pub fn shutdown_all(&mut self) {
        for distro in self.distros.values_mut() {
            distro.is_running = false;
        }
    }

    pub fn list_distros(&self) -> Vec<WslDistroSpec> {
        self.distros.values().cloned().collect()
    }
}

impl Default for DistroRegistryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drvfs_mount_manager() {
        let mut mount_mgr = DrvFsMountManager::new();

        assert!(mount_mgr.mounts.contains_key("/mnt/c"));
        assert!(mount_mgr.mounts.contains_key("/mnt/d"));

        let win_path = r"C:\Users\Sigma\Projects\main.rs";
        let linux_path = mount_mgr.resolve_linux_path(win_path);
        assert_eq!(linux_path, "/mnt/c/Users/Sigma/Projects/main.rs");

        let back_to_win = mount_mgr.resolve_host_path(&linux_path);
        assert_eq!(back_to_win, r"C:\Users\Sigma\Projects\main.rs");

        assert!(mount_mgr.unmount_drive("/mnt/d").is_ok());
        assert!(!mount_mgr.mounts.get("/mnt/d").unwrap().is_mounted);
    }

    #[test]
    fn test_wslg_gui_forwarding_engine() {
        let mut gui = WslgGuiForwardingEngine::new();
        let msg = gui.create_gui_window("GIMP", WslgDisplayProtocol::WaylandWestonRdp);

        assert!(msg.contains("GIMP"));
        assert_eq!(gui.active_windows_count, 1);

        gui.close_gui_window();
        assert_eq!(gui.active_windows_count, 0);
    }

    #[test]
    fn test_wsl_interop_bridge() {
        let bridge = WslInteropBridge::new();
        let res = bridge
            .execute_host_command("cmd.exe", &["/c", "dir"])
            .unwrap();
        assert!(res.contains("cmd.exe /c dir"));

        let translated = bridge.translate_env_var("PATH", r"C:\Windows\System32");
        assert_eq!(translated, "/mnt/c/Windows/System32");
    }

    #[test]
    fn test_distro_registry_manager() {
        let mut manager = DistroRegistryManager::new();

        let distros = manager.list_distros();
        assert!(distros.len() >= 4);

        let launch_msg = manager.launch_distro("Ubuntu").unwrap();
        assert!(launch_msg.contains("Launched WSL Distro [Ubuntu]"));
        assert!(manager.distros.get("Ubuntu").unwrap().is_running);

        assert!(manager.set_distro_version("Ubuntu", 1).is_ok());
        assert_eq!(
            manager.distros.get("Ubuntu").unwrap().mode,
            WslArchitectureMode::Wsl1PicoTranslation
        );

        manager.shutdown_all();
        assert!(!manager.distros.get("Ubuntu").unwrap().is_running);
    }
}
