# Community Build Infrastructure for SigmaOS
# Inspired by Open Build Service (OBS) from openSUSE
# Provides distributed build system for community packages

## Overview

The Community Build Infrastructure provides a distributed build system for building SigmaOS packages across multiple architectures and build workers, enabling community contributions to the package ecosystem.

## Features

### Distributed Building

- **Multi-architecture support**: Build for x86_64, ARM64, RISC-V, and more
- **Build workers**: Distributed build farm with worker management
- **Build queues**: Automated build scheduling and processing
- **Build results**: Comprehensive build logging and result tracking

### Project Management

- **Build projects**: Organize packages into projects
- **Build recipes**: Declarative package build specifications
- **Repository integration**: Git-based source management
- **Maintainer system**: Project ownership and maintenance

### Build Systems

Support for multiple build systems:
- **Autotools**: Traditional GNU build system
- **CMake**: Modern C/C++ build system
- **Meson**: Fast and user-friendly build system
- **Cargo**: Rust package manager
- **Go**: Go language build system
- **Python**: Python package management

## Usage

### Command Line Interface

```bash
# Initialize build infrastructure
sigma-build init /var/lib/sigmaos/build

# Add a new project
sigma-build add-project my-project "My Project" https://github.com/user/repo

# Submit a build
sigma-build submit my-project my-package

# Register a build worker
sigma-build register-worker worker-1 x86_64

# Process pending builds
sigma-build process

# List all projects
sigma-build list-projects

# List all workers
sigma-build list-workers

# Get build results
sigma-build results my-project
```

### Project Configuration

Projects are defined in TOML format:

```toml
# /var/lib/sigmaos/build/projects/my-project/project.toml
[project]
name = "my-project"
description = "My Project"
repository = "https://github.com/user/repo"

[project.architectures]
x86_64 = true
aarch64 = true
riscv64 = false

[project.maintainers]
maintainer1 = "user1@example.com"
maintainer2 = "user2@example.com"
```

### Build Recipes

Build recipes define how to build packages:

```toml
# /var/lib/sigmaos/build/projects/my-project/recipes/my-package.toml
[recipe]
name = "my-package"
version = "1.0.0"
source_url = "https://example.com/my-package-1.0.0.tar.gz"
build_system = "cmake"

[recipe.dependencies]
runtime = ["libssl", "libcrypto"]
build = ["cmake", "ninja"]
```

## API Usage

### Rust API

```rust
use community_build::CommunityBuild;

// Initialize build infrastructure
let build = CommunityBuild::new(PathBuf::from("/var/lib/sigmaos/build"))?;

// Add a project
let project = BuildProject {
    name: "my-project".to_string(),
    description: "My Project".to_string(),
    repository: "https://github.com/user/repo".to_string(),
    recipes: vec![],
    architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
    maintainers: vec!["user@example.com".to_string()],
};

build.add_project(project)?;

// Register a worker
let worker = BuildWorker {
    id: "worker-1".to_string(),
    architecture: "x86_64".to_string(),
    capabilities: vec!["autotools".to_string(), "cmake".to_string()],
    status: WorkerStatus::Idle,
    current_build: None,
};

build.register_worker(worker)?;

// Submit a build
let build_id = build.submit_build("my-project", "my-package")?;

// Process builds
let processed = build.process_builds()?;
println!("Processed {} builds", processed);

// Get results
let results = build.get_results("my-project");
for result in results {
    println!("{}: {} on {}", result.recipe, result.status, result.architecture);
}
```

## Build Worker Management

### Worker Registration

```bash
# Register a new worker
sigma-build register-worker worker-1 x86_64

# Register worker with capabilities
sigma-build register-worker worker-2 aarch64 --capabilities cmake,meson,cargo

# List workers
sigma-build list-workers

# Remove worker
sigma-build remove-worker worker-1
```

### Worker Configuration

