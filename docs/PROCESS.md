# SigmaOS Evidence-Based Open-Source Process

## Overview
SigmaOS enforces an evidence-driven open-source study and development cycle. Feature claims must be backed by verifiable runtime, QEMU boot, installer, hardware, or integration test evidence rather than mere unit test presence or design intention.

## Recurring Study Cadence

| Cycle | Duration | Focus | Required Output |
| :--- | :--- | :--- | :--- |
| **Weekly Study** | 1–2 hours | One narrow mechanism from Linux, FreeBSD, OpenBSD, Redox, xv6, or Arch | `docs/studies/YYYY-WW-<topic>.md` |
| **Biweekly Gap Review** | 2–3 hours | Compare one SigmaOS component against reference designs | Gap matrix + ranked PR proposals |
| **Monthly Integration Review** | Half day | Execute QEMU boot, installer dry-run, recovery, and regression suites | Status update + blockers ledger |
| **Quarterly Architecture Review** | 1–2 days | Reassess roadmap, security posture, ABI stability, release criteria | Revised roadmap + ADR updates |

## Study Note Guidelines
Every study must explicitly evaluate open-source reference implementations (Linux, FreeBSD/OpenBSD, Redox/xv6, etc.) and categorize adaptation decisions as **Adopt**, **Adapt**, or **Reject** with technical justification.

## Governing Principle
A component is not considered "implemented" or "complete" until its runtime path is exercised, its failure modes are tested, its operational boundaries are documented, and its status is updated with direct test evidence.
