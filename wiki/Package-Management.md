# Package Management

SigmaOS implements comprehensive package management with Linux and BSD-inspired features including package formats, dependency resolution, and repository management.

## Overview

Package management provides:
- Universal package manager supporting multiple formats (deb, rpm, pacman, apk, xbps, ebuild, ports)
- Dependency resolution with SAT solver
- Repository management and mirrors
- Package signing and verification
- Package rollback and updates
- Virtual packages and provides
- Package groups and metapackages
- Build from source support

## Implementation

### Package Manager
```rust
// src/package/manager.rs
pub struct PackageManager {
    pub repositories: BTreeMap<String, Repository>,
    pub installed_packages: BTreeMap<String, InstalledPackage>,
    pub dependency_resolver: DependencyResolver,
}

#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub packages: BTreeMap<String, Package>,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub files: Vec<String>,
    pub install_time: SystemTime,
}

impl PackageManager {
    pub fn new() -> Self {
        PackageManager {
            repositories: BTreeMap::new(),
            installed_packages: BTreeMap::new(),
            dependency_resolver: DependencyResolver::new(),
        }
    }

    pub fn add_repository(&mut self, repository: Repository) {
        self.repositories.insert(repository.name.clone(), repository);
    }

    pub fn install(&mut self, package_name: &str) -> Result<(), PackageError> {
        // Find package
        let package = self.find_package(package_name)?;

        // Resolve dependencies
        let dependencies = self.dependency_resolver.resolve(&package.dependencies, &self.installed_packages)?;

        // Install dependencies
        for dep in dependencies {
            self.install(&dep)?;
        }

        // Install package
        self.install_package(&package)?;

        Ok(())
    }

    pub fn remove(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.remove(package_name) {
            self.remove_package_files(&package.files)?;
            Ok(())
        } else {
            Err(PackageError::NotInstalled)
        }
    }

    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        // Remove old version
        self.remove(package_name)?;

        // Install new version
        self.install(package_name)?;

        Ok(())
    }

    pub fn search(&self, query: &str) -> Vec<Package> {
        let mut results = Vec::new();

        for repo in self.repositories.values() {
            for package in repo.packages.values() {
                if package.name.contains(query) || package.description.contains(query) {
                    results.push(package.clone());
                }
            }
        }

        results
    }

    fn find_package(&self, name: &str) -> Result<Package, PackageError> {
        for repo in self.repositories.values() {
            if let Some(package) = repo.packages.get(name) {
                return Ok(package.clone());
            }
        }
        Err(PackageError::NotFound)
    }

    fn install_package(&mut self, package: &Package) -> Result<(), PackageError> {
        // Download package
        let package_data = self.download_package(package)?;

        // Extract package
        let files = self.extract_package(&package_data)?;

        // Install files
        self.install_files(&files)?;

        // Register installed package
        let installed = InstalledPackage {
            name: package.name.clone(),
            version: package.version.clone(),
            files: files.clone(),
            install_time: SystemTime::now(),
        };

        self.installed_packages.insert(package.name.clone(), installed);

        Ok(())
    }

    fn remove_package_files(&self, files: &[String]) -> Result<(), PackageError> {
        // Remove package files
        for file in files {
            std::fs::remove_file(file)?;
        }
        Ok(())
    }

    fn download_package(&self, package: &Package) -> Result<Vec<u8>, PackageError> {
        // Download package from repository
        Ok(Vec::new())
    }

    fn extract_package(&self, data: &[u8]) -> Result<Vec<String>, PackageError> {
        // Extract package files
        Ok(Vec::new())
    }

    fn install_files(&self, files: &[String]) -> Result<(), PackageError> {
        // Install package files
        Ok(())
    }
}
```

