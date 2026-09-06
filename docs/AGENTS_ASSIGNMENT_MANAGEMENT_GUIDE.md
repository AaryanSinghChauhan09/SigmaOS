# SigmaOS AI Agent Task Assignment & Governance Guide

This guide defines task triage protocols, issue assignment routing rules, subagent delegation practices, and task completion criteria for AI coding agents developing SigmaOS.

---

## 1. Task Assignment Routing Matrix

Incoming development tasks and GitHub issues are assigned based on domain specialization:

| Category | Primary Agent Persona | Target Modules | Primary Labels |
|---|---|---|---|
| **Security & Vulnerabilities** | **🛡️ Sentinel** | `src/security/`, `src/auth/`, `.github/workflows/` | `security`, `bug`, `critical` |
| **User Experience & UI** | **🎨 Palette** | `src/desktop/`, `zenith_desktop/`, `src/customization/` | `ui`, `accessibility`, `theme` |
| **Performance & Optimization** | **⚡ Bolt** | `src/klib/`, `src/memory/`, `src/scheduler/`, `src/media/` | `performance`, `optimization` |
| **Core Distro Parity & Infrastructure** | **🛠️ Jules** | `src/compatibility/`, `src/distro/`, `src/sigpkg/`, `src/tools/` | `distro-parity`, `feature` |

---

## 2. Task Intake & Triage Protocol

1. **Scope Assessment:** Evaluate the requested change against the codebase. Identify affected Rust modules, C11 host tools, or GitHub Actions workflows.
2. **Plan Formulation:** Construct a numbered, Markdown-formatted plan detailing exact steps for implementation, verification, pre-commit checks, and submission.
3. **Plan Approval:** Request plan review using `request_plan_review` and set the approved plan via `set_plan`.
4. **Incremental Execution:** Modify source code incrementally, verifying changes after every step using `read_file`, `list_files`, or standalone test compilers.

---

## 3. Subagent Delegation Standards

When delegating tasks to subagents or agency components:

* **Clear Objective Specification:** Provide explicit, unambiguous prompt instructions including target source file paths and expected trait/struct signatures.
* **Bounded Scope:** Limit subagent tasks to a single subsystem or module to avoid overlapping merge conflicts.
* **Result Verification:** The parent agent MUST independently verify all code modifications made by subagents by compiling and executing unit tests.

---

## 4. Task Completion & Submission Criteria

A development task is considered complete ONLY when all the following criteria are met:

1. **Implementation Verification:** All requested traits, structs, functions, or fixes are implemented in source code (not artifact directories).
2. **100% Test Pass Rate:** Running `./run_sigma_tests.sh` succeeds with zero errors across all atomic, security, and subsystem inspection tests.
3. **Code Review Approval:** Code review is requested via `request_code_review` and receives an `#Approved#` rating.
4. **Memory Recording:** Learnings are recorded using `initiate_memory_recording`.
5. **Git Submission:** Final submission is committed via `submit` with a clear, descriptive title and commit message body.
