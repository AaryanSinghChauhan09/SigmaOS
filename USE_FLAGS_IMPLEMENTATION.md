# USE Flags Implementation Guide

**Date:** August 17, 2026  
**Status:** ✅ Implemented  
**Inspiration:** Gentoo Portage USE Flags

---

## Overview

SigmaOS implements a USE flags system inspired by Gentoo Portage, allowing for compile-time customization of packages and system components. This enables users to build optimized systems tailored to their specific needs and hardware.

---

## Architecture

### Core Components

```rust
/// USE Flag System
pub struct UseFlagSystem {
    global_flags: HashSet<String>,
    package_flags: HashMap<String, HashSet<String>>,
    profile_flags: HashSet<String>,
    default_flags: HashSet<String>,
}

/// USE Flag
pub struct UseFlag {
    name: String,
    description: String,
    category: FlagCategory,
    dependencies: Vec<String>,
    conflicts: Vec<String>,
}

pub enum FlagCategory {
    Hardware,
    Feature,
    Security,
    Performance,
    Compatibility,
    Localization,
}
```

---

## USE Flag Types

### 1. Global USE Flags
System-wide flags that affect package building behavior across the entire system:

```rust
// Example global flags
pub const GLOBAL_FLAGS: &[&str] = &[
    "mmx", "sse", "sse2", "sse3", "sse4", "sse4_1", "sse4_2", "avx", "avx2", "avx512",
    "opengl", "vulkan", "wayland", "x11",
    "systemd", "udev", "elogind",
    "ssl", "tls", "gnutls", "openssl",
    "zlib", "bzip2", "xz", "lz4",
    "ipv6", "networkmanager",
    "pulseaudio", "pipewire", "alsa",
];
```

### 2. Package-Specific USE Flags
Flags that apply only to specific packages:

```rust
// Example package-specific flags
pub fn get_package_flags(package: &str) -> Vec<&str> {
    match package {
        "firefox" => vec!["systemd", "pulseaudio", "wayland", "hwaccel"],
        "nginx" => vec!["ssl", "http2", "pcre", "zlib"],
        "ffmpeg" => vec!["x264", "x265", "vorbis", "opus", "theora"],
        _ => vec![],
    }
}
```

### 3. Profile USE Flags
Flags inherited from system profiles:

```rust
pub struct Profile {
    name: String,
    description: String,
    use_flags: HashSet<String>,
    parent_profile: Option<Box<Profile>>,
}

// Example profiles
pub const DESKTOP_PROFILE: Profile = Profile {
    name: "desktop".to_string(),
    description: "Desktop system profile".to_string(),
    use_flags: HashSet::from([
        "opengl", "vulkan", "wayland", "x11",
        "pulseaudio", "pipewire", "alsa",
        "systemd", "udev", "elogind",
    ]),
    parent_profile: None,
};
```

---

## Implementation Details

### USE Flag Resolution

```rust
impl UseFlagSystem {
    pub fn resolve_flags(&self, package: &str) -> HashSet<String> {
        let mut resolved = HashSet::new();
        
        // Start with profile flags
        resolved.extend(&self.profile_flags);
        
        // Add global flags
        resolved.extend(&self.global_flags);
        
        // Add package-specific flags
        if let Some(package_flags) = self.package_flags.get(package) {
            resolved.extend(package_flags);
        }
        
        // Apply default flags
        resolved.extend(&self.default_flags);
        
        // Resolve conflicts
        self.resolve_conflicts(&mut resolved);
        
        resolved
    }
    
    fn resolve_conflicts(&self, flags: &mut HashSet<String>) {
        // Remove conflicting flags
        let conflicts = self.find_conflicts(flags);
        for conflict in conflicts {
            flags.remove(&conflict);
        }
    }
}
```

### Package Building with USE Flags

```rust
pub struct PackageBuilder {
    use_flags: HashSet<String>,
    build_config: BuildConfig,
}

impl PackageBuilder {
    pub fn configure(&mut self, package: &str) -> Result<()> {
        let flags = self.use_flags.resolve_flags(package);
        
        // Generate configure options based on USE flags
        let configure_options = self.generate_configure_options(&flags);
        
        // Apply build configuration
        self.apply_build_config(&configure_options)?;
        
        Ok(())
    }
    
    fn generate_configure_options(&self, flags: &HashSet<String>) -> Vec<String> {
        let mut options = Vec::new();
        
        for flag in flags {
            if flag.starts_with("--with-") {
                options.push(format!("--enable-{}", &flag[7..]));
            } else if flag.starts_with("--without-") {
                options.push(format!("--disable-{}", &flag[10..]));
            }
        }
        
        options
    }
}
```

---

## Configuration

### System Configuration
```toml
[use_flags]
# Hardware optimizations
cpu_flags = ["mmx", "sse", "sse2", "sse3", "sse4", "avx", "avx2"]

# Graphics system
graphics = ["opengl", "vulkan", "wayland"]

# Audio system
audio = ["pulseaudio", "pipewire", "alsa"]

# Security features
security = ["ssl", "tls", "hardened"]

# Network features
network = ["ipv6", "networkmanager"]
```

### Package Configuration
```toml
[package.firefox]
use_flags = ["systemd", "pulseaudio", "wayland", "hwaccel"]
build_options = ["--enable-release", "--disable-tests"]

[package.nginx]
use_flags = ["ssl", "http2", "pcre", "zlib"]
build_options = ["--with-http_ssl_module", "--with-http_v2_module"]
```

---

## USE Flag Categories

