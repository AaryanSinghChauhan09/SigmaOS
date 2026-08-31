//! Advanced Boot Manager inspired by GRUB2, systemd-boot, and rEFInd
//! Multi-boot entry management, measured boot TPM PCR registers, custom themes,
//! and fallback boot recovery environments.
extern crate alloc;


use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct BootEntry {
    pub id: String,
    pub title: String,
    pub kernel_path: String,
    pub initrd_path: Option<String>,
    pub cmdline_params: String,
    pub is_default: bool,
    pub is_recovery: bool,
}

#[derive(Debug, Clone)]
pub struct BootTheme {
    pub name: String,
    pub background_color_rgb: (u8, u8, u8),
    pub text_color_rgb: (u8, u8, u8),
    pub highlight_color_rgb: (u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffProtocol {
    LinuxEfiStub,
    Multiboot2,
    FreeBsdBtxElf,
    OpenBsdBootConf,
    LiveIsoOverlayFs,
}

#[derive(Debug, Clone)]
pub struct SovereignDistroBootStageHandoff {
    pub protocol: HandoffProtocol,
    pub root_uuid: String,
    pub is_initramfs_mounted: bool,
    pub is_overlayfs_active: bool,
    pub kernel_entry_point_addr: u64,
}

impl SovereignDistroBootStageHandoff {
    pub fn new(protocol: HandoffProtocol, root_uuid: &str) -> Self {
        Self {
            protocol,
            root_uuid: root_uuid.to_string(),
            is_initramfs_mounted: false,
            is_overlayfs_active: false,
            kernel_entry_point_addr: 0x0010_0000,
        }
    }

    pub fn mount_initramfs_vfs(&mut self) -> Result<(), &'static str> {
        if self.root_uuid.is_empty() {
            return Err("Boot Handoff: Root UUID cannot be empty");
        }
        self.is_initramfs_mounted = true;
        Ok(())
    }

    pub fn setup_live_iso_overlayfs(&mut self) -> Result<(), &'static str> {
        if !self.is_initramfs_mounted {
            return Err("Boot Handoff: Initramfs VFS must be mounted before overlayfs setup");
        }
        self.is_overlayfs_active = true;
        Ok(())
    }

    pub fn execute_stage_handoff(&self) -> bool {
        self.is_initramfs_mounted && self.kernel_entry_point_addr > 0
    }
}

pub struct BootManager {
    pub entries: Vec<BootEntry>,
    pub default_entry_id: String,
    pub timeout_seconds: u32,
    pub theme: BootTheme,
    pub is_measured_boot_active: bool,
    pub tpm_pcr_hashes: Vec<[u8; 32]>,
}

impl BootManager {
    pub fn new(timeout_seconds: u32) -> Self {
        let default_theme = BootTheme {
            name: "SigmaOS Sovereign Dark".to_string(),
            background_color_rgb: (15, 23, 42),
            text_color_rgb: (226, 232, 240),
            highlight_color_rgb: (59, 130, 246),
        };

        let mut mgr = Self {
            entries: Vec::new(),
            default_entry_id: String::new(),
            timeout_seconds,
            theme: default_theme,
            is_measured_boot_active: true,
            tpm_pcr_hashes: Vec::new(),
        };

        // Add standard fallback recovery entry
        mgr.add_entry(BootEntry {
            id: "sigmaos-recovery".to_string(),
            title: "SigmaOS Safe Recovery Environment".to_string(),
            kernel_path: "/boot/vmlinuz-recovery.efi".to_string(),
            initrd_path: Some("/boot/initramfs-recovery.img".to_string()),
            cmdline_params: "root=UUID=sigma_root ro recovery nomodeset".to_string(),
            is_default: false,
            is_recovery: true,
        });

        mgr
    }

    pub fn add_entry(&mut self, entry: BootEntry) {
        if entry.is_default || self.entries.is_empty() {
            self.default_entry_id = entry.id.clone();
        }
        self.entries.push(entry);
    }

    pub fn set_default_entry(&mut self, id: &str) -> bool {
        if self.entries.iter().any(|e| e.id == id) {
            self.default_entry_id = id.to_string();
            for e in &mut self.entries {
                e.is_default = e.id == id;
            }
            return true;
        }
        false
    }

    pub fn measure_boot_components(&mut self, stage_bytes: &[u8]) -> [u8; 32] {
        let mut pcr = [0u8; 32];
        for (i, &b) in stage_bytes.iter().enumerate() {
            pcr[i % 32] ^= b.wrapping_add(i as u8);
        }
        self.tpm_pcr_hashes.push(pcr);
        pcr
    }

