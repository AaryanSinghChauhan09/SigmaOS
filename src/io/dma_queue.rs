//! SigmaOS DMA Queue & Ring Descriptor Definitions

/// Direction of DMA transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaDirection {
    ToDevice,
    FromDevice,
    Bidirectional,
}

/// Element in a physical Scatter-Gather list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScatterGatherElement {
    pub phys_addr: u64,
    pub length: u32,
}

/// A DMA descriptor representing a pending or active transfer.
#[derive(Debug, Clone)]
pub struct DmaDescriptor {
    pub id: u64,
    pub buffer_id: u64,
    pub sg_list: Vec<ScatterGatherElement>,
    pub direction: DmaDirection,
    pub total_bytes: usize,
    pub in_flight: bool,
}

impl DmaDescriptor {
    pub fn new(id: u64, buffer_id: u64, sg_list: Vec<ScatterGatherElement>, direction: DmaDirection) -> Self {
        let total_bytes = sg_list.iter().map(|e| e.length as usize).sum();
        Self {
            id,
            buffer_id,
            sg_list,
            direction,
            total_bytes,
            in_flight: false,
        }
    }
}
