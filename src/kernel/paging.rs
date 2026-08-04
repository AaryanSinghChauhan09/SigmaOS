// SigmaOS 4-Level Page Table Walking & Paging Subsystem
// Zero-dependency, #![no_std] compliant x86_64 paging implementation.

use core::ptr::NonNull;

pub const PAGE_SIZE: usize = 4096;
pub const ENTRY_COUNT: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableFlags(pub u64);

impl PageTableFlags {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER_ACCESSIBLE: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const NO_CACHE: u64 = 1 << 4;
    pub const HUGE_PAGE: u64 = 1 << 7;  // Page Size (PS) flag for huge pages
    pub const COW: u64 = 1 << 9;        // Copy-On-Write flag

    // x86/ARM advanced security & caching properties
    pub const NO_EXECUTE: u64 = 1 << 63; // Execute-Disable / No-Execute (XD/NX)
    pub const GLOBAL: u64 = 1 << 8;      // Global Page (prevent TLB flush on CR3 load)
    pub const PAT_WRITE_BACK: u64 = 0;   // PAT Memory Attribute: Write-Back
    pub const PAT_WRITE_THROUGH: u64 = 1 << 3; // PAT Memory Attribute: Write-Through
    pub const PAT_UNCACHED: u64 = 1 << 4;     // PAT Memory Attribute: Uncached
    pub const PAT_WRITE_COMBINING: u64 = (1 << 3) | (1 << 4); // PAT Memory Attribute: Write-Combining
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TranslationContext {
    pub pcid: u16,  // Process Context Identifier (x86_64, CR3 bits 0-11)
    pub asid: u8,   // Address Space Identifier (ARM/RISC-V)
}

impl TranslationContext {
    pub const fn new(pcid: u16, asid: u8) -> Self {
        Self { pcid, asid }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }
    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    pub fn flags(&self) -> PageTableFlags {
        PageTableFlags(self.0 & 0xFFF0_0000_0000_0FFF)
    }

    pub fn physical_frame(&self) -> Option<u64> {
        if self.flags().0 & PageTableFlags::PRESENT != 0 {
            Some(self.0 & 0x000F_FFFF_FFFF_F000)
        } else {
            None
        }
    }

    pub fn set_frame(&mut self, frame_addr: u64, flags: PageTableFlags) {
        self.0 = (frame_addr & 0x000F_FFFF_FFFF_F000) | flags.0 | PageTableFlags::PRESENT;
    }
}

#[repr(align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; ENTRY_COUNT],
}

impl PageTable {
    pub const fn new() -> Self {
        Self {
            entries: [PageTableEntry(0); ENTRY_COUNT],
        }
    }
}

/// TTBR Split Governor mimicking ARM architectural splits
/// TTBR0 handles lower virtual address space (usually user space mappings)
/// TTBR1 handles higher virtual address space (usually kernel space mappings)
pub struct TtbrSplitGovernor {
    pub ttbr0_base: u64,
    pub ttbr1_base: u64,
}

impl TtbrSplitGovernor {
    pub const fn new(ttbr0_base: u64, ttbr1_base: u64) -> Self {
        Self { ttbr0_base, ttbr1_base }
    }

    /// Resolves which translation base register to use based on the virtual address split
    /// Threshold address typically divides user space from kernel space (e.g. 0xFFFF_8000_0000_0000)
    pub fn select_translation_base(&self, virt_addr: u64) -> (u64, &'static str) {
        if virt_addr < 0xFFFF_8000_0000_0000 {
            (self.ttbr0_base, "TTBR0 (User Space)")
        } else {
            (self.ttbr1_base, "TTBR1 (Kernel Space)")
        }
    }
}

/// Stage-2 Nested Translator mimicking hardware assisted virtualization (Intel EPT, AMD NPT, ARM Stage 2)
/// Translates Guest Physical Address (GPA) to Host Physical Address (HPA).
pub struct Stage2NestedTranslator {
    pub stage2_root_phys: u64,
}

impl Stage2NestedTranslator {
    pub const fn new(stage2_root_phys: u64) -> Self {
        Self { stage2_root_phys }
    }

