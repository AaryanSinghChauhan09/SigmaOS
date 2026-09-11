# AI Agent Directive: Landlock LSM Sandboxing Subsystem Management

## Overview

The `LandlockEngine` (`src/security/landlock.rs`, re-exported in `src/security/mod.rs`) implements a zero-dependency, `#![no_std]` native Rust implementation of Linux Landlock LSM (Unprivileged Application Sandboxing) for SigmaOS.

## Key Architectural Structures

1. **`LandlockRuleset`**:
   - Manages ruleset creation with bitmasks for handled filesystem access rights (`LANDLOCK_ACCESS_FS_*`).
   - Tracks path beneath rules (`LandlockPathBeneathAttr`).
   - Enforces monotonic access restriction upon calling `restrict_self()`.

2. **`LandlockEngine`**:
   - Assigns unique `ruleset_id` handles.
   - Rejects adding new rules once a ruleset has been enforced (`restrict_self`).
   - Validates file access permissions (`validate_file_access`).

## Directives for AI Agents

- **Zero-Dependency Rule**: Maintain pure Rust implementation without invoking external Linux kernel C headers or syscall ABIs directly.
- **Monotonic Enforcement**: Ensure that once `restrict_self` is invoked, ruleset permissions cannot be expanded or mutated.
- **Verification**: Run standalone unit tests using:
  ```bash
  rustc --test --edition 2021 src/security/landlock.rs -o build/landlock_test && ./build/landlock_test
  ```
