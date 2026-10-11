//! Standalone DMA Manager Integration & Unit Tests

use sigmaos::io::{
    DmaDirection, DmaError, DmaManager, ScatterGatherElement,
};

#[test]
fn test_dma_mapping_and_completion() {
    let mut manager = DmaManager::new(8);
    let sg_list = vec![
        ScatterGatherElement { phys_addr: 0x1000, length: 1024 },
        ScatterGatherElement { phys_addr: 0x2000, length: 2048 },
    ];

    let desc_id = manager
        .map_buffer(100, sg_list, DmaDirection::ToDevice)
        .expect("Buffer mapping should succeed");

    assert_eq!(manager.pending_count(), 1);

    let bytes = manager.complete_dma(desc_id).expect("Completion should succeed");
    assert_eq!(bytes, 3072);
    assert_eq!(manager.pending_count(), 0);
}

#[test]
fn test_double_dma_prevention() {
    let mut manager = DmaManager::new(8);
    let sg_list = vec![ScatterGatherElement { phys_addr: 0x1000, length: 512 }];

    let _ = manager.map_buffer(50, sg_list.clone(), DmaDirection::ToDevice).unwrap();
    let res = manager.map_buffer(50, sg_list, DmaDirection::ToDevice);

    assert_eq!(res, Err(DmaError::DoubleDmaPrevented(50)));
}

#[test]
fn test_invalid_scatter_gather_list() {
    let mut manager = DmaManager::new(8);
    let res = manager.map_buffer(1, vec![], DmaDirection::ToDevice);
    assert_eq!(res, Err(DmaError::InvalidScatterGatherList));
}
