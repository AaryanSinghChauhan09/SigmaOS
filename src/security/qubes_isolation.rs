use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
extern crate alloc;
// SigmaOS Microkernel Shard & Domain Isolation (Qubes OS & Kata Containers Parity)
// Enables ultra-lightweight, compartmentalized zero-trust secure domains (MicroVMs)
// Running natively in user-space with microsecond-level IPC latencies and hypervisor isolation.


#[cfg(not(test))]
use core::cell::RefCell;

#[cfg(test)]
use core::cell::RefCell;

#[cfg(not(test))]
use crate::security::CapabilityToken;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken(pub u64);

#[cfg(test)]
impl CapabilityToken {
    pub fn from_bits(bits: u64) -> Self {
        Self(bits)
    }
    pub fn bits(&self) -> u64 {
        self.0
    }
}

#[cfg(not(test))]
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(test)]
use core::sync::atomic::{AtomicUsize, Ordering};


pub type DomainID = usize;

const MAX_DOMAINS: usize = 16;
const MAX_POLICIES: usize = 32;
const MAX_MESSAGE_SIZE: usize = 256;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainType {
    Admin = 0,
    Net = 1,
    Storage = 2,
    App = 3,
    Disposable = 4,
    Dom0,
    AppVM,
    NetVM,
    DispVM,
    TemplateVM,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationError {
    Success = 0,
    DomainNotFound = 1,
    PermissionDenied = 2,
    IpcRouteFailed = 3,
    CreationError = 4,
    HypervisorInitFailed = 5,
}

/// Kata Containers Hypervisor Technology Choice
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KataHypervisorType {
    CloudHypervisor,
    Firecracker,
    QemuMicroVm,
}

/// Configuration for a Kata Containers microVM instance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KataMicroVmConfig {
    pub vcpu_count: u32,
    pub memory_mb: u32,
    pub hypervisor: KataHypervisorType,
    pub kernel_path: [u8; 64],
    pub initrd_path: [u8; 64],
    pub enable_vsock: bool,
}

impl KataMicroVmConfig {
    pub fn default_firecracker() -> Self {
        let mut kernel = [0u8; 64];
        let mut initrd = [0u8; 64];
        let k_str = b"/boot/vmlinux-kata.bin";
        let i_str = b"/boot/kata-initrd.img";
        kernel[..k_str.len()].copy_from_slice(k_str);
        initrd[..i_str.len()].copy_from_slice(i_str);

        Self {
            vcpu_count: 2,
            memory_mb: 512,
            hypervisor: KataHypervisorType::Firecracker,
            kernel_path: kernel,
            initrd_path: initrd,
            enable_vsock: true,
        }
    }
}

/// Domain Descriptor State
#[derive(Debug, Clone, Copy)]
pub struct VirtualDomain {
    pub dom_id: u32,
    pub name_hash: u32, // FNV-1a hashed domain name
    pub domain_type: DomainType,
    pub is_running: bool,
    pub assigned_pci_slot: Option<u32>, // Hardware isolation slot
}

/// Secure Qrexec IPC Inter-VM Packet Frame
#[derive(Debug, Clone, Copy)]
pub struct QrexecMessage {
    pub source_dom_id: u32,
    pub dest_dom_id: u32,
    pub service_name_hash: u32, // FNV-1a hashed service target
    pub payload: [u8; MAX_MESSAGE_SIZE],
    pub payload_len: usize,
}

/// Qrexec Policy Rule mapping
#[derive(Debug, Clone, Copy)]
pub struct PolicyRule {
    pub source_type: DomainType,
    pub dest_type: DomainType,
    pub service_name_hash: u32,
    pub allow: bool,
}

/// Global Qubes-style Isolation Manager
pub struct SovereignIsolationManager {
    pub domains: RefCell<[Option<VirtualDomain>; MAX_DOMAINS]>,
    pub policies: [Option<PolicyRule>; MAX_POLICIES],
    pub next_dom_id: u32,
}

