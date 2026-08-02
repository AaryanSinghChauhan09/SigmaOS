#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Peripheral Evolution Pods (PeripheralPod)
// Encapsulates simulation of obsolete devices (Floppy drives, Tape drives, CRT graphics, Dot-matrix printers)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PodType {
    FloppyPod,
    TapePod,
    CrtGraphicsPod,
    DotMatrixPod,
}

pub struct PeripheralPod {
    pub pod_type: PodType,
    pub active_sim_buffer: Vec<u8>,
}

impl PeripheralPod {
    pub fn new(pod_type: PodType) -> Self {
        PeripheralPod {
            pod_type,
            active_sim_buffer: Vec::new(),
        }
    }

    pub fn write_to_pod(&mut self, data: &[u8]) {
        for &b in data {
            self.active_sim_buffer.push(b);
        }
    }

    pub fn read_from_pod(&self) -> &[u8] {
        &self.active_sim_buffer
    }

    pub fn clear_pod_state(&mut self) {
        self.active_sim_buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peripheral_pod_flow() {
        let mut pod = PeripheralPod::new(PodType::DotMatrixPod);
        assert_eq!(pod.read_from_pod().len(), 0);

        pod.write_to_pod(b"PRINT LINE");
        assert_eq!(pod.read_from_pod(), b"PRINT LINE");

        pod.clear_pod_state();
        assert_eq!(pod.read_from_pod().len(), 0);
    }
}
