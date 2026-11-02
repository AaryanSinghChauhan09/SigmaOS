# Governance: SigmaOS-network

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-network owns the network stack configuration and integration for SigmaOS. It integrates NetworkManager as the connection management backend, configures TLS trust roots, manages mDNS/Avahi service discovery, and provides the firewall hooks that coordinate with SigmaOS-security's firewall policy. It exposes a D-Bus API for other components to query and control network state.
- **Runtime layer:** core-system (network services run as system daemons)
- **Scope boundary with adjacent repos:** SigmaOS-security owns the firewall rule policy this repo's hooks enforce; SigmaOS-control-center provides the network settings panel; SigmaOS-panel displays network status; SigmaOS-kernel provides the networking subsystem this builds on.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-network` CLI | CLI | stable |
| `org.sigmaos.Network1` D-Bus service | D-Bus | stable |
| `/etc/sigmaos/network.toml` | Config file | stable |
| NetworkManager connection profiles (managed subset) | Config file | stable |
| Firewall hook API | Interface | unstable |
| mDNS service registration format | Config file | unstable |
| Internal connection state machine | internal | internal-only |

- **Breaking-change policy:** D-Bus API stable within a MAJOR version; connection profile subset is additive; firewall hook API may change but with deprecation notice.
- **API documentation location:** `docs/api/`, `man/sigmaos-network.8`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-kernel | 6.6.0 | networking subsystem | No |
| SigmaOS-security | 0.3 | firewall policy and hooks | No |
| NetworkManager | 1.44 | connection management backend | No |
| nftables | 1.0 | firewall enforcement | No |
| avahi | 0.8 | mDNS / service discovery | No |
| openssl / gnutls | 3.0 / 3.8 | TLS trust roots | No |
| dbus | 1.14 | IPC bus | No |
| systemd | 254 | service management | No |

