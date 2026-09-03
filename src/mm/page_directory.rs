#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// S-MM Page Directory Controller
// Sovereign AI-Native zero-dependency implementation

extern crate alloc;
use alloc::vec::Vec;

pub const PAGE_SIZE_BYTES: usize = 4096;
pub const MAX_PHYSICAL_FRAMES: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageDirectoryEntry {
    pub physical_frame_idx: usize,
    pub is_present: bool,
    pub is_writable: bool,
}

pub struct PagingController {
    pub physical_bitmap: [bool; MAX_PHYSICAL_FRAMES],
    pub page_directory: [Option<PageDirectoryEntry>; 256],
}

impl PagingController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            physical_bitmap: [false; MAX_PHYSICAL_FRAMES],
            page_directory: [None; 256],
        }
    }

    pub fn map_page(
        &mut self,
        virtual_page_idx: usize,
        is_writable: bool,
    ) -> Result<usize, &'static str> {
        if virtual_page_idx >= 256 {
            return Err("Virtual address range is out of bounds");
        }
        if self.page_directory[virtual_page_idx].is_some() {
            return Err("Virtual page is already mapped");
        }

        if let Some(frame_idx) = self.allocate_physical_frame() {
            let entry = PageDirectoryEntry {
                physical_frame_idx: frame_idx,
                is_present: true,
                is_writable,
            };
            self.page_directory[virtual_page_idx] = Some(entry);
            Ok(frame_idx)
        } else {
            Err("Out of physical memory frames")
        }
    }

    pub fn unmap_page(&mut self, virtual_page_idx: usize) -> Result<(), &'static str> {
        if virtual_page_idx >= 256 {
            return Err("Virtual address range is out of bounds");
        }
        if let Some(entry) = self.page_directory[virtual_page_idx].take() {
            self.physical_bitmap[entry.physical_frame_idx] = false;
            Ok(())
        } else {
            Err("Virtual page is not mapped")
        }
    }

    fn allocate_physical_frame(&mut self) -> Option<usize> {
        for (idx, is_allocated) in self.physical_bitmap.iter_mut().enumerate() {
            if !*is_allocated {
                *is_allocated = true;
                return Some(idx);
            }
        }
        None
    }

    pub fn get_page_entry(&self, virtual_page_idx: usize) -> Option<&PageDirectoryEntry> {
        if virtual_page_idx >= 256 {
            return None;
        }
        self.page_directory[virtual_page_idx].as_ref()
    }

    pub fn is_mapped(&self, virtual_page_idx: usize) -> bool {
        self.get_page_entry(virtual_page_idx).is_some()
    }

    pub fn mapped_page_count(&self) -> usize {
        self.page_directory.iter().filter(|e| e.is_some()).count()
    }

    pub fn free_physical_frames(&self) -> usize {
        self.physical_bitmap.iter().filter(|&&allocated| !allocated).count()
    }

    pub fn used_physical_frames(&self) -> usize {
        self.physical_bitmap.iter().filter(|&&allocated| allocated).count()
    }

    pub fn clear(&mut self) {
        self.physical_bitmap = [false; MAX_PHYSICAL_FRAMES];
        self.page_directory = [None; 256];
    }
}

impl Default for PagingController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paging_controller_creation() {
        let controller = PagingController::new();
        assert_eq!(controller.mapped_page_count(), 0);
        assert_eq!(controller.used_physical_frames(), 0);
        assert_eq!(controller.free_physical_frames(), MAX_PHYSICAL_FRAMES);
    }

    #[test]
    fn test_map_page() {
        let mut controller = PagingController::new();
        
        let frame_idx = controller.map_page(0, true).unwrap();
        assert_eq!(controller.mapped_page_count(), 1);
        assert_eq!(controller.used_physical_frames(), 1);
        assert!(controller.is_mapped(0));
        
        let entry = controller.get_page_entry(0).unwrap();
        assert_eq!(entry.physical_frame_idx, frame_idx);
        assert!(entry.is_present);
        assert!(entry.is_writable);
    }

    #[test]
    fn test_unmap_page() {
        let mut controller = PagingController::new();
        
        controller.map_page(0, true).unwrap();
        assert!(controller.is_mapped(0));
        
        controller.unmap_page(0).unwrap();
        assert!(!controller.is_mapped(0));
        assert_eq!(controller.mapped_page_count(), 0);
    }

    #[test]
    fn test_virtual_address_out_of_bounds() {
        let mut controller = PagingController::new();
        
        assert!(controller.map_page(256, true).is_err());
        assert!(controller.unmap_page(256).is_err());
    }

    #[test]
    fn test_double_map() {
        let mut controller = PagingController::new();
        
        controller.map_page(0, true).unwrap();
        assert!(controller.map_page(0, true).is_err());
    }

    #[test]
    fn test_unmap_unmapped_page() {
        let mut controller = PagingController::new();
        
        assert!(controller.unmap_page(0).is_err());
    }

    #[test]
    fn test_physical_frame_allocation() {
        let mut controller = PagingController::new();
        
        let frame1 = controller.map_page(0, true).unwrap();
        let frame2 = controller.map_page(1, false).unwrap();
        
        assert_ne!(frame1, frame2);
        assert_eq!(controller.used_physical_frames(), 2);
    }

    #[test]
    fn test_out_of_physical_memory() {
        let mut controller = PagingController::new();
        
        // Allocate all physical frames
        for i in 0..MAX_PHYSICAL_FRAMES {
            assert!(controller.map_page(i, true).is_ok());
        }
        
        // Should fail when out of physical memory
        assert!(controller.map_page(MAX_PHYSICAL_FRAMES, true).is_err());
    }

    #[test]
    fn test_writable_flag() {
        let mut controller = PagingController::new();
        
        controller.map_page(0, true).unwrap();
        let entry = controller.get_page_entry(0).unwrap();
        assert!(entry.is_writable);
        
        controller.unmap_page(0).unwrap();
        controller.map_page(0, false).unwrap();
        let entry = controller.get_page_entry(0).unwrap();
        assert!(!entry.is_writable);
    }

    #[test]
    fn test_clear_controller() {
        let mut controller = PagingController::new();
        
        controller.map_page(0, true).unwrap();
        controller.map_page(1, false).unwrap();
        
        assert_eq!(controller.mapped_page_count(), 2);
        
        controller.clear();
        
        assert_eq!(controller.mapped_page_count(), 0);
        assert_eq!(controller.used_physical_frames(), 0);
        assert_eq!(controller.free_physical_frames(), MAX_PHYSICAL_FRAMES);
    }

    #[test]
    fn test_frame_reuse_after_unmap() {
        let mut controller = PagingController::new();
        
        let frame1 = controller.map_page(0, true).unwrap();
        controller.unmap_page(0).unwrap();
        
        let frame2 = controller.map_page(1, true).unwrap();
        
        // Frame should be reused
        assert_eq!(frame1, frame2);
    }
}
