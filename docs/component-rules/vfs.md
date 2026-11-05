# Component Rules: vfs.md

## Component owner
filesystem-agent

## Upstream references
OpenZFS, HAMMER2, Ext4

## Purpose
Governance, isolation, and architectural requirements for this component domain.

## Mandatory rules
All PRs touching this component must pass targeted integration tests and adhere to safety standards.

## Prohibited
Direct commits without PR review, skipping architectural checks, or unvalidated dependencies.

## Required evidence
Test execution logs, memory safety audit, and benchmark reports where appropriate.

## Promotion criteria
Documented transition from Proposed to Prototype, Integrated, Hardened, and Supported.
