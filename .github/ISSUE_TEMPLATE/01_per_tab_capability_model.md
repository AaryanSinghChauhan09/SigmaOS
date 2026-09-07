name: "Security: Per-Tab Capability Model (seccomp / Capsicum / Pledge)"
description: "Implement fine-grained per-process and per-tab capability sandboxing inspired by OpenBSD pledge/unveil, FreeBSD Capsicum, and Linux seccomp."
title: "[SEC] Implement Per-Tab Capability Model Sandboxing"
labels: ["security", "sandboxing", "linux-bsd-paradigm", "enhancement"]
assignees: []
body:
  - type: markdown
    attributes:
      value: |
        ## Overview
        Enforce least privilege for SigmaOS processes and tab renderers by requiring declarative capability manifests (stdio, rpath, wpath, inet, exec) enforced at launch using platform-native security primitives:
        - **Linux**: seccomp-BPF filters & Landlock LSM
        - **FreeBSD**: Capsicum capability mode & rights
        - **OpenBSD**: `pledge(2)` and `unveil(2)` syscall wrappers
        - **macOS**: App Sandbox entitlements

  - type: textarea
    id: implementation-tasks
    attributes:
      label: Implementation Tasks
      description: Task list for completing this feature
      placeholder: |
        - [ ] Define capability manifest schema in Rust
        - [ ] Add Linux seccomp filter generator
        - [ ] Add FreeBSD Capsicum rights wrapper
        - [ ] Add OpenBSD pledge/unveil launcher bindings
        - [ ] Integrate capability enforcement into process launch pipeline
        - [ ] Add unit and integration tests

  - type: textarea
    id: success-metrics
    attributes:
      label: Success Metrics & Acceptance Criteria
      description: How will we measure success?
      value: |
        - 100% of untrusted helper and renderer processes launched with restricted capability sets.
        - Zero privilege escalation vulnerabilities across sandbox boundaries.
