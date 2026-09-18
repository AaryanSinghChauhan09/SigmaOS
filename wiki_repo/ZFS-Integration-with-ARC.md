# ZFS Integration with ARC

SigmaOS implements ZFS (Zettabyte File System) with ARC (Adaptive Replacement Cache) integration for advanced storage management with data integrity, compression, and snapshot capabilities.

## Overview

ZFS provides:
- Data integrity with end-to-end checksumming
- Copy-on-write (CoW) semantics
- Compression (lz4, zstd, gzip)
- RAID-Z protection (RAID-Z1, RAID-Z2, RAID-Z3)
- Snapshots and clones
- ARC (Adaptive Replacement Cache) for intelligent caching
- Scrub and resilver operations
- ZVOLs (ZFS volumes) for block devices

## Architecture

### ZFS Storage Pool
```
pool
├── vdev (virtual device)
│   ├── disk
│   ├── mirror
│   └── raidz
├── dataset (filesystem or volume)
│   ├── filesystem
│   └── volume (zvol)
└── snapshot
```

### ARC (Adaptive Replacement Cache)
- **MRU (Most Recently Used)**: Recently accessed data
- **MFU (Most Frequently Used)**: Frequently accessed data
- **Ghost lists**: Track evicted data for potential re-entry
- **Dynamic sizing**: Adjusts based on memory pressure

## Implementation

### ZFS Pool Manager
```rust
// src/storage/zfs/pool.rs
pub struct ZfsPool {
    pub name: String,
    pub vdevs: Vec<VirtualDevice>,
    pub state: PoolState,
    pub features: ZfsFeatures,
    pub arc: Arc<AdaptiveReplacementCache>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolState {
    Online,
    Degraded,
    Faulted,
    Offline,
}

#[derive(Debug, Clone)]
pub struct ZfsFeatures {
    pub compression: bool,
    pub deduplication: bool,
    pub encryption: bool,
    pub atime: bool,
    pub xattr: bool,
}

impl ZfsPool {
    pub fn create(name: &str, vdevs: Vec<VirtualDevice>) -> Result<Self, ZfsError> {
        let pool = ZfsPool {
            name: name.to_string(),
            vdevs,
            state: PoolState::Online,
            features: ZfsFeatures::default(),
            arc: Arc::new(AdaptiveReplacementCache::new()),
        };
        
        // Initialize vdevs
        for vdev in pool.vdevs.iter() {
            vdev.initialize()?;
        }
        
        // Create pool metadata
        pool.create_metadata()?;
        
        Ok(pool)
    }

    pub fn create_dataset(&mut self, name: &str) -> Result<ZfsDataset, ZfsError> {
        let dataset = ZfsDataset {
            name: name.to_string(),
            pool: self.name.clone(),
            mountpoint: format!("/{}", name),
            compression: CompressionType::Lz4,
            snapshots: Vec::new(),
        };
        
        // Create dataset metadata
        dataset.create_metadata()?;
        
        Ok(dataset)
    }

    pub fn scrub(&self) -> Result<(), ZfsError> {
        // Scan all blocks for checksum errors
        for vdev in self.vdevs.iter() {
            vdev.scrub()?;
        }
        Ok(())
    }
}
```

### ARC Implementation
```rust
// src/storage/zfs/arc.rs
pub struct AdaptiveReplacementCache {
    mru: ArcList,     // Most Recently Used
    mfu: ArcList,     // Most Frequently Used
    mru_ghost: ArcList,
    mfu_ghost: ArcList,
    size: AtomicUsize,
    max_size: usize,
    target_size: AtomicUsize,
}

#[derive(Debug, Clone)]
pub struct ArcEntry {
    pub key: u64,
    pub data: Vec<u8>,
    pub access_count: AtomicU32,
    pub size: usize,
}

impl AdaptiveReplacementCache {
    pub fn new() -> Self {
        AdaptiveReplacementCache {
            mru: ArcList::new(),
            mfu: ArcList::new(),
            mru_ghost: ArcList::new(),
            mfu_ghost: ArcList::new(),
            size: AtomicUsize::new(0),
            max_size: 8 * 1024 * 1024 * 1024, // 8GB default
            target_size: AtomicUsize::new(4 * 1024 * 1024 * 1024),
        }
    }

    pub fn lookup(&self, key: u64) -> Option<Vec<u8>> {
        // Check MRU first
        if let Some(entry) = self.mru.lookup(key) {
            entry.access_count.fetch_add(1, Ordering::Relaxed);
            self.promote_to_mfu(entry);
            return Some(entry.data.clone());
        }
        
        // Check MFU
        if let Some(entry) = self.mfu.lookup(key) {
            entry.access_count.fetch_add(1, Ordering::Relaxed);
            return Some(entry.data.clone());
        }
        
        None
    }

    pub fn insert(&self, key: u64, data: Vec<u8>) {
        let size = data.len();
        let entry = ArcEntry {
            key,
            data,
            access_count: AtomicU32::new(1),
            size,
        };
        
        // Evict if necessary
        while self.size.load(Ordering::Relaxed) + size > self.target_size.load(Ordering::Relaxed) {
            self.evict();
        }
        
        // Insert into MRU
        self.mru.insert(entry);
        self.size.fetch_add(size, Ordering::Relaxed);
    }

    fn evict(&self) {
        // Evict from MRU first
        if let Some(entry) = self.mru.pop_back() {
            self.mru_ghost.insert(entry.clone());
            self.size.fetch_sub(entry.size, Ordering::Relaxed);
            return;
        }
        
        // Evict from MFU
        if let Some(entry) = self.mfu.pop_back() {
            self.mfu_ghost.insert(entry.clone());
            self.size.fetch_sub(entry.size, Ordering::Relaxed);
            return;
        }
    }

    fn promote_to_mfu(&self, entry: ArcEntry) {
        if entry.access_count.load(Ordering::Relaxed) > 2 {
            self.mru.remove(&entry.key);
            self.mfu.insert(entry);
        }
    }
}
```

