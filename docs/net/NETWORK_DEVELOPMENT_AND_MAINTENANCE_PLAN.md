# SigmaOS Network Development and Maintenance Plan

## Overview and Status Context

SigmaOS is an experimental Rust operating system. A module compiling or passing unit tests does not prove a complete boot path, runtime integration, or supported hardware. The networking stack and userland utilities must be verified systematically against documented test matrices and runtime evidence.

SigmaOS adopts a strict 4-level status vocabulary across all networking features:
- **Proposed**: Unimplemented work described in specifications and plans.
- **Prototype**: Incomplete, unintegrated, or model-only implementation backed by unit tests.
- **Integrated**: The feature runs end-to-end within the OS boot path (e.g., driver-to-stack-to-userspace flow in QEMU).
- **Supported**: Repeatedly passes the documented test matrix with verified hardware/virtual target evidence, clear limitations, and a recovery path.

---

## Scope and Operating Rules

The network management domain encompasses both the OS kernel networking stack and user-visible facilities to configure, inspect, troubleshoot, and maintain network interfaces. The initial target is kept intentionally narrow: **one documented virtual NIC (VirtIO-Net) in QEMU with deterministic tests**, before expanding to physical hardware, Wi-Fi, VPNs, or broad Linux/BSD compatibility layers.

All networking development must strictly adhere to the following 7 operating rules:

1. **One PR, One Coherent Change**: Broad initiatives are split into small, dependency-ordered pull requests. Implementation, unit tests, integration tests, and documentation updates must be committed together.
2. **Status Follows Evidence**: Features are marked Proposed, Prototype, Integrated, or Supported strictly based on demonstrable test evidence, not source file existence or compilation success.
3. **No Silent Compatibility Claims**: Linux/BSD inspirations serve as design references only. Unsupported protocol options, devices, or socket behaviors must be documented explicitly.
4. **Safe Defaults**: Network services remain disabled by default. Network configuration changes are validated prior to application, and destructive or remote-disconnecting operations require explicit confirmation and automatic rollback.
5. **Protocol Behavior is Spec-Led**: Implementations map to current protocol specifications (RFC 1122 for host requirements across link/IP/transport layers, RFC 8200 for IPv6, RFC 9293 for TCP). Testing must cover malformed input, boundary conditions, timeouts, and resource exhaustion.
6. **Protected Branch Gateways**: Changes enter default branches only through pull requests that pass CI checks, maintainer code review, conversation resolution, and force-push protections.
7. **Private Security Disclosure**: Security-sensitive vulnerabilities follow private disclosure workflows rather than public issues or PRs until mitigations are coordinated.

---

## Proposed PR Sequence

The 13-PR roadmap structures the complete networking lifecycle:

