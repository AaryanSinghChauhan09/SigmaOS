# AI Agent State Management Architecture in SigmaOS

This document specifies state management architectures, generation snapshot rollbacks, and MVI reactive state store abstractions for AI agents modifying or extending system configuration state in SigmaOS.

---

## 🏛️ 1. State Subsystem Architecture

SigmaOS provides dual state management layers:

```
+-----------------------------------------------------------------------+
| Declarative System State Graph (`src/system/state.rs`)                |
| NixOS-inspired DeclarativeStateGraph, StateNode, generations & rollback|
+-----------------------------------------------------------------------+
| Model-View-Intent (MVI) Reactive Store (`src/klib/store.rs`)         |
| StateStore<S, A>, Reducer<S, A>, reactive subscribers & actions       |
+-----------------------------------------------------------------------+
```

---

## ⚙️ 2. Declarative State Graph Rules (`src/system/state.rs`)

1. **State Node Mutations**
   - Node values (`StateValue`) support `Boolean`, `Integer`, `String(&'static str)`, and `Array([Box<StateValue>; 16])`.
   - Node updates (`update_node`) automatically trigger generation snapshots when `create_generation` is invoked.

2. **Atomic Generation Rollback**
   - The `rollback()` method restores the previous generation of the state graph atomically, resetting all state node values to their previous historical snapshot.

3. **Dependency Graph Validation**
   - Before applying system configuration changes, `validate()` **must** be called to verify that all dependency node IDs exist (`StateError::DependencyNotFound`).

---

## ⚙️ 3. Verification Commands for State Agents

- **Declarative State Graph Tests:**
  `cargo test --lib -- system::state::tests`
- **MVI Reactive Store Tests:**
  `cargo test --lib -- klib::store::tests`