### Dataset Operations
```rust
// src/storage/zfs/dataset.rs
pub struct ZfsDataset {
    pub name: String,
    pub pool: String,
    pub mountpoint: String,
    pub compression: CompressionType,
    pub snapshots: Vec<ZfsSnapshot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Lz4,
    Zstd,
    Gzip,
}

impl ZfsDataset {
    pub fn create_snapshot(&mut self, name: &str) -> Result<ZfsSnapshot, ZfsError> {
        let snapshot = ZfsSnapshot {
            name: name.to_string(),
            dataset: self.name.clone(),
            created: std::time::SystemTime::now(),
            blocks: Vec::new(),
        };
        
        // Copy block pointers
        snapshot.capture_block_pointers()?;
        
        self.snapshots.push(snapshot);
        Ok(snapshot)
    }

    pub fn rollback(&mut self, snapshot: &ZfsSnapshot) -> Result<(), ZfsError> {
        // Restore block pointers from snapshot
        snapshot.restore_block_pointers()?;
        Ok(())
    }

    pub fn set_compression(&mut self, compression: CompressionType) {
        self.compression = compression;
    }

    pub fn write_block(&self, offset: u64, data: &[u8]) -> Result<(), ZfsError> {
        // Compress data if enabled
        let compressed = match self.compression {
            CompressionType::Lz4 => lz4_compress(data)?,
            CompressionType::Zstd => zstd_compress(data)?,
            CompressionType::Gzip => gzip_compress(data)?,
            CompressionType::None => data.to_vec(),
        };
        
        // Write compressed block with checksum
        let checksum = calculate_checksum(&compressed);
        write_block_with_checksum(offset, &compressed, checksum)?;
        
        Ok(())
    }
}
```

## Configuration

### ZFS Configuration
```toml
# /etc/sigmaos/zfs.toml
[pool]
# Default pool settings
compression = "lz4"
deduplication = false
atime = false
xattr = true

[arc]
# ARC cache settings
max_size = "8G"
target_size = "4G"
min_size = "1G"

[scrub]
# Scrub settings
enabled = true
interval_days = 30
priority = "normal"

[compression]
# Compression settings
lz4_level = 1
zstd_level = 3
```

### Runtime Control
```bash
# Create storage pool
sigzpool create mypool /dev/sda /dev/sdb

# Create dataset
sigzfs create mypool/data

# Set compression
sigzfs set compression=lz4 mypool/data

# Create snapshot
sigzfs snapshot mypool/data@backup

# Rollback to snapshot
sigzfs rollback mypool/data@backup

# List datasets
sigzfs list

# List snapshots
sigzfs list-snapshots mypool/data

# Start scrub
sigzpool scrub mypool

# View pool status
sigzpool status mypool

# View ARC statistics
sigzfs arc stats
```

## Performance Optimization

### ARC Tuning
Optimize ARC for your workload:
```bash
# Set ARC max size
sigzfs set arc_max=16G

# Set ARC target size
sigzfs set arc_target=12G

# Enable ARC prefetch
sigzfs set prefetch=true
```

### Compression
Choose compression based on workload:
- **lz4**: Fast compression, good for general use
- **zstd**: Better compression, higher CPU usage
- **gzip**: Best compression, highest CPU usage

### RAID-Z Configuration
Choose RAID-Z level based on requirements:
- **RAID-Z1**: Single parity (similar to RAID5)
- **RAID-Z2**: Double parity (similar to RAID6)
- **RAID-Z3**: Triple parity (highest redundancy)

## Troubleshooting

### Pool Degraded
If pool is degraded:
1. Check vdev status: `sigzpool status mypool`
2. Identify failed device
3. Replace failed device
4. Run resilver: `sigzpool resilver mypool`

### High ARC Memory Usage
If ARC uses too much memory:
1. Check ARC stats: `sigzfs arc stats`
2. Reduce ARC max: `sigzfs set arc_max=4G`
3. Reduce ARC target: `sigzfs set arc_target=3G`

### Scrub Taking Too Long
If scrub is slow:
1. Check scrub priority: `sigzpool status mypool`
2. Adjust scrub priority: `sigzpool set scrub_priority=low`
3. Monitor progress: `sigzpool scrub progress mypool`

### Snapshot Space Exhausted
If snapshot space is exhausted:
1. List snapshots: `sigzfs list-snapshots mypool/data`
2. Delete old snapshots: `sigzfs destroy mypool/data@old_snapshot`
3. Set snapshot limit: `sigzfs set snapshot_limit=10`

---

**[Storage & Filesystems](Category-Storage)** | **[Btrfs Subvolumes](Btrfs-Subvolumes)]** | **[Memory Management](Memory-Management)**
