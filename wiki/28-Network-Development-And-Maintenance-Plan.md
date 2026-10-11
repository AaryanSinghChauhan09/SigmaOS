# SigmaOS Network Development and Maintenance Plan

This document outlines the official architectural roadmap, operating rules, proposed Pull Request (PR) sequence, maintenance cadence, cross-component rules, merge gates, and execution criteria for the SigmaOS networking stack and userland network management capabilities.

## Overview and Status Boundaries

SigmaOS is an experimental Rust operating system. Module compilation or passing unit tests in isolated modules does not constitute proof of a complete boot path, runtime integration, or hardware support.

To maintain transparency and rigor, all networking capabilities must adhere to the standard SigmaOS status vocabulary:
- **Proposed:** Specification or plan drafted, but implementation has not started.
- **Prototype:** Model, stub, or module-level implementation without full driver/OS runtime path integration.
- **Integrated:** Code is wired into the OS boot/runtime path and verified end-to-end in QEMU.
- **Supported:** Validated through repeatable, documented test matrices with known limitations and rollback paths documented.

The initial scope is intentionally kept narrow: one documented virtual NIC (VirtIO-net) in QEMU with deterministic test matrices, before expanding to physical hardware, Wi-Fi, VPNs, or broad Linux/BSD compatibility claims.

---

## Scope and Operating Rules

Treat "network management" as both the OS networking stack and the user-visible facilities to configure, inspect, troubleshoot, and maintain network interfaces.

Adopt the following rules across all networking PRs:

1. **One PR, one coherent change:** Split broad initiatives into small, dependency-ordered PRs. Keep implementation, tests, and documentation together in each PR.
2. **Status follows evidence:** Use Proposed for unimplemented work, Prototype for incomplete or model-only code, Integrated after the path runs end-to-end in the OS, and Supported only after the documented test matrix passes repeatedly.
3. **No silent compatibility claims:** Describe Linux/BSD inspirations as design references, not as drop-in compatibility or feature parity. Document unsupported protocol options, devices, and interfaces explicitly.
4. **Safe defaults:** Network services start disabled unless explicitly required; configuration changes are validated before application; destructive or remote-disconnecting changes require confirmation or an automated rollback plan.
5. **Protocol behavior is spec-led:** Define a protocol-to-RFC map for each feature. Test malformed input, boundary cases, timeouts, and resource exhaustion. Rely on current protocol standards (e.g., RFC 8200 for IPv6, RFC 9293 for TCP) while referencing RFC 1122 for foundational host requirements.
6. **Protected branch controls:** All changes enter protected branches through PRs requiring code reviews, passing CI checks, resolved conversations, and no force pushes on the default branch.
7. **Private vulnerability disclosure:** Security-sensitive reports use the repository's private disclosure process. Do not post exploitable details publicly before maintainers coordinate a fix.

---

## Proposed PR Sequence

