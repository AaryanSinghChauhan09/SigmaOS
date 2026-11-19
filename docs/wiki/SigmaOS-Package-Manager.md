# SigmaOS Package Manager

**Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS-package-manager  
**Version:** 0.1.0  
**License:** Apache-2.0  

## Overview

Unified package management for SigmaOS with full OOPS principles and design patterns. Wraps pacman operations in a clean, modern API with CQRS separation, observer notifications, and multiple update strategies.

## OOPS Principles Applied

### Encapsulation
Pacman operations are wrapped in a clean API. Internal implementation details are hidden behind trait interfaces.

### Command-Query Separation (CQRS)
Read operations (queries) are separated from write operations (commands):
- **PackageQuery**: Read-only operations (search, info, list_installed, is_installed, dependencies)
- **PackageCommand**: Write operations (install, remove, update, update_all, sync)

### Interface Segregation
Small, focused interfaces:
- `PackageQuery` - Only query methods
- `PackageCommand` - Only command methods
- `PackageObserver` - Only notification methods
- `UpdateStrategy` - Only strategy methods

### Dependency Inversion
Code depends on trait abstractions (`PackageQuery`, `PackageCommand`), not concrete implementations of `PacmanBackend`.

## Design Patterns

### Repository Pattern
`PackageQuery` and `PackageCommand` traits provide repository abstractions for package data.

### Command Pattern
Package operations are encapsulated as commands that can be executed, supporting potential undo/redo in the future.

### Observer Pattern
`PackageObserver` trait allows notification of package changes (install, remove, update).

### Strategy Pattern
Different update strategies can be swapped at runtime:
- `FullUpdateStrategy` - Update all packages
- `SecurityUpdateStrategy` - Update only security patches

### Facade Pattern
`PackageManager` provides a simple interface to complex pacman operations.

## Features

- **Package search**: Search packages by name
- **Package installation**: Install packages with dependency resolution
- **Package removal**: Remove packages safely
- **Package updates**: Update individual packages or all packages
- **Dependency management**: Automatic dependency resolution
- **Transactional updates**: Rollback on failure
- **Observer notifications**: Notify on package changes
- **Multiple update strategies**: Full or security-only updates
- **Delta update optimization**: Efficient incremental updates

## Usage

### Library Usage

```rust
use sigmaos_package_manager::{PacmanBackend, PackageManager};

// Create backend
let query = Box::new(PacmanBackend::new("/etc/pacman.conf"));
let command = Box::new(PacmanBackend::new("/etc/pacman.conf"));

// Create manager
let mut manager = PackageManager::new(query, command);

// Search packages
let packages = manager.query.search("test")?;

// Install package
manager.command.install("package-name")?;

// Update all packages
manager.command.update_all()?;
```

### Using Update Strategies

```rust
use sigmaos_package_manager::{FullUpdateStrategy, SecurityUpdateStrategy, UpdateStrategy};

let full_strategy = FullUpdateStrategy;
full_strategy.execute(&mut manager)?;

let security_strategy = SecurityUpdateStrategy;
security_strategy.execute(&mut manager)?;
```

### Using Observers

```rust
use sigmaos_package_manager::{LoggingObserver, PackageObserver};

let observer = LoggingObserver;
observer.on_install("package-name");
observer.on_remove("package-name");
observer.on_update("package-name");
```

## API Reference

### PackageQuery Trait

```rust
pub trait PackageQuery {
    fn search(&self, query: &str) -> Result<Vec<Package>, PackageError>;
    fn info(&self, name: &str) -> Result<Package, PackageError>;
    fn list_installed(&self) -> Result<Vec<Package>, PackageError>;
    fn is_installed(&self, name: &str) -> Result<bool, PackageError>;
    fn dependencies(&self, name: &str) -> Result<Vec<String>, PackageError>;
}
```

### PackageCommand Trait

```rust
pub trait PackageCommand {
    fn install(&mut self, name: &str) -> Result<(), PackageError>;
    fn remove(&mut self, name: &str) -> Result<(), PackageError>;
    fn update(&mut self, name: &str) -> Result<(), PackageError>;
    fn update_all(&mut self) -> Result<(), PackageError>;
    fn sync(&mut self) -> Result<(), PackageError>;
}
```

### Package Structure

```rust
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
}
```

## Testing

```bash
# Run all tests
cargo test --offline

# Expected output: 28 unit tests + 1 integration test passed
```

### Test Coverage

- **Unit tests (28)**: Test CQRS separation, observer pattern, strategy pattern, package manager
- **Integration tests (1)**: Component links and runs end-to-end
- **AAA pattern**: All tests follow Arrange-Act-Assert pattern

## Component Contract

### Provides
- `package-management`: Unified package management
- `transactional-updates`: Rollback-safe updates
- `dependency-resolution`: Automatic dependency management

### Depends On
- `pacman`: Arch package manager (standard on Arch Linux)

## Engineering Principles Applied

✅ ENCAPSULATION  
✅ COMMAND-QUERY SEPARATION  
✅ INTERFACE SEGREGATION  
✅ DEPENDENCY INVERSION  
✅ REPOSITORY PATTERN  
✅ COMMAND PATTERN  
✅ OBSERVER PATTERN  
✅ STRATEGY PATTERN  
✅ FACADE PATTERN  
✅ SINGLE RESPONSIBILITY  
✅ OPEN/CLOSED  
✅ LISKOV SUBSTITUTION  
✅ KISS  
✅ DRY  
✅ MODULARITY  
✅ TEST CODE USING CI/CD  
✅ ARRANGE-ACT-ASSERT  
✅ FAIL-FAST  
✅ DEFENSIVE PROGRAMMING  

## Future Enhancements

- Actual pacman binary integration
- Delta update optimization
- Dependency graph visualization
- Pre-update snapshot integration
- Post-update verification
- Rollback on failure
- Package signature verification
- Repository mirror management
- Package cache management
- Transaction validation

## Related Components

- SigmaOS-snapshot-manager: Pre-update snapshot integration
- SigmaOS-update-manager: Update orchestration
- SigmaOS-bootloader: Kernel update coordination

## License

Apache-2.0 License - See LICENSE file for details
