#[path = "../../src/process/pid.rs"]
mod pid;

use pid::{Pid, PidAllocator, PidError};

#[test]
fn test_pid_allocation_basic() {
    let mut allocator = PidAllocator::new(1, 10);
    assert_eq!(allocator.capacity(), 10);
    assert_eq!(allocator.allocated_count(), 0);

    let pid1 = allocator.allocate().expect("should allocate pid");
    assert_eq!(pid1, Pid(1));
    assert_eq!(allocator.allocated_count(), 1);
    assert!(allocator.is_allocated(pid1));

    let pid2 = allocator.allocate().expect("should allocate pid");
    assert_eq!(pid2, Pid(2));
    assert_eq!(allocator.allocated_count(), 2);
}

#[test]
fn test_pid_exhaustion_and_reuse() {
    let mut allocator = PidAllocator::new(1, 3);
    let p1 = allocator.allocate().unwrap();
    let p2 = allocator.allocate().unwrap();
    let p3 = allocator.allocate().unwrap();

    assert_eq!(allocator.allocate(), Err(PidError::PidExhausted));

    // Free p2 and verify reuse
    allocator.free(p2).expect("free should succeed");
    assert!(!allocator.is_allocated(p2));

    let p2_reused = allocator.allocate().expect("should allocate freed pid");
    assert_eq!(p2_reused, p2);

    assert_eq!(allocator.allocate(), Err(PidError::PidExhausted));
}

#[test]
fn test_invalid_pid_free() {
    let mut allocator = PidAllocator::new(10, 20);
    assert_eq!(allocator.free(Pid(5)), Err(PidError::InvalidPid(5)));
    assert_eq!(allocator.free(Pid(15)), Err(PidError::PidNotFound(15)));
}
