# SigmaOS Developer Tools

## Overview

SigmaOS provides a comprehensive suite of developer tools designed to enhance productivity, streamline development workflows, and provide deep insights into system behavior. These tools range from integrated development environment support to advanced debugging and profiling capabilities.

## IDE Integration

### VS Code Extension

The SigmaOS VS Code extension provides seamless integration with the SigmaOS development environment.

**Features:**
- Syntax highlighting for SigmaOS-specific file types
- IntelliSense for SigmaOS kernel APIs
- Build system integration
- Debugging support with GDB integration
- Kernel module development templates
- Package manager integration (sigma-pkg)
- Remote development support for SigmaOS targets

**Installation:**
```bash
code --install-extension sigmaos.sigmaos-vscode
```

**Configuration:**
```json
{
  "sigmaos.toolchain": "/usr/local/sigmaos",
  "sigmaos.target": "x86_64-sigmaos",
  "sigmaos.debugger": "gdb-multiarch",
  "sigmaos.buildSystem": "cmake"
}
```

### JetBrains Plugin

The JetBrains plugin supports CLion, IntelliJ IDEA, and other JetBrains IDEs.

**Features:**
- CMake project integration
- Rust support for kernel development
- Debugger integration
- Code completion and navigation
- Refactoring support
- Version control integration

**Installation:**
Install from the JetBrains Marketplace or download from the SigmaOS releases page.

### Eclipse Plugin

The Eclipse plugin provides CDT integration for SigmaOS development.

**Features:**
- C/C++ Development Tooling (CDT) integration
- Makefile project support
- Remote debugging
- Memory analysis
- Static analysis integration

## Build System

### Custom Build System with Modular Configs

SigmaOS uses a modular build system based on CMake with custom configurations for different deployment profiles.

**Build Profiles:**
- `desktop` - Full desktop profile with all features
- `microkernel` - Minimal microkernel profile (< 512 KB)
- `cloud` - Cloud/container headless profile
- `mobile` - ARM64/RISC-V mobile profile
- `rtos` - Hard real-time profile
- `distributed` - Multi-node distributed cluster

**Configuration:**
```bash
# Configure build
cmake -B build -DCMAKE_BUILD_TYPE=Release -DSIGMAOS_PROFILE=desktop

# Build
cmake --build build -j$(nproc)

# Install
sudo cmake --install build
```

**Modular Components:**
```cmake
# Enable/disable components
cmake -DSIGMAOS_ENABLE_GPU=ON \
      -DSIGMAOS_ENABLE_WIFI=ON \
      -DSIGMAOS_ENABLE_BLUETOOTH=ON \
      -DSIGMAOS_ENABLE_AI=ON \
      -DSIGMAOS_ENABLE_DESKTOP=ON
```

**Cross-Compilation:**
```bash
# ARM64
cmake -DCMAKE_TOOLCHAIN_FILE=toolchains/arm64-sigmaos.cmake

# RISC-V
cmake -DCMAKE_TOOLCHAIN_FILE=toolchains/riscv64-sigmaos.cmake
```

## Debugging Tools

### Kernel Logs

SigmaOS provides comprehensive kernel logging with multiple log levels and filtering capabilities.

**Log Levels:**
- `EMERG` - Emergency (system is unusable)
- `ALERT` - Alert (action must be taken immediately)
- `CRIT` - Critical (critical conditions)
- `ERR` - Error (error conditions)
- `WARNING` - Warning (warning conditions)
- `NOTICE` - Notice (normal but significant condition)
- `INFO` - Informational (informational messages)
- `DEBUG` - Debug (debug-level messages)

**Usage:**
```bash
# View kernel logs
sigma-klog

# Filter by level
sigma-klog --level=ERR

# Filter by subsystem
sigma-klog --subsystem=scheduler

# Real-time monitoring
sigma-klog --follow

# Export to file
sigma-klog --export=kernel.log
```

**Kernel Log API:**
```c
#include <sigmaos/log.h>

// Log messages
sigma_log(KERN_INFO, "System initialized");
sigma_log(KERN_ERR, "Failed to allocate memory: %d", error);
```

### Crash Analyzers

SigmaOS includes advanced crash analysis tools for debugging kernel panics and user-space crashes.

**Features:**
- Automatic crash dump generation
- Stack trace analysis
- Memory inspection
- Register state display
- Symbol resolution
- Crash report generation

**Usage:**
```bash
# Analyze kernel crash dump
sigma-crash-analyze /var/crash/kernel.0

# Analyze user-space crash
sigma-crash-analyze /var/crash/app.1234

# Generate crash report
sigma-crash-analyze --report crash.txt /var/crash/kernel.0

# Interactive analysis
sigma-crash-analyze --interactive /var/crash/kernel.0
```

