# 🌐 SigmaOS v15.0 Zenith — Browser Edition

> **A sovereign OS purpose-built for the web. Privacy-first. PQC-secured. No tracking, ever.**

[![Release](https://img.shields.io/badge/release-v15.0--zenith--browser-cyan)](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-browser)
[![Architecture](https://img.shields.io/badge/arch-x86__64%20%7C%20ARM64-green)](https://github.com/AaryanSinghChauhan09/SigmaOS)
[![Privacy](https://img.shields.io/badge/privacy-PQC--hardened%20%7C%20zero--tracking-purple)](https://github.com/AaryanSinghChauhan09/SigmaOS)

---

## 📋 Overview

## 📋 Overview

**SigmaOS Zenith Browser** is a purpose-built, ultra-lean OS edition designed to deliver a completely sovereign web computing environment. The entire OS surface is optimized around the browsing and web-application workflow.

Built on the **SigmaOS Unified Core**, this edition includes the mandatory baseline toolset (Maintenance, Monitoring) and is layered with PQC-hardened sandboxing and SovereignBrowser for the ultimate privacy-first web experience.

| Property | Value |
|---|---|
| Edition | Zenith Browser |
| Version | v15.0.0 |
| Kernel | Sovereign Lattice Microkernel v15.0 (Browser-optimized) |
| Architecture | x86_64, ARM64 |
| Boot Time | <3 seconds to browser |
| Browser | SovereignBrowser v15.0 (Blink engine, PQC-secured) |
| Privacy | Zero telemetry, built-in ad/tracker blocking |
| Security | PQC-hardened TLS (Kyber-1024 + ECDHE), DNS-over-HTTPS |
| Target | Kiosk stations, Chromebook replacements, privacy-focused browsing |

---

## ⚡ Key Features

### 🌐 SovereignBrowser — The Flagship Application

- **Blink Rendering Engine**: Standards-compliant HTML5/CSS3/WebGL/WebAssembly
- **PQC-Hardened TLS**: Every HTTPS connection upgraded to Kyber-1024 hybrid exchange
- **Zero-Tracking Guarantee**: No telemetry, no fingerprinting APIs, sensors blocked by default
- **Built-In Ad Blocker**: SovereignBlock — 99.8% tracker/ad blocking via curated filter lists
- **DNS-over-HTTPS**: All DNS queries encrypted via SovereignDNS (supports DoH + DoT)
- **PQC Certificate Validation**: Dilithium-5 certificate chain verification
- **Multi-Profile Isolation**: Each browser profile is a separate S-ARMOR isolated process
- **Secure Clipboard**: Clipboard access requires explicit user permission per site
- **WebAssembly Sandbox**: WASM runs in a SovereignBPF-enforced sandbox

### 🔒 Privacy Architecture

- **Fingerprint Randomization**: Canvas, WebGL, audio fingerprinting randomized per session
- **Memory Isolation**: Tab processes run in separate kernel memory namespaces
- **S-ARMOR Sandboxing**: Browser process cannot access filesystem beyond `/home/user/Downloads`
- **Secure Delete**: Browser cache/temp files wiped with cryptographic overwrite on close
- **Incognito++**: "Sovereign Mode" — not just no history, but hardware-level memory wipe on session end
- **Zero DNS Leaks**: All DNS resolves through encrypted SovereignDNS proxy
- **Cookie Isolation**: Third-party cookies blocked; first-party isolated per domain

### 🚀 Performance for Web

- **3-Second Cold Boot**: Kernel loads browser-optimized shard set only
- **GPU-Accelerated Compositing**: WebGL/Canvas hardware-accelerated via Vulkan backend
- **Predictive Tab Loading**: Neural prefetch for next-tab navigation (optional)
- **Memory Pressure Control**: Tab hibernation frees RAM for active tabs automatically
- **5G/Wi-Fi 6 Optimized**: Network stack tuned for high-throughput, low-latency web

### 🛡️ Built-In S-VPN

- **Zero-Config VPN**: Activates with one click — routes all traffic via PQC-secured tunnel
- **Split Tunneling**: Choose which sites use VPN vs direct connection
- **Multi-Hop Support**: Route through multiple sovereign nodes for maximum anonymity
- **WireGuard Substrate**: Based on battle-tested WireGuard + PQC overlay

### 🖥️ Browser-Optimized Desktop

- **Minimal Zenith Shell**: Taskbar + browser launcher — nothing else visible by default
- **Fast App Switcher**: Alt+Tab cycles between browser windows instantly
- **Tab Bar Integration**: System-level tab overview (all windows, all profiles)
- **Kiosk Mode**: Full-screen locked browsing for public stations

---

## 💻 System Requirements

| Component | Minimum | Recommended |
|---|---|---|
| CPU | x86_64 (SSE4.2+) or ARM64 | Intel 8th Gen+ / AMD Zen 2+ |
| RAM | 2 GB | 8 GB+ |
| Storage | 8 GB | 32 GB SSD |
| GPU | VESA (software compositing) | Vulkan 1.1+ (hardware acceleration) |
| Network | Wi-Fi / Ethernet | Wi-Fi 6 / Gigabit Ethernet |
| Firmware | UEFI or Legacy BIOS | UEFI 2.4+ |
| Display | 1024×768 | 1920×1080+ |

---

## 🛠️ Installation Guide

### Method A — Full Installation (Dedicated Machine)

```bash

# Download Browser ISO

curl -LO https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.0-zenith-browser/SigmaOS-v15.0-Zenith-Browser-x86_64.iso

# Flash to USB (Linux/macOS)

sudo dd if=SigmaOS-v15.0-Zenith-Browser-x86_64.iso of=/dev/sdX bs=4M status=progress && sync
```

Boot → Select **"Install SigmaOS Browser Edition"**

Automated partition layout (no configuration needed):

```
/dev/sda1  →  256MB    EFI
/dev/sda2  →  4GB      Swap
/dev/sda3  →  rest     / (root — Browser OS)
```

### Method B — Live USB (No Installation Required)

```bash

# The Browser Edition ISO is fully functional as a Live USB

# Simply boot from USB — no data written to your hard drive

# Use for:

# - Private browsing sessions on foreign machines

# - Conference / hotel kiosk replacement

# - Emergency secure browsing
```

### Method C — Chromebook / Embedded ARM64

```bash

# ARM64 image for Chromebook replacements / mini PCs

curl -LO https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.0-zenith-browser/SigmaOS-v15.0-Zenith-Browser-arm64.img
sudo dd if=SigmaOS-v15.0-Zenith-Browser-arm64.img of=/dev/mmcblk0 bs=4M status=progress && sync
```

### Method D — Kiosk/Enterprise Deployment

```bash

# Unattended installation for kiosk stations

# Create preseed configuration:
cat > sigmaos-browser-preseed.conf << 'EOF'
hostname=kiosk-01
timezone=UTC
disk=/dev/sda
kiosk_mode=true
kiosk_url=https://your-enterprise-portal.com
auto_login=kiosk
lock_profile=true
vpn_auto_connect=true
EOF

# Boot with preseed:

# Kernel args: sigma.preseed=http://deploy-server/sigmaos-browser-preseed.conf

```

### Step — First Boot Configuration

```bash

# SovereignBrowser launches automatically on first boot

# Configure via GUI, or use CLI:

sovereign-browser-config --default-search brave   # Set search engine

sovereign-browser-config --enable-vpn             # Enable S-VPN

sovereign-browser-config --strict-privacy          # Maximum privacy mode

sovereign-dns-config --set-doh https://cloudflare-dns.com/dns-query  # Set DoH resolver

sigma-vpn --connect auto                           # Connect to nearest sovereign node

```

---

## 🔧 Browser & Privacy Functions Reference

### SovereignBrowser CLI

```bash
sovereign-browser --new-window                     # Open new browser window

sovereign-browser --incognito                      # Open Sovereign Mode session

sovereign-browser --profile work "https://..."     # Open URL in specific profile

sovereign-browser --kiosk "https://portal.com"     # Kiosk locked mode

sovereign-browser --import-bookmarks bookmarks.html # Import bookmarks

sovereign-browser --clear-all-data                 # Wipe all browser data

```

### SovereignBlock — Ad & Tracker Blocking

```bash
sovereignblock --status                            # Show blocking statistics

sovereignblock --update-lists                      # Update filter lists

sovereignblock --whitelist "github.com"            # Allow site tracking

sovereignblock --add-custom-list "https://..."     # Add custom filter list

sovereignblock --report                            # Generate privacy report

```

### sigma-vpn — Built-In VPN

```bash
sigma-vpn --status                                 # VPN connection status

sigma-vpn --connect auto                           # Auto-select fastest node

sigma-vpn --connect "node-sg-01"                   # Connect to specific node

sigma-vpn --disconnect                             # Disconnect VPN

sigma-vpn --list-nodes                             # Show available VPN nodes

sigma-vpn --split-tunnel add "192.168.1.0/24"     # Add to direct route (bypass VPN)

sigma-vpn --check-leak                             # DNS/WebRTC leak test

```

### sovereign-dns — DNS Privacy Manager

```bash
sovereign-dns --status                             # Current DNS configuration

sovereign-dns --set-doh "https://dns.google/dns-query"  # Set DoH provider

sovereign-dns --set-dot "1.1.1.1:853"             # Set DoT provider

sovereign-dns --flush-cache                        # Clear DNS cache

sovereign-dns --block-malware                      # Enable malware domain blocking

sovereign-dns --block-ads                          # Enable DNS-level ad blocking

sovereign-dns --query "example.com"               # Manual encrypted DNS query

```

### Profile Management

```bash
sovereign-browser --list-profiles                  # List all browser profiles

sovereign-browser --create-profile "Finance"       # Create isolated profile

sovereign-browser --delete-profile "Finance"       # Delete profile + all data

sovereign-browser --export-profile "Finance" ~/    # Export profile backup

sovereign-browser --switch-profile "Finance"       # Switch active profile

```

---

## 🔐 Privacy Capabilities Comparison

| Feature | Chrome | Firefox | SigmaOS Zenith Browser |
|---|---|---|---|
| Telemetry | ❌ Heavy | ⚠️ Some | ✅ Zero |
| Fingerprint Protection | ⚠️ Basic | ⚠️ Enhanced | ✅ Full Randomization |
| PQC-Hardened TLS | ❌ | ❌ | ✅ Kyber-1024 hybrid |
| Built-In VPN | ❌ | ❌ | ✅ S-VPN included |
| DNS Encryption | ⚠️ Optional | ⚠️ Optional | ✅ Mandatory DoH/DoT |
| Process Isolation | ⚠️ Sandbox | ⚠️ Sandbox | ✅ Kernel-level S-ARMOR |
| Memory Wipe on Close | ❌ | ❌ | ✅ Cryptographic overwrite |
| Ad Blocking | ❌ | ⚠️ Extension | ✅ Built-in SovereignBlock |

---

## 🆘 Support & Resources

- **Release Page**: [v15.0-zenith-browser](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-browser)
- **Browser Integration Guide**: [Browser-Integration](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Browser-Integration)
- **Privacy Architecture**: [Security-Safety](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Security-Safety)
- **Issue Tracker**: [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

---

*SigmaOS v15.0 Zenith Browser — The sovereign web. Your privacy, enforced at silicon level.*
