# SigmaOS Agent Safety Policy

## 1. Principles of Least Privilege
- **No Blanket Credentials:** Agents never possess persistent API keys, root credentials, or administrative access.
- **Isolated Sandboxes:** Any untrusted package script, test harness, or foreign binary execution must occur inside isolated namespaces/containers with restricted VFS, network, and memory access.
- **No Force-Pushes or Direct Commits:** All changes must arrive via branch pull requests and require validation gating.

## 2. Sandbox Permission Matrix
| Agent Class | Filesystem Scope | Network Access | Package Execution | Git Privilege |
|---|---|---|---|---|
| Documentation / Onboarding | Read repo, write `docs/` & `reports/` | Optional, allowlisted | No | PR only |
| Build & Test Sentinel | Read repo, write `reports/` & tmp | No | No | PR only (mechanical) |
| Boot & Runtime Validator | Read repo, write build artifacts | No (unless required) | No | PR only |
| Capability Auditor | Read-only repo & reports | No | No | Issue / PR only |
| Package Compatibility | Sandboxed virtual workspace | No by default | Sandboxed dry-run only | PR only |
| Kernel & Subsystem Agents | Scoped subsystem directory only | No | No | PR only |

## 3. Prohibited Autonomous Actions
- Execution of untrusted external shell scripts without sandboxing.
- Direct hardware deployment or flashing.
- Wholesale rewriting or unilateral deletion of existing subsystems.
- Falsification or modification of generated test and boot log artifacts.
