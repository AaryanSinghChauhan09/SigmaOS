pub mod init_abstraction;
pub mod runit;
pub mod s6;

pub use init_abstraction::*;
pub use runit::*;
pub use s6::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_abstraction_and_runit() {
        let mut runit = RunitInit::new();
        runit.register_service("sshd", "/etc/runit/sshd/run");

        assert_eq!(runit.init_type(), InitSystemType::Runit);
        assert_eq!(runit.service_status("sshd"), ServiceStatus::Stopped);

        runit.start_service("sshd").expect("Start sshd");
        assert_eq!(runit.service_status("sshd"), ServiceStatus::Running);

        runit.stop_service("sshd").expect("Stop sshd");
        assert_eq!(runit.service_status("sshd"), ServiceStatus::Stopped);
    }

    #[test]
    fn test_s6_init() {
        let mut s6 = S6Init::new();
        s6.register_service("networkd");

        assert_eq!(s6.init_type(), InitSystemType::S6);
        s6.start_service("networkd").expect("Start networkd");
        assert_eq!(s6.service_status("networkd"), ServiceStatus::Running);

        s6.notify_ready("networkd");
        assert!(s6.services.get("networkd").unwrap().ready_notification);
    }
}
