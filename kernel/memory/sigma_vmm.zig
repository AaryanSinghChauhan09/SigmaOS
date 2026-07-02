// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/memory/sigma_vmm.zig — Virtual Memory Manager (x86-64 4-level paging)
// Replaces: sigma_vmm.cpp (C++ stub, removed)
//
// Language: Zig — direct hardware paging, comptime page-flag constants
// Pattern: struct with methods

// ── Page Table Flags ──────────────────────────────────────────────────────────

pub const PF_PRESENT:    u64 = 1 << 0;
pub const PF_WRITABLE:   u64 = 1 << 1;
pub const PF_USER:       u64 = 1 << 2;
pub const PF_WRITE_THRU: u64 = 1 << 3;
pub const PF_NO_CACHE:   u64 = 1 << 4;
pub const PF_ACCESSED:   u64 = 1 << 5;
pub const PF_DIRTY:      u64 = 1 << 6;
pub const PF_HUGE:       u64 = 1 << 7;
pub const PF_GLOBAL:     u64 = 1 << 8;
pub const PF_NX:         u64 = 1 << 63; // No-execute (W^X enforcement)

pub const PAGE_SIZE:   usize = 4096;
pub const PAGE_MASK:   u64   = ~@as(u64, PAGE_SIZE - 1);
pub const PHYS_MASK:   u64   = 0x000F_FFFF_FFFF_F000; // bits 12..51

// ── Address Helpers ───────────────────────────────────────────────────────────

/// Extract PML4 index (bits 39..47)
pub fn pml4_idx(va: u64) u9 { return @intCast((va >> 39) & 0x1FF); }
/// Extract PDPT index (bits 30..38)
pub fn pdpt_idx(va: u64) u9 { return @intCast((va >> 30) & 0x1FF); }
/// Extract PD index (bits 21..29)
pub fn pd_idx(va: u64)   u9 { return @intCast((va >> 21) & 0x1FF); }
/// Extract PT index (bits 12..20)
pub fn pt_idx(va: u64)   u9 { return @intCast((va >> 12) & 0x1FF); }

// ── Page Table Entry ──────────────────────────────────────────────────────────

pub const Pte = packed struct {
    raw: u64,

    pub fn new(phys: u64, flags: u64) Pte {
        return Pte{ .raw = (phys & PHYS_MASK) | flags };
    }
    pub fn phys(self: Pte) u64    { return self.raw & PHYS_MASK; }
    pub fn present(self: Pte) bool { return (self.raw & PF_PRESENT) != 0; }
    pub fn writable(self: Pte) bool { return (self.raw & PF_WRITABLE) != 0; }
    pub fn is_nx(self: Pte) bool   { return (self.raw & PF_NX) != 0; }
};

/// A page table (512 entries × 8 bytes = 4096 bytes = one page)
pub const PageTable = [512]Pte;

// ── VMM ───────────────────────────────────────────────────────────────────────

/// Physical memory allocator callback (provided by buddy allocator)
pub const PhysAllocFn = *const fn () u64; // returns physical page address

pub const Vmm = struct {
    pml4:       *PageTable,
    phys_alloc: PhysAllocFn,

    /// Create a new VMM backed by a fresh PML4 page
    pub fn init(pml4_phys: u64, alloc: PhysAllocFn) Vmm {
        const pml4: *PageTable = @ptrFromInt(pml4_phys);
        // Zero the PML4
        for (pml4) |*e| e.* = Pte{ .raw = 0 };
        return Vmm{ .pml4 = pml4, .phys_alloc = alloc };
    }

    /// Map virtual address `va` → physical `pa` with given flags
    pub fn map(self: *Vmm, va: u64, pa: u64, flags: u64) void {
        const pdpt = self.ensure_table(@intFromPtr(self.pml4), pml4_idx(va));
        const pd   = self.ensure_table(pdpt, pdpt_idx(va));
        const pt   = self.ensure_table(pd,   pd_idx(va));
        const pt_table: *PageTable = @ptrFromInt(pt);
        pt_table[pt_idx(va)] = Pte.new(pa, flags | PF_PRESENT);
    }

    /// Map `va` with W^X enforcement: writable XOR executable, never both
    pub fn map_wx(self: *Vmm, va: u64, pa: u64, writable: bool) void {
        var flags: u64 = PF_PRESENT | PF_USER;
        if (writable) {
            flags |= PF_WRITABLE | PF_NX; // writable → no-execute
        }
        // if not writable, NX not set → executable (read-only code)
        self.map(va, pa, flags);
    }

    /// Unmap virtual address
    pub fn unmap(self: *Vmm, va: u64) void {
        const pml4e = self.pml4[pml4_idx(va)];
        if (!pml4e.present()) return;
        const pdpt: *PageTable = @ptrFromInt(pml4e.phys());
        const pdpte = pdpt[pdpt_idx(va)];
        if (!pdpte.present()) return;
        const pd: *PageTable = @ptrFromInt(pdpte.phys());
        const pde = pd[pd_idx(va)];
        if (!pde.present()) return;
        const pt: *PageTable = @ptrFromInt(pde.phys());
        pt[pt_idx(va)] = Pte{ .raw = 0 };
        // Invalidate TLB entry
        asm volatile ("invlpg [%[va]]" : : [va] "r" (va) : "memory");
    }

    /// Walk page tables to resolve virtual → physical
    pub fn translate(self: *const Vmm, va: u64) ?u64 {
        const pml4e = self.pml4[pml4_idx(va)];
        if (!pml4e.present()) return null;
        const pdpt: *const PageTable = @ptrFromInt(pml4e.phys());
        const pdpte = pdpt[pdpt_idx(va)];
        if (!pdpte.present()) return null;
        const pd: *const PageTable = @ptrFromInt(pdpte.phys());
        const pde = pd[pd_idx(va)];
        if (!pde.present()) return null;
        const pt: *const PageTable = @ptrFromInt(pde.phys());
        const pte = pt[pt_idx(va)];
        if (!pte.present()) return null;
        return pte.phys() | (va & 0xFFF);
    }

    /// Load this address space into CR3
    pub fn activate(self: *const Vmm) void {
        asm volatile ("mov cr3, %[cr3]"
            : : [cr3] "r" (@intFromPtr(self.pml4)) : "memory");
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    fn ensure_table(self: *Vmm, parent_phys: u64, idx: u9) u64 {
        const parent: *PageTable = @ptrFromInt(parent_phys);
        if (!parent[idx].present()) {
            const child_phys = self.phys_alloc();
            // Zero the new table
            const child: *PageTable = @ptrFromInt(child_phys);
            for (child) |*e| e.* = Pte{ .raw = 0 };
            parent[idx] = Pte.new(child_phys,
                PF_PRESENT | PF_WRITABLE | PF_USER);
            return child_phys;
        }
        return parent[idx].phys();
    }
};