    /// Walks a simulated Stage-2 page table to resolve GPA -> HPA
    pub unsafe fn translate_gpa(&self, gpa: u64) -> Option<u64> {
        let root_table = &*(self.stage2_root_phys as *const PageTable);
        let lvl4_index = ((gpa >> 39) & 0x1FF) as usize;
        let lvl4_entry = &root_table.entries[lvl4_index];
        let lvl3_phys = lvl4_entry.physical_frame()?;

        let lvl3_table = &*(lvl3_phys as *const PageTable);
        let lvl3_index = ((gpa >> 30) & 0x1FF) as usize;
        let lvl3_entry = &lvl3_table.entries[lvl3_index];

        if lvl3_entry.flags().0 & PageTableFlags::HUGE_PAGE != 0 {
            let offset = gpa & 0x3FFF_FFFF; // 1GB Offset
            return Some(lvl3_entry.physical_frame()? + offset);
        }
        let lvl2_phys = lvl3_entry.physical_frame()?;

        let lvl2_table = &*(lvl2_phys as *const PageTable);
        let lvl2_index = ((gpa >> 21) & 0x1FF) as usize;
        let lvl2_entry = &lvl2_table.entries[lvl2_index];

        if lvl2_entry.flags().0 & PageTableFlags::HUGE_PAGE != 0 {
            let offset = gpa & 0x1F_FFFF; // 2MB Offset
            return Some(lvl2_entry.physical_frame()? + offset);
        }
        let lvl1_phys = lvl2_entry.physical_frame()?;

        let lvl1_table = &*(lvl1_phys as *const PageTable);
        let lvl1_index = ((gpa >> 12) & 0x1FF) as usize;
        let lvl1_entry = &lvl1_table.entries[lvl1_index];
        let hpf = lvl1_entry.physical_frame()?;
        let offset = gpa & 0xFFF;

        Some(hpf + offset)
    }
}

/// Windows-style Page Frame Number (PFN) states for physical memory management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfnState {
    Active,
    Standby,
    Modified,
    Free,
    Bad,
    Zeroed,
}

/// Entry in the Windows-grade PFN database tracking memory state transitions
pub struct PfnEntry {
    pub frame_number: u64,
    pub state: PfnState,
    pub reference_count: u32,
    pub is_dirty: bool,
}

pub struct PfnDatabase<const N: usize> {
    pub entries: [PfnEntry; N],
}

impl<const N: usize> PfnDatabase<N> {
    pub fn new() -> Self {
        // Safe initialization loop for constant array
        let mut entries = core::mem::MaybeUninit::<[PfnEntry; N]>::uninit();
        let entries_ptr = entries.as_mut_ptr() as *mut PfnEntry;
        for i in 0..N {
            unsafe {
                entries_ptr.add(i).write(PfnEntry {
                    frame_number: i as u64,
                    state: PfnState::Free,
                    reference_count: 0,
                    is_dirty: false,
                });
            }
        }
        Self {
            entries: unsafe { entries.assume_init() },
        }
    }

    /// Mark a frame as Active (allocated)
    pub fn transition_to_active(&mut self, frame: u64) -> Result<(), &'static str> {
        if frame >= N as u64 {
            return Err("PFN: Frame out of range");
        }
        let entry = &mut self.entries[frame as usize];
        if entry.state == PfnState::Bad {
            return Err("PFN: Cannot allocate a Bad frame");
        }
        entry.state = PfnState::Active;
        entry.reference_count += 1;
        Ok(())
    }

    /// Deallocate an active frame, sending it to the Modified list if dirty, or Standby/Free if clean
    pub fn transition_to_free_or_standby(&mut self, frame: u64) -> Result<(), &'static str> {
        if frame >= N as u64 {
            return Err("PFN: Frame out of range");
        }
        let entry = &mut self.entries[frame as usize];
        if entry.reference_count > 0 {
            entry.reference_count -= 1;
        }
        if entry.reference_count == 0 {
            if entry.is_dirty {
                entry.state = PfnState::Modified;
            } else {
                entry.state = PfnState::Standby;
            }
        }
        Ok(())
    }

    /// Reclaim a Standby page back to Active without physical read/allocation overhead
    pub fn reclaim_standby_page(&mut self, frame: u64) -> Result<(), &'static str> {
        if frame >= N as u64 {
            return Err("PFN: Frame out of range");
        }
        let entry = &mut self.entries[frame as usize];
        if entry.state == PfnState::Standby {
            entry.state = PfnState::Active;
            entry.reference_count = 1;
            Ok(())
        } else {
            Err("PFN: Frame not in Standby state")
        }
    }
}

