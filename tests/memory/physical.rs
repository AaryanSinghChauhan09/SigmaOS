// Standalone unit tests for Physical Memory Manager

#[path = "../../src/memory/frame.rs"]
pub mod frame;
#[path = "../../src/memory/physical.rs"]
pub mod physical;

use physical::{FrameAllocationError, PhysicalMemoryManager};

#[test]
fn test_physical_memory_manager_basic_allocation() {
    let pmm = PhysicalMemoryManager::new(0x100000, 10, &[(0, 2)]); // Frames 0 and 1 reserved
    assert_eq!(pmm.total_frames(), 10);
    assert_eq!(pmm.reserved_frames(), 2);
    assert_eq!(pmm.free_frames(), 8);
    assert_eq!(pmm.allocated_frames(), 0);

    let frame1 = pmm.allocate_frame().expect("Frame allocation should succeed");
    assert_eq!(pmm.allocated_frames(), 1);
    assert_eq!(pmm.free_frames(), 7);

    let frame2 = pmm.allocate_frame().expect("Second allocation should succeed");
    assert_ne!(frame1.index, frame2.index);
    assert_eq!(pmm.allocated_frames(), 2);

    pmm.free_frame(frame1).expect("Freeing frame1 should succeed");
    assert_eq!(pmm.allocated_frames(), 1);
    assert_eq!(pmm.free_frames(), 7);
}

#[test]
fn test_physical_memory_manager_double_free_protection() {
    let pmm = PhysicalMemoryManager::new(0x100000, 5, &[]);
    let frame = pmm.allocate_frame().unwrap();

    let frame_copy = frame.clone();
    pmm.free_frame(frame).unwrap();

    // Second free of the same frame should return DoubleFree error
    let res = pmm.free_frame(frame_copy);
    assert_eq!(res, Err(FrameAllocationError::DoubleFree));
}

#[test]
fn test_physical_memory_manager_exhaustion_error() {
    let pmm = PhysicalMemoryManager::new(0x100000, 2, &[]);
    let f1 = pmm.allocate_frame().unwrap();
    let _f2 = pmm.allocate_frame().unwrap();

    // Exhausted memory allocation attempt
    let err = pmm.allocate_frame();
    assert_eq!(err, Err(FrameAllocationError::OutOfMemory));

    // Freeing one frame allows allocation again
    let freed_phys = f1.phys_addr;
    pmm.free_frame(f1).unwrap();
    let f3 = pmm.allocate_frame().unwrap();
    assert_eq!(f3.phys_addr, freed_phys);
}

#[test]
fn test_physical_memory_manager_reserved_region_protection() {
    let pmm = PhysicalMemoryManager::new(0x100000, 4, &[(0, 4)]); // All frames reserved
    assert_eq!(pmm.free_frames(), 0);
    assert_eq!(pmm.reserved_frames(), 4);

    let res = pmm.allocate_frame();
    assert_eq!(res, Err(FrameAllocationError::OutOfMemory));
}

#[test]
fn test_reclaim_wakeup_notification() {
    let pmm = PhysicalMemoryManager::new(0x100000, 5, &[]);
    let f1 = pmm.allocate_frame().unwrap();

    assert!(!pmm.take_reclaim_wakeup_notification());
    pmm.free_frame(f1).unwrap();
    assert!(pmm.take_reclaim_wakeup_notification());
    assert!(!pmm.take_reclaim_wakeup_notification());
}
