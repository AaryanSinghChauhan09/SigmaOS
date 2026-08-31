# ⚖️ SigmaOS Interoperability & Standards Compliance (POSIX, FHS, LSB) Development Roadmap

This document establishes the architecture and implementation roadmap for **SigmaOS's Compatibility & Standards subsystem**, allowing seamless integration with POSIX and FHS conventions without duplicating monolithic kernel bloat.

---

## 🏗️ 1. Technical Vision & Compliance Levels

Traditional microkernels require complete code rewrites to support old POSIX systems. SigmaOS introduces **Sandboxed Emulation Layers** and **Declarative Symlink Tiers** to support standard binaries at native speeds.

```
       +-------------------------------------------------------+
       |             Standard Linux / BSD Binaries             |
       +-------------------------------------------------------+
            |                        |                       |
            v                        v                       v
   +-----------------+      +-----------------+      +-----------------+
   |  POSIX Tier 2   |      |  FHS Enforcement|      |   LSB Emulation |
   | (Userland Libs) |      | (Sovereign VFS) |      | (Syscall Trans) |
   +-----------------+      +-----------------+      +-----------------+
```

---

## 💻 2. Custom POSIX Compliance Tiers (Rust / Zig)

### 2.1 Compatibility Subsystems
- **Inspiration**: Linux/POSIX APIs.
- **Tier 1 (Capability-Native)**: High-security applications compile natively with S-SEC capabilities.
- **Tier 2 (POSIX Subsystem)**: A modular, user-space POSIX compatibility layer in `src/compatibility/` translates traditional calls like `fork`, `exec`, and `pthread` to safe capability equivalents.

---

## 📂 3. Filesystem Hierarchy Standard (FHS) Subsystem (Rust)

### 3.1 Declarative Overlay Symlinks
- **Inspiration**: Linux FHS (/bin, /usr, /etc, /var, /lib).
- **Architecture**: SigmaOS utilizes an immutable, object-oriented distributed filesystem.
- **Implementation**: Standard directories are mounted as dynamic capability-gated overlay layers in `src/filesystem/vfs.rs`. This allows legacy scripts expecting `/bin/sh` or `/etc/hosts` to execute safely while keeping configurations secure and immutable.

---

## 🔄 4. Linux Standard Base (LSB) & ABI Emulation (Zig / Nim)

### 4.1 System Call Translation Subsystem
- **Inspiration**: LSB ABI standards, macOS Rosetta, and Wine.
- **Implementation (Zig)**: A lightweight ELF header parser and translation gate catches Linux x86_64 or ARM64 system call numbers and translates them on-the-fly to SigmaOS microkernel IPC transactions.
- **Implementation (Nim)**: User-space helper utilities manage environmental variables and map shared library dependencies (`ld.so`) inside micro-enclaves.

---

## 🎧 5. POSIX Standard Input & Output Device Node Interfaces (Rust / Zig)

To allow standard POSIX tools to seamlessly read and write key hardware components, SigmaOS maps the physical devices directly as compliant `/dev/` nodes in `src/filesystem/vfs.rs`:
- **Input Devices**: `/dev/input/keyboard`, `/dev/input/mouse`, `/dev/sound/mic`
- **Output Devices**: `/dev/sound/speaker`, `/dev/printer`, `/dev/fb0` (Monitor framebuffer)

```rust
// POSIX-compliant IOCTL identifiers for hardware peripherals control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceIoctlCommand {
    GetKeyboardLayout = 0x40044B01,
    SetKeyboardLeds = 0x40044B02,
    GetMouseSensitivity = 0x40044D01,
    SetSpeakerSampleRate = 0x40045301,
    PrinterStartJob = 0x40045001,
    PrinterFeedLine = 0x40045002,
}

// Representing standard metadata for device node queries
#[derive(Debug, Clone)]
pub struct DeviceNodeInfo {
    pub path: &'static str,
    pub major_number: u32,
    pub minor_number: u32,
    pub major_device_class: &'static str,
}

pub fn get_standard_device_node(path: &str) -> Option<DeviceNodeInfo> {
    match path {
        "/dev/input/keyboard" => Some(DeviceNodeInfo { path: "/dev/input/keyboard", major_number: 13, minor_number: 0, major_device_class: "input" }),
        "/dev/input/mouse" => Some(DeviceNodeInfo { path: "/dev/input/mouse", major_number: 13, minor_number: 32, major_device_class: "input" }),
        "/dev/sound/mic" => Some(DeviceNodeInfo { path: "/dev/sound/mic", major_number: 14, minor_number: 4, major_device_class: "sound" }),
        "/dev/sound/speaker" => Some(DeviceNodeInfo { path: "/dev/sound/speaker", major_number: 14, minor_number: 3, major_device_class: "sound" }),
        "/dev/printer" => Some(DeviceNodeInfo { path: "/dev/printer", major_number: 6, minor_number: 0, major_device_class: "printer" }),
        _ => None,
    }
}
```

---

## 🎨 6. Standard Theme Engines, CLI Search, and Settings Hub Interfaces (Rust)

To match the customizability of GTK/Qt while keeping a minimal, unified `#![no_std]` footprint:
*   **Theme Engine**: Employs a declarative parser for `.sigma-theme` specifications, translating UI color structures to framebuffers and terminal colors.
*   **Unified Search**: Instantiates a cached trie-based prefix matching system to resolve system files, settings, and wiki keywords.

```rust
// Represents a standardized unified theme color definition
#[derive(Debug, Clone, Copy)]
pub struct SigmaThemeColors {
    pub primary: u32,
    pub secondary: u32,
    pub background: u32,
    pub text: u32,
}

pub struct ThemeEngine {
    pub current_colors: SigmaThemeColors,
}

impl ThemeEngine {
    pub fn new() -> Self {
        Self {
            current_colors: SigmaThemeColors {
                primary: 0x7C3AED,
                secondary: 0x10B981,
                background: 0x0F172A,
                text: 0xF1F5F9,
            },
        }
    }
}
```

---

## 📅 7. Step-by-Step Implementation Roadmap

- [ ] **Phase 1 (Validation)**: Complete POSIX compliance and FHS path checker traits in `src/compatibility/standards.rs`.
- [ ] **Phase 2 (FHS Overlays)**: Integrate path verification logic directly into the VFS mount subsystem (`src/filesystem/vfs.rs`).
- [ ] **Phase 3 (Syscall Translation)**: Code the low-overhead LSB system call translation gate in Zig.
