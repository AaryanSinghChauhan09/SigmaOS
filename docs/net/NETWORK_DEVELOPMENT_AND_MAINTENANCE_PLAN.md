# SigmaOS Network Development and Maintenance Plan

## Executive Summary & Overview

SigmaOS is an experimental Rust operating system. This plan establishes the architecture, operating rules, 13-PR execution sequence, periodic maintenance cadences, cross-component rules, and review gates for the OS networking stack and userland network management utilities.

The networking stack implementation treats current module-level code as starting points to verify—not as claims of production readiness. Subsystem capabilities transition through the status vocabulary (**Proposed**, **Prototype**, **Integrated**, **Supported**) only when backed by reproducible test evidence.

---

## Scope and Operating Rules

### Initial Target Scope
Keep the initial target narrow: **one documented virtual NIC in QEMU (`virtio-net-pci`)**, with deterministic tests, before expanding to physical hardware, Wi-Fi, VPNs, or broad Linux/BSD compatibility.

### Core Operating Rules

1. **One PR, One Coherent Change:** Split broad initiatives into small dependency-ordered PRs; keep implementation, tests, and documentation together.
2. **Status Follows Evidence:**
   - **Proposed:** Unimplemented work / architectural spec.
   - **Prototype:** Incomplete or model-only code.
   - **Integrated:** End-to-end path verified in QEMU runtime.
   - **Supported:** Matrix passes repeatedly across test suites.
3. **No Silent Compatibility Claims:** Describe Linux/BSD inspirations as design references, not drop-in compatibility. Explicitly document unsupported protocol options, devices, and interfaces.
4. **Safe Defaults:** Network services start disabled unless explicitly enabled. Configuration changes are validated before application; destructive or remote-disconnecting changes require explicit confirmation or automatic rollback.
5. **Spec-Led Protocol Behavior:** Map each protocol feature to its governing RFC (RFC 1122 host requirements, RFC 8200 IPv6, RFC 9293 TCP). Test malformed input, boundary cases, timeouts, and resource limits.
6. **Protected Branch Gatekeeping:** Changes enter default branch only via PRs requiring review approvals, passing CI checks, resolved conversations, and no force pushes.
7. **Private Vulnerability Disclosure:** Security-sensitive reports follow the private disclosure process in `SECURITY.md`.

---

## Proposed PR Sequence