**Crash Dump Configuration:**
```bash
# Enable crash dumps
sudo sysctl kernel.core_pattern=/var/crash/core.%e.%p.%t

# Configure crash dump size
sudo sysctl kernel.core_dump_filter=0x3f
```

### Profilers

SigmaOS provides multiple profiling tools for performance analysis and optimization.

**CPU Profiler:**
```bash
# Profile CPU usage
sigma-prof cpu --pid=1234 --duration=30

# Profile kernel CPU usage
sigma-prof cpu --kernel --duration=30

# Generate flame graph
sigma-prof cpu --flamegraph --output=flamegraph.svg
```

**Memory Profiler:**
```bash
# Profile memory usage
sigma-prof memory --pid=1234

# Detect memory leaks
sigma-prof memory --leak-detect --pid=1234

# Profile kernel memory
sigma-prof memory --kernel
```

**I/O Profiler:**
```bash
# Profile I/O operations
sigma-prof io --pid=1234

# Profile disk I/O
sigma-prof io --disk

# Profile network I/O
sigma-prof io --network
```

**Lock Contention Profiler:**
```bash
# Profile lock contention
sigma-prof lock --pid=1234

# Profile kernel locks
sigma-prof lock --kernel
```

## Performance Benchmarking Tools

SigmaOS includes comprehensive benchmarking tools for measuring system performance.

**CPU Benchmarks:**
```bash
# CPU performance benchmark
sigma-bench cpu --iterations=1000

# Scheduler benchmark
sigma-bench scheduler --tasks=64

# Context switch benchmark
sigma-bench context-switch --iterations=100000
```

**Memory Benchmarks:**
```bash
# Memory bandwidth benchmark
sigma-bench memory --bandwidth

# Memory latency benchmark
sigma-bench memory --latency

# Cache benchmark
sigma-bench cache
```

**I/O Benchmarks:**
```bash
# Disk I/O benchmark
sigma-bench disk --device=/dev/nvme0n1

# Network benchmark
sigma-bench network --target=192.168.1.100

# Filesystem benchmark
sigma-bench filesystem --path=/mnt/sigmafs
```

**System Benchmarks:**
```bash
# Full system benchmark
sigma-bench system --all

# Boot time benchmark
sigma-bench boot

# Startup time benchmark
sigma-bench startup
```

## Package Manager Integration

### sigma-pkg CLI

The sigma-pkg command-line tool provides comprehensive package management.

**Basic Operations:**
```bash
# Search packages
sigma-pkg search firefox

# Install package
sigma-pkg install firefox

# Remove package
sigma-pkg remove firefox

# Update packages
sigma-pkg update

# Upgrade system
sigma-pkg upgrade

# List installed packages
sigma-pkg list --installed
```

**Advanced Operations:**
```bash
# View package information
sigma-pkg info firefox

# View package dependencies
sigma-pkg deps firefox

# Rollback to previous version
sigma-pkg rollback firefox

# Verify package signatures
sigma-pkg verify firefox

# Generate SBOM
sigma-pkg sbom firefox
```

**Development Operations:**
```bash
# Build package from source
sigma-pkg build ./package-spec.yaml

# Create package
sigma-pkg create ./package-dir

# Sign package
sigma-pkg sign ./package.spkg

# Upload to repository
sigma-pkg upload ./package.spkg
```

## AI-Assisted Development

### Natural Language to CLI Translator

SigmaOS includes an AI-powered natural language to CLI translator that helps developers find the right commands.

**Usage:**
```bash
# Translate natural language to command
sigma-nl2cli "install firefox browser"

# Get command explanation
sigma-nl2cli --explain "sigma-pkg install firefox"

# Interactive mode
sigma-nl2cli --interactive
```

### AI Error Explanation

The AI error explanation tool provides detailed explanations of error messages and suggests fixes.

**Usage:**
```bash
# Explain error
sigma-explain "segmentation fault"

# Get suggested fixes
sigma-explain --fixes "connection refused"

# Explain kernel error
sigma-explain --kernel "page fault"
```

## Documentation Tools

### Man Pages

SigmaOS includes comprehensive man pages for all system tools and APIs.

**Usage:**
```bash
# View man page
man sigma-pkg

# Search man pages
man -k package

# View section 2 (system calls)
man 2 open

# View section 3 (library functions)
man 3 printf
```

### API Documentation

SigmaOS uses Doxygen for API documentation generation.

**Generate Documentation:**
```bash
# Generate HTML documentation
cmake --build build --target docs

# Generate PDF documentation
cmake --build build --target docs-pdf

# Generate man pages
cmake --build build --target docs-man
```

**View Documentation:**
```bash
# Open in browser
xdg-open build/docs/html/index.html

# Serve documentation locally
python3 -m http.server 8000 --directory build/docs/html
```

