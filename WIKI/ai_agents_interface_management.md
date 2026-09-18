# AI Agent Developer Guide: Interface Management in SigmaOS

This guide provides guidelines, Rust API references, CLI commands, and safety constraints for AI Autonomous Agents managing **Network, Display/UI, and Hardware Driver Interfaces** in SigmaOS.

---

## 1. Subsystem Architecture Overview

Interface Management in SigmaOS spans three primary interface domains:

```
                  +----------------------------------------------+
                  |         SigmaOS AI Agent Core                |
                  +----------------------------------------------+
                                         |
         +-------------------------------+-------------------------------+
         |                               |                               |
         v                               v                               v
+------------------+           +------------------+           +------------------+
| Network          |           | Display / UI     |           | Hardware Driver  |
| Interfaces       |           | Interfaces       |           | Interfaces       |
+------------------+           +------------------+           +------------------+
| - Ethernet / Wi-Fi|           | - Zenith Wayland |           | - PCI ECAM Bus   |
| - WireGuard VPN  |           | - GTK3/4 Toolkit |           | - USB XHCI       |
| - eBPF Data-Plane|           | - Control Center |           | - Driver Shards  |
+------------------+           +------------------+           +------------------+
```

1. **Network Interfaces (`src/net/`, `src/network/`):** Universal socket abstraction, TCP/IP stack, eBPF programmable filters, WireGuard/OpenVPN virtual interfaces, and BGP/OSPF routing interfaces.
2. **Display & Graphical UI Interfaces (`src/graphics/`, `src/desktop/`, `src/ui/`):** Zenith Wayland layer-shell compositor, HiDPI screen scaling, GTK3/4 inspired `GtkHeaderBar` / `GtkBox` widget toolkit, and Zenith Control Center settings panels.
3. **Hardware Driver & Bus Interfaces (`src/hal/`, `src/driver/`, `src/device/`):** `SovereignDriver` OOP lifecycle (`probe`, `init`, `handle_irq`, `shutdown`), PCI ECAM bus enumeration, and hot-swappable driver shards.

---

## 2. Rust API Reference for AI Agents

AI Agents can inspect and configure interfaces directly via Rust API bindings:

### A. Network Interface Management

```rust
use sigmaos::net::{SimpleNetworkStack, NetworkInterface, IPv4Address, MacAddress};

// 1. Initialize Network Stack
let mut net_stack = SimpleNetworkStack::new();

// 2. Configure IPv4 / MAC on Ethernet Interface
let mac = MacAddress::new(0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E);
let ip = IPv4Address::new(192, 168, 1, 100);
let netmask = IPv4Address::new(255, 255, 255, 0);

net_stack.add_interface(NetworkInterface::new("eth0", mac, ip, netmask));

// 3. Query Active Network Interfaces
let active_interfaces = net_stack.list_interfaces();
assert!(!active_interfaces.is_empty());
```

### B. Display & Graphical UI Interface Management

```rust
use sigmaos::ui::toolkit::{GtkHeaderBar, GtkBox, GtkOrientation, GtkDisplayMetrics};
use sigmaos::desktop::control_center::ZenithControlCenterPanel;

// 1. Query HiDPI Scaling Metrics
let metrics = GtkDisplayMetrics::new(3840, 2160, 2.0); // 4K @ 2x HiDPI
assert_eq!(metrics.scale_factor, 2.0);

// 2. Build Zenith Client-Side Decoration HeaderBar
let mut header = GtkHeaderBar::new();
header.set_title("Network & Control Settings");
header.set_show_close_button(true);

// 3. Register Control Center Panel
let mut control_center = ZenithControlCenterPanel::new();
control_center.register_subpanel("network_interface_manager");
```

### C. Hardware Driver Interface Management

```rust
use sigmaos::driver::{DriverManager, SovereignDriver, DriverError};
use sigmaos::device::udev_devd_rules::{UdevDevdRuleEngine, HotplugDeviceEvent, DeviceEventAction};

// 1. Hotplug Device Rule Evaluation
let mut udev_engine = UdevDevdRuleEngine::new();
let event = HotplugDeviceEvent {
    action: DeviceEventAction::Add,
    subsystem: "net".to_string(),
    sysname: "eth0".to_string(),
    driver: Some("e1000e".to_string()),
    env: Default::default(),
};

let matches = udev_engine.process_event(&event);
assert!(matches >= 1);
```

---

## 3. CLI Commands for AI Agents

AI Agents invoking terminal shell pipelines or remote management agents can use structured CLI tools:

### Network Interface CLI
```bash
# List all active physical and virtual interfaces in JSON format
sigma-net link list --json

# Assign IPv4 address and netmask
sigma-net ip addr add 192.168.1.100/24 dev eth0

# Connect to Wi-Fi SSID
sigma-net wifi connect --ssid "SigmaOS-Secure" --passphrase "*****"

# Enable WireGuard VPN Interface
sigma-net vpn up wg0
```

### Zenith Display & UI CLI
```bash
# Query active display metrics, resolution, and HiDPI scaling
zenith-ctl display info --json

# Set Zenith desktop theme & dark mode
zenith-ctl theme set --mode dark --palette sovereign-emerald

# Launch control center subpanel
zenith-ctl control-center open --panel network
```

### Hardware Driver CLI
```bash
# List loaded driver shards and PCI device bindings
sigma-driver list --json

# Hot-reload network driver shard
sigma-driver reload e1000e
```

---

## 4. Security, PolicyKit & Safety Rules for AI Agents

When performing interface modifications, AI Agents MUST comply with the following constraints:

1. **PolicyKit Authorization (`polkit`):**
   - Network interface reconfigurations require PolicyKit action `org.sigmaos.network.configure`.
   - Driver shard loading or unloading requires PolicyKit action `org.sigmaos.driver.load`.
2. **Non-Disruptive Fallback:** AI Agents modifying remote or network interfaces MUST set up a auto-revert timer (e.g. 30 seconds) to prevent losing connectivity in case of invalid IP or firewall settings.
3. **Sandbox Compliance:** UI widgets created by AI Agents MUST run within retrocompatibility or process sandboxes (`src/system/sandbox.rs`) without direct raw memory access to compositor framebuffers.

---
*Maintained by the SigmaOS Core Architecture Team & AI Governance Board.*