    pub fn find_root_by_uuid(&self, uuid: &str) -> Option<&BootEntry> {
        self.entries.iter().find(|e| e.cmdline_params.contains(uuid))
    }

    pub fn generate_bootloader_config(&self) -> String {
        let mut cfg = String::new();
        cfg.push_str("# SigmaOS Boot Configuration\n");
        cfg.push_str("timeout ");
        cfg.push_str(&self.timeout_seconds.to_string());
        cfg.push_str("\ndefault ");
        cfg.push_str(&self.default_entry_id);
        cfg.push_str("\n\n");

        for entry in &self.entries {
            cfg.push_str("title ");
            cfg.push_str(&entry.title);
            cfg.push_str("\nkernel ");
            cfg.push_str(&entry.kernel_path);
            cfg.push_str("\noptions ");
            cfg.push_str(&entry.cmdline_params);
            cfg.push_str("\n\n");
        }
        cfg
    }
}

/// Linux & BSD inspired Parallel Fast-Boot Service Pipeline
/// Combines FreeBSD rc.d dependency ordering with Linux systemd socket activation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootServiceState {
    Uninitialized,
    Starting,
    Active,
    Failed,
}

#[derive(Debug, Clone)]
pub struct FastBootService {
    pub name: String,
    pub priority: u32, // Lower value = earlier startup
    pub dependencies: Vec<String>,
    pub state: BootServiceState,
}

pub struct SovereignFastBootServicePipeline {
    pub services: Vec<FastBootService>,
    pub boot_time_ms: u64,
}

impl SovereignFastBootServicePipeline {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            boot_time_ms: 0,
        }
    }

    pub fn register_service(&mut self, name: &str, priority: u32, dependencies: &[&str]) {
        self.services.push(FastBootService {
            name: name.to_string(),
            priority,
            dependencies: dependencies.iter().map(|&s| s.to_string()).collect(),
            state: BootServiceState::Uninitialized,
        });
    }

    /// Parallel boot stage runner executing services according to priority and dependency readiness
    pub fn execute_fast_boot(&mut self) -> Result<u64, &'static str> {
        let mut start_time = 0u64;

        // Sort services by priority
        self.services.sort_by_key(|s| s.priority);

        for i in 0..self.services.len() {
            // Check dependency readiness
            let deps = self.services[i].dependencies.clone();
            for dep in &deps {
                let dep_ready = self.services.iter().any(|s| &s.name == dep && s.state == BootServiceState::Active);
                if !dep_ready {
                    self.services[i].state = BootServiceState::Failed;
                    return Err("FastBoot: Dependency unresolved during boot pipeline execution");
                }
            }

            self.services[i].state = BootServiceState::Active;
            start_time += 15; // Simulated sub-millisecond stage delay
        }

        self.boot_time_ms = start_time;
        Ok(self.boot_time_ms)
    }
}

impl Default for SovereignFastBootServicePipeline {
    fn default() -> Self {
        let mut pipeline = Self::new();
        pipeline.register_service("kernel_vfs", 10, &[]);
        pipeline.register_service("dev_udev", 20, &["kernel_vfs"]);
        pipeline.register_service("network_stack", 30, &["dev_udev"]);
        pipeline.register_service("zenith_desktop", 40, &["network_stack"]);
        pipeline
    }
}

/// Linux & BSD Distro Inspired Kernel Boot Stage Handoff Protocol Engine
/// Supports Linux EFISTUB, Multiboot2, FreeBSD BTX loader ELF handoff, OpenBSD boot.conf,
/// Live ISO overlayfs staging, and Emergency Rescue Console fallback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandoffProtocol {
    LinuxEfiStub,
    Multiboot2,
    FreeBsdBtxElf,
    OpenBsdBootConf,
    LiveIsoOverlayFs,
}

