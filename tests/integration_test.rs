// SigmaOS Integration Tests
// Verifies core system legacy compatibility, multi-persona VMs, and driver bridge layers
#![allow(unused, clippy::all)]

use sigmaos::sigpkg::universal_adapter::{ApkAdapter, EbuildAdapter, NixAdapter, PackageFormatAdapter};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_distro_packaging_compatibility() {
        let apk = ApkAdapter::new();
        assert_eq!(apk.format_name(), "apk");

        let nix = NixAdapter::new();
        assert_eq!(nix.format_name(), "nix");

        let ebuild = EbuildAdapter::new();
        assert_eq!(ebuild.format_name(), "ebuild");
    }

    #[test]
    fn test_process_signals_integration() {
        use sigmaos::runtime::process::{Process, ProcessSignal, ProcessCapability};

        let cap = ProcessCapability::full();
        let process = unsafe { Process::new(10, 1, cap) };

        assert!(!process.consume_pending_signal(ProcessSignal::SigKill));
        process.send_signal(ProcessSignal::SigKill);
        assert!(process.consume_pending_signal(ProcessSignal::SigKill));
        assert!(!process.consume_pending_signal(ProcessSignal::SigKill));
    }

    #[test]
    fn test_supervised_service_targets() {
        use sigmaos::runtime::process::{Process, ProcessState, ProcessCapability, SupervisedServiceTarget};

        let cap = ProcessCapability::full();
        let process = unsafe { Process::new(11, 1, cap) };
        let mut supervisor = SupervisedServiceTarget::new(11);

        assert!(!unsafe { supervisor.monitor_and_supervise(&process) });

        process.set_state(ProcessState::Terminated);
        assert!(unsafe { supervisor.monitor_and_supervise(&process) });
        assert!(supervisor.auto_respawn_triggered);
        assert_eq!(supervisor.restart_count, 1);
        assert_eq!(process.get_state(), ProcessState::Running);
    }

}
