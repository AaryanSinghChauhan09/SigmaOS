# SigmaOS DMA Engine & IOMMU

## Overview

SigmaOS provides a sovereign DMA subsystem with IOMMU isolation, scatter-gather support, and coherent buffer allocation. Inspired by Linux DMA API and FreeBSD busdma(9).

**Location:** `src/kernel/sigma_dma.rs`

---

## Architecture

```
Device                IOMMU                    Physical RAM
──────               ────────                 ─────────────
Device sees    ←→   IOVA→PA      ←→       Actual pages
DMA addresses      page table            (can be fragmented)
(IOVA)
```

---

## IOMMU Domains

Each device gets its own IOMMU domain with a separate IOVA (I/O Virtual Address) space. This prevents DMA attacks between devices.

```rust
let mut dma = SigmaDmaSubsystem::new();

// Allocate coherent buffer (always accessible to device)
let buf = dma.alloc_coherent(65536, device_id).unwrap();
println!("Device DMA addr: 0x{:x}", buf.dma_addr.0);

// Manual IOMMU mapping
let domain_id = dma.create_domain(device_id);
let iova = dma.iommu_map(domain_id, phys_addr, 4096, true, false, device_id).unwrap();
let phys = dma.iommu_translate(domain_id, iova).unwrap();

// DMA transfer
dma.submit_transfer(0, src_addr, dst_addr, 4096, DmaDirection::ToDevice).unwrap();
dma.flush_all();
```

---

## Scatter-Gather

```rust
let mut sgl = ScatterGatherList::new(DmaDirection::FromDevice);
sgl.add_entry(DmaAddr(0x1000), 4096, 0);
sgl.add_entry(DmaAddr(0x3000), 4096, 0);
// Submit each entry as a DMA channel transfer
```

---

## Comparison

| Feature | Linux DMA API | FreeBSD busdma | SigmaOS |
|---------|--------------|---------------|---------|
| IOMMU domains | Yes (iommu_domain) | No | Yes |
| Coherent alloc | dma_alloc_coherent | bus_dma_mem_alloc | Yes |
| Scatter-gather | sg_table | bus_dmamap | Yes |
| IOVA allocation | Yes | No | Yes |
| no_std | No | No | **Yes** |