#[derive(Debug, Clone)]
pub struct BootStageDescriptor {
    pub protocol: HandoffProtocol,
    pub kernel_entry_addr: u64,
    pub cmdline: String,
    pub initrd_addr: Option<u64>,
    pub initrd_size: usize,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct OpenBsdBootDirective {
    pub key: String,
    pub value: String,
}

pub struct SovereignDistroBootStageHandoff {
    pub stage_descriptor: Option<BootStageDescriptor>,
    pub openbsd_directives: Vec<OpenBsdBootDirective>,
    pub live_overlay_mounted: bool,
    pub emergency_rescue_active: bool,
    pub last_error_log: Option<String>,
}

impl SovereignDistroBootStageHandoff {
    pub fn new() -> Self {
        Self {
            stage_descriptor: None,
            openbsd_directives: Vec::new(),
            live_overlay_mounted: false,
            emergency_rescue_active: false,
            last_error_log: None,
        }
    }

    /// Prepare Linux EFISTUB or Multiboot2 direct kernel handoff
    pub fn setup_linux_efistub(&mut self, entry_addr: u64, cmdline: &str, initrd_addr: Option<u64>, initrd_size: usize) {
        self.stage_descriptor = Some(BootStageDescriptor {
            protocol: HandoffProtocol::LinuxEfiStub,
            kernel_entry_addr: entry_addr,
            cmdline: cmdline.to_string(),
            initrd_addr,
            initrd_size,
            active: true,
        });
    }

    /// Prepare Multiboot2 kernel handoff
    pub fn setup_multiboot2(&mut self, entry_addr: u64, cmdline: &str) {
        self.stage_descriptor = Some(BootStageDescriptor {
            protocol: HandoffProtocol::Multiboot2,
            kernel_entry_addr: entry_addr,
            cmdline: cmdline.to_string(),
            initrd_addr: None,
            initrd_size: 0,
            active: true,
        });
    }

    /// Prepare FreeBSD BTX loader transition to kernel ELF entry
    pub fn setup_freebsd_btx_elf(&mut self, elf_entry: u64, boot_flags: &str) {
        self.stage_descriptor = Some(BootStageDescriptor {
            protocol: HandoffProtocol::FreeBsdBtxElf,
            kernel_entry_addr: elf_entry,
            cmdline: boot_flags.to_string(),
            initrd_addr: None,
            initrd_size: 0,
            active: true,
        });
    }

    /// Parse OpenBSD style `boot.conf` directives (e.g. `set status`, `stty com0 115200`, `boot hd0a:/bsd.mp`)
    pub fn parse_openbsd_boot_conf(&mut self, conf_content: &str) -> usize {
        let mut count = 0;
        for line in conf_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut parts = line.splitn(2, ' ');
            if let Some(key) = parts.next() {
                let value = parts.next().unwrap_or("").to_string();
                self.openbsd_directives.push(OpenBsdBootDirective {
                    key: key.to_string(),
                    value,
                });
                count += 1;
            }
        }

        self.stage_descriptor = Some(BootStageDescriptor {
            protocol: HandoffProtocol::OpenBsdBootConf,
            kernel_entry_addr: 0x100000,
            cmdline: conf_content.to_string(),
            initrd_addr: None,
            initrd_size: 0,
            active: true,
        });

        count
    }

    /// Stage Live ISO RAM disk / OverlayFS boot environment
    pub fn prepare_live_iso_overlay(&mut self, squashfs_path: &str, overlay_tmpfs_size_mb: usize) -> Result<(), &'static str> {
        if squashfs_path.is_empty() || overlay_tmpfs_size_mb == 0 {
            self.trigger_emergency_rescue("Invalid Live ISO squashfs path or overlay memory limit");
            return Err("Invalid Live ISO parameters");
        }

        self.live_overlay_mounted = true;
        self.stage_descriptor = Some(BootStageDescriptor {
            protocol: HandoffProtocol::LiveIsoOverlayFs,
            kernel_entry_addr: 0x200000,
            cmdline: alloc::format!("boot=live squashfs={} overlay_mb={}", squashfs_path, overlay_tmpfs_size_mb),
            initrd_addr: Some(0x8000000),
            initrd_size: overlay_tmpfs_size_mb * 1024 * 1024,
            active: true,
        });

        Ok(())
    }

    /// Trigger Emergency Rescue Console if boot handoff fails
    pub fn trigger_emergency_rescue(&mut self, reason: &str) {
        self.emergency_rescue_active = true;
        self.last_error_log = Some(reason.to_string());
    }

    /// Execute kernel handoff
    pub fn execute_handoff(&mut self) -> Result<u64, &'static str> {
        if self.emergency_rescue_active {
            return Err("Emergency rescue console active; boot handoff aborted");
        }

        match &self.stage_descriptor {
            Some(desc) if desc.active => Ok(desc.kernel_entry_addr),
            Some(_) => {
                self.trigger_emergency_rescue("Boot stage descriptor inactive");
                Err("Inactive boot stage")
            }
            None => {
                self.trigger_emergency_rescue("No boot stage configured");
                Err("No boot stage configured")
            }
        }
    }
}