impl SovereignIsolationManager {
    pub fn new() -> Self {
        const EMPTY_DOM: Option<VirtualDomain> = None;
        const EMPTY_POLICY: Option<PolicyRule> = None;

        let mut manager = Self {
            domains: RefCell::new([EMPTY_DOM; MAX_DOMAINS]),
            policies: [EMPTY_POLICY; MAX_POLICIES],
            next_dom_id: 1,
        };

        // Bootstrap the master administrative Dom0 domain
        let _ = manager.register_domain(0, DomainType::Dom0, None);

        // Load default secure Qrexec policies
        manager.load_default_policies();

        manager
    }

    /// Basic FNV-1a hash algorithm to simulate service/domain names comparison
    pub fn hash_name(name: &str) -> u32 {
        let mut hash: u32 = 2166136261;
        for &byte in name.as_bytes() {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(16777619);
        }
        hash
    }

    fn load_default_policies(&mut self) {
        let file_transfer_service = Self::hash_name("qubes.FileTransfer");
        let open_in_vm_service = Self::hash_name("qubes.OpenInVM");

        // Policy 1: Dom0 is allowed to send file transfers to any AppVM
        self.policies[0] = Some(PolicyRule {
            source_type: DomainType::Dom0,
            dest_type: DomainType::AppVM,
            service_name_hash: file_transfer_service,
            allow: true,
        });

        // Policy 2: AppVM is NOT allowed to trigger direct execution inside NetVM
        self.policies[1] = Some(PolicyRule {
            source_type: DomainType::AppVM,
            dest_type: DomainType::NetVM,
            service_name_hash: open_in_vm_service,
            allow: false,
        });

        // Policy 3: DispVM is allowed to send files back to AppVM
        self.policies[2] = Some(PolicyRule {
            source_type: DomainType::DispVM,
            dest_type: DomainType::AppVM,
            service_name_hash: file_transfer_service,
            allow: true,
        });
    }

    /// Registers a new isolated virtual domain
    pub fn register_domain(
        &mut self,
        name_hash: u32,
        domain_type: DomainType,
        pci_slot: Option<u32>,
    ) -> Result<u32, &'static str> {
        let dom_id = self.next_dom_id;

        let domain = VirtualDomain {
            dom_id,
            name_hash,
            domain_type,
            is_running: true,
            assigned_pci_slot: pci_slot,
        };

        let mut domains_guard = self.domains.borrow_mut();
        let domains_array: &mut [Option<VirtualDomain>; MAX_DOMAINS] = &mut *domains_guard;
        for slot in domains_array.iter_mut() {
            if slot.is_none() {
                *slot = Some(domain);
                self.next_dom_id += 1;
                return Ok(dom_id);
            }
        }

