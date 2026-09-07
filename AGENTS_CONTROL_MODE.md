# SigmaOS AI Agent Control Mode Operation Management Directive (`AGENTS_CONTROL_MODE.md`)

This document defines technical directives, session control protocols, and security access matrix guidelines for AI agents managing control mode operations in SigmaOS.

---

## 1. Core Principles for Control Mode Operation Management

Control mode operations in SigmaOS govern programmatic terminal session multiplexing (`tmux` control mode parser), system access control matrix (`AccessControlMatrix`) enforcement, and remote controller session state transitions. AI agents modifying control mode interfaces must observe the following rules:

1. **Terminal Control Mode Multiplexing & Parsing:**
   - Programmatic terminal control sessions (`tmux` control mode) communicate via framed notifications (`%output`, `%layout-change`, `%session-changed`).
   - Parsers must validate input notification escape sequences and buffer boundaries to prevent command injection or stream buffer overruns.

2. **Access Control Matrix Enforcement (`AccessControlMatrix`):**
   - User and process capabilities must be checked against the 2D grid rights mapping (`AccessControlMatrix`) before elevating session privileges or granting interactive control mode flags.
   - Mandatory Access Control (MAC - Bell-LaPadula MLS) and Discretionary Access Control (DAC) permissions must take precedence over user-requested control mode commands.

3. **Remote Control Session State Transitions:**
   - Remote controller sessions must explicitly track control states (active controlling vs passive viewing).
   - Session handovers and live process control migrations must authenticate client capabilities and record cryptographic audit logs before transferring interactive session ownership.

4. **Zero-Dependency `#![no_std]` Compatibility:**
   - Control mode parsing routines and access matrix checks in core libraries must maintain zero-dependency `#![no_std]` compliance.

---

## 2. Pre-Commit Control Mode Verification Checklist

Before submitting code modifications, AI agents must verify:
- [ ] Control mode escape sequence parsers handle malformed or truncated stream inputs safely.
- [ ] Access control matrix rights evaluations reject unauthorized session control requests.
- [ ] Session control state transitions log audit events prior to granting interactive control.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.