| PR | Proposed Title | Main Work | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(net): define network architecture, status, and support boundaries` | Document layers, data flow, ownership, supported test target, configuration model, status vocabulary, and current gaps. Link each networking feature to source, tests, and evidence. | Docs distinguish module tests from runtime integration; current limitations and target devices are explicit; no unsupported compatibility claims. |
| 2 | `test(net): add deterministic packet fixtures and parser fuzz targets` | Establish shared packet fixtures, property tests, fuzz targets, malformed/truncated packet cases, and regression corpus conventions. | Parsers reject or safely handle malformed lengths, invalid checksums, unknown values, and oversized input; fuzz targets run reproducibly in CI or documented local commands. |
| 3 | `feat(net): connect VirtIO NIC to the Ethernet receive and transmit path` | Wire the existing partial VirtIO network driver into the Ethernet path; specify queue lifecycle, buffer ownership, error reporting, and reset behavior. | QEMU can enumerate the NIC, transmit and receive test frames, recover from queue/device errors, and report test evidence; hardware support remains unclaimed. |
| 4 | `feat(net): add interface and link-state model` | Introduce stable interface identifiers, link up/down state, MTU, MAC, counters, and typed configuration events. | State changes are observable; counters have defined reset and wrap behavior; interface removal does not leave stale routes or resolver state. |
| 5 | `feat(net): add IPv4 routing and ICMP host support` | Add a route table, longest-prefix selection, default route, source-address selection, and essential ICMP handling. | Unit and integration tests cover local/remote delivery, no route, multiple routes, TTL expiry, unreachable errors, and configurable forwarding disabled by default. |
| 6 | `feat(net): add IPv6 host networking foundation` | Implement IPv6 parsing and validation, ICMPv6 essentials, Neighbor Discovery, link-local addressing, and Router Advertisement processing in staged sub-PRs. | Tests cover extension-header and packet-boundary handling, neighbor state transitions, address lifecycle, and malformed advertisements; status remains Prototype until end-to-end tests pass. |
| 7 | `feat(net): add TCP transport with socket lifecycle` | Implement TCP socket states, handshake/close, sequence/acknowledgment validation, retransmission and timeout behavior, flow control, and reset handling. Mapped to RFC 9293. | State-machine tests and packet-level integration tests cover normal and adversarial cases; behavior is mapped to current TCP requirements. |
| 8 | `feat(net): harden DHCP and DNS client behavior` | Review DHCP/DNS modules; add lease renewal, expiry, server changes, retries, DNS response validation, negative caching policy, and bounded cache/resource limits. | Tests cover loss, duplicate and malformed replies, lease expiry, resolver changes, cache expiry, and resource limits; DNS behavior is documented. |
| 9 | `feat(net): add persistent network configuration and rollback` | Define a versioned declarative configuration format for static addresses, DHCP, DNS, routes, and interface matching. Apply configuration transactionally with validation and rollback. | Invalid configuration leaves the previous working state intact; remote-risking changes have a recovery path; secrets are not written to logs. |
| 10 | `feat(net): add network control CLI and diagnostics` | Add read-only status, interface, route, DNS, and connectivity diagnostics before adding mutating commands. Add explicit confirmation for changes that can sever connectivity. | CLI output is stable enough to script or offers a versioned JSON form; permissions and error messages are documented; diagnostics are bounded. |
| 11 | `feat(net): add firewall policy framework with default-deny service rules` | Design a policy API and packet-filter hooks; begin with explicit inbound/outbound rules, connection tracking only if justified, and audit counters. | Firewall is off or conservatively configured according to a documented policy; no rule means no implicit service exposure; tests cover ordering and cleanup. |
| 12 | `ci(net): add QEMU networking integration and regression workflow` | Automate boot-to-network tests using a virtual NIC and isolated test network; archive logs and test manifests. | CI proves boot, interface discovery, address setup, DNS, and controlled connectivity; failure artifacts identify the stage; no external network dependency. |
| 13 | `docs(net): publish supported networking matrix and recovery guide` | Publish tested virtual/hardware devices, protocol coverage, known gaps, configuration examples, and recovery steps. | Every "Supported" claim links to repeatable evidence and a tested configuration; Wi-Fi/VPN/physical NIC gaps are stated plainly. |

---

## Periodic Maintenance Cadence

All recurring maintenance tasks must follow scheduled PR workflows or issues with review on protected branches:

| Cadence | Task / PR Title | Required Evidence |
|---|---|---|
| Every PR | CI Check Suite | Formatting, compilation, unit tests, targeted integration tests, static analysis. |
| Weekly | `chore(net): triage networking failures and stale work` | Triage notes linking failure to issue, regression test, owner, or explicit decision. |
| Weekly | `test(net): refresh packet regression corpus` | Reproducible corpus tests added from fuzzing/bugs; zero unvetted sensitive pcap files. |
| Monthly | `security(net): review parser and resource-exhaustion risks` | Severity-classified findings, tracked issues, and regression tests for resolved defects. |
| Monthly | `docs(net): reconcile implementation status and support evidence` | Status entries link to current tests; explicit distinction between unit, QEMU, and hardware. |
| Monthly | `ci(net): verify QEMU boot and network smoke path` | Retained boot logs, test results, environment manifest, and failure artifacts. |
| Quarterly | `security(net): review threat model and firewall defaults` | Updated threat model or documented decision; owner-approved mitigation issues. |
| Quarterly | `docs(net): review protocol standards and interoperability gaps` | Versioned protocol compliance checklist (RFCs); unit tests for adopted requirements. |
| Release Candidate | `release(net): certify networking claims for candidate` | Passing release matrix; exact target/config; documented rollback path. |
| Post-Incident | `fix(net): prevent recurrence of <incident>` | Reproducer fails before fix, passes after; impact/mitigation documented safely. |

---

## Cross-Component Rules

- **Drivers and DMA:** Require bounded queues, validated descriptor lengths, clear ownership and reset semantics, and isolated device tests before claiming support.
- **Syscalls and Sockets:** Define a versioned socket API, error semantics, permission checks, resource limits, and compatibility boundaries.
- **Init and Services:** Network-dependent services must express dependencies and timeouts; "network online" must have a defined, non-blocking meaning.
- **Configuration and Updates:** Version configuration schemas; validate before applying; preserve last-known-good state; ensure network updates are recoverable.
- **Security and Auditing:** Minimize privileges, redact credentials/tokens from logs, rate-limit logs, bound queues, and default to opt-in services.
- **Package and Distro Compatibility:** Build compatibility layers only over stable native APIs. Start with targeted command/config compatibility.
- **Documentation and Inventory:** Update component documentation and feature inventory in the same PR as implementation changes.

---

## PR Review and Merge Gates

All networking PRs must include the standard PR template:

```text
## Summary
What changes, and what user-visible or developer-visible problem does it solve?