| PR | Title | Main Work | Acceptance Criteria | Status |
|---|---|---|---|---|
| 1 | `docs(net): define network architecture, status, and support boundaries` | Document layers, data flow, ownership, supported test target, configuration model, status vocabulary, and current gaps. Link each networking feature to source, tests, and evidence. | Docs distinguish module tests from runtime integration; current limitations and target devices are explicit; no unsupported compatibility claims. | Supported |
| 2 | `test(net): add deterministic packet fixtures and parser fuzz targets` | Establish shared packet fixtures, property tests, fuzz targets, malformed/truncated packet cases, and regression corpus conventions. | Parsers reject or safely handle malformed lengths, invalid checksums, unknown values, and oversized input; fuzz targets run reproducibly in CI or documented local commands. | Prototype |
| 3 | `feat(net): connect VirtIO NIC to the Ethernet receive and transmit path` | Wire the existing partial VirtIO network driver into the Ethernet path; specify queue lifecycle, buffer ownership, error reporting, and reset behavior. | QEMU can enumerate the NIC, transmit and receive test frames, recover from queue/device errors, and report test evidence; hardware support remains unclaimed. | Prototype |
| 4 | `feat(net): add interface and link-state model` | Introduce stable interface identifiers, link up/down state, MTU, MAC, counters, and typed configuration events. | State changes are observable; counters have defined reset and wrap behavior; interface removal does not leave stale routes or resolver state. | Prototype |
| 5 | `feat(net): add IPv4 routing and ICMP host support` | Add a route table, longest-prefix selection, default route, source-address selection, and essential ICMP handling. | Unit and integration tests cover local/remote delivery, no route, multiple routes, TTL expiry, unreachable errors, and configurable forwarding disabled by default. | Prototype |
| 6 | `feat(net): add IPv6 host networking foundation` | Implement IPv6 parsing and validation, ICMPv6 essentials, Neighbor Discovery, link-local addressing, and Router Advertisement processing in staged sub-PRs if needed. | Tests cover extension-header and packet-boundary handling, neighbor state transitions, address lifecycle, and malformed or hostile advertisements; IPv6 status remains Prototype until end-to-end tests pass. | Prototype |
| 7 | `feat(net): add TCP transport with socket lifecycle` | Implement TCP socket states, handshake/close, sequence and acknowledgment validation, retransmission and timeout behavior, flow control, and reset handling. | State-machine tests and packet-level integration tests cover normal and adversarial cases; behavior is mapped to current TCP requirements, including RFC 9293. | Prototype |
| 8 | `feat(net): harden DHCP and DNS client behavior` | Review current DHCP/DNS modules; add lease renewal, expiry, server changes, retries, DNS response validation, negative caching policy, and bounded cache/resource limits. | Tests cover loss, duplicate and malformed replies, lease expiry, resolver changes, cache expiry, and resource limits; DNS behavior is documented. | Prototype |
| 9 | `feat(net): add persistent network configuration and rollback` | Define a versioned declarative configuration format for static addresses, DHCP, DNS, routes, and interface matching. Apply configuration transactionally with validation and rollback. | Invalid configuration leaves the previous working state intact; remote-risking changes have a recovery path; secrets are not written to logs. | Prototype |
| 10 | `feat(net): add network control CLI and diagnostics` | Add read-only status, interface, route, DNS, and connectivity diagnostics before adding mutating commands. Add explicit confirmation for changes that can sever connectivity. | CLI output is stable enough to script or offers a versioned JSON form; permissions and error messages are documented; diagnostics are bounded and avoid exposing secrets. | Prototype |
| 11 | `feat(net): add firewall policy framework with default-deny service rules` | Design a policy API and packet-filter hooks; begin with explicit inbound/outbound rules, connection tracking only if justified, and audit counters. | Firewall is off or conservatively configured according to a documented policy; no rule means no implicit service exposure; tests cover ordering, state cleanup, and IPv4/IPv6 parity. | Prototype |
| 12 | `ci(net): add QEMU networking integration and regression workflow` | Automate boot-to-network tests using a virtual NIC and isolated test network; archive logs and test manifests. | CI proves boot, interface discovery, address setup, DNS, and controlled connectivity; failure artifacts identify the stage; no test silently depends on an external public network. | Proposed |
| 13 | `docs(net): publish supported networking matrix and recovery guide` | Publish tested virtual/hardware devices, protocol coverage, known gaps, configuration examples, and recovery steps. | Every "Supported" claim links to repeatable evidence and a tested configuration; Wi-Fi/VPN/physical NIC gaps are stated plainly. | Supported |

---

## Periodic Maintenance Cadence

To maintain long-term reliability and security, recurring maintenance tasks are executed on structured cadences:

| Cadence | Proposed Task / PR | Required Evidence |
|---|---|---|
| Every PR | CI formatting, compilation, unit tests, targeted integration tests, static checks | Passing CI pipeline log tied to commit SHA |
| Weekly | `chore(net): triage networking failures and stale work` | Triage report linking open issues, fuzz crashes, or regressions to actionable fix PRs |
| Weekly | `test(net): refresh packet regression corpus` | Sanitized, reproducible packet corpus additions from fuzzing or bug reports |
| Monthly | `security(net): review parser and resource-exhaustion risks` | Bounded memory/queue audit, parser security classification, and regression tests |
| Monthly | `docs(net): reconcile implementation status and support evidence` | Reconciled status inventory linking source files to verified test evidence |
| Monthly | `ci(net): verify QEMU boot and network smoke path` | QEMU execution log, boot trace, network smoke test report, and retained artifacts |
| Quarterly | `security(net): review threat model and firewall defaults` | Updated threat model document, default-deny verification, and mitigation tickets |
| Quarterly | `docs(net): review protocol standards and interoperability gaps` | Protocol spec matrix cross-referencing RFC 1122, RFC 8200, and RFC 9293 requirements |
| Release Candidate | `release(net): certify networking claims for candidate` | Automated test suite run against target QEMU release artifact, release notes certification |
| Incident Postmortem | `fix(net): prevent recurrence of <incident>` | Minimal reproduction test failing prior to fix and passing after fix |

