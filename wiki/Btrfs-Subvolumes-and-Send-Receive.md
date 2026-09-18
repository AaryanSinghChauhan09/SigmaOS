# Btrfs Subvolumes and Send/Receive

SigmaOS implements Btrfs (B-tree File System) with subvolume support, copy-on-write semantics, snapshot management, and send/receive for efficient data replication.

## Overview

Btrfs provides:
- Copy-on-write (CoW) filesystem
- Subvolumes (lightweight filesystems)
- Snapshots (instantaneous filesystem copies)
- Send/Receive (incremental data transfer)
- Compression (zstd, lzo, zlib)
- RAID support (RAID0, RAID1, RAID10, RAID5, RAID6)
- Scrub for data integrity
- Quota management

## Architecture

### Btrfs Structure
```
btrfs filesystem
├── subvolume 1 (root)
├── subvolume 2 (home)
├── subvolume 3 (var)
└── snapshots
    ├── snapshot 1
    ├── snapshot 2
    └── snapshot 3
```

### Send/Receive Pipeline
- **Send**: Stream snapshot changes as binary stream
- **Receive**: Apply received stream to destination
- **Incremental**: Only send changed blocks
- **Compression**: Compress send stream

## Implementation

### Subvolume Manager
```rust
// src/storage/btrfs/subvolume.rs
pub struct BtrfsSubvolume {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub name: String,
    pub path: String,
    pub uuid: Uuid,
    pub compression: CompressionType,
    pub quota: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Zstd,
    Lzo,
    Zlib,
}

impl BtrfsSubvolume {
    pub fn create(path: &str) -> Result<Self, BtrfsError> {
        let subvol = BtrfsSubvolume {
            id: generate_subvolume_id(),
            parent_id: None,
            name: path.to_string(),
            path: path.to_string(),
            uuid: Uuid::new_v4(),
            compression: CompressionType::Zstd,
            quota: None,
        };
        
        // Create subvolume metadata
        subvol.create_metadata()?;
        
        Ok(subvol)
    }

    pub fn snapshot(&self, name: &str) -> Result<BtrfsSnapshot, BtrfsError> {
        let snapshot = BtrfsSnapshot {
            id: generate_snapshot_id(),
            parent_id: Some(self.id),
            name: name.to_string(),
            path: format!("{}/{}", self.path, name),
            uuid: Uuid::new_v4(),
            created: std::time::SystemTime::now(),
            blocks: Vec::new(),
        };
        
        // Copy block pointers (CoW)
        snapshot.capture_block_pointers()?;
        
        Ok(snapshot)
    }

    pub fn set_compression(&mut self, compression: CompressionType) {
        self.compression = compression;
    }

    pub fn set_quota(&mut self, quota: u64) {
        self.quota = Some(quota);
    }
}
```

### Send/Receive Implementation
```rust
// src/storage/btrfs/send_receive.rs
pub struct BtrfsSend {
    subvolume: BtrfsSubvolume,
    parent: Option<BtrfsSnapshot>,
    compression: bool,
}

pub struct BtrfsReceive {
    subvolume: BtrfsSubvolume,
    received_blocks: u64,
}

impl BtrfsSend {
    pub fn new(subvolume: BtrfsSubvolume, parent: Option<BtrfsSnapshot>) -> Self {
        BtrfsSend {
            subvolume,
            parent,
            compression: true,
        }
    }

    pub fn send(&self) -> Result<Vec<u8>, BtrfsError> {
        let mut stream = Vec::new();
        
        // Write header
        self.write_header(&mut stream)?;
        
        // Write commands
        if let Some(parent) = &self.parent {
            self.write_incremental_commands(parent, &mut stream)?;
        } else {
            self.write_full_commands(&mut stream)?;
        }
        
        // Compress if enabled
        if self.compression {
            stream = zstd_compress(&stream)?;
        }
        
        Ok(stream)
    }

    fn write_header(&self, stream: &mut Vec<u8>) -> Result<(), BtrfsError> {
        // Write magic number
        stream.extend_from_slice(b"btrfs-stream");
        
        // Write version
        stream.write_all(&1u32.to_le_bytes())?;
        
        // Write UUID
        stream.extend_from_slice(self.subvolume.uuid.as_bytes());
        
        Ok(())
    }

    fn write_incremental_commands(&self, parent: &BtrfsSnapshot, stream: &mut Vec<u8>) -> Result<(), BtrfsError> {
        // Compare blocks between parent and current
        let changed_blocks = self.compare_blocks(parent)?;
        
        // Write write commands for changed blocks
        for block in changed_blocks {
            self.write_write_command(block, stream)?;
        }
        
        Ok(())
    }
}

impl BtrfsReceive {
    pub fn new(subvolume: BtrfsSubvolume) -> Self {
        BtrfsReceive {
            subvolume,
            received_blocks: 0,
        }
    }

    pub fn receive(&mut self, stream: &[u8]) -> Result<(), BtrfsError> {
        let stream = if is_compressed(stream) {
            zstd_decompress(stream)?
        } else {
            stream.to_vec()
        };
        
        // Parse header
        self.parse_header(&stream)?;
        
        // Parse and execute commands
        self.parse_commands(&stream)?;
        
        Ok(())
    }

    fn parse_commands(&mut self, stream: &[u8]) -> Result<(), BtrfsError> {
        let mut offset = 0;
        
        while offset < stream.len() {
            let command = self.parse_command(&stream[offset..])?;
            self.execute_command(&command)?;
            offset += command.size;
        }
        
        Ok(())
    }

    fn execute_command(&mut self, command: &SendCommand) -> Result<(), BtrfsError> {
        match command.cmd_type {
            CommandType::Write => {
                self.write_block(command.offset, &command.data)?;
                self.received_blocks += 1;
            }
            CommandType::Clone => {
                self.clone_block(command.offset, command.source)?;
            }
            CommandType::Truncate => {
                self.truncate(command.offset)?;
            }
        }
        Ok(())
    }
}
```

