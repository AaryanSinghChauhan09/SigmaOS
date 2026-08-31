# Gentoo Linux USE Flags Compatibility in SigmaOS

## Overview

SigmaOS incorporates **Gentoo Linux's USE flag system** to provide fine-grained control over package compilation and feature selection. This enables conditional compilation, dependency management, and system-wide feature configuration following Gentoo's proven methodology.

***

## Key Modules

*   [`src/sigpkg/gentoo_use_flags.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/sigpkg/gentoo_use_flags.rs): USE flag manager, profile system, and conditional dependency resolution

***

## Features

| Feature | SigmaOS Implementation | Notes |
|---------|------------------------|-------|
| **USE Flag Management** | Global and per-package flags | Enable/disable specific features |
| **Profile System** | Predefined flag sets | Desktop, server, minimal profiles |
| **Conditional Dependencies** | USE condition support | `X? ( x11-libs/libX11 )` syntax |
| **Dynamic Resolution** | Runtime flag evaluation | Efficient dependency calculation |

***

## Core Concepts

### USE Flags

USE flags are boolean switches that control whether certain features are compiled into packages:

```bash
# Enable X11 support
X

# Enable GTK+ toolkit
gtk

# Disable Qt5 framework
-qt5

# Disable systemd integration
-systemd
```

### Profiles

Profiles provide pre-configured sets of USE flags for different system types:

*   **Desktop**: GUI applications, multimedia, desktop environments
*   **Server**: Minimal, server-oriented packages
*   **Minimal**: Core system without extra features
*   **Custom**: User-defined profiles

### Conditional Dependencies

Packages can specify dependencies that are only required when certain USE flags are enabled:

    # Only require X11 libraries if X support is enabled
    X? ( x11-libs/libX11 )

    # Require GTK if both X and gtk flags are enabled
    X gtk? ( x11-libs/gtk+ )

    # Exclude systemd if systemd flag is disabled
    !systemd? ( sys-apps/systemd )

***

## Architecture Flow

    [USE Flag Definitions]
           │
           ▼
    [Profile Selection] ──> Apply profile-specific flags
           │
           ▼
    [Flag Resolution] ───> Calculate effective flags
           │
           ▼
    [Dependency Evaluation] ──> Process conditional dependencies
           │
           ▼
    [Package Compilation] ──> Build with selected features

***

## CLI Usage

```bash
# Set a USE flag
sigma-use set X true

# Disable a USE flag
sigma-use set systemd false

# Create a custom profile
sigma-use profile create my-desktop

# Set active profile
sigma-use profile set desktop

# View effective flags
sigma-use list

# Parse USE string
sigma-use parse "X gtk -qt5 systemd"
```

***

## Implementation Details

### USE Flag Structure

```rust
pub struct UseFlag {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}
```

### Profile Structure

```rust
pub struct UseProfile {
    pub name: String,
    pub flags: Vec<UseFlag>,
}
```

### Conditional Dependency

```rust
pub struct ConditionalDependency {
    pub package: String,
    pub use_condition: Option<String>,
}
```

***

## Common USE Flags

SigmaOS supports many common Gentoo USE flags:

| Flag | Description | Default |
|------|-------------|---------|
| `X` | X11 Window System support | true |
| `gtk` | GTK+ toolkit support | true |
| `qt5` | Qt5 framework support | false |
| `systemd` | systemd integration | false |
| `openssl` | OpenSSL support | true |
| `alsa` | ALSA sound support | true |
| `pulseaudio` | PulseAudio support | false |
| `wayland` | Wayland display server | false |

***

## Integration with SigmaOS

The USE flag system integrates with:

*   **SigmaPkg**: Native package manager
*   **Build System**: Conditional compilation
*   **Dependency Resolver**: Smart dependency calculation
*   **System Configuration**: Global feature control

***

## Benefits

1.  **Fine-Grained Control**: Select exactly which features to include
2.  **Reduced Size**: Disable unused features to minimize system footprint
3.  **Security**: Exclude potentially vulnerable components
4.  **Performance**: Optimize for specific use cases
5.  **Flexibility**: Adapt system to different requirements

***

## Examples

### Desktop System

```bash
# Set up desktop profile
sigma-use profile set desktop

# Enable multimedia support
sigma-use set alsa true
sigma-use set pulseaudio true

# Enable Wayland for modern display
sigma-use set wayland true
```

### Server System

```bash
# Set up server profile
sigma-use profile set server

# Disable GUI components
sigma-use set X false
sigma-use set gtk false
sigma-use set qt5 false

# Enable server-specific features
sigma-use set openssl true
```

### Minimal System

```bash
# Set up minimal profile
sigma-use profile set minimal

# Disable most optional features
sigma-use set X false
sigma-use set gtk false
sigma-use set qt5 false
sigma-use set alsa false
```

***

**Generated:** August 24, 2026\
**Repository:** [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
