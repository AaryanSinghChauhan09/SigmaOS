//! Integration Tests for Sovereign Device Management Advancements (V81) PR Suite
//!
//! Validates:
//! - PR proposals 1045 - 1050 completeness and correct manifests
//! - udev / eudev / mdev / devd / devfs / hotplugd / drvctl integration and predictable network naming v4
//! - acpid multi-zone thermal aggregation, RAPL energy tracking & power event parsing
//! - UPower battery metrics, health SoH, dual-battery composite estimation & dynamic power profiles
//! - logind / elogind session seating, DRM leasing, idle lock, device FD passing & inhibitors
//! - polkit authorization policy matching, allowed groups & TTL cache with revocation
//! - D-Bus system broker message handling, name ownership, service activation, signal routing & XML introspection

#[path = "../src/device/sovereign_device_management_advancements_v81_pr.rs"]
mod sovereign_device_management_advancements_v81_pr;

use sovereign_device_management_advancements_v81_pr::*;
use std::collections::BTreeMap;

#[test]
fn test_v81_pr_manifests_integrity() {
    let gateway = SovereignDeviceManagementAdvancementsV81PrGateway::new();
    let proposals = gateway.collect_all_proposals();

    assert_eq!(proposals.len(), 6);
    let expected_prs = [1045, 1046, 1047, 1048, 1049, 1050];

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
fn test_v81_udev_mdev_devd_rules() {
    let mut engine = SovereignUdevEudevMdevDevdHotplugV81Engine::new();

    let uevent = KernelUeventV81Record {
        action: DynamicDeviceActionV81::Add,
        devpath: "/devices/pci0000:00/0000:00:01.0/block/nvme0n1".to_string(),
        subsystem: "block".to_string(),
        devname: "nvme0n1".to_string(),
        seqnum: 5432,
        env_vars: BTreeMap::new(),
    };

    let (symlinks, mode) = engine.process_uevent(&uevent);
    assert_eq!(mode, Some(0o660));
    assert!(symlinks.contains(&"disk/by-id/nvme-Sovereign_SSD_V81".to_string()));
    assert!(symlinks.contains(&"disk/by-path/pci-0000:00:01.0-nvme-1".to_string()));

    let ifname =
        engine.generate_predictable_net_ifname(PredictableNetIfTypeV81::Wireless, 2, 0, 0, None);
    assert_eq!(ifname, "wlp2s0");

    let usb_ifname =
        engine.generate_predictable_net_ifname(PredictableNetIfTypeV81::UsbPort, 0, 1, 0, None);
    assert_eq!(usb_ifname, "enp0s0u1");

    let sriov_ifname = engine.generate_predictable_net_ifname(
        PredictableNetIfTypeV81::SriovVirtualFunction,
        1,
        2,
        0,
        Some(3),
    );
    assert_eq!(sriov_ifname, "enp1s2f0v3");

    let thunderbolt_ifname = engine.generate_predictable_net_ifname(
        PredictableNetIfTypeV81::ThunderboltUsb4,
        0,
        4,
        0,
        None,
    );
    assert_eq!(thunderbolt_ifname, "thb0s4");

    let nvdimm_ifname = engine.generate_predictable_net_ifname(
        PredictableNetIfTypeV81::NvdimmPersistentMemory,
        0,
        1,
        0,
        None,
    );
    assert_eq!(nvdimm_ifname, "pmem0s1");

    let hwdb = engine.lookup_hwdb("pci:v00008086d000015f4");
    assert!(hwdb.is_some());
    let props = hwdb.unwrap();
    assert_eq!(
        props.get("ID_VENDOR_FROM_DATABASE").map(|s| s.as_str()),
        Some("SigmaDeviceCorp")
    );

    assert!(engine.drvctl_detach_node("wm0"));
    assert!(!engine.drvctl_detach_node("pci0"));
}

#[test]
fn test_v81_acpid_power_governor() {
    let mut acpid = SovereignAcpidPowerDaemonV81Engine::new();
    let evt = acpid.parse_acpi_event_line("button/power PWRF 00000080 00000000");
    assert!(evt.is_some());
    assert!(acpid.dispatch_event(&evt.unwrap(), 1000));

    let fan_speed = acpid.evaluate_pwm_fan_speed();
    assert_eq!(fan_speed, 80);

    acpid.record_rapl_energy(10000);
    assert_eq!(acpid.rapl_energy_microjoules, 1_260_000);

    let sleep_transition = acpid.transition_sleep_state(AcpiSleepStateV81::S0ixModernStandby);
    assert_eq!(sleep_transition, Ok(AcpiSleepStateV81::S0ixModernStandby));
}

#[test]
fn test_v81_upower_battery_estimation() {
    let upower = SovereignUpowerBatteryDaemonV81Engine::new();
    let health = upower.calculate_battery_health("BAT0");
    assert_eq!(health, Some(90));

    let time_left = upower.calculate_time_remaining_seconds("BAT0");
    assert_eq!(time_left, Some(10800)); // 27000 * 3600 / 9000 = 10800s (3 hours)

    let composite = upower.calculate_composite_battery_percent();
    assert!(composite.is_some());
    // (27000 + 20000) * 100 / (54000 + 28500) = 4700000 / 82500 = 56%
    assert_eq!(composite, Some(56));

    let profile = upower.evaluate_power_profile_hint();
    assert_eq!(profile, PowerProfileV81::Balanced);
}

#[test]
fn test_v81_logind_session_lifecycle() {
    let mut logind = SovereignLogindElogindSessionV81Engine::new();
    let fd = logind.take_device_fd("session-1", "/dev/dri/card0", 1000);
    assert_eq!(fd, Ok(84));

    let lease_fd = logind.acquire_drm_lease("session-1");
    assert_eq!(lease_fd, Ok(101));

    let lock_id = logind.create_inhibitor_lock(
        "shutdown",
        "updater",
        "System upgrade",
        InhibitorModeV81::Delay,
        120,
    );
    assert_eq!(lock_id, 1);
    assert_eq!(logind.inhibitor_locks.len(), 1);

    assert!(logind.release_inhibitor_lock(lock_id));
    assert_eq!(logind.inhibitor_locks.len(), 0);

    assert!(logind.update_session_idle("session-1", 400));
    assert!(logind.sessions.get("session-1").unwrap().is_locked);

    assert!(logind.switch_vt(2));
    assert_eq!(
        logind.sessions.get("session-1").map(|s| s.is_active),
        Some(false)
    );
}

#[test]
fn test_v81_polkit_action_cache() {
    let mut polkit = SovereignPolkitAuthorizationV81Engine::new();
    let subject = PolkitSubjectV81 {
        pid: 1234,
        uid: 1000,
        session_id: "session-1".to_string(),
        groups: vec!["wheel".to_string(), "storage".to_string()],
    };

    let result =
        polkit.check_authorization("org.freedesktop.udisks2.filesystem-mount", &subject, true);
    assert_eq!(result, PolkitAuthResultV81::Yes);

    polkit.cache_authorization("org.freedesktop.udisks2.filesystem-mount", 1000, 5000, 300);
    assert!(polkit.is_cached_authorization_valid(
        "org.freedesktop.udisks2.filesystem-mount",
        1000,
        5200
    ));
    assert!(!polkit.is_cached_authorization_valid(
        "org.freedesktop.udisks2.filesystem-mount",
        1000,
        5301
    ));

    polkit.revoke_authorization("org.freedesktop.udisks2.filesystem-mount", 1000);
    assert!(!polkit.is_cached_authorization_valid(
        "org.freedesktop.udisks2.filesystem-mount",
        1000,
        5200
    ));
}

#[test]
fn test_v81_dbus_broker_signals() {
    let mut broker = SovereignDbusSystemBusBrokerV81Engine::new();
    let req = broker.request_name("org.freedesktop.UPower", ":1.12", true, false);
    assert!(req);

    let activation = broker.activate_service("org.freedesktop.login1");
    assert_eq!(
        activation,
        Ok("/usr/lib/systemd/systemd-logind".to_string())
    );

    broker.subscribe_signal("org.freedesktop.UPower", ":1.99");
    let routes = broker.route_signal("org.freedesktop.UPower");
    assert_eq!(routes, vec![":1.99".to_string()]);

    let xml = broker.generate_introspection_xml("org.freedesktop.UPower");
    assert!(xml.contains("GetStatus"));
    assert!(xml.contains("org.freedesktop.DBus.Introspectable"));
}

#[test]
fn test_v81_gateway_master_verification() {
    let mut gateway = SovereignDeviceManagementAdvancementsV81PrGateway::new();
    assert!(gateway.run_master_verification());
}
