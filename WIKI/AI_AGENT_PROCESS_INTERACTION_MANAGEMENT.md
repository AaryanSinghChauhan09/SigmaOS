# AI Agent Process Interaction Operation Management Architecture in SigmaOS

This document specifies process signaling, inter-process signal delivery systems, signal masking rules, real-time payload queues, and capability enforcement for AI agents working on process management and IPC in SigmaOS (`src/ipc/signals.rs`).

---

## ⚡ 1. Process Signal & IPC Subsystem Architecture

SigmaOS implements a high-performance, capability-gated asynchronous signal subsystem inspired by BSD and Linux POSIX signals:

```
+---------------------------------------------------------------------------------+
| Sender Process (PID `A`)                                                        |
| Invokes `SignalDeliverySystem::send_signal(sender, receiver, signal, payload)` |
+---------------------------------------------------------------------------------+
                                       |
                                       v
+---------------------------------------------------------------------------------+
| Capability Validation (`IPCCapability`)                                          |
| Enforces `sender.can_send` and `receiver.can_receive` permissions.               |
+---------------------------------------------------------------------------------+
                                       |
                                       v
+---------------------------------------------------------------------------------+
| Receiver Process Signal Queue (`ProcessSignalState` for PID `B`)                |
| Checks `signal_mask` bitmask. If unmasked, queues `PendingSignal` with payload.  |
+---------------------------------------------------------------------------------+
                                       |
                                       v
+---------------------------------------------------------------------------------+
| Signal Dispatch (`dispatch_next_signal`)                                        |
| Delivers signal to userland handler (`CustomHandler`), default or ignore.        |
+---------------------------------------------------------------------------------+
```

---

## ⚙️ 2. Signal Dispositions & Masking Rules

1. **Standard & Real-time Signal Types (`SignalType`)**
   - `SigInt` (2): Interrupt process.
   - `SigKill` (9): Immediate termination signal. **Unmaskable and unignorable**.
   - `SigUsr1` (10) / `SigUsr2` (12): Custom userland signals supporting optional binary payload buffers (`Vec<u8>`).
   - `SigSegv` (11): Segmentation violation.
   - `SigTerm` (15): Termination signal.

2. **Signal Masking (`sigprocmask`)**
   - `set_mask(mask)` blocks signals whose bit corresponds to `mask`.
   - **SIGKILL Exemption:** `SigKill.to_bit()` is cleared from the mask buffer (`mask & !sigkill_bit`), ensuring `SigKill` can never be masked or blocked.

3. **Capability-Gated Delivery (`IPCCapability`)**
   - Signal delivery requires `sender.capability.can_send == true` and `receiver.capability.can_receive == true`.

---

## 🛡️ 3. Rules & Directives for AI Agents

1. **Unmaskable SIGKILL Rule**
   - Never attempt to suppress or ignore `SigKill` when modifying process signal dispositions or masks.
2. **Real-time Signal Payloads**
   - When transmitting user payloads (`SigUsr1`/`SigUsr2`), ensure memory allocation is freed upon signal dispatch to avoid leaks.
3. **Capability Checks**
   - Verify process IPC capability flags before attempting cross-process signaling.

---

## ⚙️ 4. Verification Commands for Process Agents

- **Signal Subsystem Unit Tests:**
  `cargo test --lib -- ipc::signals::tests`
- **Full SigmaOS Pipeline:**
  `./run_sigma_tests.sh`
