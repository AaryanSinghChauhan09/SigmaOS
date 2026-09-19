# 📄 AI Agents Paging Management Specification (`docs/AI_AGENTS_PAGING_MANAGEMENT.md`)

This specification defines virtual memory paging architecture, page table translation, TLB invalidation protocols, and page fault recovery policies for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) in SigmaOS.

---

## 1. Multi-Level Page Translation Architecture (`src/kernel/paging.rs`, `src/klib/paging.rs`)

AI agents manage multi-architecture page table structures:
- **x86_64**: 4-level PML4 (CR3 register base) and 5-level PML5 (57-bit virtual addressing).
- **AArch64**: Dual translation tables (TTBR0 for userspace, TTBR1 for kernel space, 48-bit VA).
- **RISC-V 32/64**: Sv32 (2-level), Sv39 (3-level), and Sv48 (4-level) SATP translation modes.

---

## 2. Virtual Memory Manager (VMM) & TLB Operations (`src/kernel/vmm_paging.rs`)

- **Page Table Mapping**: Allocation and deallocation of physical frame descriptors backing virtual address ranges.
- **TLB Shootdowns**: Inter-Processor Interrupt (IPI) broadcast protocol invalidating stale TLB entries across SMP CPU cores.
- **Huge Pages**: Support for 2MB and 1GB huge page allocations reducing TLB miss penalties in database and AI workloads.

---

## 3. Page Fault Exceptions & IRQL Paging Invariants

- **Page Fault Handler (`HardwareException::PageFault`)**: Extracts faulting virtual address from `CR2` register and resolves page presence, write violations, or user/supervisor privilege mismatches.
- **IRQL Paging Rules**:
  - `NonPagedPool`: Resident in physical RAM. Accessible at all IRQL levels (`IRQL >= PassiveLevel`).
  - `PagedPool`: Swappable to backing store. Accessing `PagedPool` at `IRQL >= DispatchLevel` triggers an unrecoverable `DoubleFault` (`PAGE_FAULT_IN_NONPAGED_AREA`).

---

## 4. AI Agent Paging Responsibilities

- **⚡ Bolt**: Profiles page walk latencies, monitors TLB miss rates, and optimizes huge page frame distribution.
- **🎨 Palette**: Visualizes virtual address space mappings and process memory page distribution in system diagnostic tools.
- **🛡️ Sentinel**: Audits page table flags for W^X (Write XOR Execute) memory enforcement, No-Execute (`NX`/`XN`) bit bits, and privilege level separation.
