#[path = "../../../src/process/state.rs"]
mod state;

use state::{ProcessLifecycleTracker, ProcessState, ProcessStateError};

#[test]
fn test_valid_state_transitions() {
    let mut tracker = ProcessLifecycleTracker::new();
    assert_eq!(tracker.state(), ProcessState::New);

    tracker.transition_to(ProcessState::Ready).unwrap();
    assert_eq!(tracker.state(), ProcessState::Ready);

    tracker.transition_to(ProcessState::Running).unwrap();
    assert_eq!(tracker.state(), ProcessState::Running);

    tracker.transition_to(ProcessState::Sleeping).unwrap();
    assert_eq!(tracker.state(), ProcessState::Sleeping);

    tracker.transition_to(ProcessState::Ready).unwrap();
    assert_eq!(tracker.state(), ProcessState::Ready);

    tracker.transition_to(ProcessState::Running).unwrap();
    tracker.exit(42).unwrap();
    assert_eq!(tracker.state(), ProcessState::Zombie);
    assert_eq!(tracker.exit_code(), Some(42));

    let code = tracker.reap().unwrap();
    assert_eq!(code, 42);
    assert_eq!(tracker.state(), ProcessState::Reaped);
}

#[test]
fn test_invalid_new_to_zombie_transition() {
    let mut tracker = ProcessLifecycleTracker::new();
    let res = tracker.transition_to(ProcessState::Zombie);
    assert_eq!(
        res,
        Err(ProcessStateError::InvalidStateTransition {
            from: ProcessState::New,
            to: ProcessState::Zombie,
        })
    );
}

#[test]
fn test_double_reap_fails() {
    let mut tracker = ProcessLifecycleTracker::new();
    tracker.transition_to(ProcessState::Ready).unwrap();
    tracker.transition_to(ProcessState::Running).unwrap();
    tracker.exit(0).unwrap();

    assert!(tracker.reap().is_ok());
    assert_eq!(
        tracker.reap(),
        Err(ProcessStateError::InvalidStateTransition {
            from: ProcessState::Reaped,
            to: ProcessState::Reaped,
        })
    );
}
