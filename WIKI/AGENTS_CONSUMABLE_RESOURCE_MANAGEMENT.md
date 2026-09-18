# AI Agent Guidelines: Consumable Resource Management in SigmaOS

## 📌 1. Architectural Distinction & Overview

In **SigmaOS**, system resources are categorized into two fundamental operational classes:
* **Reusable Resources:** Allocated to a process and subsequently returned to the system for reuse (e.g. Physical Memory Pages, CPU Time Slices, File Descriptors, PCI Devices).
* **Consumable Resources:** Produced dynamically, consumed during execution, and destroyed upon usage (e.g. Cryptographic Entropy Bits, Ephemeral Network Ports, Hardware Interrupt Signals, Power/Energy Budgets, IPC Messages).

As an AI agent developing microkernel components, security managers, or network stacks, you must enforce **depletion prevention, backpressure flow control, and dynamic replenishment** for all consumable resources.

---

## ⚙️ 2. Key Consumable Resource Subsystems

### 2.1 Cryptographic Entropy Pool Management
* **Module Location:** `src/klib/rand.rs`, `src/security/secrets.rs`
* **Invariants:**
  * Tracks hardware CSPRNG entropy bits generated via RDRAND / RDSEED / TRNG hardware sources.
  * Consumed when generating Post-Quantum Cryptographic keys (Kyber-1024, Dilithium-5) or TLS session tokens.
  * **Depletion Rule:** If entropy drops below **256 bits**, key generation requests MUST block until hardware jitter entropy replenishment finishes.

### 2.2 Power & Energy Budget Quotas
* **Module Location:** `src/power/governor.rs`, `src/process/activity_manager.rs`
* **Invariants:**
  * Tracks energy consumption in milliwatt-hours ($mWh$).
  * Process cgroups v2 resource limits enforce maximum energy budgets for background AI inference or batch builds.
  * **Power Throttling:** When battery capacity drops below $15\%$, background consumable energy quotas are reduced by $80\%$.

### 2.3 Ephemeral Network Ports & Socket Allocation
* **Module Location:** `src/kernel/net/socket_layer.rs`, `src/network/`
* **Invariants:**
  * Manages the consumable ephemeral port range ($49152\text{--}65535$).
  * On high-concurrency TCP connections, ports are consumed and enter `TIME_WAIT` states before recycling.
  * Uses `SO_REUSEADDR` / `SO_REUSEPORT` flags and port reuse hashing to mitigate port exhaustion.

### 2.4 Interrupt Signals & APC Message Queues
* **Module Location:** `src/interrupt/handler.rs`, `src/kernel/structures.rs`
* **Invariants:**
  * Interrupt vector signals and Asynchronous Procedure Calls (APCs) are consumed upon execution by target threads.
  * Bounded APC queues use lock-free ring buffers with drop policies (`DropOldest` / `RejectNew`) to prevent kernel queue memory starvation.

---

## 📊 3. Consumable Resource Depletion & Flow Control Matrix

| Consumable Resource | Producer Source | Consumer Subsystem | Depletion Prevention Rule |
| :--- | :--- | :--- | :--- |
| **Entropy Bits** | Hardware TRNG / Jitter | PQC Vault / WireGuard | Block crypto requests if pool $<256$ bits |
| **Ephemeral Ports** | Network Stack Pool | TCP/UDP Sockets | Enable socket reuse; alert if available ports $<1000$ |
| **Energy ($mWh$)** | Battery / Power Supply | Process Cgroups / Background Tasks | Throttle background threads when battery $<15\%$ |
| **IPC Ring Buffers** | Sender Process | Receiver Process | Apply producer backpressure when ring buffer $\ge 90\%$ full |

---

## 🛡️ 4. AI Agent Guidelines

1. **Backpressure Flow Control:**
   * Never allow unthrottled producers to consume kernel memory by flooding IPC message queues. Apply `EAGAIN` or block producers when buffers reach $90\%$ capacity.
2. **Entropy Safety:**
   * Never use pseudo-random fallback algorithms when real CSPRNG entropy is depleted for security-sensitive operations. Always block until entropy is replenished.
3. **Zero-Leakage Consumable Destruction:**
   * Ensure consumed IPC message buffers and ephemeral tokens are securely zeroed in memory (`secure_zeroize()`) upon consumption.

---

## 🧪 5. Standalone Testing Procedures

AI agents can verify consumable resource trackers, entropy managers, and power governors via standalone unit compilation:

```bash
# Test CSPRNG entropy accumulation and random generation
rustc --test --edition=2021 src/klib/rand.rs -o build/rand_tests && ./build/rand_tests && rm build/rand_tests

# Test power governor & energy quota management
rustc --test --edition=2021 src/power/governor.rs -o build/power_tests && ./build/power_tests && rm build/power_tests
```
