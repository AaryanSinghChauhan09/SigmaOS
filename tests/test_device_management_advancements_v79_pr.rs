//! Integration Tests for Sovereign Device Management Advancements (V79) PR Suite
//!
//! Validates:
//! - PR proposals 1033 - 1038 completeness and correct manifests
//! - udev / eudev / mdev / devd / devfs / hotplugd / drvctl integration and predictable network naming
//! - acpid thermal PWM fan governor & power event parsing
//! - UPower battery metrics, health, discharge estimation & power profiles
//! - logind / elogind session seating, VT switching, device FD passing & inhibitors
//! - polkit authorization policy matching & TTL cache
//! - D-Bus system broker message handling, name ownership, signal routing & XML introspection

#[path = "../src/device/sovereign_device_management_advancements_v79_pr.rs"]
mod sovereign_device_management_advancements_v79_pr;

use sovereign_device_management_advancements_v79_pr::*;
use std::collections::BTreeMap;

#[test]
fn test_v79_pr_manifests_integrity() {
    let gateway = SovereignDeviceManagementAdvancementsV79PrGateway::new();
    let proposals = gateway.collect_all_proposals();

    assert_eq!(proposals.len(), 6);
    let expected_prs = [1033, 1034, 1035, 1036, 1037, 1038];

    for (idx, manifest) in proposals.iter().enumerate() {
        assert_eq!(manifest.pr_id, expected_prs[idx]);
        assert!(!manifest.pr_title.is_empty());
        assert!(!manifest.target_subsystem.is_empty());
        assert!(!manifest.original_distros.is_empty());
        assert!(!manifest.components_added.is_empty());
        assert_eq!(manifest.status, "APPROVED_AND_READY");
    }
}

#[test]
fn test_v79_udev_mdev_devd_rules() {
    let engine = SovereignUdevEudevMdevDevdHotplugV79Engine::new();

    let uevent = KernelUeventV79Record {
        action: DynamicDeviceActionV79::Add,
        devpath: "/devices/pci0000:00/0000:00:01.0/block/nvme0n1".to_string(),
        subsystem: "block".to_string(),
        devname: "nvme0n1".to_string(),
        seqnum: 5432,
        env_vars: BTreeMap::new(),
    };

    let (symlinks, mode) = engine.process_uevent(&uevent);
    assert_eq!(mode, Some(0o660));
    assert!(symlinks.contains(&"disk/by-id/nvme-Sovereign_SSD".to_string()));

    let ifname = engine.generate_predictable_net_ifname(PredictableNetIfTypeV79::Wireless, 2, 0, 0);
    assert_eq!(ifname, "wlp2s0");

    let usb_ifname = engine.generate_predictable_net_ifname(PredictableNetIfTypeV79::UsbPort, 0, 1, 0);
    assert_eq!(usb_ifname, "enp0s0u1");

    let hwdb = engine.lookup_hwdb("pci:v00008086d000015f3");
    assert!(hwdb.is_some());
    let props = hwdb.unwrap();
    assert_eq!(props.get("ID_VENDOR_FROM_DATABASE").map(|s| s.as_str()), Some("SigmaCorp"));
}

#[test]
fn test_v79_acpid_power_governor() {
    let mut acpid = SovereignAcpidPowerDaemonV79Engine::new();
    let evt = acpid.parse_acpi_event_line("button/power PWRF 00000080 00000000");
    assert!(evt.is_some());
    assert!(acpid.dispatch_event(&evt.unwrap()));

    let fan_cold = acpid.evaluate_pwm_fan_speed(30);
    assert_eq!(fan_cold, 0);

    let fan_warm = acpid.evaluate_pwm_fan_speed(55);
    assert_eq!(fan_warm, 100);

    let fan_hot = acpid.evaluate_pwm_fan_speed(75);
    assert_eq!(fan_hot, 180);

    let fan_crit = acpid.evaluate_pwm_fan_speed(90);
    assert_eq!(fan_crit, 255);
}

#[test]
fn test_v79_upower_battery_estimation() {
    let upower = SovereignUpowerBatteryDaemonV79Engine::new();
    let health = upower.calculate_battery_health("BAT0");
    assert_eq!(health, Some(90));

    let time_left = upower.calculate_time_remaining_seconds("BAT0");
    assert_eq!(time_left, Some(10800)); // 22500 * 3600 / 7500 = 10800s (3 hours)

    let profile = upower.evaluate_power_profile_hint();
    assert_eq!(profile, PowerProfileV79::Performance);
}

#[test]
fn test_v79_logind_session_lifecycle() {
    let mut logind = SovereignLogindElogindSessionV79Engine::new();
    let fd = logind.take_device_fd("session-1", "/dev/dri/card0", 1000);
    assert_eq!(fd, Ok(42));

    let lock_id = logind.create_inhibitor_lock("shutdown", "updater", "System upgrade", InhibitorModeV79::Delay);
    assert_eq!(lock_id, 1);
    assert_eq!(logind.inhibitor_locks.len(), 1);

    assert!(logind.release_inhibitor_lock(lock_id));
    assert_eq!(logind.inhibitor_locks.len(), 0);

    assert!(logind.switch_vt(2));
    assert_eq!(logind.sessions.get("session-1").map(|s| s.is_active), Some(false));
}

#[test]
fn test_v79_polkit_action_cache() {
    let mut polkit = SovereignPolkitAuthorizationV79Engine::new();
    let subject = PolkitSubjectV79 {
        pid: 1234,
        uid: 1000,
        session_id: "session-1".to_string(),
    };

    let result = polkit.check_authorization("org.freedesktop.udisks2.filesystem-mount", &subject, true);
    assert_eq!(result, PolkitAuthResultV79::Yes);

    polkit.cache_authorization("org.freedesktop.udisks2.filesystem-mount", 1000, 5000, 300);
    assert!(polkit.is_cached_authorization_valid("org.freedesktop.udisks2.filesystem-mount", 1000, 5200));
    assert!(!polkit.is_cached_authorization_valid("org.freedesktop.udisks2.filesystem-mount", 1000, 5301));
}

#[test]
fn test_v79_dbus_broker_signals() {
    let mut broker = SovereignDbusSystemBusBrokerV79Engine::new();
    let req = broker.request_name("org.freedesktop.UPower", ":1.12", true);
    assert!(req);

    broker.subscribe_signal("org.freedesktop.UPower", ":1.99");
    let routes = broker.route_signal("org.freedesktop.UPower");
    assert_eq!(routes, vec![":1.99".to_string()]);

    let xml = broker.generate_introspection_xml("org.freedesktop.UPower");
    assert!(xml.contains("GetStatus"));
}

#[test]
fn test_v79_gateway_master_verification() {
    let mut gateway = SovereignDeviceManagementAdvancementsV79PrGateway::new();
    assert!(gateway.run_master_verification());
}
