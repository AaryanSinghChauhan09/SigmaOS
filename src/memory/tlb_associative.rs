use std::vec;
use std::vec::Vec;
// Hardware-Inspired Associative Page Table Lookup & TLB Caching Subsystem for SigmaOS
// Implements Fully Associative, 4-Way Set Associative, and Direct Mapped Translation Lookaside Buffer (TLB) translation.

/// TLB Associativity Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlbAssociativityMode {
    FullyAssociative,
    FourWaySetAssociative,
    DirectMapped,
}

/// TLB Page Protection & Attribute Flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TlbPageFlags {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub execute_disable: bool,
    pub is_global: bool,
}

impl TlbPageFlags {
    pub fn rw_user() -> Self {
        Self {
            present: true,
            writable: true,
            user_accessible: true,
            execute_disable: false,
            is_global: false,
        }
    }
}

/// TLB Entry Record
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TlbEntry {
    pub vpn: u64,  // Virtual Page Number
    pub pfn: u64,  // Physical Frame Number
    pub asid: u16, // Address Space ID / PCID tag
    pub flags: TlbPageFlags,
    pub lru_tick: u64, // LRU replacement timestamp
    pub is_valid: bool,
}

/// Associative TLB Cache Engine
pub struct AssociativeTlbCache {
    pub mode: TlbAssociativityMode,
    pub capacity_entries: usize,
    pub entries: Vec<TlbEntry>,
    pub current_tick: u64,
    pub lookup_hits: u64,
    pub lookup_misses: u64,
}

impl AssociativeTlbCache {
    pub fn new(mode: TlbAssociativityMode, capacity: usize) -> Self {
        let dummy_entry = TlbEntry {
            vpn: 0,
            pfn: 0,
            asid: 0,
            flags: TlbPageFlags::rw_user(),
            lru_tick: 0,
            is_valid: false,
        };

        Self {
            mode,
            capacity_entries: capacity,
            entries: vec![dummy_entry; capacity],
            current_tick: 0,
            lookup_hits: 0,
            lookup_misses: 0,
        }
    }

    /// Perform associative TLB lookup to translate Virtual Page Number (VPN) to Physical Frame Number (PFN)
    pub fn lookup_page_translation(
        &mut self,
        vpn: u64,
        asid: u16,
        is_write: bool,
        is_execute: bool,
    ) -> Result<u64, &'static str> {
        self.current_tick += 1;

        let index_range = match self.mode {
            TlbAssociativityMode::FullyAssociative => 0..self.capacity_entries,
            TlbAssociativityMode::DirectMapped => {
                let slot = (vpn as usize) % self.capacity_entries;
                slot..slot + 1
            }
            TlbAssociativityMode::FourWaySetAssociative => {
                let set_count = self.capacity_entries / 4;
                let set_idx = (vpn as usize) % set_count;
                (set_idx * 4)..(set_idx * 4 + 4)
            }
        };

        for i in index_range {
            let entry = &mut self.entries[i];
            if entry.is_valid && entry.vpn == vpn && (entry.flags.is_global || entry.asid == asid) {
                // Access checks
                if is_write && !entry.flags.writable {
                    return Err("TLB Protection Fault: Page is Read-Only");
                }
                if is_execute && entry.flags.execute_disable {
                    return Err("TLB Protection Fault: Page is Non-Executable (NX/XD)");
                }

                entry.lru_tick = self.current_tick; // Update LRU timestamp
                self.lookup_hits += 1;
                return Ok(entry.pfn);
            }
        }

        self.lookup_misses += 1;
        Err("TLB Miss")
    }

    /// Insert a newly translated page into the associative TLB cache
    pub fn insert_translation(&mut self, vpn: u64, pfn: u64, flags: TlbPageFlags, asid: u16) {
        self.current_tick += 1;

        let index_range = match self.mode {
            TlbAssociativityMode::FullyAssociative => 0..self.capacity_entries,
            TlbAssociativityMode::DirectMapped => {
                let slot = (vpn as usize) % self.capacity_entries;
                slot..slot + 1
            }
            TlbAssociativityMode::FourWaySetAssociative => {
                let set_count = self.capacity_entries / 4;
                let set_idx = (vpn as usize) % set_count;
                (set_idx * 4)..(set_idx * 4 + 4)
            }
        };

        // 1. Look for invalid/empty slot in set
        for i in index_range.clone() {
            if !self.entries[i].is_valid {
                self.entries[i] = TlbEntry {
                    vpn,
                    pfn,
                    asid,
                    flags,
                    lru_tick: self.current_tick,
                    is_valid: true,
                };
                return;
            }
        }

        // 2. If all slots full, evict LRU (Least Recently Used) entry in set
        let mut lru_idx = index_range.start;
        let mut min_tick = self.entries[lru_idx].lru_tick;

        for i in index_range {
            if self.entries[i].lru_tick < min_tick {
                min_tick = self.entries[i].lru_tick;
                lru_idx = i;
            }
        }

        self.entries[lru_idx] = TlbEntry {
            vpn,
            pfn,
            asid,
            flags,
            lru_tick: self.current_tick,
            is_valid: true,
        };
    }

    /// Flush single page entry (invlpg)
    pub fn flush_tlb_page(&mut self, vpn: u64) {
        for entry in &mut self.entries {
            if entry.is_valid && entry.vpn == vpn {
                entry.is_valid = false;
            }
        }
    }

    /// Selective TLB shootdown by ASID/PCID
    pub fn flush_tlb_by_asid(&mut self, asid: u16) {
        for entry in &mut self.entries {
            if entry.is_valid && entry.asid == asid && !entry.flags.is_global {
                entry.is_valid = false;
            }
        }
    }

    /// Complete TLB cache flush (CR3 reload)
    pub fn flush_tlb_all(&mut self) {
        for entry in &mut self.entries {
            if !entry.flags.is_global {
                entry.is_valid = false;
            }
        }
    }

    /// Get TLB cache hit ratio percentage
    pub fn get_hit_ratio_pct(&self) -> f32 {
        let total = self.lookup_hits + self.lookup_misses;
        if total == 0 {
            0.0
        } else {
            (self.lookup_hits as f32 / total as f32) * 100.0
        }
    }
}

