name: "⚙️ Compatibility & Portability RFC"
description: "Propose a new userspace compatibility layer, dynamic translator (e.g. Wine/Proton/Android), or standard POSIX/libc shim."
title: "[RFC]: <Short title describing target compatibility layer>"
labels: ["rfc", "compatibility-group", "needs-review"]
body:
  - type: markdown
    attributes:
      value: |
        Thank you for proposing an interoperability enhancement for SigmaOS! Our goal is to outclass and absorb the best of Windows, macOS, Linux, and Android under a strict, capability-gated microkernel model. Please use this template to specify your design.
  - type: textarea
    id: executive-summary
    attributes:
      label: "📌 Executive Summary"
      description: "A brief 2-3 sentence overview of what platform, runtime, or ABI is being emulated or integrated."
      placeholder: "e.g., Implementing the standard Linux epoll/eventfd syscall subsystem for high-performance network services."
    validations:
      required: true
  - type: textarea
    id: problem-statement
    attributes:
      label: "🛑 The Gap Analysis (Problem Statement)"
      description: "What pre-existing legacy OS capability is currently missing or restricted inside the microkernel?"
      placeholder: "e.g., Existing POSIX shims lack socket-activated non-blocking selectors, preventing Node.js and Go from running natively."
    validations:
      required: true
  - type: dropdown
    id: implementation-strategy
    attributes:
      label: "🛠️ Integration Strategy"
      description: "How should this compatibility be achieved?"
      options:
        - "Syscall Translation / Shims (e.g., Wine-like, low overhead)"
        - "Sandboxed User-Mode Containers (e.g., OCI / S-Compartment)"
        - "Virtualization / Light VM (e.g., SovereignVMM, hypervisor)"
        - "WASM / WASI Portable Runtime"
        - "Native Reimplementation in Rust"
    validations:
      required: true
  - type: textarea
    id: technical-specification
    attributes:
      label: "📝 Technical Design Specification"
      description: "Detail the structs, traits, or C++ registries being added, and explain how the microkernel CapabilityToken rules are preserved."
      placeholder: "Specify the exact file paths, thread scheduling model, and memory mapping logic."
    validations:
      required: true
  - type: textarea
    id: testing-validation
    attributes:
      label: "🧪 Test Plan & Acceptance Criteria"
      description: "How will this change be verified? What LTP (Linux Test Project) or native unit tests apply?"
      placeholder: "e.g., Run `rustc --test src/compatibility/epoll.rs` and assert correct callback wakeups."
    validations:
      required: true
