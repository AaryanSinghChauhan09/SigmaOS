# SigmaOS development and maintenance plan

## Scope and operating rules
Deterministic networking stack rules and safety invariants.

## Proposed PR sequence
| PR | Title |
| 1 | `docs(net): define network architecture, status, and support boundaries` |
| 2 | `test(net): add deterministic packet fixtures and parser fuzz targets` |
| 3 | `feat(net): connect VirtIO NIC to the Ethernet receive and transmit path` |
| 4 | `feat(net): add interface and link-state model` |
| 5 | `feat(net): add IPv4 routing and ICMP host support` |
| 6 | `feat(net): add IPv6 host networking foundation` |
| 7 | `feat(net): add TCP transport with socket lifecycle` |
| 8 | `feat(net): harden DHCP and DNS client behavior` |
| 9 | `feat(net): add persistent network configuration and rollback` |
| 10 | `feat(net): add network control CLI and diagnostics` |
| 11 | `feat(net): add firewall policy framework with default-deny service rules` |
| 12 | `ci(net): add QEMU networking integration and regression workflow` |
| 13 | `docs(net): publish supported networking matrix and recovery guide` |

## Periodic maintenance cadence
Continuous validation, fuzzing, and regression tests.

## Cross-component rules to add
Zero-leak packet buffers and isolated namespaces.

## PR review and merge gates
Required CI gates and security review.

## Execution order and exit criteria
13-PR roadmap completion with 100% test pass rate.
