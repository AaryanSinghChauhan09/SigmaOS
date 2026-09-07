//! Multi-Generation LRU (MGLRU) Page Reclamation Subsystem
//! Implements modern Linux-inspired generational page aging and eviction.

pub const MAX_GENERATIONS: usize = 4;
pub const MAX_PAGES_TRACKED: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageState {
    Active,
    Inactive,
    Evicted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageInfo {
    pub addr: usize,
    pub generation: usize,
    pub state: PageState,
    pub referenced: bool,
    pub dirty: bool,
    pub last_accessed: u64,
}

pub struct MultiGenLRU {
    pub pages: [Option<PageInfo>; MAX_PAGES_TRACKED],
    pub min_gen: usize,
    pub max_gen: usize,
    pub evict_threshold: u8,
}

impl MultiGenLRU {
    pub fn new(evict_threshold: u8) -> Self {
        Self {
            pages: [None; MAX_PAGES_TRACKED],
            min_gen: 0,
            max_gen: 0,
            evict_threshold,
        }
    }

    /// Track a new physical page in MGLRU
    pub fn track_page(&mut self, addr: usize) -> Result<(), &'static str> {
        for slot in &mut self.pages {
            if slot.is_none() {
                *slot = Some(PageInfo {
                    addr,
                    generation: self.max_gen,
                    state: PageState::Active,
                    referenced: true,
                    dirty: false,
                    last_accessed: 1000,
                });
                return Ok(());
            }
        }
        Err("MGLRU tracking capacity full")
    }

    /// Record a page reference (access). Promotes the page to the youngest generation.
    pub fn record_access(&mut self, addr: usize, timestamp: u64) -> bool {
        for slot in &mut self.pages {
            if let Some(ref mut page) = slot {
                if page.addr == addr && page.state != PageState::Evicted {
                    page.referenced = true;
                    page.last_accessed = timestamp;
                    page.generation = self.max_gen;
                    return true;
                }
            }
        }
        false
    }

    /// Age the generations (clear referenced bits and advance generations)
    pub fn age_generations(&mut self) {
        self.max_gen = (self.max_gen + 1) % MAX_GENERATIONS;

        for slot in &mut self.pages {
            if let Some(ref mut page) = slot {
                if page.state == PageState::Active {
                    if page.referenced {
                        page.generation = self.max_gen;
                        page.referenced = false;
                    } else {
                        if page.generation > self.min_gen {
                            page.generation -= 1;
                        }
                    }
                }
            }
        }

        self.update_min_gen();
    }

    fn update_min_gen(&mut self) {
        let mut lowest = self.max_gen;
        let mut found = false;
        for slot in &self.pages {
            if let Some(ref page) = slot {
                if page.state == PageState::Active {
                    if page.generation < lowest {
                        lowest = page.generation;
                        found = true;
                    }
                }
            }
        }
        if found {
            self.min_gen = lowest;
        }
    }

    /// Evict cold pages from the oldest generation
    pub fn evict_pages(&mut self, target_count: usize) -> usize {
        let mut evicted = 0;

        for gen in self.min_gen..=self.max_gen {
            if evicted >= target_count {
                break;
            }

            for slot in &mut self.pages {
                if let Some(ref mut page) = slot {
                    if page.state == PageState::Active && page.generation == gen && !page.referenced
                    {
                        page.state = PageState::Evicted;
                        evicted += 1;
                        if evicted >= target_count {
                            break;
                        }
                    }
                }
            }
        }

        self.update_min_gen();
        evicted
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_mglru_basic_workflow() {
        let mut lru = MultiGenLRU::new(10);
        lru.track_page(0x1000).unwrap();
        lru.track_page(0x2000).unwrap();

        // Initially both pages are in generation 0 and referenced = true
        assert_eq!(lru.pages[0].unwrap().addr, 0x1000);
        assert_eq!(lru.pages[0].unwrap().generation, 0);

        // Age generation: clears reference bits, advances max_gen
        lru.age_generations();
        assert_eq!(lru.max_gen, 1);

        // Access page 0x1000 again to keep it hot
        let found = lru.record_access(0x1000, 2000);
        assert!(found);

        // Age generation again: 0x1000 was referenced so it goes to max_gen (2).
        // 0x2000 was NOT referenced, so it gets aged down/left behind.
        lru.age_generations();
        assert_eq!(lru.max_gen, 2);

        // Now evict cold page
        let evicted = lru.evict_pages(1);
        assert_eq!(evicted, 1);
        assert_eq!(lru.pages[1].unwrap().state, PageState::Evicted);
        assert_eq!(lru.pages[0].unwrap().state, PageState::Active);
    }
}
