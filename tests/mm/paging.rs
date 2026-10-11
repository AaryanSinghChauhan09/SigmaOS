// Standalone unit tests for Paging and Page-Table Manager

#[path = "../../src/mm/page_table.rs"]
pub mod page_table;
#[path = "../../src/mm/paging.rs"]
pub mod paging;

use page_table::{MultiLevelPageTableTree, PageTableFlags};
use paging::{PagingError, PagingManager};

#[test]
fn test_paging_canonical_address_validation() {
    assert!(MultiLevelPageTableTree::is_canonical(0x00007FFF_FFFF_FFFF));
    assert!(MultiLevelPageTableTree::is_canonical(0xFFFF8000_0000_0000));
    assert!(!MultiLevelPageTableTree::is_canonical(0x000F8000_0000_0000));
}

#[test]
fn test_paging_map_unmap_basic() {
    let pm = PagingManager::new(0x1000, 100);
    assert_eq!(pm.mapped_pages(), 0);

    let flags = PageTableFlags {
        present: true,
        writable: true,
        user_accessible: false,
        ..Default::default()
    };

    pm.map_page(0x1000, 0x20000, flags).expect("Mapping page should succeed");
    assert_eq!(pm.mapped_pages(), 1);
    assert!(pm.check_and_clear_tlb_flush());

    // Duplicate map should return AlreadyMapped
    let res = pm.map_page(0x1000, 0x30000, flags);
    assert_eq!(res, Err(PagingError::AlreadyMapped(0x1000)));

    // Unmap page
    let freed_phys = pm.unmap_page(0x1000).expect("Unmap should succeed");
    assert_eq!(freed_phys, 0x20000);
    assert_eq!(pm.mapped_pages(), 0);

    // Double unmap should return NotMapped
    let double_unmap = pm.unmap_page(0x1000);
    assert_eq!(double_unmap, Err(PagingError::NotMapped(0x1000)));
}

#[test]
fn test_paging_capacity_exceeded() {
    let pm = PagingManager::new(0x1000, 2); // Capacity limit of 2 pages
    let flags = PageTableFlags { present: true, ..Default::default() };

    pm.map_page(0x1000, 0x10000, flags).unwrap();
    pm.map_page(0x2000, 0x20000, flags).unwrap();

    let overflow = pm.map_page(0x3000, 0x30000, flags);
    assert_eq!(overflow, Err(PagingError::CapacityExceeded));
}

#[test]
fn test_paging_non_canonical_error() {
    let pm = PagingManager::new(0x1000, 10);
    let flags = PageTableFlags { present: true, ..Default::default() };

    let res = pm.map_page(0x000F8000_0000_0000, 0x10000, flags);
    assert_eq!(res, Err(PagingError::NonCanonicalAddress(0x000F8000_0000_0000)));
}

#[test]
fn test_paging_cr3_reload() {
    let pm = PagingManager::new(0x1000, 10);
    assert_eq!(pm.active_cr3(), 0x1000);

    pm.reload_cr3(0x2000);
    assert_eq!(pm.active_cr3(), 0x2000);
    assert!(pm.check_and_clear_tlb_flush());
}
