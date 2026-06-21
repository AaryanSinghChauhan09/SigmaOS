# 📂 File System Notes

> SigmaOS implements filesystems entirely from scratch to avoid importing massive, tangled VFS layers from Linux. 

## 1. FAT32 Implementation
FAT32 is the bootstrap filesystem because UEFI standardizes on it.

**Core Algorithms:**
- **BPB Parsing:** Read LBA 0 of the partition, parse the BIOS Parameter Block to find `BytesPerSector`, `SectorsPerCluster`, `ReservedSectors`, and `NumFATs`.
- **Cluster Chain Traversal:** To read a file, read the directory entry to get the `FirstCluster`. Then, read the FAT array to find the next cluster in the chain until the End-Of-Cluster (EOC) marker `0x0FFFFFF8` is reached.
- **LBA Calculation:** `LBA = Partition_Start + ReservedSectors + (NumFATs * SectorsPerFAT) + (Cluster - 2) * SectorsPerCluster`.

*See: `fs/fat32/sigma_fat32.cpp`*

## 2. ext2 Implementation
Ext2 provides inodes and a more robust UNIX-like structure.

**Core Algorithms:**
- **Superblock Location:** The superblock is always at byte offset 1024 from the start of the partition. 
- **Block Groups:** The disk is divided into Block Groups. The Block Group Descriptor Table (BGDT) immediately follows the superblock.
- **Inodes:** Each file/directory is an inode. To traverse `/usr/bin/shell`, you start at Inode 2 (Root Directory), read its data blocks to find the entry for `usr`, get its inode, and repeat.

*See: `fs/ext2/sigma_ext2.cpp`*

## 3. Future: Sovereign ZFS (S-ZFS)
SigmaOS plans to implement a Copy-On-Write (COW) storage pool system inspired by ZFS, but simplified.
- O(1) snapshots using root tree cloning.
- Inline deduplication via Dilithium-5 hashing (or SHA-256 fallback).