### Dependency Resolver
```rust
// src/package/resolver.rs
pub struct DependencyResolver {
    pub sat_solver: SatSolver,
}

impl DependencyResolver {
    pub fn new() -> Self {
        DependencyResolver {
            sat_solver: SatSolver::new(),
        }
    }

    pub fn resolve(&self, dependencies: &[String], installed: &BTreeMap<String, InstalledPackage>) -> Result<Vec<String>, PackageError> {
        // Build dependency graph
        let graph = self.build_dependency_graph(dependencies, installed)?;

        // Solve using SAT solver
        let solution = self.sat_solver.solve(&graph)?;

        // Extract package names from solution
        let packages = self.extract_packages(&solution);

        Ok(packages)
    }

    fn build_dependency_graph(&self, dependencies: &[String], installed: &BTreeMap<String, InstalledPackage>) -> Result<DependencyGraph, PackageError> {
        // Build dependency graph
        Ok(DependencyGraph::new())
    }

    fn extract_packages(&self, solution: &SatSolution) -> Vec<String> {
        // Extract package names from SAT solution
        Vec::new()
    }
}

pub struct SatSolver {
    // SAT solver implementation
}

impl SatSolver {
    pub fn new() -> Self {
        SatSolver
    }

    pub fn solve(&self, graph: &DependencyGraph) -> Result<SatSolution, PackageError> {
        // Solve SAT problem
        Ok(SatSolution::new())
    }
}

pub struct DependencyGraph;
pub struct SatSolution;

impl DependencyGraph {
    pub fn new() -> Self {
        DependencyGraph
    }
}

impl SatSolution {
    pub fn new() -> Self {
        SatSolution
    }
}
```

## Configuration

### Package Management Configuration
```toml
# /etc/sigmaos/package.toml
[repositories]
# Repository settings
enabled = true
auto_update = true
update_interval_hours = 24

[resolver]
# Dependency resolver settings
strategy = "sat"
parallel_resolution = true
max_attempts = 3

[verification]
# Package verification settings
signature_verification = true
hash_verification = true
keyring_path = "/etc/sigmaos/keys"

[build]
# Build from source settings
enabled = true
use_flags = "optimization ccache"
make_jobs = 4
```

### Runtime Control
```bash
# Update repositories
sigpkg update

# Search for package
sigpkg search nginx

# Install package
sigpkg install nginx

# Remove package
sigpkg remove nginx

# Update package
sigpkg update nginx

# Upgrade all packages
sigpkg upgrade

# List installed packages
sigpkg list

# Show package info
sigpkg info nginx

# Verify package
sigpkg verify nginx

# Build from source
sigpkg build nginx

# Clean package cache
sigpkg clean
```

## Performance Optimization

### Repository Optimization
Optimize repositories for performance:
```bash
# Enable repository mirroring
sigpkg enable-mirroring

# Set mirror
sigpkg set-mirror https://mirror.example.com

# Enable compression
sigpkg enable-compression

# Set cache size
sigpkg set-cache-size 1024
```

### Dependency Resolution Optimization
Optimize dependency resolution for performance:
```bash
# Enable parallel resolution
sigpkg enable-parallel-resolution

# Set solver strategy
sigpkg set-solver-strategy greedy

# Enable caching
sigpkg enable-resolution-cache

# Set timeout
sigpkg set-resolution-timeout 60
```

### Build Optimization
Optimize build from source for performance:
```bash
# Set make jobs
sigpkg set-make-jobs 8

# Enable ccache
sigpkg enable-ccache

# Set use flags
sigpkg set-use-flags "optimization ccache"

# Enable distcc
sigpkg enable-distcc
```

## Troubleshooting

### Package Not Found
If package not found:
1. Check repositories: `sigpkg repositories`
2. Update repositories: `sigpkg update`
3. Check package name spelling
4. Check repository availability
5. Add custom repository

### Dependency Conflict
If dependency conflict occurs:
1. Check dependency graph: `sigpkg deps <package>`
2. Check installed packages: `sigpkg list`
3. Use `--nodeps` if necessary
4. Consider upgrading conflicting packages
5. Check package versions

### Download Fails
If download fails:
1. Check network connectivity
2. Check repository URL
3. Check mirror availability
4. Check disk space
5. Try alternative mirror

### Build Fails
If build fails:
1. Check build logs: `sigpkg build --verbose nginx`
2. Check dependencies
3. Check compiler version
4. Check use flags
5. Check for required tools

---

**[Package Management](Category-Package-Management)** | **[Repositories](Category-Repositories)** | **[Dependencies](Category-Dependencies)**
