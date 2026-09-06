# SigmaOS sysfs — Kernel Object Virtual Filesystem

## Overview

SigmaOS sysfs exposes kernel objects (kobjects), device attributes, driver info, and tunable parameters through a virtual filesystem at `/sys`. Fully sovereign — no Linux kernel dependency.

**Location:** `src/fs/sigma_sysfs.rs`

---

## /sys Layout

```
/sys/
├── block/            Block devices
├── bus/              Bus types (PCI, USB, platform)
├── class/
│   └── net/
│       ├── lo/       Loopback interface
│       │   ├── address (RO)    00:00:00:00:00:00
│       │   ├── mtu (RO)        65536
│       │   └── flags (RW)      0x49
│       └── eth0/
│           ├── address (RO)    02:42:ac:11:00:02
│           ├── mtu (RO)        1500
│           ├── speed (RO)      1000
│           └── operstate (RO)  up
├── devices/          Device tree (PCI at 0000:XX:XX.X)
├── kernel/
│   ├── mm/
│   │   ├── overcommit_memory (RW)  0
│   │   ├── swappiness (RW)         60
│   │   └── dirty_ratio (RW)        20
│   └── debug/
│       ├── kprobes_enabled (RO)
│       └── tracing_enabled (RW)
└── module/           Loaded modules
```

---

## API Reference

```rust
let mut sysfs = SigmaSysfs::new();

// Read an attribute
let val = sysfs.read("/sys/kernel/mm/swappiness").unwrap();
// Returns b"60\n"

// Write an attribute
sysfs.write("/sys/kernel/mm/swappiness", "10").unwrap();

// List directory
let entries = sysfs.readdir("/sys/class/net").unwrap();
// Returns ["lo", "eth0"]

// Register a PCI device
let id = sysfs.register_pci_device(0, 2, 0, 0x8086, 0x1234, 0x030200, "vga");

// Create custom kobject
let parent = sysfs.find_path("/sys/devices").unwrap();
let dev = sysfs.mkdir_at(parent, "my-device").unwrap();
sysfs.add_attr_ro(dev, "model", AttrValue::Text("SigmaOS Device".into()));
sysfs.add_attr_rw(dev, "enable", AttrValue::Bool(true));
```

---

## Attribute Types

| Type | Format | Example |
|------|--------|---------|
| `Integer(i64)` | `"-42\n"` | irq count |
| `Unsigned(u64)` | `"1000\n"` | MTU |
| `Bool(bool)` | `"0\n"` or `"1\n"` | enabled |
| `Text(String)` | `"up\n"` | operstate |
| `Hex(u64)` | `"0x1043\n"` | flags |

---

## Comparison

| Feature | Linux sysfs | BSD devfs | SigmaOS sysfs |
|---------|------------|----------|--------------|
| Kobject tree | Yes | No | Yes |
| Network attrs | Yes | No | Yes |
| PCI device attrs | Yes | Limited | Yes |
| Writeable attrs | Yes | No | Yes |
| no_std | No | No | **Yes** |