---

## Cross-Component Integration Rules

1. **Drivers and DMA**: VirtIO NIC queues must enforce descriptor length validation, bounded ring sizes, explicit ownership handoff, and full reset handling.
2. **Syscalls and Sockets**: Sockets expose explicit non-blocking, blocking, timeout, and permission boundaries governed by process capability checks.
3. **Init and Service Supervision**: Network-dependent services express explicit system service dependencies (`Wants=network.target`, `After=network.target`). Boot must never block indefinitely if network initialization fails.
4. **Configuration and Rollback**: Declarative network configurations must be schema-validated before application. Transactional changes trigger automatic rollback if connectivity checks fail within a configurable grace window.
5. **Security and Auditing**: Default-deny firewall rules, rate-limited diagnostic logging, credential/secret redaction, and bounded buffer allocation are enforced across all layers.
6. **Package and Distro Interoperability**: Foreign distro networking tools interface with SigmaOS via clean translation layers, maintaining native kernel isolation.
7. **Documentation and Inventory**: Every pull request updating network functionality must update `FEATURE_STATUS.toml` and documentation in the same commit.

---

## PR Review and Merge Gates

Pull requests affecting the networking stack must use the standardized PR template:

```text
## Summary
What changes, and what user-visible or developer-visible problem does it solve?

## Scope
Subsystems/files affected. State what this PR does not attempt to implement.

## Status claim
Proposed | Prototype | Integrated | Supported
Explain the evidence for the selected status.

## Design and compatibility
API/configuration changes, protocol/RFC references, Linux/BSD compatibility impact,
migration or rollback behavior.

## Security and failure behavior
Trust boundaries, malformed input, resource limits, privileges, logging/redaction,
failure recovery, and disclosure considerations.

## Tests
Commands run; unit/property/fuzz/integration/QEMU/hardware results; skipped tests and why.

## Documentation and inventory
Docs/status files updated, or a reason they do not apply.

## Checklist
- [ ] One focused, reviewable change
- [ ] Tests added or updated for behavior changed
- [ ] No unsupported compatibility or hardware claim
- [ ] Failure and rollback behavior documented where relevant
- [ ] Required CI checks pass
- [ ] Review conversations resolved
```

**Merge Policy**:
- Requires at least 1 maintainer approval for routine changes.
- Requires 2 maintainer approvals for changes affecting packet parsing, driver DMA, security policy, or public socket ABI.
- Mandates passing CI, resolved review threads, stale review dismissal upon commit updates, and branch protection against force pushes.

---

## Execution Order and Exit Criteria

1. **Phase 1 Baseline (PRs 1–3)**: Focus on specification, parser fixtures/fuzzing, and VirtIO queue connectivity in QEMU.
2. **Phase 2 Stack Foundation (PRs 4–8)**: Interface link state, IPv4/ICMP routing, IPv6/ND, RFC 9293 TCP transport, DHCP/DNS hardening.
3. **Phase 3 Management & Control (PRs 9–11)**: Declarative transactional config, network CLI diagnostics, default-deny firewall framework.
4. **Phase 4 Integration & Certification (PRs 12–13)**: Automated QEMU boot-to-network CI integration and candidate certification documentation.

**Exit Criteria**:
- A feature reaches **Integrated** status only when a documented boot path executes driver-to-stack-to-userspace communication in QEMU.
- A feature reaches **Supported** status only when it repeatedly passes the end-to-end automated test matrix, documents target configurations and limitations, and provides a verified recovery path.
