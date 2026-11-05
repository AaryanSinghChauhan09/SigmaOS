# Security Component Rules

## Component owner
- Owner: security-agent

## Upstream references
- Linux kernel, FreeBSD, OpenBSD, NetBSD, and sovereign OS specifications.

## Purpose
Governance, development standards, and gate verification rules for Security subsystem.

## Mandatory rules
- Strict zero-dependency or `#![no_std]` core adherence where specified.
- Passing unit and integration tests for every code change.
- Safe error handling without panicking in production runtime.
- Gate: Denial tests and fuzzing.

## Prohibited
- Direct commits to main without PR review.
- Memory unsafety and undefined behavior.
- Hardcoded secrets or credentials.

## Required evidence
- Unit and integration test logs.
- Gate verification evidence (Denial tests and fuzzing).
- Static analysis and compilation report.

## Promotion criteria
- Proposed: ADR and gap analysis
- Prototype: Unit tests
- Integrated: Runtime path plus integration tests
- Hardened: Fuzzing, negative tests, security review
- Supported: Reproducible boot/install/update evidence
