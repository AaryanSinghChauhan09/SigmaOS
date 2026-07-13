# Distro Absorption: ChromeOS

> **Status**: 📋 Planned | **Source Paradigm**: ChromeOS (Google) | **Target Shard**: `SigmaOS Verified Boot & Recovery Layer`

---

## 1. Executive Summary

ChromeOS is a lightweight, heavily locked-down operating system built around the Chrome browser, designed for low-maintenance consumer and enterprise devices. Despite its simplicity, it introduced several groundbreaking system architecture concepts that have since influenced the wider Linux ecosystem.

SigmaOS absorbs two core innovations from ChromeOS:
- **A/B Partitioning (Seamless Updates)**: Background updates with instant rollback on failure.
- **Verified Boot (vboot)**: Cryptographic verification of every stage from firmware to rootfs.

---

## 2. Key Features to Absorb

### 2.1 A/B Partitioning for Zero-Downtime Updates

Instead of modifying the live system, SigmaOS utilizes an A/B partition scheme for the immutable core rootfs. Updates are downloaded and written to the inactive partition in the background.

```
┌────────────────────────────────────────────────────────┐
│                   SIGMA STORAGE LAYOUT                 │
│                                                        │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────┐  │
│  │ Rootfs Slot A  │ │ Rootfs Slot B  │ │ Stateful   │  │
│  │ (Active, v1.0) │ │ (Updating...)  │ │ Partition  │  │
│  │ Read-only      │ │ Read-write     │ │ Read-write │  │
│  └────────────────┘ └────────────────┘ └────────────┘  │
└────────────────────────────────────────────────────────┘
```

```bash
$ sigma update status
Σ [UPDATE] Background update downloading...
  Target: SigmaOS Zenith v15.1
  Writing to: Slot B
  Progress: 85% [████████░░]

$ sigma update apply
Σ [UPDATE] Update staged to Slot B. 
  Next boot will load from Slot B.
  If boot fails, system automatically falls back to Slot A.
```

### 2.2 Verified Boot (dm-verity)

Every block of the immutable root filesystem is hashed into a Merkle tree. The root hash is signed by the OS vendor and verified by the bootloader. If a single bit is modified offline, the kernel refuses to read the block (`dm-verity`), ensuring the OS is completely tamper-proof.

```rust
// kernel/block/verity.rs
// SPDX-License-Identifier: MIT

pub struct VerityTarget {
    pub data_device: BlockDevice,
    pub hash_device: BlockDevice,
    pub root_digest: [u8; 32], // SHA-256 root of Merkle tree
}

impl VerityTarget {
    /// Read a block, verifying its hash up the Merkle tree
    pub fn read_block_verified(&self, lba: u64, buf: &mut [u8]) -> Result<()> {
        self.data_device.read(lba, buf)?;
        
        let hash = sha256(buf);
        let expected_hash = self.get_hash_from_tree(lba)?;
        
        if hash != expected_hash {
            panic!("SECURITY VIOLATION: Block {} fails verity check!", lba);
        }
        Ok(())
    }
}
```

### 2.3 Powerwash (Factory Reset)

ChromeOS's "Powerwash" feature allows the user to cryptographically obliterate all personal data and return the device to a pristine state in seconds. SigmaOS achieves this by securely discarding the encryption keys for the stateful partition.

```bash
$ sigma system powerwash
Σ [SECURITY] WARNING: This will permanently delete all user data.
  Type 'OBLITERATE' to confirm: OBLITERATE
  
Σ [RESET] Destroying LUKS master key for /dev/mapper/stateful...
  Key destroyed.
  Rebooting into pristine factory state...
```

---

## 3. References & Standards

- Chromium OS Docs — `chromium.googlesource.com/chromiumos/docs`
- dm-verity — Linux Kernel Documentation