impl Default for SovereignDistroBootStageHandoff {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_manager_measured_boot() {
        let mut boot = BootManager::new(5);
        let std_entry = BootEntry {
            id: "sigmaos-main".to_string(),
            title: "SigmaOS 2.0 Sovereign".to_string(),
            kernel_path: "/boot/vmlinuz-sigma.efi".to_string(),
            initrd_path: Some("/boot/initramfs.img".to_string()),
            cmdline_params: "root=UUID=root quiet splash".to_string(),
            is_default: true,
            is_recovery: false,
        };
        boot.add_entry(std_entry);

        assert_eq!(boot.default_entry_id, "sigmaos-main");

        let pcr = boot.measure_boot_components(b"KERNEL_STAGE_1_BINARY");
        assert_ne!(pcr, [0u8; 32]);
        assert_eq!(boot.tpm_pcr_hashes.len(), 1);

        let cfg = boot.generate_bootloader_config();
        assert!(cfg.contains("SigmaOS 2.0 Sovereign"));
    }

    #[test]
    fn test_fast_boot_pipeline() {
        let mut pipeline = SovereignFastBootServicePipeline::default();
        let result = pipeline.execute_fast_boot();
        assert!(result.is_ok());
        assert!(pipeline.boot_time_ms > 0);
        assert_eq!(pipeline.services.iter().filter(|s| s.state == BootServiceState::Active).count(), 4);
    }

    #[test]
    fn test_sovereign_distro_boot_stage_handoff() {
        let mut handoff = SovereignDistroBootStageHandoff::new();

        // 1. Linux EFISTUB & Multiboot2 handoffs
        handoff.setup_linux_efistub(0x1000000, "root=UUID=1234 quiet", Some(0x2000000), 8388608);
        assert_eq!(handoff.execute_handoff().unwrap(), 0x1000000);

        handoff.setup_multiboot2(0x1500000, "sigma_kernel debug");
        assert_eq!(handoff.execute_handoff().unwrap(), 0x1500000);

        // 2. FreeBSD BTX ELF boot handoff
        handoff.setup_freebsd_btx_elf(0x2000000, "-v -s");
        assert_eq!(handoff.execute_handoff().unwrap(), 0x2000000);

        // 3. OpenBSD boot.conf parsing
        let count = handoff.parse_openbsd_boot_conf("stty com0 115200\nset status on\nboot hd0a:/bsd.mp\n");
        assert_eq!(count, 3);
        assert_eq!(handoff.openbsd_directives.len(), 3);
        assert_eq!(handoff.execute_handoff().unwrap(), 0x100000);

        // 4. Live ISO OverlayFS staging
        assert!(handoff.prepare_live_iso_overlay("/live/rootfs.squashfs", 512).is_ok());
        assert!(handoff.live_overlay_mounted);
        assert_eq!(handoff.execute_handoff().unwrap(), 0x200000);

        // 5. Emergency Rescue Console fallback
        handoff.trigger_emergency_rescue("Corrupted initrd checksum");
        assert!(handoff.execute_handoff().is_err());
        assert!(handoff.emergency_rescue_active);
    fn test_boot_stage_handoff_and_root_discovery() {
        let mut boot = BootManager::new(3);
        boot.add_entry(BootEntry {
            id: "sigma-root-uuid".to_string(),
            title: "SigmaOS Main Root".to_string(),
            kernel_path: "/boot/vmlinuz".to_string(),
            initrd_path: Some("/boot/initramfs".to_string()),
            cmdline_params: "root=UUID=1234-5678 ro".to_string(),
            is_default: true,
            is_recovery: false,
        });

        let found = boot.find_root_by_uuid("1234-5678");
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "SigmaOS Main Root");

        let mut handoff = SovereignDistroBootStageHandoff::new(HandoffProtocol::LinuxEfiStub, "1234-5678");
        assert!(!handoff.execute_stage_handoff());

        handoff.mount_initramfs_vfs().unwrap();
        assert!(handoff.is_initramfs_mounted);

        handoff.setup_live_iso_overlayfs().unwrap();
        assert!(handoff.is_overlayfs_active);
        assert!(handoff.execute_stage_handoff());
    }
}
