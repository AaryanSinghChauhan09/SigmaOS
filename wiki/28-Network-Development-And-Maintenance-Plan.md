# SigmaOS Network Development and Maintenance Plan

## Scope and operating rules

Treat "network management" as both the OS networking stack and the user-visible facilities to configure, inspect, troubleshoot, and maintain network interfaces. Keep the initial target narrow: one documented virtual NIC in QEMU, with deterministic tests, before expanding to physical hardware, Wi-Fi, VPNs, or broad Linux/BSD compatibility.

Adopt these rules across the PR series:

- **One PR, one coherent change.** Split broad initiatives into small dependency-ordered PRs; keep implementation, tests, and the documentation for that change together.
- **Status follows evidence.** Use Proposed for unimplemented work, Prototype for incomplete or model-only code, Integrated after the path runs end to end in the OS, and Supported only after the documented test matrix passes repeatedly.
- **No silent compatibility claims.** Describe Linux/BSD inspirations as design references, not as compatibility or feature parity. Document unsupported protocol options, devices, and interfaces.
- **Safe defaults.** Network services start disabled unless explicitly required; configuration changes are validated before application; destructive or remote-disconnecting changes require confirmation or a rollback plan.
- **Protocol behavior is spec-led.** Define a protocol-to-RFC map for each feature and test malformed input, boundary cases, timeouts, and resource exhaustion. RFC 1122 describes host requirements across link, IP, and transport layers and emphasizes robustness, configurable behavior, and bounded diagnostic logging. Use current protocol specifications for individual implementations rather than treating RFC 1122 alone as a complete modern checklist.
- **Changes enter protected branches through PRs.** Require reviews, successful CI checks, conversation resolution, and no force pushes on the default branch.
- **Security-sensitive reports use the repository's private disclosure process.** Do not put exploitable details in a public issue or PR before maintainers coordinate a fix and disclosure.

## Proposed PR sequence

| PR | Proposed title | Main work | Acceptance criteria |
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

## Periodic maintenance cadence

Make the tasks below recurring **issue/PR templates or scheduled PRs**, not automatic changes to protected branches.

| Cadence | Proposed recurring PR or task | Required evidence |
|---|---|---|
| Every PR | Run formatting, compile/check, unit tests, targeted integration tests, and security/static checks applicable to the changed code. | CI results tied to the PR commit; test scope and any skipped checks stated in the PR. |
| Weekly | `chore(net): triage networking failures and stale work` — review failures, open bugs, fuzz crashes, regressions, and support-matrix changes; file focused fix PRs. | Triage notes link each failure to an issue, regression test, owner, or explicit non-action decision. |
| Weekly | `test(net): refresh packet regression corpus` — add minimized inputs from fuzzing and real bug reports; remove duplicates only with justification. | Reproducible corpus tests; no sensitive packet captures committed without review and sanitization. |
| Monthly | `security(net): review parser and resource-exhaustion risks` — review packet parsing, queue bounds, timers, allocation behavior, privilege boundaries, and logging. | Findings classified by severity; fixes or tracked issues; regression tests for resolved defects. |
| Monthly | `docs(net): reconcile implementation status and support evidence` — compare source, tests, runtime results, and documentation. | Inventory entries link to current tests and explicitly distinguish compile, unit, QEMU, and hardware evidence. |
| Monthly | `ci(net): verify QEMU boot and network smoke path` — run clean build and isolated end-to-end network tests on the supported virtual target. | Boot logs, network test results, environment manifest, and retained failure artifacts. |
| Quarterly | `security(net): review threat model and firewall defaults` — revisit exposed services, privileged configuration, remote lockout, IPv6 behavior, and security disclosure handling. | Updated threat model or a documented decision that it remains current; owner-approved mitigation issues. |
| Quarterly | `docs(net): review protocol standards and interoperability gaps` — check the relevant RFCs and document which requirements are implemented, deferred, or intentionally unsupported. | Versioned protocol compliance checklist; tests added for any adopted requirements. |
| Each release candidate | `release(net): certify networking claims for candidate` — run the release matrix and verify release notes and compatibility statements. | Passing release tests; exact target/configuration; known limitations and rollback path. |
| After every network incident | `fix(net): prevent recurrence of <incident>` — add regression coverage, correct the root cause, and update diagnostics or documentation. | Reproducer fails before and passes after the fix; impact and mitigation documented without exposing undisclosed vulnerability details. |

## Cross-component rules to add

- **Drivers and DMA:** Require bounded queues, validated descriptor lengths, clear ownership and reset semantics, and isolated device tests before claiming device support. A simulated device test alone is not proof of real hardware compatibility.
- **Syscalls and sockets:** Define a versioned socket API, error semantics, permission checks, resource limits, and compatibility boundaries before user programs depend on it.
- **Init and services:** Make network-dependent services express dependencies and timeouts; "network online" must have a defined meaning and must not block boot indefinitely.
- **Configuration and updates:** Version configuration schemas; validate before application; preserve the last-known-good configuration; make network updates recoverable if connectivity is lost.
- **Security and auditing:** Minimize privileges, redact credentials and tokens from logs, rate-limit repetitive events, bound queues and logs, and make forwarding or listening services opt-in.
- **Package and distro compatibility:** Create a compatibility layer only after SigmaOS has a stable, documented native network API. Start with a small, explicit command/config compatibility target instead of claiming broad Linux/BSD drop-in behavior.
- **Documentation and inventory:** Update the owning component page and feature inventory in the same PR as meaningful implementation changes. Record exact test environment and limitations.

## PR review and merge gates

Use a standard PR body with required fields: Summary, Scope, Status claim (Proposed | Prototype | Integrated | Supported), Design and compatibility, Security and failure behavior, Tests, Documentation and inventory, and Checklist.

Recommended merge policy: at least one maintainer approval for ordinary changes; two relevant reviewers for changes to protocol parsing, drivers, privileged networking, firewall policy, or public interfaces. Require CI, resolved conversations, stale-review dismissal after meaningful changes, and protection against force pushes/deletions on the default branch.

## Execution order and exit criteria

Start with PRs 1–3 and the CI baseline, then proceed from interface state and routing through IPv6 and TCP. Do not prioritize distro-compatibility promises or Wi-Fi ahead of a reproducible boot-to-network path.

A networking capability may be called **Integrated** only when a documented boot path exercises the actual driver-to-stack-to-userspace flow in QEMU. Call it **Supported** only when the project can repeat the relevant test matrix, publish the tested configuration, state known limitations, and demonstrate a recovery path.
