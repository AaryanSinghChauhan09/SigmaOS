//! Integration test suite for Sovereign Init Systems & Supervisors Advancements V92 PR.

#[path = "../src/system/sovereign_init_and_supervisor_advancements_v92_pr.rs"]
mod sovereign_init_and_supervisor_advancements_v92_pr;

use sovereign_init_and_supervisor_advancements_v92_pr::*;
extern crate alloc;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::ToString;
use alloc::vec;

#[test]
fn test_init_and_supervisor_v92_pr_integration() {
    let master_suite = SovereignInitAndSupervisorAdvancementsV92Suite::new();
    let manifests = master_suite.collect_all_manifests();

    assert_eq!(manifests.len(), 7);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 9201 && manifest.pr_id <= 9207);
        assert!(!manifest.pr_title.is_empty());
        assert_eq!(manifest.status, "Ready for Integration");
    }

    // 1. Verify systemd V92 state machine, freezer & cgroup v2 & socket activation & journal
    let systemd = SystemdAdvancementsV92Engine::new();
    let state = systemd.transition_unit_state(SystemdUnitStateV92::Inactive, "start");
    assert_eq!(state, SystemdUnitStateV92::Activating);
    let state_active = systemd.transition_unit_state(state, "started");
    let state_freezing = systemd.transition_unit_state(state_active, "freeze");
    assert_eq!(state_freezing, SystemdUnitStateV92::Freezing);
    let state_frozen = systemd.transition_unit_state(state_freezing, "frozen");
    assert_eq!(state_frozen, SystemdUnitStateV92::Frozen);
    let state_thawing = systemd.transition_unit_state(state_frozen, "thaw");
    assert_eq!(state_thawing, SystemdUnitStateV92::Thawing);
    let state_resumed = systemd.transition_unit_state(state_thawing, "active");
    assert_eq!(state_resumed, SystemdUnitStateV92::Active);

    let limits = SystemdCgroupV2LimitsV92 {
        memory_max_bytes: Some(8192),
        memory_high_bytes: Some(4096),
        cpu_quota_percent: Some(200),
        tasks_max: Some(500),
        io_weight: Some(150),
        io_read_iops_max: Some(10000),
        io_write_iops_max: Some(5000),
    };
    let cgroup_params = systemd.build_cgroup_v2_control_parameters(&limits);
    assert_eq!(cgroup_params.get("memory.max"), Some(&"8192".to_string()));
    assert_eq!(cgroup_params.get("memory.high"), Some(&"4096".to_string()));
    assert_eq!(cgroup_params.get("cpu.max"), Some(&"200000 100000".to_string()));
    assert_eq!(cgroup_params.get("pids.max"), Some(&"500".to_string()));
    assert_eq!(cgroup_params.get("io.weight"), Some(&"150".to_string()));
    assert_eq!(cgroup_params.get("io.max"), Some(&"riops=10000".to_string()));
    assert_eq!(cgroup_params.get("io.max_write"), Some(&"wiops=5000".to_string()));

    let socket_spec = SystemdSocketActivationSpecV92 {
        socket_name: "test.socket".to_string(),
        listen_stream: "/run/test.sock".to_string(),
        pass_fds_count: 2,
        service_to_trigger: "test.service".to_string(),
        reuse_port: true,
        non_blocking: true,
    };
    let env_map = systemd.evaluate_socket_activation_env(&socket_spec, 4001);
    assert_eq!(env_map.get("LISTEN_FDS"), Some(&"2".to_string()));
    assert_eq!(env_map.get("LISTEN_PID"), Some(&"4001".to_string()));
    assert_eq!(env_map.get("LISTEN_FDNAMES"), Some(&"test.socket".to_string()));
    assert_eq!(env_map.get("LISTEN_NONBLOCK"), Some(&"1".to_string()));

    let journal_entry = systemd.format_sd_journal_record(
        "api-worker",
        6,
        "Worker thread started successfully",
        "boot-v92-id",
        1700000000,
    );
    assert_eq!(journal_entry.unit, "api-worker");
    assert_eq!(journal_entry.priority, 6);
    assert!(journal_entry.cursor.starts_with("s="));
    assert_eq!(journal_entry.fss_hmac, format!("{:016x}:{:08x}", 1700000000u64 ^ 0x5a5a5a5a5a5a5a5a, 6u32));

    // Dependency check
    let deps = SystemdDependencySpecV92 {
        requires: vec!["db.service".to_string()],
        ..Default::default()
    };
    let mut active_set = BTreeSet::new();
    let (startable_without_db, _) = systemd.evaluate_unit_dependencies_startable("app.service", &deps, &active_set);
    assert!(!startable_without_db);
    active_set.insert("db.service".to_string());
    let (startable_with_db, _) = systemd.evaluate_unit_dependencies_startable("app.service", &deps, &active_set);
    assert!(startable_with_db);

    // 2. Verify OpenRC V92 cascading dependency solver & conf overrides & rc-status
    let openrc = OpenRcAdvancementsV92Engine::new();
    let rc_conf = "rc_parallel=\"YES\"\nrc_interactive=\"NO\"\n";
    let directives = openrc.parse_rc_conf(rc_conf);
    assert_eq!(directives.get("rc_parallel"), Some(&"YES".to_string()));

    let mut overrides = BTreeMap::new();
    overrides.insert("rc_need".to_string(), "localmount".to_string());
    let services = vec![
        OpenRcServiceSpecV92 {
            name: "fs".to_string(),
            runlevel: "boot".to_string(),
            dependencies: vec![],
            conf_overrides: BTreeMap::new(),
        },
        OpenRcServiceSpecV92 {
            name: "network".to_string(),
            runlevel: "default".to_string(),
            dependencies: vec![(OpenRcDepKindV92::Need, "fs".to_string())],
            conf_overrides: overrides,
        },
    ];
    let order = openrc.solve_runlevel_execution_order(&services).unwrap();
    assert_eq!(order, vec!["fs", "network"]);

    let mut status_map = BTreeMap::new();
    status_map.insert("network".to_string(), true);
    status_map.insert("cron".to_string(), false);
    let report = openrc.format_rc_status_report("default", &status_map);
    assert!(report.contains("Runlevel: default"));
    assert!(report.contains("[ started ]"));
    assert!(report.contains("network"));
    assert!(report.contains("[ stopped ]"));
    assert!(report.contains("cron"));

    // 3. Verify runit & s6 V92 active scanner, readiness notifications & backoff
    let runit_s6 = RunitS6AdvancementsV92Engine::new();
    let scanned = runit_s6.scan_service_dir(&[".hidden", "cron", "ntp", "ssh"]);
    assert_eq!(scanned, vec!["cron", "ntp", "ssh"]);

    let readiness_spec = S6ServiceReadinessSpecV92 {
        service_name: "logger".to_string(),
        notify_fd: Some(3),
        check_script: None,
        timeout_ms: 3000,
        max_restart_attempts: 10,
    };
    assert!(runit_s6.evaluate_readiness_notification(&readiness_spec, Some(3), false));
    assert!(!runit_s6.evaluate_readiness_notification(&readiness_spec, Some(4), false));
    assert_eq!(runit_s6.calculate_exponential_backoff_delay_ms(3, 100, 1000), 400);

    let transition_plan = runit_s6.compile_s6_rc_transition("boot", "default");
    assert_eq!(transition_plan, "s6-rc -a change boot -> default");

    // 4. Verify SysVinit V92 /etc/inittab, telinit & priority ordering
    let sysv = SysVinitAdvancementsV92Engine::new();
    let line = "l2:2:wait:/etc/init.d/rc 2";
    let entry = sysv.parse_inittab_line(line).unwrap();
    assert_eq!(entry.id, "l2");
    assert_eq!(entry.action, SysVinitActionV92::Wait);
    assert_eq!(entry.runlevels, vec!['2']);

    let active_specs = sysv.evaluate_runlevel_switch(&[entry], '2');
    assert_eq!(active_specs.len(), 1);

    let script_order = sysv.sort_sysv_rc_scripts(&["K90syslog", "S01network", "S20sshd", "K10db"]);
    assert_eq!(script_order, vec!["S01network", "K10db", "S20sshd", "K90syslog"]);

    // 5. Verify BSD rc (FreeBSD, OpenBSD, NetBSD V92) hooks & jail command
    let bsd = BsdRcAdvancementsV92Engine::new();
    let rc_content = "name=\"unbound\"\nrcvar=\"unbound_enable\"\ncommand=\"/usr/sbin/unbound\"\nstart_precmd=\"unbound_check\"\nflags=\"-c /etc/unbound/unbound.conf\"\njail_name=\"dns_jail\"\n";
    let hooks = bsd.parse_rc_d_script_hooks(rc_content);
    assert_eq!(hooks.name, "unbound");
    assert_eq!(hooks.command, "/usr/sbin/unbound");
    assert_eq!(hooks.jail_name, Some("dns_jail".to_string()));
    assert_eq!(bsd.build_jail_chroot_command(&hooks), "jail -n dns_jail exec /usr/sbin/unbound");

    // 6. Verify launchd & Android init V92 keep-alive, triggers & SELinux
    let launchd_android = LaunchdAndroidInitAdvancementsV92Engine::new();
    let mut path_map = BTreeMap::new();
    path_map.insert("/tmp/active.flag".to_string(), true);
    let keep_alive = LaunchdKeepAliveSpecV92 {
        path_state: path_map,
        other_job_enabled: BTreeMap::new(),
        successful_exit: Some(true),
        crashed: None,
        network_state: None,
    };
    let mut existing_paths = BTreeSet::new();
    existing_paths.insert("/tmp/active.flag".to_string());
    assert!(launchd_android.evaluate_keep_alive_conditions(&keep_alive, &existing_paths, &BTreeSet::new(), true));
    assert!(!launchd_android.evaluate_keep_alive_conditions(&keep_alive, &BTreeSet::new(), &BTreeSet::new(), true));

    let trigger = launchd_android.parse_android_trigger_condition("on boot");
    assert_eq!(trigger, Some(("event".to_string(), "boot".to_string())));

    assert!(launchd_android.evaluate_selinux_domain_policy("u:r:init:s0", "init"));
    assert!(!launchd_android.evaluate_selinux_domain_policy("u:r:untrusted_app:s0", "system_server"));

    // 7. Verify Cron, Anacron, Timers & At V92 calendar solver & load thresholds
    let cron_timers = CronAnacronTimersAtV92Engine::new();
    let parsed_cron = cron_timers.parse_crontab_expression("*/15 0-23 * * *").unwrap();
    assert_eq!(parsed_cron.len(), 5);

    let timer = SystemdTimerSpecV92 {
        on_calendar: Some("daily".to_string()),
        on_boot_sec: Some(30),
        on_unit_active_sec: None,
        persistent: true,
        accuracy_sec: 1,
        randomized_delay_sec: 0,
    };
    assert!(cron_timers.evaluate_timer_readiness(&timer, 100, 0, 0));
    assert!(!cron_timers.evaluate_timer_readiness(&timer, 20, 0, 0));

    assert!(cron_timers.evaluate_batch_load_average(0.65, 0.80));
    assert!(!cron_timers.evaluate_batch_load_average(0.85, 0.80));
}
