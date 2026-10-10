# Upstream OS Study Framework & Guidelines

The research framework under `research/` provides structured documentation and architecture decision records (ADRs) for studying upstream OS mechanisms (from Linux, FreeBSD, OpenBSD, xv6, Arch, Debian, Fedora, Redox, etc.) before implementing connected component slices in SigmaOS.

## Directory Structure

- `research/upstream/`: In-depth study notes analyzing specific upstream OS components and source code.
- `research/decisions/`: Architecture Decision Records (ADRs) specifying adopted designs, deviations, and non-goals.
- `research/comparisons/`: Matrix and comparative analysis between upstream mechanisms and SigmaOS architecture.

## ADR Template Standard

Every Architecture Decision Record in `research/decisions/` should follow:

1. **Title & Status** (Proposed, Accepted, Rejected, Deprecated)
2. **Upstream Mechanism Studied** (Project, version/commit reference)
3. **Context & Problem Statement**
4. **Decision Drivers**
5. **Options Considered**
6. **Decision Outcome (What SigmaOS Adopts)**
7. **What SigmaOS Changes or Rejects**
8. **Consequences & Failure Assumptions**
9. **Required Validation & Evidence**
