# Universal Package Manager

SigmaOS Universal Package Manager (UPM) is a unified package management system that absorbs the best features from multiple package managers including apt, yum, pacman, snap, and flatpak into a single cohesive system.

## Features

### Multi-Format Support
- **Debian (.deb)**: Full compatibility with Debian package format
- **RPM (.rpm)**: Support for Red Hat package format
- **Pacman**: Arch Linux package format support
- **Snap**: Ubuntu Snap packages
- **Flatpak**: Universal Linux application format
- **SigmaPkg**: Native SigmaOS package format

### Dependency Conflict Resolution
The UPM includes intelligent dependency resolution with multiple strategies:
- **Prefer Newest**: Choose the most recent versions
- **Prefer Oldest**: Choose stable, older versions
- **Prefer Native**: Prioritize SigmaOS native packages
- **Manual**: User-controlled conflict resolution

### Package Adapters
Each package format has a dedicated adapter that handles:
- Installation
- Removal
- Updates
- Dependency management

## Architecture

```
UniversalPackageManager
├── Package Adapters (apt, yum, pacman, snap, flatpak, sigpkg)
├── Dependency Resolver
├── Conflict Detection
├── Package Registry
└── Installation Engine
```

## Usage

### Installing Packages
```rust
let mut manager = UniversalPackageManager::new();
let package = UnifiedPackage::new("nginx".to_string(), "1.18.0".to_string())
    .with_format(PackageFormat::Deb)
    .with_dependency("libc6".to_string());

manager.add_package(package);
manager.install("nginx")?;
```

### Searching Packages
```rust
let results = manager.search("nginx");
for pkg in results {
    println!("Found: {} - {}", pkg.name, pkg.version);
}
```

### Listing Installed Packages
```rust
let installed = manager.list_installed();
for pkg in installed {
    println!("{} - {}", pkg.name, pkg.version);
}
```

## Package Format Support

### Debian (apt)
- Full .deb package support
- Dependency resolution
- Repository integration

### RPM (yum)
- .rpm package support
- Dependency management
- Repository integration

### Pacman
- Arch Linux packages
- AUR support (planned)
- Dependency resolution

### Snap
- Snap package support
- Confinement handling
- Automatic updates

### Flatpak
- Flatpak runtime support
- Application sandboxing
- Dependency bundling

### SigmaPkg
- Native SigmaOS format
- Content-addressed storage
- Cryptographic verification
- Zero-dependency architecture

## Dependency Resolution

The dependency resolver uses a SAT solver approach to:
1. Build dependency graphs
2. Detect conflicts
3. Apply resolution strategies
4. Generate installation plans

### Conflict Detection
Conflicts are detected when:
- Packages declare conflicting dependencies
- Version constraints cannot be satisfied
- Circular dependencies exist

### Resolution Strategies
- **Automatic**: Apply configured strategy
- **Interactive**: Prompt user for decisions
- **Safe Mode**: Always prefer stability

## Security

- Cryptographic package verification
- Repository signing
- Dependency integrity checks
- Vulnerability scanning integration

## Integration

The Universal Package Manager integrates with:
- SigmaOS Security Framework
- Capability-based access control
- System resilience modules
- Update automation

## Future Enhancements

- Transactional updates
- Rollback capabilities
- Package version pinning
- Virtual package support
- Repository mirroring
- Cache optimization
- Delta updates
