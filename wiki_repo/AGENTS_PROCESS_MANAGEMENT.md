# SigmaOS Process Lifecycle, Signal ABI Translation & Supervision Guide for AI Agents

This guide provides technical specifications, process lifecycle state machine transitions, cross-OS signal ABI translation, pseudo-terminal (PTY) master/slave pairing, process sandboxing, and service supervision rules for AI agents managing processes in SigmaOS.

---

## 1. Zero-Dependency Process Architecture

SigmaOS implements a high-performance process management engine under `#![no_std]` Rust (`src/process/advanced_process_control.rs`, `src/compatibility/abi_translator.rs`, `src/shell/terminal_emulator.rs`):

* **Process Lifecycle Controller (`SovereignProcessLifecycleController`):**
  Manages process lifecycle state transitions (`Created`, `Ready`, `Running`, `Blocked`, `Stopped`, `Zombie`, `Terminated`) under thread-safe synchronization.
* **Cross-OS Signal & Syscall ABI Translation (`translate_syscall_abi`):**
  Translates POSIX, Linux x86_64, and BSD syscall vectors and signal frame delivery layouts dynamically across execution environments.
* **Pseudo-TTY Master/Slave Pairing (`PtyMasterSlavePair`):**
  Provides termios job control, session line discipline, window resize SIGWINCH propagation, and master/slave character buffer streaming.
* **Systemd-Free Runit Service Supervision (`SovereignRunitSupervisor`):**
  Provides 3-stage service lifecycle supervision (`stage1` boot initialization, `stage2` process monitoring & auto-restart, `stage3` graceful shutdown).

---

## 2. Process Descriptor $O(1)$ Name Lookup Invariant

When modifying process structures or scheduler process descriptors:

* **Task Name Optimization (`SimpleProcess` in `src/scheduler/process.rs`):**
  `SimpleProcess` MUST store an explicit `name_len: u8` field initialized during process creation (`new()`).
* **Slice Access:**
  `Process::name(&self)` MUST use `&self.name_bytes[..self.name_len as usize]` to achieve $O(1)$ direct slice lookups, avoiding $O(N)$ null-byte search scans (`.position(|&b| b == 0)`).

---

## 3. Sandboxing & Resource Isolation Rules

1. **OpenBSD Pledge & Unveil Enforcement:**
   Processes MUST enforce syscall promises (`pledge`) and restricted VFS path visibility (`unveil`) via `AutomatedSandboxPolicy` (`src/automation/system_level.rs`).
2. **Resource Throttling:**
   Process memory RSS, CPU percentage limits, and swap bounds MUST be enforced via FreeBSD RACCT/RCTL rules (`AutomatedRacctPolicy`).

---

## 4. Checklist for AI Agents Managing Process Subsystems

1. **Verify $O(1)$ Name Invariant:** Ensure process structs maintain explicit name byte length fields.
2. **Test Process & Terminal Emulator Pipelines:**
   Run process control and terminal emulator unit tests:
   ```bash
   cargo test --lib -- process::advanced_process_control::tests
   ./run_sigma_tests.sh
   ```