/// Windows Virtual Address Descriptor (VAD) nodes defining mapped ranges
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VadNode {
    pub start_vpn: u64, // Virtual Page Number start
    pub end_vpn: u64,   // Virtual Page Number end
    pub protection: u32, // Bitmask: Read (1), Write (2), Execute (4)
}

/// Managing VAD mappings matching Windows kernel tree architectures
pub struct VadManager<const MAX_VAD: usize> {
    pub nodes: [Option<VadNode>; MAX_VAD],
    pub count: usize,
}

impl<const MAX_VAD: usize> VadManager<MAX_VAD> {
    pub const fn new() -> Self {
        Self {
            nodes: [None; MAX_VAD],
            count: 0,
        }
    }

    pub fn insert_range(&mut self, start_vpn: u64, end_vpn: u64, protection: u32) -> Result<(), &'static str> {
        if self.count >= MAX_VAD {
            return Err("VAD: Maximum descriptors reached");
        }
        // Validate overlaps
        for node in self.nodes.iter().flatten() {
            if start_vpn <= node.end_vpn && end_vpn >= node.start_vpn {
                return Err("VAD: Overlapping virtual range detected");
            }
        }
        self.nodes[self.count] = Some(VadNode { start_vpn, end_vpn, protection });
        self.count += 1;
        Ok(())
    }

    pub fn find_node(&self, vpn: u64) -> Option<VadNode> {
        for node in self.nodes.iter().flatten() {
            if vpn >= node.start_vpn && vpn <= node.end_vpn {
                return Some(*node);
            }
        }
        None
    }
}

/// Linux/BSD Kernel Page Table Isolation (KPTI / KAISER) Governor
/// Ensures user-space page table directory contains minimum kernel mappings
/// to mitigate spec-execution leakage vectors (Meltdown).
pub struct KptiGovernor {
    pub user_pml4_phys: u64,
    pub kernel_pml4_phys: u64,
}

impl KptiGovernor {
    pub const fn new(user_pml4_phys: u64, kernel_pml4_phys: u64) -> Self {
        Self { user_pml4_phys, kernel_pml4_phys }
    }

    /// Verifies if a virtual address should be visible under KPTI User Space table
    /// Typical policy: User space addresses (< 0x8000_0000_0000) and specific trampoline regions are visible
    pub fn is_visible_to_user(&self, virt_addr: u64) -> bool {
        if virt_addr < 0x8000_0000_0000 {
            true // standard user land
        } else {
            // Only allow specialized CPU context regions / page-fault vectors (trampolines)
            virt_addr == 0xFFFF_FFFF_FFFF_0000 || virt_addr == 0xFFFF_FFFF_FFFF_1000
        }
    }
}

/// BSD-inspired physical map (`pmap`) hardware-translation abstraction engine
pub struct PmapEngine {
    pub pml4_phys: u64,
    pub resident_count: usize,
    pub wired_count: usize,
}

impl PmapEngine {
    pub const fn new(pml4_phys: u64) -> Self {
        Self {
            pml4_phys,
            resident_count: 0,
            wired_count: 0,
        }
    }

    /// Register active mapping metrics
    pub fn enter_mapping(&mut self, is_wired: bool) {
        self.resident_count += 1;
        if is_wired {
            self.wired_count += 1;
        }
    }

    pub fn remove_mapping(&mut self, is_wired: bool) {
        if self.resident_count > 0 {
            self.resident_count -= 1;
        }
        if is_wired && self.wired_count > 0 {
            self.wired_count -= 1;
        }
    }
}

pub struct VirtualMemoryManagerV2 {
    pml4_table: NonNull<PageTable>,
    pub tlb_invalidations: core::sync::atomic::AtomicUsize,
    pub tlb_flushes: core::sync::atomic::AtomicUsize,
    pub is_5level_enabled: bool,
    pub context: TranslationContext,
}

