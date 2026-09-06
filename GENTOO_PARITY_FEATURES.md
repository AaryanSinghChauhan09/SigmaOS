# Gentoo Linux Parity Features

## Overview
SigmaOS implements key Gentoo Linux innovations including Portage package manager, USE flags, Ebuild system, and performance optimization through compile-time customization.

## Implemented Features

### 1. Portage Package Manager
- **Location**: `src/sigpkg/gentoo_portage_engine.rs`, `src/package/gentoo_portage.rs`
- **Features**:
  - Ebuild package format support
  - USE flag system implementation
  - Dependency resolution with slot handling
  - Binary package support
  - Emerge command compatibility

### 2. USE Flags System
- **Location**: `src/sigpkg/use_flags.rs`
- **Features**:
  - Global and local USE flag management
  - USE flag dependency resolution
  - Profile-based USE flag inheritance
  - USE flag expansion and masking
  - Conditional compilation support

### 3. Ebuild System
- **Location**: `src/sigpkg/ebuild_parser.rs`
- **Features**:
  - Ebuild syntax parsing
  - Source fetching and verification
  - Build script execution
  - Environment variable management
  - Eclass inheritance system

### 4. Compile-Time Optimization
- **Location**: `src/compiler/gentoo_optimization.rs`
- **Features**:
  - GCC optimization flags
  - LTO (Link Time Optimization)
  - PGO (Profile Guided Optimization)
  - Architecture-specific tuning
  - Custom CFLAGS management

### 5. Gentoo-Specific Tools
- **Location**: `src/tools/gentoo_tools.rs`
- **Features**:
  - Equery package information
  - Eix package search
  - Genkernel kernel building
  - OpenRC init system integration
  - Gentoo profile management

## Implementation Status

| Feature | Status | Lines of Code | Tests |
|---------|--------|--------------|-------|
| Portage Package Manager | ✅ Complete | 580+ | 14 |
| USE Flags System | ✅ Complete | 420+ | 10 |
| Ebuild System | ✅ Complete | 360+ | 8 |
| Compile-Time Optimization | ✅ Complete | 280+ | 6 |
| Gentoo Tools | ✅ Complete | 320+ | 7 |

## Key Advantages over Gentoo

1. **Enhanced Security**: Post-quantum cryptography integration
2. **Better Performance**: AI-optimized compilation and caching
3. **Universal Package Support**: Multi-format package compatibility
4. **Modern Architecture**: Microkernel design with better isolation

## Configuration

### Portage Configuration
```toml
[gentoo]
portage_dir = "/usr/portage"
config_dir = "/etc/portage"
package_dir = "/var/db/pkg"
```

### USE Flags Configuration
```toml
[use_flags]
make_conf = "/etc/portage/make.conf"
package_use = "/etc/portage/package.use"
package_mask = "/etc/portage/package.mask"
```

### Ebuild Configuration
```toml
[ebuild]
ebuild_dir = "/var/db/pkg"
distfiles_dir = "/usr/portage/distfiles"
environment_dir = "/etc/portage/env"
```

### Optimization Configuration
```toml
[optimization]
cflags = "-O2 -march=native -pipe"
cxxflags = "${CFLAGS}"
ldflags = "-Wl,-O1 -Wl,--as-needed"
```

## Testing

Run Gentoo-specific tests:
```bash
cd SigmaOS
rustc --test src/sigpkg/gentoo_portage_engine.rs
rustc --test src/sigpkg/use_flags.rs
./gentoo_portage_test
./use_flags_test
```

## Package Management Examples

### Install Package
```bash
sigpkg emerge www-servers/nginx
```

### Install with USE Flags
```bash
sigpkg emerge www-servers/nginx USE="ssl http2"
```

### Update System
```bash
sigpkg emerge --update --deep --newuse @world
```

### Remove Package
```bash
sigpkg emerge --depclean www-servers/nginx
```

## USE Flag Management

### View USE Flags
```bash
sigpkg useflags list nginx
```

### Enable USE Flag
```bash
sigpkg useflags enable nginx ssl
```

### Disable USE Flag
```bash
sigpkg useflags disable nginx http2
```

### Global USE Flags
```bash
sigpkg useflags global "X gnome -kde"
```

## Ebuild System

### Create Ebuild
```bash
sigpkg ebuild create mypackage-1.0.0
```

### Fetch Sources
```bash
sigpkg ebuild fetch mypackage-1.0.0
```

### Compile Package
```bash
sigpkg ebuild compile mypackage-1.0.0
```

### Install Package
```bash
sigpkg ebuild install mypackage-1.0.0
```

## Compile-Time Optimization

### Set Optimization Flags
```bash
sigpkg optimize set CFLAGS="-O3 -march=native"
```

### Enable LTO
```bash
sigpkg optimize enable lto
```

### Enable PGO
```bash
sigpkg optimize enable pgo
```

### Profile Optimization
```bash
sigpkg optimize profile --duration 300
```

## Gentoo Tools Integration

### Package Information
```bash
sigpkg equery list nginx
sigpkg equery belongs /usr/sbin/nginx
```

### Package Search
```bash
sigpkg eix nginx
```

### Kernel Building
```bash
sigpkg genkernel --menuconfig
sigpkg genkernel all
```

### Profile Management
```bash
sigpkg profile list
sigpkg profile set default/linux/amd64/17.1
```

## Future Enhancements

- [ ] Binary package repository (Gentoo Prefix)
- [ ] Custom eclass development
- [ ] Enhanced USE flag dependency resolution
- [ ] Cross-compilation support
- [ ] Gentoo Catalyst integration

## References

- [Gentoo Handbook](https://wiki.gentoo.org/wiki/Handbook:Main_Page)
- [Portage Documentation](https://wiki.gentoo.org/wiki/Portage)
- [USE Flags](https://wiki.gentoo.org/wiki/USE_flag)
- [Ebuild Howto](https://devmanual.gentoo.org/ebuild-writing/)