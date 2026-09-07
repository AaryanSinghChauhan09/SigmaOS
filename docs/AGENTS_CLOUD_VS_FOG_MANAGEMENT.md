# AI Agent Guidelines: Cloud vs. Fog Management in SigmaOS

## 📌 1. Architectural Distinction & Overview

In **SigmaOS**, computing topologies are partitioned into two complementary operational tiers: **Cloud Computing** and **Fog/Edge Computing**.

As an AI agent developing orchestrators, network stacks, cluster managers, or distributed workload schedulers, you must understand when to execute tasks locally on Fog edge nodes versus when to offload workloads to centralized Cloud clusters.

---

## 📊 2. Cloud vs. Fog Comparison Matrix

| Attribute | ☁️ Cloud Tier | 🌫️ Fog/Edge Tier |
| :--- | :--- | :--- |
| **Primary Location** | Centralized Data Centers / Sovereign Clusters | Decentralized LAN Peers, Gateways, Edge Devices |
| **Target Latency** | High / Tolerant ($50\text{--}200\text{ ms}$) | Ultra-Low / Real-Time ($1\text{--}10\text{ ms}$) |
| **Bandwidth Usage** | High (transfers raw telemetry/data) | Low (transfers pre-processed deltas/CRDTs) |
| **Autonomy Level** | Dependent on persistent InternetWAN | Fully Autonomous (operates offline / disconnected) |
| **Core Subsystems** | `src/remote/shell.rs`, `src/open_source_obsoletion.rs` | `src/orchestration/mod.rs`, `src/network/distro_net.rs` |
| **Security Perimeter** | SPIFFE/SPIRE, Post-Quantum WireGuard/PQC VPN | Capability Tokens, OpenBSD Pledge/Unveil, CARP Failover |

---

## ⚙️ 3. Offloading Decision Engine & Mathematical Model

When scheduling background tasks, container workloads, or AI inference requests, AI agents must evaluate the **Dynamic Offloading Cost Function**:

$$\text{Cost} = w_1 \cdot \text{Latency} + w_2 \cdot \text{EnergyPenalty} + w_3 \cdot \text{BandwidthCost} + w_4 \cdot \text{ComputeLoad}$$

### Decision Rules:
1. **Real-Time / Hard Deadline Tasks ($<10\text{ ms}$):**
   * *Examples:* Audio DSP processing, UI/Zenith compositor rendering, local file I/O, device driver interrupts.
   * **Rule:** Execute **Locally** on the local node or immediate LAN Fog peer.
2. **Collaborative Edge Processing ($10\text{--}50\text{ ms}$):**
   * *Examples:* Local P2P file sharing, localized ML feature extraction, zero-copy packet inspection.
   * **Rule:** Offload to adjacent **Fog Peers** via lock-free P2P mesh channels.
3. **Compute-Intensive Batch Tasks ($>100\text{ ms}$):**
   * *Examples:* Heavy neural network training, full-system vulnerability scanning, multi-year log archiving.
   * **Rule:** Offload asynchronously to **Cloud Clusters** via encrypted PQC WireGuard tunnels.

---

## 🛡️ 4. Key Subsystem Design Directives

### 4.1 Local Fog Autonomy
* Edge nodes must continue functioning without degradation if WAN/Cloud connectivity drops.
* All state updates on Fog nodes use **Conflict-Free Replicated Data Types (CRDTs)** or local append-only journal logs.
* When WAN connection is restored, state synchronization is performed asynchronously using compact delta-patches.

### 4.2 Security & Capability Sandboxing Across Tiers
* **Fog Nodes:** Authorized via localized hardware-bound 64-bit `CapabilityToken` instances.
* **Cloud Endpoints:** Authorized via SPIFFE ID verification (`spiffe://sigmaos/cloud/worker`) and post-quantum Dilithium-5 signatures.

---

## 🧪 5. Verification & Testing Commands

AI agents must verify orchestration and cloud/fog workload routing via standalone unit compilation:

```bash
# Test cross-device orchestration & fog node management
rustc --test --edition=2021 src/orchestration/mod.rs -o build/orchestration_tests && ./build/orchestration_tests && rm build/orchestration_tests

# Test P2P mesh identity & cloud obsoletion engine
rustc --test --edition=2021 src/open_source_obsoletion.rs -o build/obsoletion_tests && ./build/obsoletion_tests && rm build/obsoletion_tests
```