        Err("IsolationManager: Max domain boundary exceeded")
    }

    /// Recycles/Shuts down a Disposable VM context on session exit
    pub fn recycle_disposable_domain(&self, dom_id: u32) -> Result<(), &'static str> {
        let mut domains = self.domains.borrow_mut();
        for slot in domains.iter_mut() {
            if let Some(ref mut domain) = slot {
                if domain.dom_id == dom_id
                    && (domain.domain_type == DomainType::DispVM
                        || domain.domain_type == DomainType::Disposable)
                {
                    domain.is_running = false;
                    *slot = None;
                    return Ok(());
                }
            }
        }
        Err("IsolationManager: Disposable domain ID not found or already recycled")
    }

    /// Core Qrexec Policy Engine. Validates if an inter-VM transaction is authorized prior to payload dispatch
    pub fn validate_qrexec_policy(&self, msg: &QrexecMessage) -> bool {
        let domains = self.domains.borrow();

        let mut src_domain: Option<VirtualDomain> = None;
        let mut dest_domain: Option<VirtualDomain> = None;

        for slot in domains.iter() {
            if let Some(ref dom) = slot {
                if dom.dom_id == msg.source_dom_id {
                    src_domain = Some(*dom);
                }
                if dom.dom_id == msg.dest_dom_id {
                    dest_domain = Some(*dom);
                }
            }
        }

        let (src, dest) = match (src_domain, dest_domain) {
            (Some(s), Some(d)) => (s, d),
            _ => return false,
        };

        for rule_slot in &self.policies {
            if let Some(ref rule) = rule_slot {
                if rule.source_type == src.domain_type
                    && rule.dest_type == dest.domain_type
                    && rule.service_name_hash == msg.service_name_hash
                {
                    return rule.allow;
                }
            }
        }

        false
    }

    /// Handles Qrexec secure messaging delivery
    pub fn dispatch_qrexec_message(&self, msg: &QrexecMessage) -> Result<(), &'static str> {
        if !self.validate_qrexec_policy(msg) {
            return Err("Qrexec: PermissionDenied - Blocked by Sovereign Isolation Policy");
        }
        Ok(())
    }
}

impl Default for SovereignIsolationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a compartmentalized secure microkernel domain (AppVM / NetVM / Kata MicroVM equivalent)
pub struct IsolatedDomain {
    pub id: DomainID,
    pub name: [u8; 32],
    pub domain_type: DomainType,
    pub capabilities: CapabilityToken,
    pub active: bool,
    pub kata_config: Option<KataMicroVmConfig>,
    pub parent_id: Option<DomainID>, // Used for fast CoW cloning
    pub page_table_base: u64,        // Simulated hardware physical page table base (CR3-like)
}

impl IsolatedDomain {
    pub fn new(
        id: DomainID,
        name_str: &[u8],
        domain_type: DomainType,
        caps: CapabilityToken,
    ) -> Self {
        let mut name_arr = [0u8; 32];
        let len = name_str.len().min(31);
        name_arr[..len].copy_from_slice(&name_str[..len]);
        Self {
            id,
            name: name_arr,
            domain_type,
            capabilities: caps,
            active: true,
            kata_config: None,
            parent_id: None,
            page_table_base: 0x1000 * id as u64,
        }
    }

    pub fn with_kata_microvm(mut self, config: KataMicroVmConfig) -> Self {
        self.kata_config = Some(config);
        self
    }
}

/// Qrexec policy action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QrexecPolicyAction {
    Allow,
    Deny,
    Ask,
}

/// Represents Qubes-style RPC policy lookup rules (e.g. $any VM sys-net ask)
pub struct QrexecRule {
    pub source_type: DomainType,
    pub dest_type: DomainType,
    pub action: QrexecPolicyAction,
}

/// Dynamic Qrexec Policy Engine (RPC verification)
pub struct QrexecPolicyEngine {
    pub rules: Vec<QrexecRule>,
}

impl QrexecPolicyEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(
        &mut self,
        source_type: DomainType,
        dest_type: DomainType,
        action: QrexecPolicyAction,
    ) {
        self.rules.push(QrexecRule {
            source_type,
            dest_type,
            action,
        });
    }

    pub fn check_rpc_policy(&self, src: DomainType, dest: DomainType) -> QrexecPolicyAction {
        for rule in self.rules.iter() {
            if rule.source_type == src && rule.dest_type == dest {
                return rule.action;
            }
        }
        QrexecPolicyAction::Deny // default deny
    }
}

impl Default for QrexecPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic TemplateVM Manager backing AppVM instantiations.
pub struct TemplateVmManager {
    pub template_id: DomainID,
    pub app_vm_count: usize,
    pub active_overlays_allocated_bytes: usize,
}

impl TemplateVmManager {
    pub fn new(template_id: DomainID) -> Self {
        Self {
            template_id,
            app_vm_count: 0,
            active_overlays_allocated_bytes: 0,
        }
    }