## Scope
Subsystems/files affected. State what this PR does not attempt to implement.

## Status Claim
Proposed | Prototype | Integrated | Supported
Explain the evidence for the selected status.

## Design and Compatibility
API/configuration changes, protocol/RFC references, Linux/BSD compatibility impact, migration or rollback behavior.

## Security and Failure Behavior
Trust boundaries, malformed input, resource limits, privileges, logging/redaction, failure recovery, and disclosure considerations.

## Tests
Commands run; unit/property/fuzz/integration/QEMU/hardware results; skipped tests and why.

## Documentation and Inventory
Docs/status files updated, or a reason they do not apply.

## Checklist
- [ ] One focused, reviewable change
- [ ] Tests added or updated for behavior changed
- [ ] No unsupported compatibility or hardware claim
- [ ] Failure and rollback behavior documented where relevant
- [ ] Required CI checks pass
- [ ] Review conversations resolved
```

### Review Thresholds
- **Standard changes:** Minimum 1 maintainer approval.
- **High-risk changes (parsers, drivers, firewall, socket API, security):** Minimum 2 maintainer/expert reviews required.
- **Branch protection:** Enforced CI status checks, resolved conversations, no force-pushes, and stale review dismissal.

---

## Execution Order and Exit Criteria

1. Begin with PRs 1–3 and the CI baseline.
2. Proceed through interface model (PR 4), routing (PR 5), IPv6 foundation (PR 6), and TCP (PR 7).
3. Hardening and userland tools (PRs 8–11) follow transport stability.
4. Finalize with QEMU network smoke CI (PR 12) and published support matrix (PR 13).

**Exit Criteria:**
- **Integrated Status:** Requires a documented boot path exercising driver-to-stack-to-userspace flow in QEMU.
- **Supported Status:** Requires repeatable test matrix runs, published target configurations, documented limitations, and verified rollback paths.
