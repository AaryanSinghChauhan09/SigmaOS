#[path = "../../../src/process/state.rs"]
mod state;
#[path = "../../../src/process/pid.rs"]
mod pid;
#[path = "../../../src/process/table.rs"]
mod table;

use pid::Pid;
use table::{ProcessTable, ProcessTableError};

#[test]
fn test_reap_and_single_reap_guarantee() {
    let table = ProcessTable::new(100, 1, 100);
    let init_pid = Pid(1);

    let child = table.create_process(init_pid, "test_child").unwrap();
    let child_pid = child.pid;

    child.tracker.lock().unwrap().transition_to(crate::state::ProcessState::Running).unwrap();
    child.tracker.lock().unwrap().exit(127).unwrap();

    let exit_code = table.reap_process(child_pid).expect("reap should succeed");
    assert_eq!(exit_code, 127);

    // Duplicate reap attempt
    let second_reap = table.reap_process(child_pid);
    assert_eq!(second_reap, Err(ProcessTableError::ProcessNotFound(child_pid.0)));
}

#[test]
fn test_orphan_adoption_on_parent_exit() {
    let table = ProcessTable::new(100, 1, 100);
    let parent_pid = Pid(10);
    let init_pid = Pid(1);

    table.set_orphan_reaper(init_pid);

    let child1 = table.create_process(parent_pid, "child1").unwrap();
    let child2 = table.create_process(parent_pid, "child2").unwrap();

    assert_eq!(child1.parent_pid(), parent_pid);
    assert_eq!(child2.parent_pid(), parent_pid);

    let adopted = table.handle_parent_exit(parent_pid);
    assert_eq!(adopted, 2);

    assert_eq!(child1.parent_pid(), init_pid);
    assert_eq!(child2.parent_pid(), init_pid);
}
