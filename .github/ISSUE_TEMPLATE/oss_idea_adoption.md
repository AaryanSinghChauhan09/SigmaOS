name: Open-Source OS Idea Adoption
description: Propose adopting, prototyping, or adapting an innovation from open-source OS projects (Redox, seL4, Tock, Fuchsia, WASI, Linux, BSD).
title: "[OSS-ADOPTION]: "
labels: ["oss-adoption", "research", "proposal"]
body:
  - type: markdown
    attributes:
      value: |
        Track and adapt innovations from mature open-source OS projects to continuously advance SigmaOS.

  - type: dropdown
    id: source_project
    attributes:
      label: Source Project
      description: Select the open-source OS project inspiring this idea.
      options:
        - Redox OS
        - seL4 Microkernel
        - Tock OS
        - Fuchsia / Zircon
        - WASI / Wasmtime
        - Linux Kernel
        - FreeBSD / OpenBSD / NetBSD
        - Other Open-Source OS
    validations:
      required: true

  - type: textarea
    id: feature_description
    attributes:
      label: Feature / Innovation Description
      description: Describe the feature or design pattern to adapt into SigmaOS.
      placeholder: Describe the mechanism (e.g. Tock-style capsule driver isolation, seL4 capability revocation)...
    validations:
      required: true

  - type: textarea
    id: sigma_alignment
    attributes:
      label: SigmaOS Architecture Alignment
      description: How will this feature fit into SigmaOS's `#![no_std]`, CapabilityToken, and memory pool architecture?
    validations:
      required: true

  - type: textarea
    id: prototype_plan
    attributes:
      label: 2-Week Prototyping Plan
      description: Outline the timeboxed prototype plan, verification criteria, and microbenchmark metrics.
    validations:
      required: true