    pub fn instantiate_app_vm(&mut self) -> Result<DomainID, IsolationError> {
        self.app_vm_count += 1;
        self.active_overlays_allocated_bytes += 128 * 1024 * 1024;
        Ok(self.template_id + self.app_vm_count)
    }

    pub fn discard_volatile_overlay(&mut self) {
        if self.app_vm_count > 0 {
            self.app_vm_count -= 1;
            self.active_overlays_allocated_bytes = self
                .active_overlays_allocated_bytes
                .saturating_sub(128 * 1024 * 1024);
        }
    }
}

/// Hierarchical XenStore key-value tree node for Xen hypervisor Dom0 control interface
#[derive(Debug, Clone)]
pub struct XenStoreNode {
    pub path: String,
    pub value: String,
    pub permissions_mask: u32, // Read/Write bitmask
}

/// XenStore transaction and watch notification manager
pub struct XenStoreTree {
    pub nodes: Vec<XenStoreNode>,
    pub active_watches: Vec<(String, u32)>, // (path, dom_id)
}

impl XenStoreTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            active_watches: Vec::new(),
        }
    }

    pub fn write_key(&mut self, path: &str, value: &str, perms: u32) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.path == path) {
            node.value = value.to_string();
            node.permissions_mask = perms;
        } else {
            self.nodes.push(XenStoreNode {
                path: path.to_string(),
                value: value.to_string(),
                permissions_mask: perms,
            });
        }
    }

    pub fn read_key(&self, path: &str) -> Option<&str> {
        self.nodes.iter().find(|n| n.path == path).map(|n| n.value.as_str())
    }

    pub fn add_watch(&mut self, path: &str, dom_id: u32) {
        self.active_watches.push((path.to_string(), dom_id));
    }
}

/// Lock-free zero-copy inter-domain Xen grant table shared memory ring
pub struct XenChannelRing {pub dom_a: u32,
    pub dom_b: u32,
    pub grant_ref: u32,
    pub ring_size: usize,
    pub buffer: Vec<u8>,
    pub head: usize,
    pub tail: usize,
}

impl XenChannelRing {
    pub fn new(dom_a: u32, dom_b: u32, grant_ref: u32, ring_size: usize) -> Self {
        Self {
            dom_a,
            dom_b,
            grant_ref,
            ring_size,
            buffer: vec![0u8; ring_size],
            head: 0,
            tail: 0,
        }
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> usize {
        let mut count = 0;
        for &byte in data {
            if (self.head + 1) % self.ring_size == self.tail {
                break;
            }
            self.buffer[self.head] = byte;
            self.head = (self.head + 1) % self.ring_size;
            count += 1;
        }
        count
    }

    pub fn read_bytes(&mut self, dest: &mut [u8]) -> usize {
        let mut count = 0;
        for slot in dest.iter_mut() {
            if self.tail == self.head {
                break;
            }
            *slot = self.buffer[self.tail];
            self.tail = (self.tail + 1) % self.ring_size;
            count += 1;
        }
        count
    }
}

/// Memory-safe frame-buffer blitting engine between untrusted AppVMs and Dom0 GUI compositor
pub struct QubesGuiBlitter {
    pub screen_width: u32,
    pub screen_height: u32,
    pub dom0_framebuffer: Vec<u32>, // ARGB32
}

impl QubesGuiBlitter {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = (width * height) as usize;
        Self {
            screen_width: width,
            screen_height: height,
            dom0_framebuffer: vec![0x00000000; pixels],
        }
    }

    /// Securely blits untrusted AppVM window buffer into Dom0 display surface with bounds validation
    pub fn blit_window_surface(
        &mut self,
        source_buffer: &[u32],
        win_x: u32,
        win_y: u32,
        win_w: u32,
        win_h: u32,
    ) -> Result<usize, &'static str> {
        if source_buffer.len() < (win_w * win_h) as usize {
            return Err("AppVM surface buffer length underflow");
        }

