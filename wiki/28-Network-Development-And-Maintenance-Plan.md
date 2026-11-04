# SigmaOS development and maintenance plan

## Scope and operating rules
This document defines the SigmaOS development and maintenance plan for the network stack.

## Proposed PR sequence
| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 01 | `docs(net): define network architecture, status, and support boundaries` | Network architecture definition | Architecture spec |
| 02 | `test(net): add deterministic packet fixtures and parser fuzz targets` | Packet fixtures and fuzzers | Test coverage |
| 03 | `feat(net): connect VirtIO NIC to the Ethernet receive and transmit path` | VirtIO NIC queue integration | RX/TX operations |
| 04 | `feat(net): add interface and link-state model` | Interface counters and states | Operational model |
| 05 | `feat(net): add IPv4 routing and ICMP host support` | IPv4 routing and ICMP replies | ICMP host support |
| 06 | `feat(net): add IPv6 host networking foundation` | Link-local and RA parsing | IPv6 foundation |
| 07 | `feat(net): add TCP transport with socket lifecycle` | RFC 9293 state machine | TCP connections |
| 08 | `feat(net): harden DHCP and DNS client behavior` | Dynamic address acquisition and DNS cache | Hardened client |
| 09 | `feat(net): add persistent network configuration and rollback` | Transactional rollback support | Safe config update |
| 10 | `feat(net): add network control CLI and diagnostics` | Diagnostic JSON and commands | CLI diagnostics |
| 11 | `feat(net): add firewall policy framework with default-deny service rules` | Inbound/outbound filtering | Firewall rules |
| 12 | `ci(net): add QEMU networking integration and regression workflow` | Emulated test validation | Regression suite |
| 13 | `docs(net): publish supported networking matrix and recovery guide` | Matrix and recovery documentation | Published guide |

## Periodic maintenance cadence
Regular testing and audits.

## Cross-component rules to add
Cross-component requirements and stability gates.

## PR review and merge gates
Required checks and validation standards.

## Execution order and exit criteria
Execution criteria for network maturity.