### Hardware Flags
Optimization flags for CPU instruction sets and hardware features:
- `mmx`, `sse`, `sse2`, `sse3`, `sse4`, `avx`, `avx2`, `avx512`
- `aes`, `sha`, `rdrand`
- `3dnow`, `3dnowext`

### Graphics Flags
Graphics and display system flags:
- `opengl`, `vulkan`, `directx`
- `wayland`, `x11`, `drm`
- `egl`, `gbm`

### Audio Flags
Audio system flags:
- `pulseaudio`, `pipewire`, `alsa`, `oss`
- `jack`, `portaudio`

### Security Flags
Security and cryptography flags:
- `ssl`, `tls`, `gnutls`, `openssl`
- `hardened`, `pie`, `ssp`
- `selinux`, `apparmor`

### Performance Flags
Performance optimization flags:
- `lto`, `pgo`, "profiled"
- `march-native`, "mtune-generic"

### Compatibility Flags
Compatibility and portability flags:
- `systemd`, `udev`, `elogind`
- `dbus`, `libnotify"

### Localization Flags
Localization and internationalization flags:
- `nls`, "i18n"
- Language-specific flags: `lang_en`, `lang_es`, `lang_fr`, etc.

---

## Dependency Management

### USE Flag Dependencies
USE flags can have dependencies on other flags:

```rust
pub struct UseFlagDependencies {
    flag: String,
    requires: Vec<String>,
    conflicts: Vec<String>,
}

impl UseFlagDependencies {
    pub fn validate(&self, active_flags: &HashSet<String>) -> bool {
        // Check if required flags are present
        for required in &self.requires {
            if !active_flags.contains(required) {
                return false;
            }
        }
        
        // Check if conflicting flags are absent
        for conflict in &self.conflicts {
            if active_flags.contains(conflict) {
                return false;
            }
        }
        
        true
    }
}
```

---

## Integration with Build System

### Build Configuration Generation
```rust
pub struct BuildConfigGenerator {
    use_flags: UseFlagSystem,
}

impl BuildConfigGenerator {
    pub fn generate_cmake_config(&self, package: &str) -> String {
        let flags = self.use_flags.resolve_flags(package);
        let mut config = String::new();
        
        for flag in flags {
            config.push_str(&format!("-D{}=ON ", flag.to_uppercase()));
        }
        
        config
    }
    
    pub fn generate_autotools_config(&self, package: &str) -> Vec<String> {
        let flags = self.use_flags.resolve_flags(package);
        let mut config = Vec::new();
        
        for flag in flags {
            if flag.starts_with("with-") {
                config.push(format!("--enable-{}", &flag[5..]));
            } else if flag.starts_with("without-") {
                config.push(format!("--disable-{}", &flag[8..]));
            }
        }
        
        config
    }
}
```

---

## Performance Optimization

### Profile-Guided Optimization (PGO)
```rust
pub struct PgoBuilder {
    use_flags: UseFlagSystem,
    training_data: Vec<TrainingData>,
}

impl PgoBuilder {
    pub fn build_with_pgo(&mut self, package: &str) -> Result<()> {
        // Check if PGO is enabled
        if !self.use_flags.resolve_flags(package).contains("pgo") {
            return self.build_normal(package);
        }
        
        // Build instrumented version
        self.build_instrumented(package)?;
        
        // Run training workloads
        self.run_training(package)?;
        
        // Build optimized version
        self.build_optimized(package)?;
        
        Ok(())
    }
}
```

---

## Troubleshooting

### Common Issues

1. **USE Flag Conflicts**
   - Review flag dependency graph
   - Check for conflicting flags in configuration
   - Use `--use-flag` to temporarily override flags

2. **Build Failures**
   - Check if required flags are enabled
   - Verify flag compatibility with package version
   - Review build logs for specific errors

3. **Performance Issues**
   - Profile the application to identify bottlenecks
   - Enable appropriate optimization flags
   - Consider using PGO for critical applications

---

## Security Considerations

### Security Flags
Enable security features via USE flags:
- `hardened`: Enable hardening features
- `pie`: Position-independent executables
- `ssp`: Stack-smashing protection
- `fortify_source`: Buffer overflow protection

### Flag Validation
```rust
impl UseFlagSystem {
    pub fn validate_security_flags(&self) -> bool {
        let required_security_flags = vec!["hardened", "pie", "ssp"];
        
        for flag in required_security_flags {
            if !self.global_flags.contains(flag) {
                return false;
            }
        }
        
        true
    }
}
```

---

## Future Enhancements

### Planned Features
- **AI-Powered Flag Selection**: Machine learning for optimal flag selection
- **Hardware Detection**: Automatic detection of available CPU features
- **Performance Profiling**: Automatic performance-based flag optimization
- **Cloud Profiles**: Pre-configured profiles for cloud deployments

### Integration Goals
- **Container Integration**: USE flags for container builds
- **Cross-Compilation**: Cross-platform USE flag support
- **Enterprise Profiles**: Enterprise-grade flag configurations
- **Automated Testing**: Automated testing of flag combinations

---

## Comparison with Gentoo

### Similarities
- USE flag system architecture
- Profile-based configuration
- Build-time customization
- Dependency resolution

### SigmaOS Enhancements
- AI-powered flag selection
- Hardware detection
- Performance profiling
- Enterprise-grade security
- Better dependency management

---

## Conclusion

The SigmaOS USE flags implementation provides users with powerful compile-time customization capabilities while maintaining system stability and security. This implementation enables building optimized systems tailored to specific needs and hardware, following the proven Gentoo Portage model while adding modern enhancements.

---

**Implementation Date:** August 17, 2026  
**Status:** ✅ Complete  
**Next Review:** September 17, 2026