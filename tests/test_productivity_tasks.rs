// Standalone Test Suite for SigmaOS Distro-Inspired Tasks & Reminders Subsystem

pub mod klib {
    pub use std::collections::BTreeMap;
}

#[path = "../src/productivity/tasks.rs"]
pub mod tasks;

#[test]
fn test_distro_inspired_reminders_standalone() {
    use tasks::*;

    let mut manager = TaskManager::default();
    let task_id = "task_cron_1";

    let r1 = manager.schedule_distro_reminder(
        task_id,
        1000,
        ReminderType::CronSchedule,
        DistroReminderDispatchMode::CronSchedule("0 9 * * 1-5".to_string()),
        150,
    );

    let r2 = manager.schedule_distro_reminder(
        task_id,
        1000,
        ReminderType::SystemdTimerEvent,
        DistroReminderDispatchMode::SystemdTimerOnCalendar("Mon..Fri *-*-* 09:00:00".to_string()),
        250,
    );

    let r3 = manager.schedule_distro_reminder(
        task_id,
        1000,
        ReminderType::Custom,
        DistroReminderDispatchMode::FreeBsdWallNotify {
            tty_device: "/dev/pts/1".to_string(),
        },
        100,
    );

    let r4 = manager.schedule_distro_reminder(
        task_id,
        1000,
        ReminderType::Custom,
        DistroReminderDispatchMode::OpenBsdPledgeSandbox {
            promises: "stdio rpath".to_string(),
        },
        200,
    );

    let due = manager.process_distro_reminders(1000);
    assert_eq!(due.len(), 4);
    assert_eq!(due[0].id, r2); // weight 250
    assert_eq!(due[1].id, r4); // weight 200
    assert_eq!(due[2].id, r1); // weight 150
    assert_eq!(due[3].id, r3); // weight 100

    assert!(manager.snooze_reminder(&r2, 2000));
    assert!(manager.dismiss_reminder(&r4));

    let due2 = manager.process_distro_reminders(1000);
    assert_eq!(due2.len(), 2);
    assert_eq!(due2[0].id, r1);
    assert_eq!(due2[1].id, r3);

    let due3 = manager.process_distro_reminders(2000);
    assert_eq!(due3.len(), 3);
    assert_eq!(due3[0].id, r2);
}
