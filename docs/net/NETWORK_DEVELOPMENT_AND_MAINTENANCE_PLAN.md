# SigmaOS Network Development and Maintenance Plan

## Overview

SigmaOS is an experimental Rust operating system. Module-level passing unit tests do not prove a complete boot path, runtime integration, or supported hardware. Current network implementations (Ethernet, ARP, IPv4, UDP, DHCP, DNS, VirtIO networking) are evaluated based on rigorous status vocabulary—Proposed, Prototype, Integrated, Supported—without assuming production readiness or inferring capability from source files alone.

Treat "network management" as both the OS networking stack and the user-visible facilities to configure, inspect, troubleshoot, and maintain network interfaces. The initial target is narrow: one documented virtual NIC in QEMU, with deterministic tests, before expanding to physical hardware, Wi-Fi, VPNs, or broad Linux/BSD compatibility.

---

## Scope and Operating Rules

1. **One PR, One Coherent Change:** Split broad initiatives into small dependency-ordered PRs; keep implementation, tests, and documentation together.
2. **Status Follows Evidence:**
   - **Proposed:** Unimplemented work.
   - **Prototype:** Incomplete or model-only code.
   - **Integrated:** Path runs end to end in the OS.
   - **Supported:** Documented test matrix passes repeatedly.
3. **No Silent Compatibility Claims:** Describe Linux/BSD inspirations as design references, not as compatibility or feature parity. Document unsupported protocol options, devices, and interfaces.
4. **Safe Defaults:** Network services start disabled unless explicitly required; configuration changes are validated before application; destructive or remote-disconnecting changes require confirmation or a rollback plan.
5. **Protocol Behavior is Spec-Led:** Map each feature to protocol specs (RFC 1122 for host requirements across link, IP, and transport layers; RFC 8200 for IPv6; RFC 9293 for TCP) and test malformed input, boundary cases, timeouts, and resource exhaustion.
6. **Changes Enter Protected Branches Through PRs:** Require reviews, successful CI checks, conversation resolution, and no force pushes on the default branch.
7. **Security-Sensitive Reports:** Use the repository's private disclosure process. Do not post exploitable details publicly.

---

## Proposed PR Sequence

