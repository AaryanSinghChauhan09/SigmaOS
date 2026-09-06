name: "Resource Control: Per-Tab Resource Quotas (cgroups v2 / rctl / Jails)"
description: "Implement OS-level resource limits for CPU, memory, and IO per tab and renderer process using cgroups v2 and rctl."
title: "[PERF] Implement cgroups v2 & FreeBSD rctl Resource Quotas"
labels: ["resource-management", "cgroups", "freebsd-rctl", "performance"]
assignees: []
body:
  - type: markdown
    attributes:
      value: |
        ## Overview
        Prevent runaway background tabs or helper processes from causing Out-Of-Memory (OOM) events or CPU starvation by enforcing strict resource quotas:
        - **Linux**: cgroups v2 controller (cpu, memory, io) per renderer
        - **FreeBSD**: `rctl(8)` resource limits & Jails allocation
        - **UI Integration**: Expose resource monitoring & allocation sliders in Zenith Desktop Settings

  - type: textarea
    id: implementation-tasks
    attributes:
      label: Implementation Tasks
      placeholder: |
        - [ ] Add Linux cgroup v2 controller manager in `src/virtualization/cgroups.rs`
        - [ ] Add FreeBSD `rctl` rule generator and jail limit binder
        - [ ] Connect tab lifecycle manager to resource control groups
        - [ ] Expose active tab memory/CPU resource usage in task manager
        - [ ] Benchmark system responsiveness under heavy memory pressure

  - type: textarea
    id: success-metrics
    attributes:
      label: Success Metrics & Acceptance Criteria
      value: |
        - 0 OOM kernel panics during 100-tab stress test.
        - UI frame rate remains >60 FPS during background task throttling.