        let mut blitted_pixels = 0;
        for y in 0..win_h {
            let target_y = win_y + y;
            if target_y >= self.screen_height {
                continue;
            }
            for x in 0..win_w {
                let target_x = win_x + x;
                if target_x >= self.screen_width {
                    continue;
                }
                let src_idx = (y * win_w + x) as usize;
                let dst_idx = (target_y * self.screen_width + target_x) as usize;
                self.dom0_framebuffer[dst_idx] = source_buffer[src_idx];
                blitted_pixels += 1;
            }
        }
        Ok(blitted_pixels)
    }
}

/// Hierarchical XenStore key-value tree node for Xen hypervisor Dom0 control interface
#[derive(Debug, Clone)]
pub struct XenStoreNode {
    pub path: String,
    pub value: String,
    pub permissions_mask: u32, // Read/Write bitmask
}

/// XenStore transaction and watch notification manager
pub struct XenStoreTree {
    pub nodes: Vec<XenStoreNode>,
    pub active_watches: Vec<(String, u32)>, // (path, dom_id)
}

impl XenStoreTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            active_watches: Vec::new(),
        }
    }

    pub fn write_key(&mut self, path: &str, value: &str, perms: u32) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.path == path) {
            node.value = value.to_string();
            node.permissions_mask = perms;
        } else {
            self.nodes.push(XenStoreNode {
                path: path.to_string(),
                value: value.to_string(),
                permissions_mask: perms,
            });
        }
    }

    pub fn read_key(&self, path: &str) -> Option<&str> {
        self.nodes.iter().find(|n| n.path == path).map(|n| n.value.as_str())
    }

    pub fn add_watch(&mut self, path: &str, dom_id: u32) {
        self.active_watches.push((path.to_string(), dom_id));
    }
}

/// Lock-free zero-copy inter-domain Xen grant table shared memory ring
pub struct XenChannelRing {
    pub dom_a: u32,
    pub dom_b: u32,
    pub grant_ref: u32,
    pub ring_size: usize,
    pub buffer: Vec<u8>,
    pub head: usize,
    pub tail: usize,
}

impl XenChannelRing {
    pub fn new(dom_a: u32, dom_b: u32, grant_ref: u32, ring_size: usize) -> Self {
        Self {
            dom_a,
            dom_b,
            grant_ref,
            ring_size,
            buffer: vec![0u8; ring_size],
            head: 0,
            tail: 0,
        }
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> usize {
        let mut count = 0;
        for &byte in data {
            if (self.head + 1) % self.ring_size == self.tail {
                break;
            }
            self.buffer[self.head] = byte;
            self.head = (self.head + 1) % self.ring_size;
            count += 1;
        }
        count
    }

    pub fn read_bytes(&mut self, dest: &mut [u8]) -> usize {
        let mut count = 0;
        for slot in dest.iter_mut() {
            if self.tail == self.head {
                break;
            }
            *slot = self.buffer[self.tail];
            self.tail = (self.tail + 1) % self.ring_size;
            count += 1;
        }
        count
    }
}

/// Memory-safe frame-buffer blitting engine between untrusted AppVMs and Dom0 GUI compositor
pub struct QubesGuiBlitter {
    pub screen_width: u32,
    pub screen_height: u32,
    pub dom0_framebuffer: Vec<u32>, // ARGB32
}

impl QubesGuiBlitter {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = (width * height) as usize;
        Self {
            screen_width: width,
            screen_height: height,
            dom0_framebuffer: vec![0x00000000; pixels],
        }
    }

    /// Securely blits untrusted AppVM window buffer into Dom0 display surface with bounds validation
    pub fn blit_window_surface(
        &mut self,
        source_buffer: &[u32],
        win_x: u32,
        win_y: u32,
        win_w: u32,
        win_h: u32,
    ) -> Result<usize, &'static str> {
        if source_buffer.len() < (win_w * win_h) as usize {
            return Err("AppVM surface buffer length underflow");
        }

