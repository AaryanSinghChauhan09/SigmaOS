// SigmaOS Implementation Plan Master Standalone Test Suite
// Verifies all 11 newly implemented modules across Phase 1, 2, 3, and 4

#[path = "../src/system/snapshot_schedule.rs"]
mod snapshot_schedule;

#[path = "../src/boot/boot_snapshot.rs"]
mod boot_snapshot;

#[path = "../src/network/device_discovery.rs"]
mod device_discovery;

#[path = "../src/plugin/framework.rs"]
mod plugin_framework;

#[path = "../src/plugin/marketplace.rs"]
mod plugin_marketplace;

#[path = "../src/desktop/network_panel.rs"]
mod network_panel;

#[path = "../src/desktop/notifications.rs"]
mod notifications;

#[path = "../src/desktop/text_scaling.rs"]
mod text_scaling;

#[path = "../src/ai/usage_tracker.rs"]
mod usage_tracker;

#[path = "../src/desktop/clipboard_advanced.rs"]
mod clipboard_advanced;

#[path = "../src/productivity/reminders_advanced.rs"]
mod reminders_advanced;

#[test]
fn test_all_11_implementation_plan_modules() {
    // 1. Snapshot Schedule
    let mut sched = snapshot_schedule::ScheduledSnapshotEngine::new();
    let job_id = sched.add_job(
        "Hourly System Backup",
        "rootfs",
        snapshot_schedule::SnapshotFrequency::Hourly,
        snapshot_schedule::RetentionPolicy::default(),
        1700000000,
    );
    assert_eq!(job_id, 1);
    let created = sched.evaluate_and_run_schedules(1700003601);
    assert_eq!(created.len(), 1);

    // 2. Boot Snapshot
    let mut boot = boot_snapshot::BootSnapshotEngine::new(boot_snapshot::BootSnapshotConfig::default());
    let snap = boot.create_preboot_snapshot("Pre-Kernel-Update", 1700000000);
    assert!(boot.verify_boot_integrity(&snap.id));

    // 3. Device Discovery
    let mut disc = device_discovery::DeviceDiscoverySyncEngine::new("host_node", true);
    let peer_id = disc.register_peer(
        "PeerNode",
        device_discovery::DeviceType::Desktop,
        "192.168.1.200",
        "11:22:33:44:55:66",
        &["ssh"],
        device_discovery::DiscoveryProtocol::MDns,
        1700000000,
    );
    assert!(disc.pair_device(&peer_id));

    // 4. Plugin Framework
    let mut plugin_fw = plugin_framework::SovereignPluginFramework::new(5);
    let manifest = plugin_framework::PluginManifest {
        id: "sys_plug".into(),
        name: "SysPlug".into(),
        version: "1.0.0".into(),
        author: "Sigma".into(),
        description: "Desc".into(),
        required_capabilities: vec![plugin_framework::PluginCapability::FileSystemAccess],
        entry_point: "entry.wasm".into(),
    };
    let plug_id = plugin_fw.load_plugin(manifest, vec![plugin_framework::PluginCapability::FileSystemAccess], 64).unwrap();
    assert_eq!(plugin_fw.active_plugins_count(), 1);
    assert!(plugin_fw.unload_plugin(&plug_id));

    // 5. Plugin Marketplace
    let mut market = plugin_marketplace::PluginMarketplaceEngine::new();
    assert!(market.verify_signature("plugin_dock_autohide"));

    // 6. Network Panel GUI
    let mut net_gui = network_panel::NetworkPanelGuiEngine::new();
    assert!(net_gui.connect_wifi("wlan0", "SSID", "pass"));

    // 7. Advanced Notifications
    let mut notif = notifications::AdvancedNotificationEngine::new(10);
    let n_id = notif.send_notification(
        "App", "Title", "Body",
        notifications::NotificationCategory::System,
        notifications::NotificationUrgency::Normal,
        1700000000, &[],
    ).unwrap();
    assert!(notif.mark_as_read(n_id));

    // 8. Text Scaling
    let mut scaling = text_scaling::UnifiedTextScalingEngine::new();
    scaling.set_profile(text_scaling::ScalingProfile::HighDpi2K);
    assert_eq!(scaling.resolve_scale_factor(None), 1.25);

    // 9. AI Usage Tracker
    let mut ai_track = usage_tracker::AiUsageTrackerEngine::default();
    ai_track.record_usage("sess", "ModelX", usage_tracker::ModelInferenceType::TextGeneration, 1000, 500, 100, 1700000000);
    assert_eq!(ai_track.total_tokens_used(), 1500);

    // 10. Advanced Clipboard
    let mut clip = clipboard_advanced::AdvancedClipboardEngine::new("dev", 10);
    let c_id = clip.push_clip(clipboard_advanced::ClipboardContentType::Text, b"hello", "hello", clipboard_advanced::ClipboardCategory::Work, 1700000000);
    assert_eq!(clip.search_clips("hello").len(), 1);
    assert!(clip.toggle_pin(c_id));

    // 11. Enhanced Reminders
    let mut rem = reminders_advanced::EnhancedRemindersEngine::new();
    let r_id = rem.add_reminder("Task", "Desc", "Cat", reminders_advanced::ReminderPriority::High, 1700000000, reminders_advanced::RecurrencePattern::Once, &["tag"]);
    assert_eq!(rem.filter_by_tag("tag").len(), 1);
    assert!(rem.mark_completed(r_id, 1700000000));
}