## Testing Tools

### Unit Testing

SigmaOS uses a custom unit testing framework for kernel and user-space testing.

**Usage:**
```bash
# Run all tests
sigma-test

# Run specific test suite
sigma-test --suite=scheduler

# Run specific test
sigma-test --test=test_thread_creation

# Run with verbose output
sigma-test --verbose

# Generate coverage report
sigma-test --coverage
```

### Integration Testing

Integration tests verify the interaction between different system components.

**Usage:**
```bash
# Run integration tests
sigma-integration-test

# Run specific integration test
sigma-integration-test --test=vfs_integration

# Run with mock hardware
sigma-integration-test --mock-hardware
```

### Fuzz Testing

SigmaOS includes fuzz testing tools for finding security vulnerabilities.

**Usage:**
```bash
# Fuzz test syscall handler
sigma-fuzz --target=syscall --input=syscalls.txt

# Fuzz test filesystem
sigma-fuzz --target=filesystem --input=operations.txt

# Fuzz test network stack
sigma-fuzz --target=network --input=packets.bin
```

## Security Tools

### Static Analysis

SigmaOS includes static analysis tools for detecting security vulnerabilities.

**Usage:**
```bash
# Run static analysis
sigma-static-analyze kernel/

# Generate security report
sigma-static-analyze --report security-report.md kernel/

# Check for specific vulnerabilities
sigma-static-analyze --check=buffer-overflow kernel/
```

### Dynamic Analysis

Dynamic analysis tools monitor program execution for security issues.

**Usage:**
```bash
# Monitor for memory safety violations
sigma-dynamic-analyze --memory-safety ./program

# Monitor for information leaks
sigma-dynamic-analyze --info-leak ./program

# Monitor for privilege escalation
sigma-dynamic-analyze --privilege-escalation ./program
```

## Configuration

### Developer Environment Setup

**Install Development Tools:**
```bash
# Install all development tools
sigma-pkg install sigmaos-dev-tools

# Install specific tools
sigma-pkg install sigmaos-ide-integration
sigma-pkg install sigmaos-debugging-tools
sigma-pkg install sigmaos-profiling-tools
```

**Configure Environment:**
```bash
# Set environment variables
export SIGMAOS_TOOLCHAIN=/usr/local/sigmaos
export SIGMAOS_SDK=$SIGMAOS_TOOLCHAIN/sdk
export PATH=$SIGMAOS_TOOLCHAIN/bin:$PATH

# Load configuration
source /etc/sigmaos-dev.conf
```

### IDE Configuration

**VS Code:**
```json
{
  "sigmaos.toolchain": "/usr/local/sigmaos",
  "sigmaos.target": "x86_64-sigmaos",
  "sigmaos.debugger": "gdb-multiarch",
  "sigmaos.buildSystem": "cmake",
  "sigmaos.autoBuild": true,
  "C_Cpp.default.configurationProvider": "sigmaos.cmake-tools"
}
```

**JetBrains:**
```properties
# idea.properties
sigmaos.toolchain=/usr/local/sigmaos
sigmaos.target=x86_64-sigmaos
sigmaos.cmake.options=-DSIGMAOS_PROFILE=desktop
```

## Troubleshooting

### Build Issues

**Missing Dependencies:**
```bash
# Check for missing dependencies
sigma-build-check

# Install missing dependencies
sigma-build-install-deps
```

**Configuration Errors:**
```bash
# Reset build configuration
cmake --build build --target clean
rm -rf build/CMakeCache.txt
cmake -B build
```

### Debugging Issues

**GDB Connection Failed:**
```bash
# Check GDB installation
gdb-multiarch --version

# Check target connection
sigma-gdb-check --target=localhost:1234
```

**Symbol Loading Failed:**
```bash
# Rebuild with debug symbols
cmake -DCMAKE_BUILD_TYPE=Debug -B build
cmake --build build

# Check symbol file
sigma-symbols-check ./kernel.elf
```

### Performance Issues

**Slow Build Times:**
```bash
# Use ccache
export CC="ccache gcc"
export CXX="ccache g++"

# Increase parallel jobs
cmake --build build -j$(nproc)
```

**Profiling Not Working:**
```bash
# Check profiler permissions
sudo sigma-perf-check

# Enable kernel profiling
sudo sysctl kernel.perf_event_paranoid=0
```

## References

- [Kernel Architecture](Kernel-Architecture.md)
- [Driver Development Guide](Driver-Development-Guide.md)
- [Package Manager Usage](Package-Manager-Usage.md)
- [Security Documentation](Security.md)
- [API Documentation](https://sigmaos.dev/docs/api)

## License

All SigmaOS developer tools are licensed under MIT License. See [LICENSE](../LICENSE) for details.
