# Software Package Manager

## Overview

The Software Package Manager provides comprehensive software management inspired by Linux Mint's Software Manager and Omarchy's software utilities. It supports package search, installation, removal, repository management, and categorization.

## Features

- **Package Status**: Installed, Not Installed, Update Available, Pinned
- **Package Categories**: System, Development, Multimedia, Network, Graphics, Office, Games, Education, Utility
- **Package Management**: Install, remove, update, pin packages
- **Repository Management**: Add, enable, disable repositories
- **Package Search**: Search by name and description
- **Category Filtering**: List packages by category
- **Status Filtering**: List packages by status
- **Package Metadata**: Version, size, rating, description
- **Default Configuration**: Pre-configured repositories and packages
- **Statistics**: Track package and repository counts

## Components

### AppPackageStatus

```rust
pub enum AppPackageStatus {
    Installed,         // Package is installed
    NotInstalled,     // Package is not installed
    UpdateAvailable,  // Update is available
    Pinned,           // Package is pinned (won't be updated)
}
```

### AppPackageCategory

```rust
pub enum AppPackageCategory {
    System,       // System packages
    Development,  // Development tools
    Multimedia,   // Multimedia applications
    Network,      // Network applications
    Graphics,     // Graphics applications
    Office,       // Office applications
    Games,        // Games
    Education,    // Education applications
    Utility,      // Utility applications
}
```

### AppPackage

Represents a software package with:
- Unique package ID
- Package name
- Version
- Description
- Category
- Status
- Size (bytes)
- Rating (0-5)

### SoftwarePackageRepository

Represents a software repository with:
- Unique repository ID
- Repository name
- Repository URL
- Enabled flag

### SoftwarePackageManager

Main management interface with:
- Package addition and listing
- Package search and filtering
- Install/remove/update/pin operations
- Repository management
- Enable/disable repositories
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::package::SoftwarePackageManager;

let manager = SoftwarePackageManager::new();

// List all packages
let packages = manager.list_packages();
for pkg in packages {
    println!("{}: {} ({})", pkg.name, pkg.version, pkg.status.as_str());
}
```

### Package Management

```rust
// Add a package
let pkg = AppPackage::new(
    "myapp".to_string(),
    "My App".to_string(),
    "1.0.0".to_string(),
    "My application".to_string(),
    AppPackageCategory::Utility,
);
manager.add_package(pkg);

// Install a package
manager.install("sigma-terminal")?;

// Remove a package
manager.remove("sigma-terminal")?;

// Update a package
manager.update("sigma-terminal")?;

// Pin a package
manager.pin("sigma-terminal")?;
```

### Package Search

```rust
// Search packages
let results = manager.search("terminal");
for pkg in results {
    println!("{}: {}", pkg.name, pkg.description);
}

// List by category
let system = manager.list_by_category(AppPackageCategory::System);
let dev = manager.list_by_category(AppPackageCategory::Development);

// List by status
let installed = manager.list_by_status(AppPackageStatus::Installed);
let updates = manager.list_by_status(AppPackageStatus::UpdateAvailable);
```

### Repository Management

```rust
// Add a repository
let repo = SoftwarePackageRepository::new(
    "extra".to_string(),
    "Extra Repository".to_string(),
    "https://packages.sigmaos.org/extra".to_string(),
);
manager.add_repository(repo);

// Enable/disable a repository
manager.enable_repository("extra")?;
manager.disable_repository("extra")?;

// List repositories
let repos = manager.list_repositories();
```

### Package Metadata

```rust
let mut pkg = AppPackage::new(
    "app".to_string(),
    "App".to_string(),
    "1.0.0".to_string(),
    "Description".to_string(),
    AppPackageCategory::Utility,
);

// Set size (in bytes)
pkg.set_size(50 * 1024 * 1024); // 50 MB

// Set rating (0-5)
pkg.set_rating(4.5);

// Set status
pkg.set_status(AppPackageStatus::Installed);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total packages: {}", stats.total_packages);
println!("Installed: {}", stats.installed_count);
println!("Updates available: {}", stats.update_available_count);
println!("Total repositories: {}", stats.total_repositories);
println!("Enabled repositories: {}", stats.enabled_repositories);
```

## Default Configuration

The Software Package Manager includes pre-configured repositories and packages:

**Repositories:**
- **main**: Main Repository (https://packages.sigmaos.org/main)
- **community**: Community Repository (https://packages.sigmaos.org/community)

**Default Packages:**
- **sigma-terminal**: Terminal emulator
- **sigma-file-manager**: File manager
- **sigma-text-editor**: Text editor
- **sigma-web-browser**: Web browser
- **sigma-media-player**: Media player

## AI Agent Maintenance Instructions

When maintaining the Software Package Manager:

1. **Repository Validation**: Validate repository URLs before adding
2. **Package Validation**: Validate package IDs and versions
3. **Dependency Tracking**: Implement dependency resolution for packages
4. **Version Comparison**: Implement proper version comparison
5. **Size Validation**: Validate package sizes are reasonable
6. **Rating Validation**: Ensure ratings are within valid range (0-5)

## Testing

Run the unit tests with:

```bash
cargo test --lib package::software_manager
```

## Future Enhancements

- Integration with actual package manager (apt, pacman, dnf)
- Dependency resolution
- Automatic updates
- Package verification and signing
- Transaction management
- Rollback support
- Package groups
- Package snapshots
- User reviews and ratings
- Package recommendations
- Download progress tracking
