name: Feature Request
description: Propose a new feature, API enhancement, or subsystem extension.
title: "[FEATURE]: "
labels: ["enhancement", "proposal"]
body:
  - type: markdown
    attributes:
      value: |
        Propose a new capability or enhancement for SigmaOS!

  - type: textarea
    id: summary
    attributes:
      label: Feature Summary
      description: What feature or enhancement are you proposing?
      placeholder: Describe the proposed feature...
    validations:
      required: true

  - type: textarea
    id: motivation
    attributes:
      label: Motivation & Problem Statement
      description: Why is this feature needed? What problem does it solve?
    validations:
      required: true

  - type: textarea
    id: design
    attributes:
      label: Proposed Implementation & Architecture
      description: Outline the technical design, including capability checks, `#![no_std]` compliance, and API signatures.
    validations:
      required: true

  - type: textarea
    id: verification
    attributes:
      label: Verification & Testing Plan
      description: How will this feature be verified and tested (unit tests, QEMU smoke test, fuzzing)?
    validations:
      required: true
