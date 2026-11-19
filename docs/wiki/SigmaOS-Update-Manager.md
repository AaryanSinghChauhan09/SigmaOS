# SigmaOS Update Manager

**Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS-update-manager  
**Version:** 0.1.0  
**License:** MIT  

## Overview

Rolling release update orchestration for SigmaOS with Observer, State, and Strategy patterns. Manages automated updates with pre-update snapshots, post-update verification, and rollback on failure.

## OOPS Principles Applied

### Observer Pattern
Notify subscribers on update state changes through the `UpdateObserver` trait.

### State Pattern
Update states are managed through the `UpdateState` enum:
- Idle
- Checking
- Downloading
- Installing
- Verifying
- Completed
- Failed

### Strategy Pattern
Different update strategies can be swapped at runtime:
- `FullUpdateStrategy` - Update all packages
- `SecurityUpdateStrategy` - Update only security patches

## Design Patterns

- **Observer Pattern**: Notify on update state changes
- **State Pattern**: Manage update lifecycle states
- **Strategy Pattern**: Interchangeable update strategies

## Features

- **Automated updates with scheduling**: Scheduled update execution
- **Pre-update snapshot creation**: Create BTRFS snapshots before updates
- **Post-update verification**: Verify system integrity after updates
- **Rollback on failure**: Automatic rollback if update fails
- **State notifications**: Real-time state change notifications

## Usage

### Command Line

```bash
# Full update
sigmaos-update-manager full

# Security-only update
sigmaos-update-manager security
```

### Library Usage

```rust
use sigmaos_update_manager::{UpdateManager, FullUpdateStrategy, LoggingObserver};

let mut manager = UpdateManager::new();
manager.add_observer(Box::new(LoggingObserver));
manager.execute_update(Box::new(FullUpdateStrategy))?;
```

## API Reference

### UpdateState Enum

```rust
pub enum UpdateState {
    Idle,
    Checking,
    Downloading,
    Installing,
    Verifying,
    Completed,
    Failed(String),
}
```

### UpdateObserver Trait

```rust
pub trait UpdateObserver {
    fn on_update(&self, event: &UpdateEvent);
}
```

### UpdateStrategy Trait

```rust
pub trait UpdateStrategy {
    fn execute(&self, context: &mut UpdateContext) -> Result<(), String>;
}
```

## Testing

```bash
cargo test --offline
```

### Test Coverage

- **Unit tests (4)**: Test state pattern, observer pattern, strategy pattern, update manager
- **AAA pattern**: All tests follow Arrange-Act-Assert pattern

## Component Contract

### Provides
- `update-orchestration`: Rolling release update management
- `pre-update-snapshot`: Snapshot creation before updates
- `post-update-verification`: System verification after updates

### Depends On
- `sigmaos-package-manager`: Package installation
- `sigmaos-snapshot-manager`: BTRFS snapshot management

## Engineering Principles Applied

✅ OBSERVER PATTERN  
✅ STATE PATTERN  
✅ STRATEGY PATTERN  
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

- Actual package manager integration
- BTRFS snapshot integration
- Scheduled updates via cron/systemd
- Update notification UI
- Rollback UI
- Update history tracking
- Dependency graph visualization
- Delta update optimization

## Related Components

- SigmaOS-package-manager: Package installation
- SigmaOS-snapshot-manager: BTRFS snapshot management
- SigmaOS-bootloader: Kernel update coordination

## License

MIT License - See LICENSE file for details
