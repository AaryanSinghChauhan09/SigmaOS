# Component Rules: memory.md

## Component owner
memory-agent

## Upstream references
jemalloc, mimalloc, FreeBSD VM

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