- **Optional/soft dependencies:** `ModemManager` for cellular; `bluez` for Bluetooth networking; `iwd` as alternative Wi-Fi backend.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** NetworkManager and TLS libraries updated within 1 week of security releases.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-network-team
- **Co-maintainers:** @security-lead, @core-lead
- **Review quorum:** 2 maintainer approvals required; firewall hook changes require @security-lead approval.
- **Escalation path:** Network regressions are `priority:high`; connectivity loss on a server is `priority:critical`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 55% — config parsing, firewall hook logic, and D-Bus handlers unit-tested.
- **Integration tests:** Present — NetworkManager connection bring-up tested in a network namespace fixture.
- **CI pipeline file:** `.github/workflows/network-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `connection-bringup-test`, `firewall-hook-test`, `lint`
- **Known flaky tests:** `connection-bringup-test` flaky on CI network timing (issue #11).
- **CI build badge:** `![network CI](https://github.com/SigmaOS/SigmaOS-network/actions/workflows/network-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |

- **Hardware classes:** desktop, laptop, server, VM — all supported.
- **Backend variants:** Linux only; tested in CI with network namespaces.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** NetworkManager integration validated on 3 reference NICs, firewall hooks coordinated with SigmaOS-security, mDNS discovery reliable, TLS trust roots rotation documented.

## 8. What This Repo Does NOT Do

- Does not define firewall policy → handled by SigmaOS-security (this enforces via hooks)
- Does not provide the network settings UI → handled by SigmaOS-control-center (network panel)
- Does not provide network status display → handled by SigmaOS-panel (network applet)
- Does not manage kernel networking subsystem → handled by SigmaOS-kernel
- Does not provide VPN protocols → handled by NetworkManager VPN plugins (external)

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/network.toml`, NetworkManager profiles at `/etc/NetworkManager/system-connections/`
- **Schema file location:** `schemas/network.toml.schema.json`
- **Schema format:** TOML (SigmaOS config), INI (NetworkManager profiles)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_NETWORK_BACKEND` | `networkmanager` | Connection backend |
| `SIGMAOS_NETWORK_MDNS` | `1` | Enable mDNS |
| `SIGMAOS_NETWORK_FIREWALL_HOOKS` | `1` | Enable firewall hook coordination |

- **Breaking config change policy:** Config keys additive within a MAJOR version.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Connection bring-up fails | Bad profile or missing driver | No connectivity | Check `nmcli`; verify driver via SigmaOS-drivers |
| Firewall hook conflict | Rules out of sync with policy | Traffic blocked unexpectedly | Reconcile via `sigmaos-network --sync-firewall` |
| mDNS not resolving | Avahi daemon down | Services not discoverable | Restart avahi: `systemctl restart avahi-daemon` |
| TLS verification fails | Stale trust roots | HTTPS connections refused | Update CA certs: `sigmaos-network --update-ca` |
| D-Bus service crash | Handler bug | Other components lose network state | systemd auto-restarts; reconnect automatic |

- **Fallback behavior:** If NetworkManager fails, falls back to a minimal DHCP/static config via systemd-networkd; if firewall hooks fail, defaults to the most restrictive SigmaOS-security policy.
- **Operator escalation path:** Collect `journalctl -u NetworkManager` and `/var/log/sigmaos/network.log`; file issue.
- **Crash dump artifacts:** Network log at `/var/log/sigmaos/network.log`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager; post-install hook restarts the D-Bus service and reloads firewall hooks.
- **Rollback mechanism:** Previous package retained; `sigmaos-network --rollback-config` restores previous connection profiles; firewall rules re-synced.
- **Expected downtime on update:** Brief network blip on firewall reload (~1 second); connections maintained where possible.
- **Pre-update state checks:** No active critical network operation; firewall in consistent state.
- **Post-update validation:** `sigmaos-network --self-check` verifies backend connectivity and firewall sync.
- **Atomic update guarantee:** Partial — config writes atomic; firewall reload transactional via nftables.

## 12. Security Boundary

- **Privilege level required:** Runs as a dedicated `sigmaos-network` system user with net admin capability; D-Bus queries unprivileged, mutations gated by polkit.
- **Trust boundary:** Trusts SigmaOS-security's firewall policy; TLS trust roots from SigmaOS-security; does not trust network-provided input without validation.
- **Attack surface:** Network (all ingress/egress), D-Bus IPC, config parsing, firewall rule parsing
- **Hardening measures:** Runs with restricted capabilities; TLS verification enforced; firewall defaults deny; mDNS restricted to local link; runs with `NoNewPrivileges`.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Internal review 2024-12.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-kernel | 6.6.x | tested |
| SigmaOS-security | 0.3+ | tested |
| SigmaOS-control-center | 0.2+ | tested (network panel) |
| SigmaOS-panel | 0.2+ | tested (network applet) |

| Package | Version Range | Status |
|---------|---------------|--------|
| NetworkManager | 1.44+ | tested |
| nftables | 1.0+ | tested |
| avahi | 0.8+ | tested |

- **Known incompatibilities:** NetworkManager < 1.40 lacks some connection features.
- **Forward-compatibility pledge:** D-Bus API stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-network-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/firewall-hooks.md`
- **Man page or in-app help status:** present — `sigmaos-network(8)`
- **Documentation parity policy:** D-Bus or firewall hook changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-12

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:networkmanager` | NetworkManager integration issues |
| `component:firewall-hooks` | Firewall hook coordination issues |
| `component:mdns` | mDNS / Avahi issues |
| `component:tls` | TLS trust root issues |
| `component:wifi` | Wi-Fi specific issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-network/milestones`
- **Issue triage cadence:** Every Monday and Thursday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for D-Bus API or firewall hook incompatibility.
- **Version pinning in dependents:** SigmaOS-control-center pins `sigmaos-network >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depended-on-by-core — core's services depend on network availability for network-aware startup ordering; core queries this service for network status.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Network1` queried by `org.sigmaos.Core1`); firewall hook files shared with SigmaOS-security.
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — network-aware startup order tested with core.
- **Change coordination policy:** Firewall hook changes require a tracking issue in SigmaOS-security; network status API changes require a tracking issue in SigmaOS-core.
- **Core team review required for cross-boundary changes:** yes
