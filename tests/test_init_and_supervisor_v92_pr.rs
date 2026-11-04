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
    let gateway = SovereignInitAndSupervisorV92PrGatewayEngine::new();
    let sig = [0x5au8; 64];
    let manifests = gateway.orchestrate_core_userland_v92(&sig).unwrap();

    assert_eq!(manifests.len(), 7);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 9200 && manifest.pr_id <= 9206);
        assert!(!manifest.pr_title.is_empty());
        assert_eq!(manifest.status, "Ready for Integration");
    }

    // 1. Verify systemd V92 engine
    let systemd = SystemdAdvancementsV92Engine::new();
    let mut state = SystemdUnitStateV92::Inactive;
    state = systemd.transition_unit_state(state, "start");
    assert_eq!(state, SystemdUnitStateV92::Activating);
    state = systemd.transition_unit_state(state, "started");
    assert_eq!(state, SystemdUnitStateV92::Active);
    state = systemd.transition_unit_state(state, "freeze");
    assert_eq!(state, SystemdUnitStateV92::Freezing);
    state = systemd.transition_unit_state(state, "frozen");
    assert_eq!(state, SystemdUnitStateV92::Frozen);
    state = systemd.transition_unit_state(state, "thaw");
    assert_eq!(state, SystemdUnitStateV92::Thawing);
    state = systemd.transition_unit_state(state, "active");
    assert_eq!(state, SystemdUnitStateV92::Active);

    let limits = SystemdCgroupV2LimitsV92 {
        memory_max_bytes: Some(1024 * 1024 * 512),
        memory_high_bytes: Some(1024 * 1024 * 384),
        cpu_quota_percent: Some(250),
        tasks_max: Some(500),
        io_weight: Some(150),
        io_read_iops_max: Some(10000),
        io_write_iops_max: Some(5000),
    };
    let cgroup_params = systemd.build_cgroup_v2_control_parameters(&limits);
    assert_eq!(
        cgroup_params.get("memory.max"),
        Some(&"536870912".to_string())
    );
    assert_eq!(
        cgroup_params.get("memory.high"),
        Some(&"402653184".to_string())
    );
    assert_eq!(
        cgroup_params.get("cpu.max"),
        Some(&"250000 100000".to_string())
    );
    assert_eq!(cgroup_params.get("pids.max"), Some(&"500".to_string()));
    assert_eq!(cgroup_params.get("io.weight"), Some(&"150".to_string()));
    assert_eq!(
        cgroup_params.get("io.max"),
        Some(&"riops=10000".to_string())
    );
    assert_eq!(
        cgroup_params.get("io.max_write"),
        Some(&"wiops=5000".to_string())
    );

    let sock_spec = SystemdSocketActivationSpecV92 {
        socket_name: "sigma-http.socket".to_string(),
        listen_stream: "/run/sigma/http.sock".to_string(),
        pass_fds_count: 2,
        service_to_trigger: "sigma-http.service".to_string(),
        reuse_port: true,
        non_blocking: true,
    };
    let sock_env = systemd.evaluate_socket_activation_env(&sock_spec, 1000);
    assert_eq!(sock_env.get("LISTEN_PID"), Some(&"1000".to_string()));
    assert_eq!(sock_env.get("LISTEN_FDS"), Some(&"2".to_string()));
    assert_eq!(
        sock_env.get("LISTEN_FDNAMES"),
        Some(&"sigma-http.socket".to_string())
    );

    let journal = systemd.format_sd_journal_record(
        "systemd-journald.service",
        6,
        "Journal started",
        "boot-92",
        1000000,
    );
    assert_eq!(journal.unit, "systemd-journald.service");
    assert!(journal.cursor.contains("boot-92"));
    assert!(!journal.fss_hmac.is_empty());

    let mut active_units = BTreeSet::new();
    active_units.insert("network.target".to_string());
    active_units.insert("syslog.socket".to_string());
    let deps = SystemdDependencySpecV92 {
        requires: vec!["network.target".to_string()],
        wants: vec!["syslog.socket".to_string()],
        binds_to: vec![],
        part_of: vec![],
        conflicts: vec!["shutdown.target".to_string()],
        before: vec![],
        after: vec!["network.target".to_string()],
    };
    let (can_start, missing) =
        systemd.evaluate_unit_dependencies_startable("app.service", &deps, &active_units);
    assert!(can_start);
    assert!(missing.is_empty());

    // 2. Verify OpenRC V92 engine
    let openrc = OpenRcAdvancementsV92Engine::new();
    let rc_conf = "rc_parallel=\"YES\"\nrc_env_allow=\"*\"\nrc_interactive=\"NO\"\n";
    let conf_map = openrc.parse_rc_conf(rc_conf);
    assert_eq!(conf_map.get("rc_parallel"), Some(&"YES".to_string()));
    assert_eq!(conf_map.get("rc_interactive"), Some(&"NO".to_string()));

    let services = vec![
        OpenRcServiceSpecV92 {
            name: "fs".to_string(),
            runlevel: "boot".to_string(),
            dependencies: vec![],
            conf_overrides: BTreeMap::new(),
        },
        OpenRcServiceSpecV92 {
            name: "net".to_string(),
            runlevel: "default".to_string(),
            dependencies: vec![(OpenRcDepKindV92::Need, "fs".to_string())],
            conf_overrides: BTreeMap::new(),
        },
        OpenRcServiceSpecV92 {
            name: "daemon".to_string(),
            runlevel: "default".to_string(),
            dependencies: vec![(OpenRcDepKindV92::Need, "net".to_string())],
            conf_overrides: BTreeMap::new(),
        },
    ];
    let exec_order = openrc.solve_runlevel_execution_order(&services).unwrap();
    assert_eq!(exec_order, vec!["fs", "net", "daemon"]);

    let mut health_map = BTreeMap::new();
    health_map.insert("fs".to_string(), true);
    health_map.insert("net".to_string(), false);
    let status_report = openrc.format_rc_status_report("default", &health_map);
    assert!(status_report.contains("Runlevel: default"));
    assert!(status_report.contains("[ started ]"));
    assert!(status_report.contains("[ stopped ]"));

    // 3. Verify runit & s6 V92 engine
    let s6 = RunitS6AdvancementsV92Engine::new();
    let scanned = s6.scan_service_dir(&[".git", "sshd", "nginx", ".ignored"]);
    assert_eq!(scanned, vec!["sshd", "nginx"]);

    let s6_spec = S6ServiceReadinessSpecV92 {
        service_name: "redis".to_string(),
        notify_fd: Some(3),
        check_script: None,
        timeout_ms: 3000,
        max_restart_attempts: 5,
    };
    assert!(s6.evaluate_readiness_notification(&s6_spec, Some(3), false));
    assert!(!s6.evaluate_readiness_notification(&s6_spec, Some(4), false));
    assert_eq!(s6.calculate_exponential_backoff_delay_ms(3, 100, 5000), 400);

    let s6_transition = s6.compile_s6_rc_transition("current_state", "target_state");
    assert!(s6_transition.contains("s6-rc -a change"));

    // 4. Verify SysVinit V92 engine
    let sysv = SysVinitAdvancementsV92Engine::new();
    let inittab_line = "1:2345:respawn:/sbin/getty 38400 tty1";
    let entry = sysv.parse_inittab_line(inittab_line).unwrap();
    assert_eq!(entry.id, "1");
    assert_eq!(entry.runlevels, vec!['2', '3', '4', '5']);
    assert_eq!(entry.action, SysVinitActionV92::Respawn);
    assert_eq!(entry.process, "/sbin/getty 38400 tty1");

    let entries = vec![entry];
    let to_run = sysv.evaluate_runlevel_switch(&entries, '3');
    assert_eq!(to_run.len(), 1);
    let to_run_fail = sysv.evaluate_runlevel_switch(&entries, '1');
    assert_eq!(to_run_fail.len(), 0);

    let rc_scripts = vec!["S20nginx", "S10network", "S01syslog", "K90bluetooth"];
    let sorted_scripts = sysv.sort_sysv_rc_scripts(&rc_scripts);
    assert_eq!(
        sorted_scripts,
        vec!["S01syslog", "S10network", "S20nginx", "K90bluetooth"]
    );

    // 5. Verify BSD rc V92 engine
    let bsd = BsdRcAdvancementsV92Engine::new();
    let script_content = "name=\"sshd\"\nrcvar=\"sshd_enable\"\ncommand=\"/usr/sbin/sshd\"\nstart_precmd=\"sshd_keygen\"\njail_name=\"myjail\"\n";
    let hooks = bsd.parse_rc_d_script_hooks(script_content);
    assert_eq!(hooks.name, "sshd");
    assert_eq!(hooks.rcvar, "sshd_enable");
    assert_eq!(hooks.start_precmd, Some("sshd_keygen".to_string()));
    let jail_cmd = bsd.build_jail_chroot_command(&hooks);
    assert_eq!(jail_cmd, "jail -n myjail exec /usr/sbin/sshd");

    // 6. Verify launchd & Android init V92 engine
    let launchd_android = LaunchdAndroidInitAdvancementsV92Engine::new();
    let mut path_state = BTreeMap::new();
    path_state.insert("/run/cron.pid".to_string(), true);
    let keep_alive = LaunchdKeepAliveSpecV92 {
        successful_exit: Some(false),
        path_state,
        other_job_enabled: BTreeMap::new(),
        crashed: Some(true),
        network_state: Some(true),
    };
    let mut existing_paths = BTreeSet::new();
    existing_paths.insert("/run/cron.pid".to_string());
    assert!(launchd_android.evaluate_keep_alive_conditions(
        &keep_alive,
        &existing_paths,
        &BTreeSet::new(),
        false
    ));

    let trigger = launchd_android.parse_android_trigger_condition("on post-fs-data");
    assert_eq!(
        trigger,
        Some(("event".to_string(), "post-fs-data".to_string()))
    );
    assert!(launchd_android.evaluate_selinux_domain_policy("u:r:init:s0", "init"));

    // 7. Verify Cron, Anacron, systemd Timers & At V92 engine
    let cron_timers = CronAnacronTimersAtV92Engine::new();
    let crontab_parts = cron_timers
        .parse_crontab_expression("*/15 2 * * 1-5")
        .unwrap();
    assert_eq!(crontab_parts, vec!["*/15", "2", "*", "*", "1-5"]);

    let timer_spec = SystemdTimerSpecV92 {
        on_calendar: Some("*-*-* 04:00:00".to_string()),
        on_boot_sec: Some(300),
        on_unit_active_sec: Some(3600),
        persistent: true,
        accuracy_sec: 60,
        randomized_delay_sec: 30,
    };
    assert!(cron_timers.evaluate_timer_readiness(&timer_spec, 400, 10, 0));
    assert!(!cron_timers.evaluate_timer_readiness(&timer_spec, 200, 10, 0));
    assert!(cron_timers.evaluate_batch_load_average(0.65, 0.80));
    assert!(!cron_timers.evaluate_batch_load_average(0.85, 0.80));

    // 8. Verify Master Suite
    let suite = SovereignInitAndSupervisorAdvancementsV92Suite::new();
    let suite_manifests = suite.collect_all_manifests();
    assert_eq!(suite_manifests.len(), 7);
}
