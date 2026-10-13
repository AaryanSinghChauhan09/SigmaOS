# SigmaOS Network Development and Maintenance Plan

## Executive Summary

SigmaOS is an experimental Rust operating system. A module or passing unit test in source code does not prove a complete boot path, runtime integration, or supported hardware. Current networking modules (Ethernet, ARP, IPv4, UDP, DHCP, DNS) represent starting points to verify rather than claims of production readiness. VirtIO networking is partially implemented and being integrated with the Ethernet receive/transmit path.

This plan specifies the architecture, 13 proposed PR series, periodic maintenance cadences, cross-component integration rules, PR review/merge gates, and execution order for SigmaOS network management and host networking.

---

## Scope and Operating Rules

Network management encompasses both the operating system networking stack and user-visible facilities to configure, inspect, troubleshoot, and maintain network interfaces.

Initial Target Boundary: One documented virtual NIC in QEMU (VirtIO-net), with deterministic tests, before expanding to physical hardware, Wi-Fi, VPNs, or broad Linux/BSD compatibility.

### Operating Rules
1. **One PR, one coherent change:** Split broad initiatives into small dependency-ordered PRs; keep implementation, tests, and documentation together.
2. **Status follows evidence:** Use Proposed for unimplemented work, Prototype for incomplete or model-only code, Integrated after the path runs end-to-end in the OS, and Supported only after the documented test matrix passes repeatedly.
3. **No silent compatibility claims:** Describe Linux/BSD inspirations as design references, not as compatibility or feature parity. Document unsupported protocol options, devices, and interfaces.
4. **Safe defaults:** Network services start disabled unless explicitly required; configuration changes are validated before application; destructive or remote-disconnecting changes require confirmation or a rollback plan.
5. **Protocol behavior is spec-led:** Define a protocol-to-RFC map for each feature and test malformed input, boundary cases, timeouts, and resource exhaustion. RFC 1122 describes host requirements across link, IP, and transport layers. Modern protocol specifications (RFC 8200 for IPv6, RFC 9293 for TCP) govern specific protocol implementations.
6. **Protected branch controls:** Changes enter protected branches through PRs requiring reviews, successful CI checks, conversation resolution, and no force pushes on default branches.
7. **Private security disclosure:** Security-sensitive reports use the repository's private disclosure process before maintainers coordinate a fix and public disclosure.

---

## Proposed PR Sequence