// ── HARDWARE ASID ALLOCATOR & KERNEL ASLR ENTROPY ENGINE ───────────────────

pub struct AsidAllocator {
    pub max_asid: u16,
    pub current_asid: u16,
    pub generation: u64,
}

impl AsidAllocator {
    pub fn new(max_asid: u16) -> Self {
        Self {
            max_asid: if max_asid == 0 { 4095 } else { max_asid },
            current_asid: 1,
            generation: 1,
        }
    }

    pub fn allocate_asid(&mut self, tlb: &mut AssociativeTlbCache) -> u16 {
        let assigned = self.current_asid;
        if self.current_asid >= self.max_asid {
            self.current_asid = 1;
            self.generation += 1;
            tlb.flush_tlb_all(); // Complete TLB flush on generation rollover
        } else {
            self.current_asid += 1;
        }
        assigned
    }
}

#[derive(Debug, Clone)]
pub struct AslrLayout {
    pub text_base: u64,
    pub heap_base: u64,
    pub stack_top: u64,
    pub mmap_base: u64,
}

pub struct KernelAslrEntropyEngine {
    pub seed: u64,
}

impl KernelAslrEntropyEngine {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    fn next_prng(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.seed
    }

    pub fn generate_aslr_layout(&mut self, code_size_bytes: u64) -> AslrLayout {
        let text_entropy = (self.next_prng() % 0x0100_0000) & !0xFFF;
        let heap_entropy = (self.next_prng() % 0x0200_0000) & !0xFFF;
        let stack_entropy = (self.next_prng() % 0x0010_0000) & !0xFFF;
        let mmap_entropy = (self.next_prng() % 0x0400_0000) & !0xFFF;

        let text_base = 0x0000_7FFF_0000_0000 + text_entropy;
        let heap_base = text_base + code_size_bytes + 0x0010_0000 + heap_entropy;
        let mmap_base = 0x0000_7FFF_8000_0000 + mmap_entropy;
        let stack_top = 0x0000_7FFF_FFFF_0000 - stack_entropy;

        AslrLayout {
            text_base,
            heap_base,
            stack_top,
            mmap_base,
        }
    }
}

impl Default for AssociativeTlbCache {
    fn default() -> Self {
        Self::new(TlbAssociativityMode::FourWaySetAssociative, 64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associative_tlb_lookup_and_lru_eviction() {
        let mut tlb = AssociativeTlbCache::new(TlbAssociativityMode::FourWaySetAssociative, 16);

        let flags = TlbPageFlags::rw_user();

        // 1. Initial lookup -> TLB Miss
        assert!(tlb.lookup_page_translation(0x100, 1, false, false).is_err());

        // 2. Insert translation into TLB
        tlb.insert_translation(0x100, 0x2000, flags, 1);

        // 3. Subsequent lookup -> TLB Hit
        let pfn = tlb.lookup_page_translation(0x100, 1, false, false).unwrap();
        assert_eq!(pfn, 0x2000);
        assert_eq!(tlb.lookup_hits, 1);

        // 4. Test single page flush (invlpg)
        tlb.flush_tlb_page(0x100);
        assert!(tlb.lookup_page_translation(0x100, 1, false, false).is_err());
    }

    #[test]
    fn test_asid_selective_tlb_shootdown() {
        let mut tlb = AssociativeTlbCache::new(TlbAssociativityMode::FullyAssociative, 8);
        let flags = TlbPageFlags::rw_user();

        tlb.insert_translation(0x10, 0x100, flags, 1);
        tlb.insert_translation(0x20, 0x200, flags, 2);

        // ASID 1 shootdown should flush 0x10, leaving ASID 2 (0x20) intact
        tlb.flush_tlb_by_asid(1);

        assert!(tlb.lookup_page_translation(0x10, 1, false, false).is_err());
        assert_eq!(
            tlb.lookup_page_translation(0x20, 2, false, false).unwrap(),
            0x200
        );
    }

    #[test]
    fn test_asid_allocator_rollover() {
        let mut tlb = AssociativeTlbCache::new(TlbAssociativityMode::FullyAssociative, 8);
        let mut allocator = AsidAllocator::new(3);

        let asid1 = allocator.allocate_asid(&mut tlb);
        let asid2 = allocator.allocate_asid(&mut tlb);
        assert_eq!(asid1, 1);
        assert_eq!(asid2, 2);

        // Rollover at max_asid = 3
        let asid3 = allocator.allocate_asid(&mut tlb);
        assert_eq!(asid3, 3);
        assert_eq!(allocator.generation, 2);
    }

    #[test]
    fn test_kernel_aslr_layout_generation() {
        let mut aslr_engine = KernelAslrEntropyEngine::new(0x12345678_9ABCDEF0);
        let layout1 = aslr_engine.generate_aslr_layout(0x0008_0000);
        let layout2 = aslr_engine.generate_aslr_layout(0x0008_0000);

        assert_ne!(layout1.text_base, layout2.text_base);
        assert_ne!(layout1.heap_base, layout2.heap_base);
        assert_ne!(layout1.stack_top, layout2.stack_top);
        assert!(layout1.heap_base > layout1.text_base);
        assert!(layout1.stack_top > layout1.mmap_base);
    }
}