| PR | Proposed Title | Main Work | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(net): define network architecture, status, and support boundaries` | Document layers, data flow, ownership, supported test target, configuration model, status vocabulary, and current gaps. Link each networking feature to source, tests, and evidence. | Docs distinguish module tests from runtime integration; current limitations and target devices are explicit; no unsupported compatibility claims. |
| 2 | `test(net): add deterministic packet fixtures and parser fuzz targets` | Establish shared packet fixtures, property tests, fuzz targets, malformed/truncated packet cases, and regression corpus conventions. | Parsers reject or safely handle malformed lengths, invalid checksums, unknown values, and oversized input; fuzz targets run reproducibly in CI or documented local commands. |
| 3 | `feat(net): connect VirtIO NIC to the Ethernet receive and transmit path` | Wire the existing partial VirtIO network driver into the Ethernet path; specify queue lifecycle, buffer ownership, error reporting, and reset behavior. | QEMU can enumerate the NIC, transmit and receive test frames, recover from queue/device errors, and report test evidence; hardware support remains unclaimed. |
| 4 | `feat(net): add interface and link-state model` | Introduce stable interface identifiers, link up/down state, MTU, MAC, counters, and typed configuration events. | State changes are observable; counters have defined reset and wrap behavior; interface removal does not leave stale routes or resolver state. |
| 5 | `feat(net): add IPv4 routing and ICMP host support` | Add a route table, longest-prefix selection, default route, source-address selection, and essential ICMP handling. | Unit and integration tests cover local/remote delivery, no route, multiple routes, TTL expiry, unreachable errors, and configurable forwarding disabled by default. |
| 6 | `feat(net): add IPv6 host networking foundation` | Implement IPv6 parsing and validation, ICMPv6 essentials, Neighbor Discovery, link-local addressing, and Router Advertisement processing in staged sub-PRs if needed. | Tests cover extension-header and packet-boundary handling, neighbor state transitions, address lifecycle, and malformed or hostile advertisements; IPv6 status remains Prototype until end-to-end tests pass. |
| 7 | `feat(net): add TCP transport with socket lifecycle` | Implement TCP socket states, handshake/close, sequence and acknowledgment validation, retransmission and timeout behavior, flow control, and reset handling. | State-machine tests and packet-level integration tests cover normal and adversarial cases; behavior is mapped to current TCP requirements, including RFC 9293. |
| 8 | `feat(net): harden DHCP and DNS client behavior` | Review current DHCP/DNS modules; add lease renewal, expiry, server changes, retries, DNS response validation, negative caching policy, and bounded cache/resource limits. | Tests cover loss, duplicate and malformed replies, lease expiry, resolver changes, cache expiry, and resource limits; DNS behavior is documented. |
| 9 | `feat(net): add persistent network configuration and rollback` | Define a versioned declarative configuration format for static addresses, DHCP, DNS, routes, and interface matching. Apply configuration transactionally with validation and rollback. | Invalid configuration leaves the previous working state intact; remote-risking changes have a recovery path; secrets are not written to logs. |
| 10 | `feat(net): add network control CLI and diagnostics` | Add read-only status, interface, route, DNS, and connectivity diagnostics before adding mutating commands. Add explicit confirmation for changes that can sever connectivity. | CLI output is stable enough to script or offers a versioned JSON form; permissions and error messages are documented; diagnostics are bounded and avoid exposing secrets. |
| 11 | `feat(net): add firewall policy framework with default-deny service rules` | Design a policy API and packet-filter hooks; begin with explicit inbound/outbound rules, connection tracking only if justified, and audit counters. | Firewall is off or conservatively configured according to a documented policy; no rule means no implicit service exposure; tests cover ordering, state cleanup, and IPv4/IPv6 parity. |
| 12 | `ci(net): add QEMU networking integration and regression workflow` | Automate boot-to-network tests using a virtual NIC and isolated test network; archive logs and test manifests. | CI proves boot, interface discovery, address setup, DNS, and controlled connectivity; failure artifacts identify the stage; no test silently depends on an external public network. |
| 13 | `docs(net): publish supported networking matrix and recovery guide` | Publish tested virtual/hardware devices, protocol coverage, known gaps, configuration examples, and recovery steps. | Every "Supported" claim links to repeatable evidence and a tested configuration; Wi-Fi/VPN/physical NIC gaps are stated plainly. |

---

## Periodic Maintenance Cadence

Recurring tasks are executed via issue/PR templates or scheduled PRs:

- **Every PR:** Run formatting, check, unit tests, targeted integration tests, and security/static checks applicable to changed code.
- **Weekly:**
  - `chore(net): triage networking failures and stale work` — review failures, open bugs, fuzz crashes, regressions, and support-matrix changes.
  - `test(net): refresh packet regression corpus` — add minimized inputs from fuzzing and bug reports.
- **Monthly:**
  - `security(net): review parser and resource-exhaustion risks` — review packet parsing, queue bounds, timers, allocation behavior, privilege boundaries, and logging.
  - `docs(net): reconcile implementation status and support evidence` — compare source, tests, runtime results, and documentation.
  - `ci(net): verify QEMU boot and network smoke path` — run clean build and isolated end-to-end network tests on virtual target.
- **Quarterly:**
  - `security(net): review threat model and firewall defaults` — revisit exposed services, privileged configuration, remote lockout, IPv6 behavior, and security disclosure handling.
  - `docs(net): review protocol standards and interoperability gaps` — check relevant RFCs and document implemented/deferred requirements.
- **Each Release Candidate:** `release(net): certify networking claims for candidate` — run release matrix and verify compatibility statements.
- **Post-Incident:** `fix(net): prevent recurrence of <incident>` — add regression coverage, correct root cause, and update diagnostics/documentation.

---

## Cross-Component Integration Rules

- **Drivers and DMA:** Require bounded queues, validated descriptor lengths, clear ownership/reset semantics, and isolated device tests before claiming device support.
- **Syscalls and Sockets:** Define versioned socket API, error semantics, permission checks, resource limits, and compatibility boundaries.
- **Init and Services:** Network-dependent services express explicit dependencies and timeouts; "network online" must not block boot indefinitely.
- **Configuration and Updates:** Version configuration schemas, validate before apply, preserve last-known-good configuration, and provide recovery paths.
- **Security and Auditing:** Minimize privileges, redact credentials from logs, rate-limit events, bound queues/logs, and make listening/forwarding opt-in.
- **Package and Distro Compatibility:** Create compatibility layers only after establishing a stable, native network API.
- **Documentation and Inventory:** Update component docs and feature inventory (`FEATURE_STATUS.toml`) in the same PR as implementation changes.

---

## PR Review and Merge Gates

Standard PR requirements include:
- Summary of changes and problem solved.
- Affected subsystems/files and explicit scope boundaries.
- Status claim (Proposed, Prototype, Integrated, Supported) with evidence.
- Design, compatibility, and RFC specification mapping.
- Security and failure behavior analysis.
- Detailed test execution log and commands.
- Documentation and inventory updates.

Review policy: At least one maintainer approval for standard changes; two relevant reviewers for protocol parsing, drivers, privileged networking, or firewall policy changes. CI pass and conversation resolution required.

---

## Execution Order and Exit Criteria

1. Begin with PRs 1–3 and CI baseline.
2. Advance sequentially: Interface State -> Routing -> IPv6 -> TCP -> Hardened DHCP/DNS -> Configuration Rollback -> CLI Diagnostics -> Firewall -> CI Integration -> Supported Matrix.
3. **Integrated Threshold:** Achieved when a documented boot path exercises driver-to-stack-to-userspace flow in QEMU.
4. **Supported Threshold:** Achieved when the test matrix is repeatedly reproducible, tested configuration is published, limitations are stated, and a recovery path is demonstrated.
