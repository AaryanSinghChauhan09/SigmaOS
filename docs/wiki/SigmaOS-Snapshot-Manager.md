# SigmaOS Snapshot Manager

**Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS-snapshot-manager  
**Version:** 0.1.0  
**License:** MIT  

## Overview

BTRFS snapshot lifecycle management for SigmaOS with Command, Factory, and Singleton patterns. Manages pre-update snapshots, manual snapshots, and automatic cleanup.

## OOPS Principles Applied

### Command Pattern
Snapshot operations are encapsulated as commands that can be executed and potentially undone:
- `CreateSnapshotCommand` - Create a new snapshot
- `DeleteSnapshotCommand` - Delete an existing snapshot

### Factory Pattern
Different snapshot types can be created through specialized factories:
- `PreUpdateSnapshotFactory` - Creates snapshots before updates
- `ManualSnapshotFactory` - Creates manual user snapshots

### Singleton Pattern
Single snapshot manager instance ensures consistent state across the system.

## Design Patterns

- **Command Pattern**: Snapshot operations as commands
- **Factory Pattern**: Create different snapshot types
- **Singleton Pattern**: Single snapshot manager instance

## Features

- **Pre-update snapshots**: Automatic snapshot creation before updates
- **Manual snapshots**: User-initiated snapshots with descriptions
- **Automatic cleanup**: Retention policy to manage disk space
- **Snapshot boot selection**: Boot from specific snapshots
- **Undo support**: Commands support undo operations

## Usage

### Command Line

```bash
# Pre-update snapshot
sigmaos-snapshot-manager pre-update "Before kernel update"

# Manual snapshot
sigmaos-snapshot-manager manual "Manual backup"
```

### Library Usage

```rust
use sigmaos_snapshot_manager::{SnapshotManager, PreUpdateSnapshotFactory};

let mut manager = SnapshotManager::new();
let factory = PreUpdateSnapshotFactory;
let command = factory.create_snapshot("Update", "/btrfs");
manager.execute_command(command)?;
```

## API Reference

### SnapshotCommand Trait

```rust
pub trait SnapshotCommand {
    fn execute(&self) -> Result<Snapshot, String>;
    fn undo(&self) -> Result<(), String>;
}
```

### SnapshotFactory Trait

```rust
pub trait SnapshotFactory {
    fn create_snapshot(&self, description: &str, path: &str) -> Box<dyn SnapshotCommand>;
}
```

### Snapshot Structure

```rust
pub struct Snapshot {
    pub id: String,
    pub description: String,
    pub timestamp: u64,
    pub path: String,
}
```

## Testing

```bash
cargo test --offline
```

### Test Coverage

- **Unit tests (4)**: Test command pattern, factory pattern, snapshot manager, cleanup
- **AAA pattern**: All tests follow Arrange-Act-Assert pattern

## Component Contract

### Provides
- `snapshot-management`: BTRFS snapshot lifecycle
- `pre-update-snapshots`: Automatic snapshots before updates
- `snapshot-cleanup`: Retention policy management

### Depends On
- `btrfs-progs`: BTRFS filesystem tools

## Engineering Principles Applied

✅ COMMAND PATTERN  
✅ FACTORY PATTERN  
✅ SINGLETON PATTERN  
✅ SINGLE RESPONSIBILITY  
✅ OPEN/CLOSED  
✅ KISS  
✅ DRY  
✅ MODULARITY  
✅ TEST CODE USING CI/CD  
✅ ARRANGE-ACT-ASSERT  
✅ FAIL-FAST  
✅ DEFENSIVE PROGRAMMING  

## Future Enhancements

- Actual BTRFS command integration
- Snapshot boot menu integration
- Snapshot compression
- Remote snapshot storage
- Snapshot verification
- Differential snapshots
- Snapshot scheduling
- Snapshot retention policies

## Related Components

- SigmaOS-update-manager: Pre-update snapshot integration
- SigmaOS-package-manager: Update coordination
- SigmaOS-bootloader: Snapshot boot selection

## License

MIT License - See LICENSE file for details
