# ADR 0001: Evidence-Based Component Status Claims

## Context
In OS development, "code present" and "unit tests pass" do not guarantee kernel integration, hardware compatibility, bootability, or failure resilience. Previous status markers risked misleading users and developers regarding true system capabilities.

## Decision
SigmaOS adopts an evidence-based status policy:
1. No component, driver, or subsystem may be marked as `implemented` or `complete` without reproducible test logs, QEMU boot evidence, or hardware diagnostics evidence.
2. Unverified capabilities must be explicitly marked as `experimental`, `partial`, or `untested`.
3. All feature matrices (`FEATURE_STATUS.toml`, `CAPABILITY_MATRIX.toml`) must reference specific test execution outputs or issue IDs.

## Consequences
- Prevents premature claims of daily-driver readiness or hardware support.
- Requires all PRs altering component status to include corresponding test or runtime logs.
- Establishes a transparent, reproducible software quality standard.