impl VirtualMemoryManagerV2 {
    pub unsafe fn new(pml4_phys_addr: u64) -> Self {
        Self {
            pml4_table: NonNull::new_unchecked(pml4_phys_addr as *mut PageTable),
            tlb_invalidations: core::sync::atomic::AtomicUsize::new(0),
            tlb_flushes: core::sync::atomic::AtomicUsize::new(0),
            is_5level_enabled: false,
            context: TranslationContext::new(0, 0),
        }
    }

    /// Update active translation context (ASID / PCID context registers)
    pub fn set_context(&mut self, context: TranslationContext) {
        self.context = context;
        // Simulates reloading CR3 or context registers, requiring TLB invalidation adjustments
        if context.pcid == 0 {
            self.flush_tlb_all();
        } else {
            self.invlpg(0x0); // partial tag clear
        }
    }

    /// Simulate TLB invalidation for a specific virtual page address.
    pub fn invlpg(&self, _virt_addr: u64) {
        self.tlb_invalidations.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    }

    /// Simulate a complete TLB flush across all mapping caches.
    pub fn flush_tlb_all(&self) {
        self.tlb_flushes.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    }

    /// Dump translation tree details recursively for advanced telemetry
    pub unsafe fn dump_page_tables<F>(&self, mut printer: F)
    where
        F: FnMut(usize, u64, u64, PageTableFlags),
    {
        let root_addr = self.pml4_table.as_ptr() as u64;
        if self.is_5level_enabled {
            let pml5 = &*(root_addr as *const PageTable);
            for (i5, e5) in pml5.entries.iter().enumerate() {
                if let Some(pml4_phys) = e5.physical_frame() {
                    printer(5, i5 as u64, pml4_phys, e5.flags());
                    self.dump_pml4_level(pml4_phys, &mut printer);
                }
            }
        } else {
            self.dump_pml4_level(root_addr, &mut printer);
        }
    }

