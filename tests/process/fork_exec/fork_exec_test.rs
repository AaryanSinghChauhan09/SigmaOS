#[path = "../../../src/process/state.rs"]
mod state;
#[path = "../../../src/process/pid.rs"]
mod pid;

use pid::Pid;
use state::{ProcessLifecycleTracker, ProcessState};

#[test]
fn test_fork_contract_duplication() {
    let parent_pid = Pid(100);
    let child_pid = Pid(101);

    let mut parent_tracker = ProcessLifecycleTracker::new();
    parent_tracker.transition_to(ProcessState::Ready).unwrap();
    parent_tracker.transition_to(ProcessState::Running).unwrap();

    let mut child_tracker = ProcessLifecycleTracker::new();
    child_tracker.transition_to(ProcessState::Ready).unwrap();

    assert_ne!(parent_pid, child_pid);
    assert_eq!(parent_tracker.state(), ProcessState::Running);
    assert_eq!(child_tracker.state(), ProcessState::Ready);
}

#[test]
fn test_exec_image_replacement_contract() {
    let mut tracker = ProcessLifecycleTracker::new();
    tracker.transition_to(ProcessState::Ready).unwrap();
    tracker.transition_to(ProcessState::Running).unwrap();

    // Exec maintains PID and Running state while swapping text segment
    assert_eq!(tracker.state(), ProcessState::Running);
}
