# Networking

**Capability state: Prototype.** Network data structures and protocol code do not establish working network connectivity. See the [shared status vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

No end-to-end network interface, driver, IP configuration, DNS, socket, or transport path has been verified on the real kernel boot path. Do not use `sigif`, Linux networking commands, example config files, or protocol module names as working SigmaOS interfaces. No supported NIC matrix or QEMU network transfer evidence is published.

## Design references

- Linux and FreeBSD socket APIs and network layering for explicit protocol boundaries.
- OpenBSD PF for auditable filtering policy after packets traverse a real interface.
- Arch documentation for exact configuration and troubleshooting.
- Mint onboarding for clear network setup and connection failure guidance.

## Roadmap

1. Validate one QEMU virtual NIC from enumeration through transmit and receive.
2. Add bounded Ethernet, ARP, IPv4, DHCP or static configuration, and DNS paths with malformed-packet tests.
3. Establish a tested UDP path, then TCP connection setup and transfer; publish captures and supported machine configuration.
4. Add IPv6, firewalling, Wi-Fi, and additional adapters individually with named hardware and failure tests.

**Completion evidence:** bidirectional packet transfer and connection tests on a named QEMU profile, protocol boundary tests, failure behavior, and a model-specific hardware support record.