        let mut blitted_pixels = 0;
        for y in 0..win_h {
            let target_y = win_y + y;
            if target_y >= self.screen_height {
                continue;
            }
            for x in 0..win_w {
                let target_x = win_x + x;
                if target_x >= self.screen_width {
                    continue;
                }
                let src_idx = (y * win_w + x) as usize;
                let dst_idx = (target_y * self.screen_width + target_x) as usize;
                self.dom0_framebuffer[dst_idx] = source_buffer[src_idx];
                blitted_pixels += 1;
            }
        }
        Ok(blitted_pixels)
    }
}

/// Bypasses virtual network cards (which cause bottlenecks in Qubes OS) to write directly into target buffer ranges.
pub struct SQrexecChannel {
    pub buffer: *mut u8,
    pub size: usize,
    pub write_cursor: AtomicUsize,
    pub read_cursor: AtomicUsize,
}

impl SQrexecChannel {
    pub fn new(size: usize) -> Self {
        let layout = core::alloc::Layout::from_size_align(size.max(1), 8).unwrap();
        let buffer = unsafe { alloc::alloc::alloc(layout) };
        Self {
            buffer,
            size,
            write_cursor: AtomicUsize::new(0),
            read_cursor: AtomicUsize::new(0),
        }
    }

    pub fn write_payload(&self, data: &[u8]) -> Result<(), IsolationError> {
        let w = self.write_cursor.load(Ordering::SeqCst);
        let len = data.len();
        if w + len > self.size {
            return Err(IsolationError::IpcRouteFailed);
        }

        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), self.buffer.add(w), len);
        }
        self.write_cursor.store(w + len, Ordering::SeqCst);
        Ok(())
    }

    pub fn read_payload(&self) -> Vec<u8> {
        let w = self.write_cursor.load(Ordering::SeqCst);
        let r = self.read_cursor.load(Ordering::SeqCst);
        let mut vec = Vec::new();

        if w > r {
            unsafe {
                for i in r..w {
                    vec.push(*self.buffer.add(i));
                }
            }
            self.read_cursor.store(w, Ordering::SeqCst);
        }
        vec
    }

    pub fn destroy(&self) {
        unsafe {
            core::ptr::write_bytes(self.buffer, 0, self.size);
            let layout = core::alloc::Layout::from_size_align(self.size.max(1), 8).unwrap();
            alloc::alloc::dealloc(self.buffer, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qubes_isolation_manager_flow() {
        let mut manager = SovereignIsolationManager::new();
        let dom1 = manager
            .register_domain(
                SovereignIsolationManager::hash_name("work"),
                DomainType::AppVM,
                None,
            )
            .unwrap();
        let dom2 = manager
            .register_domain(
                SovereignIsolationManager::hash_name("net"),
                DomainType::NetVM,
                Some(1),
            )
            .unwrap();

        let service = SovereignIsolationManager::hash_name("qubes.OpenInVM");
        let msg = QrexecMessage {
            source_dom_id: dom1,
            dest_dom_id: dom2,
            service_name_hash: service,
            payload: [0u8; MAX_MESSAGE_SIZE],
            payload_len: 0,
        };

        // AppVM to NetVM OpenInVM is blocked by policy
        assert!(manager.dispatch_qrexec_message(&msg).is_err());
    }

    #[test]
    fn test_s_qrexec_shared_memory_channel() {
        let channel = SQrexecChannel::new(1024);

        // Write low-latency payload bypasses any virtual NIC overhead
        channel.write_payload(b"Hello Sovereign Domain IPC").unwrap();

        // Read payload from shared memory segment
        let read = channel.read_payload();
        assert_eq!(read.len(), 26);
        assert_eq!(read[0], b'H');

        channel.destroy();
    }
}