    unsafe fn dump_pml4_level<F>(&self, pml4_phys: u64, printer: &mut F)
    where
        F: FnMut(usize, u64, u64, PageTableFlags),
    {
        let pml4 = &*(pml4_phys as *const PageTable);
        for (i4, e4) in pml4.entries.iter().enumerate() {
            if let Some(pdpt_phys) = e4.physical_frame() {
                printer(4, i4 as u64, pdpt_phys, e4.flags());
                let pdpt = &*(pdpt_phys as *const PageTable);
                for (i3, e3) in pdpt.entries.iter().enumerate() {
                    if let Some(pd_phys) = e3.physical_frame() {
                        printer(3, i3 as u64, pd_phys, e3.flags());
                        if e3.flags().0 & PageTableFlags::HUGE_PAGE != 0 {
                            continue; // 1GB huge page
                        }
                        let pd = &*(pd_phys as *const PageTable);
                        for (i2, e2) in pd.entries.iter().enumerate() {
                            if let Some(pt_phys) = e2.physical_frame() {
                                printer(2, i2 as u64, pt_phys, e2.flags());
                                if e2.flags().0 & PageTableFlags::HUGE_PAGE != 0 {
                                    continue; // 2MB huge page
                                }
                                let pt = &*(pt_phys as *const PageTable);
                                for (i1, e1) in pt.entries.iter().enumerate() {
                                    if let Some(frame) = e1.physical_frame() {
                                        printer(1, i1 as u64, frame, e1.flags());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Translates a virtual address to its corresponding physical address by walking PML4 -> PDPT -> PD -> PT
    /// Supports 5-Level virtual memory paging matching PML5/P4D indexes when enabled.
    /// Also supports Huge Pages translation (2MB/1GB).
    pub unsafe fn translate(&self, virt_addr: u64) -> Option<u64> {
        let mut root_addr = self.pml4_table.as_ptr() as u64;

        if self.is_5level_enabled {
            // PML5 level translation (bits 48-56 of virtual address)
            let pml5_index = ((virt_addr >> 48) & 0x1FF) as usize;
            let pml5 = &*(root_addr as *const PageTable);
            let pml5_entry = &pml5.entries[pml5_index];
            root_addr = pml5_entry.physical_frame()?;
        }

        let pml4_index = ((virt_addr >> 39) & 0x1FF) as usize;
        let pdpt_index = ((virt_addr >> 30) & 0x1FF) as usize;
        let pd_index = ((virt_addr >> 21) & 0x1FF) as usize;
        let pt_index = ((virt_addr >> 12) & 0x1FF) as usize;

        let pml4 = &*(root_addr as *const PageTable);
        let pml4_entry = &pml4.entries[pml4_index];
        let pdpt_addr = pml4_entry.physical_frame()?;

        let pdpt = &*(pdpt_addr as *const PageTable);
        let pdpt_entry = &pdpt.entries[pdpt_index];

        // 1GB Huge Page translation
        if pdpt_entry.flags().0 & PageTableFlags::HUGE_PAGE != 0 {
            let page_offset = virt_addr & 0x3FFF_FFFF; // 1GB offset mask
            return Some(pdpt_entry.physical_frame()? + page_offset);
        }

        let pd_addr = pdpt_entry.physical_frame()?;

        let pd = &*(pd_addr as *const PageTable);
        let pd_entry = &pd.entries[pd_index];

        // 2MB Huge Page translation
        if pd_entry.flags().0 & PageTableFlags::HUGE_PAGE != 0 {
            let page_offset = virt_addr & 0x1F_FFFF; // 2MB offset mask
            return Some(pd_entry.physical_frame()? + page_offset);
        }

        let pt_addr = pd_entry.physical_frame()?;

        let pt = &*(pt_addr as *const PageTable);
        let pt_entry = &pt.entries[pt_index];
        let frame_addr = pt_entry.physical_frame()?;
        let page_offset = virt_addr & 0xFFF;

        Some(frame_addr + page_offset)
    }

    /// Simulate a Page Fault resolution trigger. If it encounters a write on a COW-gated page,
    /// it resolves the violation by copying the page frame on-the-fly.
    pub unsafe fn handle_page_fault(&mut self, fault_addr: u64, is_write: bool) -> Result<u64, &'static str> {
        let pml4_index = ((fault_addr >> 39) & 0x1FF) as usize;
        let pdpt_index = ((fault_addr >> 30) & 0x1FF) as usize;
        let pd_index = ((fault_addr >> 21) & 0x1FF) as usize;
        let pt_index = ((fault_addr >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table.as_mut();
        let pml4_entry = &mut pml4.entries[pml4_index];
        let pdpt_addr = pml4_entry.physical_frame().ok_or("PF: PDPT missing")?;

        let pdpt = &mut *(pdpt_addr as *mut PageTable);
        let pdpt_entry = &mut pdpt.entries[pdpt_index];
        let pd_addr = pdpt_entry.physical_frame().ok_or("PF: PD missing")?;

        let pd = &mut *(pd_addr as *mut PageTable);
        let pd_entry = &mut pd.entries[pd_index];
        let pt_addr = pd_entry.physical_frame().ok_or("PF: PT missing")?;

        let pt = &mut *(pt_addr as *mut PageTable);
        let pt_entry = &mut pt.entries[pt_index];

        let flags = pt_entry.flags();
        if is_write && (flags.0 & PageTableFlags::COW != 0) {
            // Copy-On-Write page-fault triggering!
            let old_frame = pt_entry.physical_frame().ok_or("PF: Frame missing")?;
            let new_frame = old_frame + 0x1000_0000; // mock copy reallocation offset

            let mut new_flags = flags.0;
            new_flags &= !PageTableFlags::COW;       // Clear COW flag
            new_flags |= PageTableFlags::WRITABLE;   // Enable write permission

            pt_entry.set_frame(new_frame, PageTableFlags(new_flags));
            self.invlpg(fault_addr);
            return Ok(new_frame);
        }

        Err("Page fault cannot be resolved as COW")
    }

    /// Maps a virtual page to a physical frame
    pub unsafe fn map_page(
        &mut self,
        virt_addr: u64,
        phys_frame: u64,
        flags: PageTableFlags,
        allocator: &mut dyn FnMut() -> Option<NonNull<PageTable>>,
    ) -> Result<(), &'static str> {
        let pml4_index = ((virt_addr >> 39) & 0x1FF) as usize;
        let pdpt_index = ((virt_addr >> 30) & 0x1FF) as usize;
        let pd_index = ((virt_addr >> 21) & 0x1FF) as usize;
        let pt_index = ((virt_addr >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table.as_mut();

        let pml4_entry = &mut pml4.entries[pml4_index];
        let pdpt_addr = if pml4_entry.is_unused() {
            let mut table_ptr = allocator().ok_or("Out of memory for PDPT")?;
            table_ptr
                .as_mut()
                .entries
                .iter_mut()
                .for_each(|e| e.set_unused());
            let addr = table_ptr.as_ptr() as u64;
            pml4_entry.set_frame(addr, flags);
            addr
        } else {
            pml4_entry.physical_frame().unwrap()
        };

        let pdpt = &mut *(pdpt_addr as *mut PageTable);
        let pdpt_entry = &mut pdpt.entries[pdpt_index];
        let pd_addr = if pdpt_entry.is_unused() {
            let mut table_ptr = allocator().ok_or("Out of memory for PD")?;
            table_ptr
                .as_mut()
                .entries
                .iter_mut()
                .for_each(|e| e.set_unused());
            let addr = table_ptr.as_ptr() as u64;
            pdpt_entry.set_frame(addr, flags);
            addr
        } else {
            pdpt_entry.physical_frame().unwrap()
        };

        let pd = &mut *(pd_addr as *mut PageTable);
        let pd_entry = &mut pd.entries[pd_index];
        let pt_addr = if pd_entry.is_unused() {
            let mut table_ptr = allocator().ok_or("Out of memory for PT")?;
            table_ptr
                .as_mut()
                .entries
                .iter_mut()
                .for_each(|e| e.set_unused());
            let addr = table_ptr.as_ptr() as u64;
            pd_entry.set_frame(addr, flags);
            addr
        } else {
            pd_entry.physical_frame().unwrap()
        };

        let pt = &mut *(pt_addr as *mut PageTable);
        let pt_entry = &mut pt.entries[pt_index];
        if !pt_entry.is_unused() {
            return Err("Page already mapped!");
        }

        pt_entry.set_frame(phys_frame, flags);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4level_page_table_walking() {
        let mut pml4 = PageTable::new();
        let mut pdpt = PageTable::new();
        let mut pd = PageTable::new();
        let mut pt = PageTable::new();

        let pml4_ptr = &mut pml4 as *mut PageTable;
        let mut pdpt_ptr = NonNull::new(&mut pdpt as *mut PageTable);
        let mut pd_ptr = NonNull::new(&mut pd as *mut PageTable);
        let mut pt_ptr = NonNull::new(&mut pt as *mut PageTable);

        let mut allocator_calls = 0;
        let mut allocator = || {
            allocator_calls += 1;
            match allocator_calls {
                1 => Some(pdpt_ptr.unwrap()),
                2 => Some(pd_ptr.unwrap()),
                3 => Some(pt_ptr.unwrap()),
                _ => None,
            }
        };

        let mut vmm = unsafe { VirtualMemoryManagerV2::new(pml4_ptr as u64) };
        let virt = 0x0000_7FFF_FFFF_F000;
        let phys = 0x0000_0000_1000_0000;
        let flags = PageTableFlags(PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);

        // Map the page
        assert!(unsafe { vmm.map_page(virt, phys, flags, &mut allocator).is_ok() });
        assert_eq!(allocator_calls, 3);

        // Translate the page
        let translated = unsafe { vmm.translate(virt).unwrap() };
        assert_eq!(translated, phys);
    }

    #[test]
    fn test_5level_paging() {
        let mut pml5 = PageTable::new();
        let mut pml4 = PageTable::new();
        let mut pdpt = PageTable::new();
        let mut pd = PageTable::new();
        let mut pt = PageTable::new();

        let pml5_ptr = &mut pml5 as *mut PageTable;

        // Wire up the 5 levels manually
        pml5.entries[1].set_frame(&pml4 as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        pml4.entries[1].set_frame(&pdpt as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        pdpt.entries[1].set_frame(&pd as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        pd.entries[1].set_frame(&pt as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        pt.entries[1].set_frame(0x8000_0000, PageTableFlags(PageTableFlags::PRESENT));

        let mut vmm = unsafe { VirtualMemoryManagerV2::new(pml5_ptr as u64) };
        vmm.is_5level_enabled = true;

        // virt index [1, 1, 1, 1, 1] offset 0
        let virt = (1 << 48) | (1 << 39) | (1 << 30) | (1 << 21) | (1 << 12);
        let translated = unsafe { vmm.translate(virt).unwrap() };
        assert_eq!(translated, 0x8000_0000);
    }

    #[test]
    fn test_tlb_tracking() {
        let pml4 = PageTable::new();
        let vmm = unsafe { VirtualMemoryManagerV2::new(&pml4 as *const PageTable as u64) };

        assert_eq!(vmm.tlb_invalidations.load(core::sync::atomic::Ordering::SeqCst), 0);
        vmm.invlpg(0x1000);
        assert_eq!(vmm.tlb_invalidations.load(core::sync::atomic::Ordering::SeqCst), 1);

        assert_eq!(vmm.tlb_flushes.load(core::sync::atomic::Ordering::SeqCst), 0);
        vmm.flush_tlb_all();
        assert_eq!(vmm.tlb_flushes.load(core::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn test_huge_pages_translation() {
        let pml4 = PageTable::new();
        let mut pdpt = PageTable::new();
        let mut pd = PageTable::new();

        let mut vmm = unsafe { VirtualMemoryManagerV2::new(&pml4 as *const PageTable as u64) };

        // 1. Test 1GB Huge Page
        unsafe {
            let pml4_ptr = vmm.pml4_table.as_ptr();
            (*pml4_ptr).entries[0].set_frame(&pdpt as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        }
        pdpt.entries[0].set_frame(0x4000_0000, PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::HUGE_PAGE));

        let translated_1gb = unsafe { vmm.translate(0x1234).unwrap() };
        assert_eq!(translated_1gb, 0x4000_1234);

        // 2. Test 2MB Huge Page
        pdpt.entries[0].set_frame(&pd as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT)); // Reset to point to PD
        pd.entries[0].set_frame(0x20_0000, PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::HUGE_PAGE));

        let translated_2mb = unsafe { vmm.translate(0x4567).unwrap() };
        assert_eq!(translated_2mb, 0x20_4567);
    }

    #[test]
    fn test_cow_page_fault_resolution() {
        let mut pml4 = PageTable::new();
        let mut pdpt = PageTable::new();
        let mut pd = PageTable::new();
        let mut pt = PageTable::new();

        pml4.entries[0].set_frame(&pdpt as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        pdpt.entries[0].set_frame(&pd as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        pd.entries[0].set_frame(&pt as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));

        // Frame starting with Copy-On-Write flag active
        pt.entries[0].set_frame(0x1000, PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::COW));

        let mut vmm = unsafe { VirtualMemoryManagerV2::new(&pml4 as *const PageTable as u64) };

        // Trigger write fault -> resolves on-the-fly and copies frame
        let new_frame = unsafe { vmm.handle_page_fault(0x0, true).unwrap() };
        assert_eq!(new_frame, 0x1000 + 0x1000_0000);

        // Check that page is now writable and COW flag is cleared
        let flags = pt.entries[0].flags();
        assert_eq!(flags.0 & PageTableFlags::COW, 0);
        assert_ne!(flags.0 & PageTableFlags::WRITABLE, 0);
    }

    #[test]
    fn test_advanced_paging_features() {
        // Test PAT / Advanced security flags
        let mut flags = PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::NO_EXECUTE | PageTableFlags::GLOBAL | PageTableFlags::PAT_WRITE_COMBINING);
        assert_ne!(flags.0 & PageTableFlags::NO_EXECUTE, 0);
        assert_ne!(flags.0 & PageTableFlags::GLOBAL, 0);
        assert_eq!(flags.0 & PageTableFlags::PAT_UNCACHED, PageTableFlags::PAT_UNCACHED);

        // Test PCID/ASID translation context
        let mut vmm = unsafe { VirtualMemoryManagerV2::new(&PageTable::new() as *const PageTable as u64) };
        vmm.set_context(TranslationContext::new(42, 7));
        assert_eq!(vmm.context.pcid, 42);
        assert_eq!(vmm.context.asid, 7);

        // Test TTBR split
        let ttbr = TtbrSplitGovernor::new(0x1000, 0x2000);
        let (base_user, label_user) = ttbr.select_translation_base(0x0000_7FFF_FFFF_F000);
        assert_eq!(base_user, 0x1000);
        assert_eq!(label_user, "TTBR0 (User Space)");

        let (base_kernel, label_kernel) = ttbr.select_translation_base(0xFFFF_8000_0000_0000);
        assert_eq!(base_kernel, 0x2000);
        assert_eq!(label_kernel, "TTBR1 (Kernel Space)");

        // Test Stage 2 Translation
        let mut s2_lvl4 = PageTable::new();
        let mut s2_lvl3 = PageTable::new();
        let mut s2_lvl2 = PageTable::new();
        let mut s2_lvl1 = PageTable::new();

        s2_lvl4.entries[0].set_frame(&s2_lvl3 as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        s2_lvl3.entries[0].set_frame(&s2_lvl2 as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        s2_lvl2.entries[0].set_frame(&s2_lvl1 as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        s2_lvl1.entries[0].set_frame(0x9000_0000, PageTableFlags(PageTableFlags::PRESENT));

        let s2_translator = Stage2NestedTranslator::new(&s2_lvl4 as *const PageTable as u64);
        let hpa = unsafe { s2_translator.translate_gpa(0x123).unwrap() };
        assert_eq!(hpa, 0x9000_0123);

        // Test PFN Database transitions
        let mut pfn_db = PfnDatabase::<16>::new();
        assert_eq!(pfn_db.entries[5].state, PfnState::Free);
        assert!(pfn_db.transition_to_active(5).is_ok());
        assert_eq!(pfn_db.entries[5].state, PfnState::Active);
        assert_eq!(pfn_db.entries[5].reference_count, 1);

        pfn_db.entries[5].is_dirty = true;
        assert!(pfn_db.transition_to_free_or_standby(5).is_ok());
        assert_eq!(pfn_db.entries[5].state, PfnState::Modified);

        pfn_db.entries[6].is_dirty = false;
        assert!(pfn_db.transition_to_active(6).is_ok());
        assert!(pfn_db.transition_to_free_or_standby(6).is_ok());
        assert_eq!(pfn_db.entries[6].state, PfnState::Standby);

        assert!(pfn_db.reclaim_standby_page(6).is_ok());
        assert_eq!(pfn_db.entries[6].state, PfnState::Active);

        // Test VAD Range management
        let mut vad = VadManager::<8>::new();
        assert!(vad.insert_range(0x1000, 0x1FFF, 3).is_ok()); // read + write
        assert!(vad.insert_range(0x2000, 0x2FFF, 5).is_ok()); // read + execute
        assert!(vad.insert_range(0x1800, 0x2500, 1).is_err()); // Overlap detection

        let mapped_node = vad.find_node(0x1500).unwrap();
        assert_eq!(mapped_node.protection, 3);
        assert_eq!(mapped_node.end_vpn, 0x1FFF);

        // Test KPTI visibility checks
        let kpti = KptiGovernor::new(0x1000, 0x2000);
        assert!(kpti.is_visible_to_user(0x0000_7FFF_FFFF_F000));
        assert!(!kpti.is_visible_to_user(0xFFFF_8000_0000_0000));
        assert!(kpti.is_visible_to_user(0xFFFF_FFFF_FFFF_0000)); // Trampoline exception

        // Test PMAP Physical mapping metrics
        let mut pmap = PmapEngine::new(0x1000);
        pmap.enter_mapping(true);
        pmap.enter_mapping(false);
        assert_eq!(pmap.resident_count, 2);
        assert_eq!(pmap.wired_count, 1);

        pmap.remove_mapping(true);
        assert_eq!(pmap.resident_count, 1);
        assert_eq!(pmap.wired_count, 0);

        // Test diagnostics tree dumper execution
        let mut dump_count = 0;
        let mut pml4 = PageTable::new();
        let mut pdpt = PageTable::new();
        pml4.entries[0].set_frame(&pdpt as *const PageTable as u64, PageTableFlags(PageTableFlags::PRESENT));
        pdpt.entries[0].set_frame(0x5000_0000, PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::HUGE_PAGE));

        let vmm_dump = unsafe { VirtualMemoryManagerV2::new(&pml4 as *const PageTable as u64) };
        unsafe {
            vmm_dump.dump_page_tables(|_lvl, _idx, phys_frame, _flags| {
                dump_count += 1;
                if dump_count == 2 {
                    assert_eq!(phys_frame, 0x5000_0000);
                }
            });
        }
        assert_eq!(dump_count, 2);
    }
}
