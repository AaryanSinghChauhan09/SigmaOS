# Musl libc Integration for SigmaOS
# Alternative to glibc for minimal footprint and security
# Inspired by Alpine Linux's use of musl

## Overview

Musl libc is a lightweight, fast, and secure C library implementation. SigmaOS provides optional musl integration for minimal footprint builds.

## Benefits

- **Smaller footprint**: Musl is significantly smaller than glibc
- **Security-focused**: Smaller attack surface, fewer vulnerabilities
- **Performance**: Optimized for embedded and minimal systems
- **Standards compliance**: POSIX compliant

## Configuration

### Enable Musl in Build

Add to `sigma.toml`:

```toml
[build]
libc = "musl"  # Options: glibc, musl
```

### Profile Configuration

The `sigma-core` profile already includes musl as the default:

```toml
[profile]
name = "sigma-core"
libc = "musl"
```

## Build System Integration

### Cross-Compilation

Musl enables easier cross-compilation for multiple architectures:

```bash
# Build with musl for x86_64
cargo build --target x86_64-unknown-linux-musl

# Build with musl for ARM64
cargo build --target aarch64-unknown-linux-musl
```

### Package Compatibility

Packages must be built against musl to ensure compatibility:

```toml
[package]
libc = "musl"
static_linking = true  # Recommended for musl
```

## Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| Build system integration | ✅ Complete | Musl targets supported |
| Core utilities | ✅ Complete | Built with musl |
| Network stack | ✅ Complete | Musl-compatible |
| Graphics system | 🟡 In Progress | Some dependencies require glibc |
| AI tools | 🟡 In Progress | TensorFlow/PyTorch musl support |

## Migration from glibc

### Step 1: Update Configuration

```toml
[build]
libc = "musl"
```

### Step 2: Rebuild System

```bash
sigma-build --profile sigma-core --libc musl
```

### Step 3: Test Compatibility

```bash
sigma-test --libc musl
```

## Known Limitations

- Some proprietary software may require glibc
- Certain GPU drivers may have glibc dependencies
- Some Python packages may require glibc-specific features

## Troubleshooting

### Missing Symbols

If you encounter missing symbol errors:

```bash
# Check for glibc-specific dependencies
ldd /path/to/binary

# Rebuild with musl
cargo build --target x86_64-unknown-linux-musl
```

### Dynamic Linking Issues

For static linking with musl:

```toml
[build]
static_linking = true
libc = "musl"
```

## References

- Musl libc: https://musl.libc.org/
- Alpine Linux: https://alpinelinux.org/
- Rust musl targets: https://doc.rust-lang.org/nightly/rustc/platform-support.html