```toml
# /var/lib/sigmaos/build/workers/worker-1.toml
[worker]
id = "worker-1"
architecture = "x86_64"
status = "idle"

[worker.capabilities]
autotools = true
cmake = true
meson = true
cargo = true
```

## Build Process

### Build Lifecycle

1. **Submission**: Build request submitted to queue
2. **Scheduling**: Build assigned to available worker
3. **Building**: Worker executes build process
4. **Logging**: Build output logged to file
5. **Packaging**: Result packaged for distribution
6. **Completion**: Build status updated and result stored

### Build Results

Build results include:
- **Status**: Success, failed, or cancelled
- **Build time**: Duration of build process
- **Build log**: Complete build output
- **Package path**: Location of built package
- **Architecture**: Target architecture

## Repository Integration

### Git-based Sources

Projects can use Git repositories as sources:

```toml
[project]
repository = "https://github.com/user/repo.git"
branch = "main"
commit = "latest"
```

### Source Management

```bash
# Clone repository
sigma-build clone-source my-project

# Update repository
sigma-build update-source my-project

# Checkout specific commit
sigma-build checkout my-project abc123
```

## Implementation Status

| Feature | Status | Notes |
|---------|--------|-------|
| Project management | ✅ Complete | Add, list, manage projects |
| Build recipes | ✅ Complete | Declarative build specifications |
| Build workers | ✅ Complete | Worker registration and management |
| Build processing | ✅ Complete | Automated build scheduling |
| Build results | ✅ Complete | Result tracking and logging |
| Multi-architecture | ✅ Complete | x86_64, ARM64, RISC-V support |
| Build systems | ✅ Complete | Autotools, CMake, Meson, Cargo, Go, Python |
| Repository integration | ✅ Complete | Git-based source management |
| Web interface | ⏳ Planned | Future enhancement |
| Build badges | ⏳ Planned | Future enhancement |

## Best Practices

1. **Use descriptive project names**: Clear identification of projects
2. **Specify maintainers**: Assign responsibility for projects
3. **Test locally first**: Validate recipes before submission
4. **Monitor build logs**: Check build output for issues
5. **Use appropriate workers**: Match worker capabilities to build requirements

## Troubleshooting

### Build Failures

```bash
# Check build log
sigma-build log my-project my-package x86_64

# Check worker status
sigma-build worker-status worker-1

# Retry failed build
sigma-build retry my-project my-package
```

### Worker Issues

```bash
# Check worker availability
sigma-build list-workers

# Restart worker
sigma-build restart-worker worker-1

# Remove stuck worker
sigma-build remove-worker worker-1
```

### Repository Issues

```bash
# Check repository status
sigma-build repo-status my-project

# Update repository
sigma-build update-source my-project

# Reclone repository
sigma-build reclone my-project
```

## Architecture Support

### Supported Architectures

- **x86_64**: Primary desktop/server architecture
- **aarch64**: ARM64 for embedded and mobile
- **riscv64**: RISC-V for experimental builds
- **x86**: Legacy 32-bit support (limited)

### Cross-Compilation

Build workers can cross-compile for different architectures:

```toml
[worker]
id = "worker-cross"
architecture = "x86_64"
cross_compile = ["aarch64", "riscv64"]
```

## Security Considerations

### Build Isolation

- **Chroot environments**: Isolated build environments
- **Network restrictions**: Controlled network access during builds
- **Resource limits**: CPU and memory limits per build
- **Signature verification**: Verify source integrity

### Access Control

- **Maintainer authentication**: Verified maintainer accounts
- **Project permissions**: Controlled project access
- **Build worker authentication**: Secure worker registration
- **API rate limiting**: Prevent abuse of build system

## References

- Open Build Service: https://openbuildservice.org/
- Buildbot: https://buildbot.net/
- Jenkins: https://www.jenkins.io/
- SigmaOS Package Management: [sigpkg](../userland/sigpkg/)
