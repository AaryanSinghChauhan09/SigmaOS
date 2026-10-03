// SigmaOS Universal Subsystem Interoperability Engine
// Provides cross-subsystem protocol bridging, event routing, policy translation,
// and capability matrix evaluation across all Linux & BSD distro innovations.

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// High-level categorization for all SigmaOS subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UniversalSubsystemCategory {
    KernelScheduling,
    SecuritySandboxing,
    FilesystemStorage,
    Networking,
    PackageManagement,
    InitSupervisor,
    DesktopCompositor,
    AudioSound,
    HardwarePower,
    ContainerVirt,
    ObservabilityDiagnostics,
}

impl UniversalSubsystemCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::KernelScheduling => "KernelScheduling",
            Self::SecuritySandboxing => "SecuritySandboxing",
            Self::FilesystemStorage => "FilesystemStorage",
            Self::Networking => "Networking",
            Self::PackageManagement => "PackageManagement",
            Self::InitSupervisor => "InitSupervisor",
            Self::DesktopCompositor => "DesktopCompositor",
            Self::AudioSound => "AudioSound",
            Self::HardwarePower => "HardwarePower",
            Self::ContainerVirt => "ContainerVirt",
            Self::ObservabilityDiagnostics => "ObservabilityDiagnostics",
        }
    }
}

/// Cross-subsystem policy rule defining translation and enforcement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubsystemInteropPolicy {
    pub policy_id: String,
    pub source_subsystem: UniversalSubsystemCategory,
    pub target_subsystem: UniversalSubsystemCategory,
    pub translation_rule: String,
    pub enforce_strict_isolation: bool,
}

impl SubsystemInteropPolicy {
    pub fn new(
        policy_id: &str,
        source: UniversalSubsystemCategory,
        target: UniversalSubsystemCategory,
        rule: &str,
        strict: bool,
    ) -> Self {
        Self {
            policy_id: String::from(policy_id),
            source_subsystem: source,
            target_subsystem: target,
            translation_rule: String::from(rule),
            enforce_strict_isolation: strict,
        }
    }
}

/// Record of an inter-subsystem event dispatched across protocol bridges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubsystemInteropEvent {
    pub event_id: u64,
    pub timestamp: u64,
    pub origin_subsystem: String,
    pub target_subsystem: String,
    pub event_kind: String,
    pub payload: String,
    pub handled: bool,
}

/// Capability metrics and protocol details for a registered subsystem adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubsystemAdapterCapabilities {
    pub category: UniversalSubsystemCategory,
    pub linux_distro_parity: bool,
    pub bsd_distro_parity: bool,
    pub supported_protocols: Vec<String>,
    pub active_driver_count: usize,
}

impl SubsystemAdapterCapabilities {
    pub fn new(
        category: UniversalSubsystemCategory,
        linux_parity: bool,
        bsd_parity: bool,
        protocols: &[&str],
        driver_count: usize,
    ) -> Self {
        Self {
            category,
            linux_distro_parity: linux_parity,
            bsd_distro_parity: bsd_parity,
            supported_protocols: protocols.iter().map(|s| String::from(*s)).collect(),
            active_driver_count: driver_count,
        }
    }
}

/// Universal Subsystem Interoperability Engine for SigmaOS.
/// Integrates all kernel, security, storage, network, packaging, supervisor, UI,
/// audio, power, container, and diagnostic components into a unified cross-distro mesh.
#[derive(Debug, Clone)]
pub struct SovereignUniversalSubsystemInteropEngine {
    pub active_distro_mode: String,
    pub subsystem_adapters: BTreeMap<String, SubsystemAdapterCapabilities>,
    pub interop_policies: Vec<SubsystemInteropPolicy>,
    pub event_log: Vec<SubsystemInteropEvent>,
    pub event_counter: u64,
}

impl SovereignUniversalSubsystemInteropEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            active_distro_mode: String::from("LinuxArch"),
            subsystem_adapters: BTreeMap::new(),
            interop_policies: Vec::new(),
            event_log: Vec::new(),
            event_counter: 0,
        };

        engine.initialize_default_subsystems();
        engine.initialize_default_policies();
        engine
    }

    /// Populate standard SigmaOS subsystems with Linux & BSD distro capabilities.
    fn initialize_default_subsystems(&mut self) {
        self.register_subsystem(
            "kernel",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::KernelScheduling,
                true,
                true,
                &["eevdf", "bore", "sched_ext", "ule", "nuttx_rt"],
                12,
            ),
        );

        self.register_subsystem(
            "security",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::SecuritySandboxing,
                true,
                true,
                &["capsicum", "pledge_unveil", "landlock_v5", "hardenedbsd_pax", "selinux"],
                8,
            ),
        );

        self.register_subsystem(
            "filesystem",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::FilesystemStorage,
                true,
                true,
                &["zfs_arc", "btrfs_snapper", "bcachefs_tiering", "hammer2_pfs", "apfs_cow"],
                10,
            ),
        );

        self.register_subsystem(
            "networking",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::Networking,
                true,
                true,
                &["freebsd_vnet", "ebpf_xdp", "openbsd_pf", "wireguard_pqc", "crossbow_vnic"],
                14,
            ),
        );

        self.register_subsystem(
            "package",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::PackageManagement,
                true,
                true,
                &["alpm_aur", "dpkg_apt", "nix_flakes", "apk3_cas", "xbps", "portage_ebuild", "freebsd_pkg"],
                16,
            ),
        );

        self.register_subsystem(
            "init",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::InitSupervisor,
                true,
                true,
                &["systemd_v258", "openrc", "runit", "chimera_dinit", "guix_shepherd", "illumos_smf"],
                9,
            ),
        );

        self.register_subsystem(
            "desktop",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::DesktopCompositor,
                true,
                true,
                &["hyprland_wayland", "omarchy_omakase", "cosmic_launcher", "solus_raven", "haiku_beapi"],
                11,
            ),
        );

        self.register_subsystem(
            "audio",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::AudioSound,
                true,
                true,
                &["pipewire_alsa_bt", "openbsd_sndio", "pulseaudio_compat"],
                6,
            ),
        );

        self.register_subsystem(
            "power",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::HardwarePower,
                true,
                true,
                &["intel_speedstep", "amd_pstate", "system76_power", "fan_curves"],
                5,
            ),
        );

        self.register_subsystem(
            "container",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::ContainerVirt,
                true,
                true,
                &["firecracker_microvm", "bhyve_jails", "qubes_isolation", "podman_oci"],
                7,
            ),
        );

        self.register_subsystem(
            "observability",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::ObservabilityDiagnostics,
                true,
                true,
                &["dtrace", "ebpf_profiler", "htop_monitor", "opentelemetry_traces"],
                8,
            ),
        );
    }

    /// Establish default cross-subsystem translation and isolation policies.
    fn initialize_default_policies(&mut self) {
        self.add_interop_policy(SubsystemInteropPolicy::new(
            "policy_sched_sec",
            UniversalSubsystemCategory::KernelScheduling,
            UniversalSubsystemCategory::SecuritySandboxing,
            "Propagate task affinity and Landlock v5 capability set during context switch",
            true,
        ));

        self.add_interop_policy(SubsystemInteropPolicy::new(
            "policy_sec_net",
            UniversalSubsystemCategory::SecuritySandboxing,
            UniversalSubsystemCategory::Networking,
            "Enforce Pledge/Unveil network restrictions on socket bind and eBPF XDP hook attach",
            true,
        ));

        self.add_interop_policy(SubsystemInteropPolicy::new(
            "policy_fs_pkg",
            UniversalSubsystemCategory::FilesystemStorage,
            UniversalSubsystemCategory::PackageManagement,
            "Trigger CoW snapshot before ALPM/Dpkg transaction and verify CAS hashes",
            false,
        ));

        self.add_interop_policy(SubsystemInteropPolicy::new(
            "policy_init_desktop",
            UniversalSubsystemCategory::InitSupervisor,
            UniversalSubsystemCategory::DesktopCompositor,
            "Synchronize Dinit/Systemd socket activation with Wayland display compositor readiness",
            false,
        ));

        self.add_interop_policy(SubsystemInteropPolicy::new(
            "policy_power_audio",
            UniversalSubsystemCategory::HardwarePower,
            UniversalSubsystemCategory::AudioSound,
            "Adjust PipeWire buffer latency during AC disconnect or low-power state transition",
            false,
        ));
    }

    pub fn register_subsystem(&mut self, name: &str, adapter: SubsystemAdapterCapabilities) {
        self.subsystem_adapters.insert(String::from(name), adapter);
    }

    pub fn add_interop_policy(&mut self, policy: SubsystemInteropPolicy) {
        self.interop_policies.push(policy);
    }

    pub fn set_active_distro_mode(&mut self, mode: &str) {
        self.active_distro_mode = String::from(mode);
    }

    /// Dispatch an inter-subsystem operation event and record execution.
    pub fn dispatch_event(
        &mut self,
        origin: &str,
        target: &str,
        kind: &str,
        payload: &str,
    ) -> Result<u64, &'static str> {
        if !self.subsystem_adapters.contains_key(origin) {
            return Err("Unknown origin subsystem");
        }
        if !self.subsystem_adapters.contains_key(target) {
            return Err("Unknown target subsystem");
        }

        self.event_counter += 1;
        let event = SubsystemInteropEvent {
            event_id: self.event_counter,
            timestamp: self.event_counter * 1000,
            origin_subsystem: String::from(origin),
            target_subsystem: String::from(target),
            event_kind: String::from(kind),
            payload: String::from(payload),
            handled: true,
        };

        self.event_log.push(event);
        Ok(self.event_counter)
    }

    pub fn translate_policy(
        &self,
        source: UniversalSubsystemCategory,
        target: UniversalSubsystemCategory,
    ) -> Option<&SubsystemInteropPolicy> {
        self.interop_policies
            .iter()
            .find(|p| p.source_subsystem == source && p.target_subsystem == target)
    }

    /// Verify that all registered subsystems maintain full Linux and BSD distro parity.
    pub fn verify_all_subsystems_interoperability(&self) -> bool {
        if self.subsystem_adapters.is_empty() {
            return false;
        }

        self.subsystem_adapters
            .values()
            .all(|adapter| adapter.linux_distro_parity && adapter.bsd_distro_parity && !adapter.supported_protocols.is_empty())
    }

    /// Compute overall ecosystem harmony score (0..100).
    pub fn evaluate_system_wide_harmony_score(&self) -> u32 {
        let total = self.subsystem_adapters.len();
        if total == 0 {
            return 0;
        }

        let compliant_count = self
            .subsystem_adapters
            .values()
            .filter(|a| a.linux_distro_parity && a.bsd_distro_parity)
            .count();

        ((compliant_count as u32) * 100) / (total as u32)
    }

    /// Synchronize all subsystems with a target Linux/BSD distribution mode.
    pub fn sync_all_subsystems_with_distro_innovations(
        &mut self,
        distro_mode: &str,
    ) -> Result<usize, &'static str> {
        self.set_active_distro_mode(distro_mode);

        let mut synced_count = 0;
        let adapter_names: Vec<String> = self.subsystem_adapters.keys().cloned().collect();

        for name in adapter_names {
            self.dispatch_event(
                &name,
                "kernel",
                "sync_distro_mode",
                distro_mode,
            )?;
            synced_count += 1;
        }

        Ok(synced_count)
    }
}

