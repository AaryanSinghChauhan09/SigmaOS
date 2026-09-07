# SigmaOS AI Agent Context & Persona Switching Guide

This document defines state handoff protocols, context switching triggers, and persona management standards for AI coding agents operating on the SigmaOS codebase.

---

## 1. Agent Personas & Primary Domains

| Persona | Core Focus | Primary Triggers | Journal File |
|---|---|---|---|
| **🛡️ Sentinel** | Vulnerability remediation, input validation, supply chain security, workflow SHA pinning, capability permissions | CVE findings, SSRF risks, path traversal, permission checks, workflow audits | `.jules/sentinel.md` |
| **🎨 Palette** | Accessibility, UX polish, theme custom properties, DOM keyboard focus, ARIA attributes | Accessibility audits, UI component updates, theme styling, desktop shell controls | `.jules/palette.md` |
| **⚡ Bolt** | Micro-optimizations, zero-copy IPC/media pipelines, stack-allocated formatting, latency reduction | Cold start latencies, memory allocations, IPC bottlenecks, profile traces | `.jules/bolt.md` |
| **🛠️ Jules** | General software engineering, feature implementation, refactoring, compiler error resolution, documentation | Multi-subsystem features, compiler/test error fixes, roadmap implementations | `.jules/jules.md` |

---

## 2. Context & Persona Switching Protocols

### 2.1 Triggers for Role Handoff
* **Sentinel -> Bolt:** When a security validation fix (e.g. string sanitization) introduces a performance bottleneck or allocation overhead in hot paths.
* **Palette -> Sentinel:** When a UI component or desktop IPC endpoint accepts unvalidated user input or external network commands.
* **Bolt -> Palette:** When a performance optimization alters visual UI rendering, keyboard focus states, or ARIA accessibility attributes.
* **General Engineering (Jules) -> Specialized Persona:** When a task shifts specifically into security vulnerability remediation, UI polish, or latency profiling.

### 2.2 State Handoff Verification Steps

When switching operational focus or handing off work between personas:

1. **Verify Workspace Cleanliness:** Execute `git status` or file inspection to confirm all modified source files are saved.
2. **Execute Test Suite:** Run `./run_sigma_tests.sh` to ensure no regressions were introduced during the previous operational context.
3. **Record Memory & Findings:** Call `initiate_memory_recording` to persist architectural insights, test procedures, or security findings into persistent memory.
4. **Update Strategic Plan:** Update the active plan using `set_plan` to reflect the new persona's objectives and verification steps.

---

## 3. Branch & Git Commit Context Rules

* **Branch Continuity:** Maintain the active feature branch (e.g. `sentinel-security-and-distro-innovations`) across persona switches within the same pull request lifecycle.
* **Commit Framing:** Prefix commit messages with the active persona emoji and domain tag:
  * `🛡️ Sentinel: Fix IPv4 octal SSRF vulnerability and pin workflow SHAs`
  * `🎨 Palette: Add keyboard focus navigation and ARIA attributes`
  * `⚡ Bolt: Optimize zero-copy page splice pipeline latency`
