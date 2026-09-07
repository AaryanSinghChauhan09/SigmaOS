# AI Agent Control Mode Operation Management Architecture (`docs/AGENTS_CONTROL_MODE.md`)

This guide details the technical architecture, session multiplexing parsers, and AI agent monitoring protocols for control mode operations in SigmaOS.

---

## 1. Subsystem Architecture

SigmaOS implements control mode session management across terminal productivity and access control subsystems:

### A. Terminal Control Mode Session Multiplexing
- Located in `src/productivity/tmux.rs`.
- Implements `tmux` control mode parser protocol, parsing framed stream notifications (`%output`, `%session-changed`, `%layout-change`) for programmatic terminal automation and AI agent session control.

### B. Access Control Matrix & Security
- Located in `src/access/control.rs`.
- Enforces subject/object permission rights via `AccessControlMatrix` (ACM 2D grid rights mapping) combined with Bell-LaPadula MLS (Multilevel Security) rules.

### C. Remote Session Control & Migration
- Located in `src/access/mod.rs`.
- Controls interactive remote access sessions, switching between passive view mode and active controlling mode with live process migration controls.

---

## 2. AI Agent Operational Directives

1. **Parser Input Sanitization:** Ensure control mode stream notification parsers validate framed boundaries to prevent command injection.
2. **Access Rights Validation:** Confirm all control mode privilege elevations evaluate `AccessControlMatrix` permissions prior to granting interactive access.
3. **Automated Verification:** Execute `./run_sigma_tests.sh` to confirm productivity and access control unit tests pass.
