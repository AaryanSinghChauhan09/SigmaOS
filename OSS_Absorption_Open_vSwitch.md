# OSS Absorption: Open vSwitch — Virtual Networking

> **Status**: 🔄 Active | **Source Project**: Open vSwitch (OVS) | **Target Shard**: `SigmaOS Virtual Network Switch`

---

## 1. Executive Summary

Open vSwitch (OVS) is a production quality, multilayer virtual switch designed to enable massive network automation through programmatic extension, while supporting standard management interfaces and protocols (e.g. NetFlow, sFlow, IPFIX, RSPAN, CLI, LACP, 802.1ag).

SigmaOS absorbs OVS concepts for `sigma-vswitch`, providing the advanced networking backbone required by `sigma-vm` and `sigma-podman` to seamlessly bridge isolated components.

---

## 2. Key Features Absorbed

### 2.1 Virtual Switching Architecture

Instead of relying solely on the Linux bridge module, SigmaOS uses an OVS-inspired architecture capable of handling complex VLAN tagging, tunneling (VXLAN/GRE), and flow-based routing required for container and VM networks.

```bash
# Create a virtual switch for VM networking
$ sigma net switch create br-int
Σ [NET] Virtual switch 'br-int' created.

# Attach physical interface to switch
$ sigma net switch attach br-int eth0
Σ [NET] eth0 attached to br-int.

# Create an isolated VLAN for untrusted VMs
$ sigma net vlan create 10 br-int --name untrusted
```

### 2.2 OpenFlow-inspired Programmability

SigmaOS exposes a programmatic interface (via IPC) that allows `sigma-net-policy` to dynamically install network flows, directly mirroring the SDN (Software Defined Networking) capabilities of Open vSwitch.

```rust
// kernel/net/vswitch.rs
// SPDX-License-Identifier: MIT

pub struct FlowRule {
    pub priority: u16,
    pub match_criteria: Match,
    pub action: Action,
}

impl VirtualSwitch {
    pub fn add_flow(&mut self, rule: FlowRule) -> Result<()> {
        // Drop all traffic from the untrusted VLAN heading to the internal network
        self.datapath.install_flow(rule)?;
        Ok(())
    }
}
```

---

## 3. References & Standards

- Open vSwitch — `openvswitch.org` (Apache-2.0)
- OpenFlow Specification