### Quota Management
```rust
// src/storage/btrfs/quota.rs
pub struct BtrfsQuota {
    pub subvolume_id: u64,
    pub max_bytes: u64,
    pub used_bytes: AtomicU64,
}

impl BtrfsQuota {
    pub fn check(&self, additional_bytes: u64) -> Result<(), BtrfsError> {
        let current = self.used_bytes.load(Ordering::Relaxed);
        if current + additional_bytes > self.max_bytes {
            return Err(BtrfsError::QuotaExceeded);
        }
        Ok(())
    }

    pub fn allocate(&self, bytes: u64) -> Result<(), BtrfsError> {
        self.check(bytes)?;
        self.used_bytes.fetch_add(bytes, Ordering::Relaxed);
        Ok(())
    }

    pub fn free(&self, bytes: u64) {
        self.used_bytes.fetch_sub(bytes, Ordering::Relaxed);
    }
}
```

## Configuration

### Btrfs Configuration
```toml
# /etc/sigmaos/btrfs.toml
[subvolume]
# Default subvolume settings
compression = "zstd"
compression_level = 3
autodefrag = true

[quota]
# Quota settings
enabled = true
default_limit = "100G"

[scrub]
# Scrub settings
enabled = true
interval_days = 30
priority = "normal"

[send_receive]
# Send/Receive settings
compression = true
buffer_size = "1G"
```

### Runtime Control
```bash
# Create subvolume
sigbtrfs subvolume create /home

# Create snapshot
sigbtrfs snapshot /home /home/backup

# List subvolumes
sigbtrfs subvolume list

# List snapshots
sigbtrfs snapshot list /home

# Set compression
sigbtrfs set compression=zstd /home

# Set quota
sigbtrfs quota set /home 50G

# Send snapshot to stream
sigbtrfs send /home/backup > backup.stream

# Receive from stream
sigbtrfs receive /home/new < backup.stream

# Start scrub
sigbtrfs scrub start

# View quota usage
sigbtrfs quota usage /home
```

## Performance Optimization

### Compression Tuning
Choose compression based on workload:
- **zstd**: Best compression ratio, moderate CPU
- **lzo**: Fast compression, lower ratio
- **zlib**: Good balance, deprecated for zstd

### Send/Receive Optimization
Optimize for faster replication:
```bash
# Increase buffer size
sigbtrfs set send_buffer_size=2G

# Disable compression for local transfers
sigbtrfs send --no-compress /home/backup

# Use incremental sends
sigbtrfs send -p /home/previous /home/current > incremental.stream
```

### Defragmentation
Enable auto-defrag for performance:
```bash
# Enable auto-defrag
sigbtrfs set autodefrag=true /home

# Manual defrag
sigbtrfs defrag /home
```

## Troubleshooting

### Out of Space
If subvolume is out of space:
1. Check quota: `sigbtrfs quota usage /home`
2. Increase quota: `sigbtrfs quota set /home 200G`
3. Delete old snapshots: `sigbtrfs snapshot delete /home/old_snapshot`
4. Check disk usage: `df -h`

### Send/Receive Fails
If send/receive fails:
1. Check subvolume UUID: `sigbtrfs subvolume show /home`
2. Verify parent snapshot exists
3. Check for incompatibilities: `sigbtrfs check /home`
4. Try full send instead of incremental

### Scrub Takes Too Long
If scrub is slow:
1. Check scrub priority: `sigbtrfs scrub status`
2. Adjust priority: `sigbtrfs scrub set priority=low`
3. Monitor progress: `sigbtrfs scrub progress`

### Performance Degradation
If performance degrades:
1. Check fragmentation: `sigbtrfs defrag status /home`
2. Run defrag: `sigbtrfs defrag /home`
3. Check compression settings
4. Consider disabling compression for SSDs

---

**[Storage & Filesystems](Category-Storage)** | **[ZFS Integration](ZFS-Integration-with-ARC)** | **[Data Replication](Data-Replication)**