impl Default for SovereignUniversalSubsystemInteropEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsystem_interop_engine_initialization() {
        let engine = SovereignUniversalSubsystemInteropEngine::new();
        assert_eq!(engine.active_distro_mode, "LinuxArch");
        assert!(engine.subsystem_adapters.len() >= 11);
        assert!(engine.interop_policies.len() >= 5);
        assert!(engine.verify_all_subsystems_interoperability());
        assert_eq!(engine.evaluate_system_wide_harmony_score(), 100);
    }

    #[test]
    fn test_dispatch_event_and_policy_translation() {
        let mut engine = SovereignUniversalSubsystemInteropEngine::new();

        let event_id = engine
            .dispatch_event("kernel", "security", "context_switch", "pid=1024")
            .expect("Event dispatch should succeed");
        assert_eq!(event_id, 1);
        assert_eq!(engine.event_log.len(), 1);

        let policy = engine
            .translate_policy(
                UniversalSubsystemCategory::KernelScheduling,
                UniversalSubsystemCategory::SecuritySandboxing,
            )
            .expect("Policy should exist");
        assert_eq!(policy.policy_id, "policy_sched_sec");
        assert!(policy.enforce_strict_isolation);
    }

    #[test]
    fn test_sync_distro_innovations() {
        let mut engine = SovereignUniversalSubsystemInteropEngine::new();

        let synced = engine
            .sync_all_subsystems_with_distro_innovations("FreeBSD14")
            .expect("Sync should succeed");
        assert_eq!(synced, engine.subsystem_adapters.len());
        assert_eq!(engine.active_distro_mode, "FreeBSD14");
        assert_eq!(engine.event_log.len(), engine.subsystem_adapters.len());
    }

    #[test]
    fn test_category_as_str_and_unknown_event_errors() {
        let mut engine = SovereignUniversalSubsystemInteropEngine::new();

        assert_eq!(UniversalSubsystemCategory::KernelScheduling.as_str(), "KernelScheduling");
        assert_eq!(UniversalSubsystemCategory::AudioSound.as_str(), "AudioSound");

        let err_origin = engine.dispatch_event("unknown_sub", "kernel", "test", "");
        assert_eq!(err_origin, Err("Unknown origin subsystem"));

        let err_target = engine.dispatch_event("kernel", "unknown_sub", "test", "");
        assert_eq!(err_target, Err("Unknown target subsystem"));
    }

    #[test]
    fn test_custom_subsystem_and_policy_addition() {
        let mut engine = SovereignUniversalSubsystemInteropEngine::new();

        engine.register_subsystem(
            "custom_ai",
            SubsystemAdapterCapabilities::new(
                UniversalSubsystemCategory::ObservabilityDiagnostics,
                true,
                true,
                &["ai_inference_v1"],
                3,
            ),
        );

        assert!(engine.subsystem_adapters.contains_key("custom_ai"));
        assert!(engine.verify_all_subsystems_interoperability());

        engine.add_interop_policy(SubsystemInteropPolicy::new(
            "policy_ai_obs",
            UniversalSubsystemCategory::ObservabilityDiagnostics,
            UniversalSubsystemCategory::KernelScheduling,
            "Optimize EEVDF weights based on AI inference telemetry",
            false,
        ));

        let pol = engine.translate_policy(
            UniversalSubsystemCategory::ObservabilityDiagnostics,
            UniversalSubsystemCategory::KernelScheduling,
        );
        assert!(pol.is_some());
        assert_eq!(pol.unwrap().policy_id, "policy_ai_obs");
    }
}
