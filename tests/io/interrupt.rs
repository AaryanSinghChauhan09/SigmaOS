//! Standalone Interrupt Manager Integration & Unit Tests

use sigmaos::io::{
    HandlerMode, InterruptError, InterruptManager, InterruptPriority,
};

#[test]
fn test_interrupt_registration_and_dispatch() {
    let mut manager = InterruptManager::new(16);
    let id = manager
        .register_handler(10, "test_irq_10", InterruptPriority::High, HandlerMode::TopHalf)
        .expect("Registration should succeed");

    assert_eq!(manager.dispatch_irq(10), Ok(id));

    let handler = manager.get_handler(id).expect("Handler metadata should exist");
    assert_eq!(handler.name, "test_irq_10");
    assert_eq!(handler.irq, 10);
    assert_eq!(handler.priority, InterruptPriority::High);
}

#[test]
fn test_double_handler_prevention() {
    let mut manager = InterruptManager::new(16);
    let _ = manager
        .register_handler(5, "handler_a", InterruptPriority::Normal, HandlerMode::BottomHalf)
        .unwrap();

    let res = manager.register_handler(5, "handler_b", InterruptPriority::Normal, HandlerMode::BottomHalf);
    assert_eq!(res, Err(InterruptError::AlreadyRegistered(5)));
}

#[test]
fn test_bounded_interrupt_table() {
    let mut manager = InterruptManager::new(2);
    let _ = manager.register_handler(1, "h1", InterruptPriority::Low, HandlerMode::TopHalf).unwrap();
    let _ = manager.register_handler(2, "h2", InterruptPriority::Low, HandlerMode::TopHalf).unwrap();

    let res = manager.register_handler(3, "h3", InterruptPriority::Low, HandlerMode::TopHalf);
    assert_eq!(res, Err(InterruptError::TableFull));
}

#[test]
fn test_spurious_and_lost_interrupt_detection() {
    let manager = InterruptManager::new(16);
    assert_eq!(manager.dispatch_irq(99), Err(InterruptError::IrqNotAssigned(99)));
    assert_eq!(manager.spurious_count(), 1);
}