| PR | Title | Main Work | Acceptance Criteria | Status |
|---|---|---|---|---|
| 1 | `docs(net): define network architecture, status, and support boundaries` | Document layers, data flow, ownership, supported test target, configuration model, status vocabulary, and current gaps. Link each networking feature to source, tests, and evidence. | Docs distinguish module tests from runtime integration; current limitations and target devices are explicit; no unsupported compatibility claims. | Supported |
| 2 | `test(net): add deterministic packet fixtures and parser fuzz targets` | Establish shared packet fixtures, property tests, fuzz targets, malformed/truncated packet cases, and regression corpus conventions. | Parsers reject or safely handle malformed lengths, invalid checksums, unknown values, and oversized input; fuzz targets run reproducibly in CI or documented local commands. | Prototype |
| 3 | `feat(net): connect VirtIO NIC to the Ethernet receive and transmit path` | Wire the existing partial VirtIO network driver into the Ethernet path; specify queue lifecycle, buffer ownership, error reporting, and reset behavior. | QEMU can enumerate the NIC, transmit and receive test frames, recover from queue/device errors, and report test evidence; hardware support remains unclaimed. | Prototype |
| 4 | `feat(net): add interface and link-state model` | Introduce stable interface identifiers, link up/down state, MTU, MAC, counters, and typed configuration events. | State changes are observable; counters have defined reset and wrap behavior; interface removal does not leave stale routes or resolver state. | Prototype |
| 5 | `feat(net): add IPv4 routing and ICMP host support` | Add a route table, longest-prefix selection, default route, source-address selection, and essential ICMP handling. | Unit and integration tests cover local/remote delivery, no route, multiple routes, TTL expiry, unreachable errors, and configurable forwarding disabled by default. | Prototype |
| 6 | `feat(net): add IPv6 host networking foundation` | Implement IPv6 parsing and validation, ICMPv6 essentials, Neighbor Discovery, link-local addressing, and Router Advertisement processing in staged sub-PRs if needed. | Tests cover extension-header and packet-boundary handling, neighbor state transitions, address lifecycle, and malformed or hostile advertisements; IPv6 status remains Prototype until end-to-end tests pass. | Prototype |
| 7 | `feat(net): add TCP transport with socket lifecycle` | Implement TCP socket states, handshake/close, sequence and acknowledgment validation, retransmission and timeout behavior, flow control, and reset handling. | State-machine tests and packet-level integration tests cover normal and adversarial cases; behavior is mapped to current TCP requirements, including RFC 9293, not merely legacy RFC 1122. | Prototype |
| 8 | `feat(net): harden DHCP and DNS client behavior` | Review current DHCP/DNS modules; add lease renewal, expiry, server changes, retries, DNS response validation, negative caching policy, and bounded cache/resource limits. | Tests cover loss, duplicate and malformed replies, lease expiry, resolver changes, cache expiry, and resource limits; DNS behavior is documented. | Prototype |
| 9 | `feat(net): add persistent network configuration and rollback` | Define a versioned declarative configuration format for static addresses, DHCP, DNS, routes, and interface matching. Apply configuration transactionally with validation and rollback. | Invalid configuration leaves the previous working state intact; remote-risking changes have a recovery path; secrets are not written to logs. | Prototype |
| 10 | `feat(net): add network control CLI and diagnostics` | Add read-only status, interface, route, DNS, and connectivity diagnostics before adding mutating commands. Add explicit confirmation for changes that can sever connectivity. | CLI output is stable enough to script or offers a versioned JSON form; permissions and error messages are documented; diagnostics are bounded and avoid exposing secrets. | Prototype |
| 11 | `feat(net): add firewall policy framework with default-deny service rules` | Design a policy API and packet-filter hooks; begin with explicit inbound/outbound rules, connection tracking only if justified, and audit counters. | Firewall is off or conservatively configured according to a documented policy; no rule means no implicit service exposure; tests cover ordering, state cleanup, and IPv4/IPv6 parity. | Prototype |
| 12 | `ci(net): add QEMU networking integration and regression workflow` | Automate boot-to-network tests using a virtual NIC and isolated test network; archive logs and test manifests. | CI proves boot, interface discovery, address setup, DNS, and controlled connectivity; failure artifacts identify the stage; no test silently depends on an external public network. | Proposed |
| 13 | `docs(net): publish supported networking matrix and recovery guide` | Publish tested virtual/hardware devices, protocol coverage, known gaps, configuration examples, and recovery steps. | Every "Supported" claim links to repeatable evidence and a tested configuration; Wi-Fi/VPN/physical NIC gaps are stated plainly. | Supported |

---

## Periodic Maintenance Cadence