| PR | Proposed Title | Main Work | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(net): define network architecture, status, and support boundaries` | Document layers, data flow, ownership, supported test target, configuration model, status vocabulary, and current gaps. Link each networking feature to source, tests, and evidence. | Docs distinguish module tests from runtime integration; current limitations and target devices are explicit; no unsupported compatibility claims. |
| 2 | `test(net): add deterministic packet fixtures and parser fuzz targets` | Establish shared packet fixtures, property tests, fuzz targets, malformed/truncated packet cases, and regression corpus conventions. | Parsers reject or safely handle malformed lengths, invalid checksums, unknown values, and oversized input; fuzz targets run reproducibly in CI or documented local commands. |
| 3 | `feat(net): connect VirtIO NIC to the Ethernet receive and transmit path` | Wire the existing partial VirtIO network driver into the Ethernet path; specify queue lifecycle, buffer ownership, error reporting, and reset behavior. | QEMU can enumerate the NIC, transmit and receive test frames, recover from queue/device errors, and report test evidence; hardware support remains unclaimed. |
| 4 | `feat(net): add interface and link-state model` | Introduce stable interface identifiers, link up/down state, MTU, MAC, counters, and typed configuration events. | State changes are observable; counters have defined reset and wrap behavior; interface removal does not leave stale routes or resolver state. |
| 5 | `feat(net): add IPv4 routing and ICMP host support` | Add a route table, longest-prefix selection, default route, source-address selection, and essential ICMP handling. | Unit and integration tests cover local/remote delivery, no route, multiple routes, TTL expiry, unreachable errors, and configurable forwarding disabled by default. |
| 6 | `feat(net): add IPv6 host networking foundation` | Implement IPv6 parsing and validation, ICMPv6 essentials, Neighbor Discovery, link-local addressing, and Router Advertisement processing. | Tests cover extension-header and packet-boundary handling, neighbor state transitions, address lifecycle, and malformed or hostile advertisements; IPv6 status remains Prototype until end-to-end tests pass. |
| 7 | `feat(net): add TCP transport with socket lifecycle` | Implement TCP socket states, handshake/close, sequence and acknowledgment validation, retransmission and timeout behavior, flow control, and reset handling. | State-machine tests and packet-level integration tests cover normal and adversarial cases; behavior is mapped to current TCP requirements, including RFC 9293. |
| 8 | `feat(net): harden DHCP and DNS client behavior` | Review current DHCP/DNS modules; add lease renewal, expiry, server changes, retries, DNS response validation, negative caching policy, and bounded cache/resource limits. | Tests cover loss, duplicate and malformed replies, lease expiry, resolver changes, cache expiry, and resource limits; DNS behavior is documented. |
| 9 | `feat(net): add persistent network configuration and rollback` | Define a versioned declarative configuration format for static addresses, DHCP, DNS, routes, and interface matching. Apply configuration transactionally with validation and rollback. | Invalid configuration leaves the previous working state intact; remote-risking changes have a recovery path; secrets are not written to logs. |
| 10 | `feat(net): add network control CLI and diagnostics` | Add read-only status, interface, route, DNS, and connectivity diagnostics before adding mutating commands. Add explicit confirmation for changes that can sever connectivity. | CLI output is stable enough to script or offers a versioned JSON form; permissions and error messages are documented; diagnostics are bounded and avoid exposing secrets. |
| 11 | `feat(net): add firewall policy framework with default-deny service rules` | Design a policy API and packet-filter hooks; begin with explicit inbound/outbound rules, connection tracking only if justified, and audit counters. | Firewall is off or conservatively configured according to a documented policy; no rule means no implicit service exposure; tests cover ordering, state cleanup, and IPv4/IPv6 parity. |
| 12 | `ci(net): add QEMU networking integration and regression workflow` | Automate boot-to-network tests using a virtual NIC and isolated test network; archive logs and test manifests. | CI proves boot, interface discovery, address setup, DNS, and controlled connectivity; failure artifacts identify the stage; no test silently depends on an external public network. |
| 13 | `docs(net): publish supported networking matrix and recovery guide` | Publish tested virtual/hardware devices, protocol coverage, known gaps, configuration examples, and recovery steps. | Every "Supported" claim links to repeatable evidence and a tested configuration; Wi-Fi/VPN/physical NIC gaps are stated plainly. |

---

## Periodic Maintenance Cadence

| Cadence | Task | Required Evidence |
|---|---|---|
| Every PR | Formatting, build, unit tests, targeted integration tests, static checks. | CI results tied to PR commit. |
| Weekly | Triage networking failures and stale work. | Triage notes linking failure to issue or test. |
| Weekly | Refresh packet regression corpus. | Reproducible corpus tests. |
| Monthly | Review parser and resource-exhaustion risks. | Findings classified with fix or issue track. |
| Monthly | Reconcile implementation status and support evidence. | Inventory entries linking to current tests. |
| Monthly | Verify QEMU boot and network smoke path. | Boot logs and network test results. |
| Quarterly | Review threat model and firewall defaults. | Updated threat model or decision doc. |
| Quarterly | Review protocol standards and interoperability gaps. | Protocol compliance checklist. |
| Each RC | Certify networking claims for release candidate. | Passing release test matrix. |
| Post-Incident | Prevent recurrence of incident. | Reproducer test failing before fix and passing after. |

---

## Cross-Component Rules

- **Drivers and DMA:** Require bounded queues, validated descriptor lengths, clear ownership and reset semantics, and isolated device tests before claiming device support.
- **Syscalls and sockets:** Define a versioned socket API, error semantics, permission checks, resource limits, and compatibility boundaries before user programs depend on it.
- **Init and services:** Make network-dependent services express dependencies and timeouts; "network online" must have a defined meaning and must not block boot indefinitely.
- **Configuration and updates:** Version configuration schemas; validate before application; preserve the last-known-good configuration; make network updates recoverable if connectivity is lost.
- **Security and auditing:** Minimize privileges, redact credentials and tokens from logs, rate-limit repetitive events, bound queues and logs, and make forwarding or listening services opt-in.
- **Package and distro compatibility:** Create a compatibility layer only after SigmaOS has a stable, documented native network API.
- **Documentation and inventory:** Update the owning component page and feature inventory in the same PR as meaningful implementation changes.

---

## Execution Order and Exit Criteria

Start with PRs 1–3 and the CI baseline, then proceed from interface state and routing through IPv6 and TCP. Do not prioritize distro-compatibility promises or Wi-Fi ahead of a reproducible boot-to-network path.

A networking capability may be called **Integrated** only when a documented boot path exercises the actual driver-to-stack-to-userspace flow in QEMU. Call it **Supported** only when the project can repeat the relevant test matrix, publish the tested configuration, state known limitations, and demonstrate a recovery path.
