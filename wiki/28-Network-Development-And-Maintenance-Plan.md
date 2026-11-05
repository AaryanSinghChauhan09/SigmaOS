# SigmaOS development and maintenance plan

## Scope and operating rules
Comprehensive networking subsystem plan.

## Proposed PR sequence
- `docs(net): define network architecture, status, and support boundaries`
- `test(net): add deterministic packet fixtures and parser fuzz targets`
- `feat(net): connect VirtIO NIC to the Ethernet receive and transmit path`
- `feat(net): add interface and link-state model`
- `feat(net): add IPv4 routing and ICMP host support`
- `feat(net): add IPv6 host networking foundation`
- `feat(net): add TCP transport with socket lifecycle`
- `feat(net): harden DHCP and DNS client behavior`
- `feat(net): add persistent network configuration and rollback`
- `feat(net): add network control CLI and diagnostics`
- `feat(net): add firewall policy framework with default-deny service rules`
- `ci(net): add QEMU networking integration and regression workflow`
- `docs(net): publish supported networking matrix and recovery guide`

## Periodic maintenance cadence
- Regular maintenance cadences.

## Cross-component rules to add
- Cross-component rules.

## PR review and merge gates
- Quality gates.

## Execution order and exit criteria
- Milestones and exit criteria.