| Cadence | Proposed Task | Required Evidence |
|---|---|---|
| **Every PR** | Run formatting, `rustc --test`, integration tests, and security/static checks applicable to touched code. | CI results tied to PR commit; test scope and any skipped checks stated in PR. |
| **Weekly** | `chore(net): triage networking failures and stale work` — review failures, open bugs, fuzz crashes, regressions, and support-matrix changes. | Triage notes link each failure to an issue, regression test, owner, or explicit non-action decision. |
| **Weekly** | `test(net): refresh packet regression corpus` — add minimized inputs from fuzzing and bug reports; remove duplicates with rationale. | Reproducible corpus tests; no sensitive packet captures committed without sanitization. |
| **Monthly** | `security(net): review parser and resource-exhaustion risks` — review packet parsing, queue bounds, timers, allocation, privileges, and logging. | Severity-classified findings; fixes or tracked issues; regression tests for resolved defects. |
| **Monthly** | `docs(net): reconcile implementation status and support evidence` — compare source, tests, runtime results, and docs. | Feature inventory entries link to current tests and explicitly distinguish compile, unit, QEMU, and hardware evidence. |
| **Monthly** | `ci(net): verify QEMU boot and network smoke path` — run clean build and isolated end-to-end network tests on supported target. | Boot logs, network test results, environment manifest, and retained failure artifacts. |
| **Quarterly** | `security(net): review threat model and firewall defaults` — revisit exposed services, privileged config, remote lockout, IPv6, disclosure process. | Updated threat model or documented decision; owner-approved mitigation issues. |
| **Quarterly** | `docs(net): review protocol standards and interoperability gaps` — check relevant RFCs and document implemented vs deferred requirements. | Versioned protocol compliance checklist; tests added for any adopted requirements. |
| **Each RC** | `release(net): certify networking claims for candidate` — run release matrix and verify release notes and compatibility statements. | Passing release tests; exact target/configuration; known limitations and rollback path. |
| **Post-Incident** | `fix(net): prevent recurrence of <incident>` — add regression coverage, correct root cause, update diagnostics/docs. | Reproducer fails before and passes after fix; impact and mitigation documented without exposing unpatched flaw details. |

---

## Cross-Component Rules

- **Drivers and DMA:** Bounded descriptor queues, validated buffer lengths, clear ownership and reset semantics, and isolated device unit/model tests before claiming support.
- **Syscalls and Sockets:** Versioned socket API, error semantics (`EINVAL`, `EWOULDBLOCK`, `ECONNREFUSED`), permission checks, resource limits, and ABI stability contracts.
- **Init and Services:** Network-dependent services express explicit dependencies and timeouts; "network online" must have a defined criteria and must not block boot indefinitely.
- **Configuration and Updates:** Version declarative schemas; validate before application; preserve last-known-good configuration; ensure updates have an automatic rollback on loss of connectivity.
- **Security and Auditing:** Minimize privileges, redact credentials/tokens from logs, rate-limit repetitive events, bound queues, and make listening services opt-in.
- **Package and Distro Compatibility:** Build compatibility layers only over stable native APIs. Target specific CLI/config interfaces rather than claiming drop-in Linux/BSD compatibility.
- **Documentation and Inventory:** Update `FEATURE_STATUS.toml` and wiki docs in the same PR as implementation changes. Record exact test environments and limitations.

---

## PR Review and Merge Gates

Every PR targeting the networking stack must use this pull request template:

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

### Review Policy
- **Standard PRs:** At least 1 maintainer approval required.
- **Critical PRs** (packet parsers, drivers, privileged networking, firewall policy, public APIs): At least 2 maintainer approvals required.
- **Merge Requirements:** Required CI checks pass, all review comments resolved, stale reviews dismissed after new commits, and protected branch rules enforced.

---

## Execution Order & Exit Criteria

1. **Phase 1 (Baseline):** PRs 1–3 + CI packet test harness.
2. **Phase 2 (Host Stack):** PRs 4–6 (Interface state, IPv4 routing/ICMP, IPv6 host foundation).
3. **Phase 3 (Transport & Services):** PRs 7–8 (RFC 9293 TCP transport, hardened DHCP/DNS).
4. **Phase 4 (Management & Security):** PRs 9–11 (Persistent config/rollback, CLI diagnostics, default-deny firewall).
5. **Phase 5 (CI & Release Matrix):** PRs 12–13 (QEMU automated regression, support matrix publication).

### Status Exit Criteria Thresholds
- **Integrated:** Documented boot path exercises driver-to-stack-to-userspace flow in QEMU.
- **Supported:** Test matrix passes repeatedly across CI runs, tested configuration is published, limitations are documented, and a tested recovery/rollback path exists.
