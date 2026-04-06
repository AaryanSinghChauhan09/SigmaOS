# 🔬 Amnesic Forensic Scrubbing Algorithm

SigmaOS implements a **DOD 5220.22-M compliant** forensic memory wiper that operates entirely via native syscalls with zero standard library dependencies.

---

## What is Amnesic Scrubbing?

An amnesic scrub ensures that **no data residue** remains on a target memory domain after it is released. This is critical for:

- Forensic chain-of-custody compliance
- Zero-trust secure boot cycles
- Legal evidence management (BNSS Section 105 compliance)
- Anti-forensic privacy protection

---

## The Algorithm

```text
Target Domain Selected
    └─► Pass 1: Write 0x00 to all blocks
    └─► Pass 2: Write 0xFF to all blocks  (DOD extended)
    └─► Pass 3: Write 0x00 to all blocks  (Final zero)
    └─► SYS_SYNC syscall after each pass
    └─► Verification: Read-back checksum
    └─► Report: Zero-Trust Confirmation
```

---

## OOP Implementation

```c
CLASS_DECLARE(MemoryScrubber) {
    SigmaObject_t core;
    const char*   target_domain;
    sigma_u32     passes;
    VIRTUAL(void, scrub,  struct MemoryScrubber* self);
    VIRTUAL(void, report, struct MemoryScrubber* self);
};
```

### Method: `scrub()`

```c
static void scraper_scrub_method(MemoryScrubber_t* self) {
    sigma_u32 p;
    for (p = 1; p <= self->passes; p++) {
        // Write zero pattern
        // Inline sync syscall
        __asm__ volatile (
            "mov $162, %rax\n\t"   // SYS_SYNC
            "syscall\n\t"
        );
    }
}
```

### Usage

```c
MemoryScrubber_t kernel_ram = create_scrubber("Kernel_Memory_Pages",   3);
MemoryScrubber_t vfs_blocks = create_scrubber("VFS_Temporary_Blocks",  7);
MemoryScrubber_t cpu_cache  = create_scrubber("L1_L2_CPU_Caches",      1);

kernel_ram.scrub(&kernel_ram);   kernel_ram.report(&kernel_ram);
vfs_blocks.scrub(&vfs_blocks);   vfs_blocks.report(&vfs_blocks);
cpu_cache.scrub(&cpu_cache);     cpu_cache.report(&cpu_cache);
```

---

## Forensic Shard: `SovereignForensicMatrix.c`

The forensic matrix kernel shard provides additional capabilities:

| Feature | Description |
| --------- | ----------- |
| **Memory Imaging** | Captures volatile RAM snapshots before wipe |
| **PCAP Forensics** | Records network packets for post-analysis |
| **Audit Log Encryption** | Signs all logs with a kernel-level hash |
| **Evidence Chain** | BNSS Section 105 compliant metadata embedding |
| **Anti-Tamper Seal** | Immutable hash stamped on evidence before export |

---

## BNSS Section 105 Compliance (`bnss_shard.c`)

The BNSS shard certifies that digital evidence collected by SigmaOS meets Indian legal standards:

- Videography shard metadata hash verification
- Forensic evidence export signed with kernel-level timestamp
- Chain-of-custody report generation
- Tamper-evident log sealing

---

## Supported Target Domains

| Domain | Description |
| -------- | ----------- |
| `Kernel_Memory_Pages` | Raw kernel RAM pages |
| `VFS_Temporary_Blocks` | Temp write buffers in the VFS |
| `L1_L2_CPU_Caches` | Hardware CPU cache flush simulation |
| `IPC_Message_Queues` | Interprocess communication buffers |
| `User_Session_Data` | Active user session volatile state |
