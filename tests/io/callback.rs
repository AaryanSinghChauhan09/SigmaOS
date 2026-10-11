//! Standalone Callback Manager Integration & Unit Tests

use std::sync::atomic::{AtomicBool, Ordering};
use sigmaos::io::{CallbackManager, CallbackStatus};

static CALLBACK_EXECUTED: AtomicBool = AtomicBool::new(false);

fn dummy_callback(_req_id: u64, status: CallbackStatus) {
    if status == CallbackStatus::Success {
        CALLBACK_EXECUTED.store(true, Ordering::SeqCst);
    }
}

#[test]
fn test_callback_registration_and_execution() {
    let mut manager = CallbackManager::new();
    manager.register_callback(42, dummy_callback);

    assert_eq!(manager.pending_callbacks_count(), 1);

    let invoked = manager.invoke_callbacks(42, CallbackStatus::Success);
    assert_eq!(invoked, 1);
    assert!(CALLBACK_EXECUTED.load(Ordering::SeqCst));
    assert_eq!(manager.pending_callbacks_count(), 0);
}
