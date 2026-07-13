# Distro Absorption: Tails

> **Status**: 📋 Planned | **Source Paradigm**: Tails (The Amnesic Incognito Live System) | **Target Shard**: `SigmaOS Amnesic Network Profile`

---

## 1. Executive Summary

Tails is a portable Linux distribution designed to protect against surveillance and censorship. It forces all outbound network connections through the Tor network and is "amnesic" — it leaves no trace on the host computer unless explicitly configured to do so.

SigmaOS absorbs the **Amnesic Live Profile** and **Transparent Tor Routing** features, allowing any SigmaOS installation to dynamically switch into a mathematically secure, trace-free state.

---

## 2. Key Features to Absorb

### 2.1 The Amnesic State (`sigma-amnesia`)

When SigmaOS is booted in (or switched to) Amnesic Mode, it immediately unmounts all persistent storage, running entirely from RAM (`tmpfs`). 

```bash
$ sigma mode enter amnesic
Σ [AMNESIA] Transitioning to Amnesic Mode...
  Locking and unmounting /home
  Locking and unmounting /var/log
  Creating 4GB tmpfs overlay...
  Done. All future writes will vanish on power loss.
```

To ensure memory forensics cannot recover data after shutdown, SigmaOS implements a kernel shutdown hook that explicitly overwrites all RAM with random noise before the ACPI power-off signal is sent (`sdmem` equivalent).

### 2.2 Transparent Tor Proxying

In the Amnesic profile, the `sigma-networking` shard configures `sigma-net-policy` (eBPF firewall) to drop all non-Tor traffic.

```rust
// kernel/net/amnesic_policy.rs
// SPDX-License-Identifier: MIT

#[xdp_program]
pub fn amnesic_packet_filter(ctx: XdpContext) -> XdpAction {
    // Only allow traffic from the tor daemon (UID tor)
    if get_socket_uid(ctx) != UID_TOR {
        return XdpAction::Drop; // Block leaks entirely at the driver level
    }
    
    XdpAction::Pass
}
```

```bash
# Verify network isolation
$ sigma net status
Σ [NET] Amnesic Profile Active:
  All outbound TCP routed through Tor proxy (127.0.0.1:9050)
  All UDP dropped (except DNS over Tor)
  IPv6 disabled to prevent leaks
  Current Tor Exit Node: CH (Switzerland)
```

### 2.3 MAC Address Spoofing

Before bringing up any physical network interface, SigmaOS automatically randomizes the hardware MAC address to prevent tracking across physical locations.

```bash
Σ [NET] wlan0 initializing...
  Real MAC: a4:83:e7:xx:xx:xx
  Spoofed MAC: 00:50:56:xx:xx:xx (Vendor: VMware)
```

---

## 3. References & Standards

- Tails Project — `tails.net` (GPL-3.0)
- Tor Project — `torproject.org` (BSD-3-Clause)
- macchanger — `github.com/alobbs/macchanger` (GPL-2.0)
