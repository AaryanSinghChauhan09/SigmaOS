name: Bug Report / Regression
description: Report a bug, crash, or unexpected behavior in SigmaOS.
title: "[BUG]: "
labels: ["bug", "triage"]
body:
  - type: markdown
    attributes:
      value: |
        Thank you for reporting an issue in SigmaOS! Please fill out the form below to help us reproduce and fix the problem.

  - type: textarea
    id: description
    attributes:
      label: Bug Description
      description: Provide a clear and concise description of the bug.
      placeholder: Describe what happened...
    validations:
      required: true

  - type: textarea
    id: reproduction
    attributes:
      label: Steps to Reproduce
      description: Steps to reproduce the behavior in QEMU, bare-metal, or standalone unit test.
      placeholder: |
        1. Boot SigmaOS in QEMU via '...'
        2. Execute command '...'
        3. Observe kernel panic / unexpected behavior.
    validations:
      required: true

  - type: textarea
    id: expected
    attributes:
      label: Expected Behavior
      description: What did you expect to happen?
    validations:
      required: true

  - type: textarea
    id: logs
    attributes:
      label: Kernel Logs / Serial Output
      description: Paste relevant serial logs, QEMU console output, or stack traces here.
      render: shell

  - type: dropdown
    id: environment
    attributes:
      label: Environment
      description: Where did this bug occur?
      options:
        - QEMU x86_64
        - QEMU AArch64
        - QEMU RISC-V64
        - Bare-metal hardware
        - Standalone rustc unit test
    validations:
      required: true
