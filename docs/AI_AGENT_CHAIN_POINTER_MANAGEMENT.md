# AI Agent Chain Pointer Management in SigmaOS

## Overview
SigmaOS incorporates a clean-room Kernel Notifier Chain Subsystem managed by autonomous AI Agents (**Sentinel** 🛡️, **Bolt** ⚡, **Palette** 🎨). This document defines operational directives, priority sorting rules, event propagation return codes, and monitoring interfaces for AI agents supervising callback chains and event notification structures.

AI agents interact directly with `src/kernel/linux_parity.rs` (`KernelNotifierChain`, `NotifierBlock`).

---

## 1. Kernel Notifier Chain Architecture

### 1.1 Priority-Ordered Notification Chain (`KernelNotifierChain`)
Implemented in `src/kernel/linux_parity.rs`. Provides synchronous, priority-ordered event broadcast chains across kernel subsystems (e.g., netdev state changes, reboot events, module loading):
```rust
pub struct NotifierBlock {
    pub priority: i32,
    pub name: &'static str,
}

pub struct KernelNotifierChain {
    pub blocks: Vec<NotifierBlock>,
}
```

### 1.2 Registration & Priority Sorting Protocol
* **Registration (`notifier_chain_register`)**:
  Subsystems register notification callbacks with an integer `priority`. Higher numerical priorities execute earlier in the chain:
  ```rust
  pub fn notifier_chain_register(&mut self, name: &'static str, priority: i32) {
      let block = NotifierBlock { priority, name };
      self.blocks.push(block);
      self.blocks.sort_by(|a, b| b.priority.cmp(&a.priority));
  }
  ```

### 1.3 Event Call Propagation & Return Status Codes
* **Notification Invocation (`notifier_call_chain`)**:
  Events propagate through registered blocks in priority order. Return status codes govern chain execution:
  - `NOTIFY_DONE` (`0x0000`): Event processing completed without action.
  - `NOTIFY_OK` (`0x0001`): Event handled successfully.
  - `NOTIFY_BAD` (`0x0002`): Error occurred; halts further chain traversal.

---

## 2. AI Agent Operational Directives & Monitoring Rules

### 2.1 Chain Ordering Invariants
1. **Priority Sorting Enforcement**:
   After any `notifier_chain_register` operation, AI agents verify that `self.blocks` remains sorted in descending order (`b.priority.cmp(&a.priority)`).
2. **Re-Entrancy & Deadlock Prevention**:
   Notifier callbacks must not register new blocks into the active chain during invocation to prevent deadlocks and iterator invalidation.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Query registered kernel notifier blocks sorted by priority
sigma-chain status --chain netdev

# Register a custom security monitoring notifier block
sigma-chain register --chain security --name lsm_audit --priority 100

# Benchmark notifier chain call latency under heavy event load
sigma-chain bench --chain reboot --iters 100000
```
