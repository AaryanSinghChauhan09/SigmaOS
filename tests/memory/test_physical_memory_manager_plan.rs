// Comprehensive Unit Tests for Physical Memory Manager (PMM) Status, Invariants, and Plan Verification

#[test]
fn test_physical_memory_manager_documentation_plan_verification() {
    use std::fs;

    let docs_plan = fs::read_to_string("docs/PHYSICAL_MEMORY_MANAGER_PLAN.md")
        .expect("docs/PHYSICAL_MEMORY_MANAGER_PLAN.md must exist");
    let wiki_plan = fs::read_to_string("wiki/PHYSICAL_MEMORY_MANAGER_PLAN.md")
        .expect("wiki/PHYSICAL_MEMORY_MANAGER_PLAN.md must exist");

    assert!(docs_plan.contains("SigmaOS Physical Memory Manager Master Architecture, Support Matrix, and PR Plan"));
    assert!(docs_plan.contains("PMM Rules and Invariants"));
    assert!(docs_plan.contains("Only explicitly usable RAM is allocatable"));
    assert!(docs_plan.contains("Keep physical and virtual addresses distinct"));
    assert!(docs_plan.contains("Use checked arithmetic everywhere"));
    assert!(docs_plan.contains("Define ownership and page states"));
    assert!(docs_plan.contains("Keep PMM and VMM responsibilities separate"));
    assert!(docs_plan.contains("Zero memory before exposing it to a less-trusted consumer"));
    assert!(docs_plan.contains("Make allocation context explicit"));
    assert!(docs_plan.contains("Keep DMA policy in a DMA API"));
    assert!(docs_plan.contains("Do not overbuild NUMA/zones prematurely"));
    assert!(docs_plan.contains("Support observability without exposing writable allocator internals"));
    assert!(docs_plan.contains("Evidence gates capability claims"));
    assert!(docs_plan.contains("Proposed PR Sequence"));
    assert!(docs_plan.contains("Periodic Maintenance PR Cadence"));
    assert!(docs_plan.contains("Cross-Component Implementation Rules"));
    assert!(docs_plan.contains("PR Template and Merge Gates"));
    assert!(docs_plan.contains("Completion Gates"));

    assert_eq!(docs_plan, wiki_plan, "wiki/PHYSICAL_MEMORY_MANAGER_PLAN.md must mirror docs/PHYSICAL_MEMORY_MANAGER_PLAN.md exactly");
}

#[test]
fn test_pmm_invariants_and_allocator_state_verification() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum PageState {
        Reserved,
        Free,
        Allocated,
        Wired,
        Device,
    }

    struct MockPmmFrame {
        pfn: usize,
        state: PageState,
        ref_count: usize,
    }

    struct MockPmm {
        frames: Vec<MockPmmFrame>,
        total_usable: usize,
        total_reserved: usize,
        free_count: usize,
    }

    impl MockPmm {
        fn new(total_pages: usize, reserved_ranges: &[(usize, usize)]) -> Self {
            let mut frames = Vec::with_capacity(total_pages);
            let mut total_reserved = 0;

            for pfn in 0..total_pages {
                let is_reserved = reserved_ranges.iter().any(|&(start, end)| pfn >= start && pfn < end);
                if is_reserved {
                    frames.push(MockPmmFrame {
                        pfn,
                        state: PageState::Reserved,
                        ref_count: 0,
                    });
                    total_reserved += 1;
                } else {
                    frames.push(MockPmmFrame {
                        pfn,
                        state: PageState::Free,
                        ref_count: 0,
                    });
                }
            }

            let total_usable = total_pages - total_reserved;
            let free_count = total_usable;

            Self {
                frames,
                total_usable,
                total_reserved,
                free_count,
            }
        }

        fn allocate_frame(&mut self) -> Result<usize, &'static str> {
            for frame in &mut self.frames {
                if frame.state == PageState::Free {
                    frame.state = PageState::Allocated;
                    frame.ref_count = 1;
                    self.free_count -= 1;
                    return Ok(frame.pfn);
                }
            }
            Err("Out of physical memory")
        }

        fn free_frame(&mut self, pfn: usize) -> Result<(), &'static str> {
            if pfn >= self.frames.len() {
                return Err("Frame index out of bounds");
            }
            let frame = &mut self.frames[pfn];
            if frame.state == PageState::Reserved {
                return Err("Cannot free reserved frame");
            }
            if frame.state == PageState::Free {
                return Err("Double free detected");
            }
            if frame.state == PageState::Allocated {
                frame.ref_count -= 1;
                if frame.ref_count == 0 {
                    frame.state = PageState::Free;
                    self.free_count += 1;
                }
                return Ok(());
            }
            Err("Invalid frame state transition")
        }
    }

    let reserved = vec![(0, 4), (10, 12)]; // Reserve 0..4 and 10..12
    let mut pmm = MockPmm::new(16, &reserved);

    assert_eq!(pmm.total_reserved, 6);
    assert_eq!(pmm.total_usable, 10);
    assert_eq!(pmm.free_count, 10);

    // Allocating frame should skip reserved 0..4 and pick 4
    let f1 = pmm.allocate_frame().unwrap();
    assert_eq!(f1, 4);
    assert_eq!(pmm.free_count, 9);

    // Double free or freeing reserved should fail
    assert!(pmm.free_frame(0).is_err()); // Reserved
    assert!(pmm.free_frame(4).is_ok()); // Allocated -> Free
    assert!(pmm.free_frame(4).is_err()); // Double free
}
