# SigmaOS v1.0 Release Criteria

## Definition of Done

A build is eligible for stable release when ALL of the following criteria are met.

## Installation

| Criterion | Target | Gate |
|---|---|---|
| Clean install success rate on Tier 1 hardware | ≥ 99% | BLOCKING |
| Dual-boot install alongside Windows 11 | ≥ 95% | BLOCKING |
| Encrypted install (LUKS2) success rate | ≥ 99% | BLOCKING |
| Install duration (default partitioning) | ≤ 20 min | BLOCKING |
| First-boot wizard completion without error | 100% | BLOCKING |
| Default apps present after first login | 24/24 | BLOCKING |

## Boot Performance

| Criterion | Target | Gate |
|---|---|---|
| Time from GRUB to desktop (SSD) | ≤ 30 s | BLOCKING |
| Time from GRUB to desktop (HDD) | ≤ 60 s | ADVISORY |
| Resume from S3 sleep | ≤ 5 s | BLOCKING |
| Resume from S4 hibernate | ≤ 15 s | ADVISORY |

## Update and Recovery

| Criterion | Target | Gate |
|---|---|---|
| Pre-update snapshot always created | 100% | BLOCKING |
| Snapshot restore success rate | 100% | BLOCKING |
| Auto-rollback triggers on failed update | 100% | BLOCKING |
| Manual rollback via `sigma rescue --rollback` | Works | BLOCKING |
| Filesystem repair succeeds after simulated corruption | ≥ 95% | BLOCKING |
| GRUB repair from emergency shell | Works | BLOCKING |

## Package Trust

| Criterion | Target | Gate |
|---|---|---|
| All packages in official repos have valid Ed25519 signature | 100% | BLOCKING |
| Invalid signature blocks install | 100% | BLOCKING |
| Revoked key blocks install | 100% | BLOCKING |
| Repo metadata signature verified on every sync | 100% | BLOCKING |

## Smoke Tests

| Criterion | Target | Gate |
|---|---|---|
| All 50 smoke tests pass | 50/50 | BLOCKING |
| P0 test failures | 0 | BLOCKING |
| P1 test failures | ≤ 5 | BLOCKING |
| P2 test failures | ≤ 15 | ADVISORY |

See `src/testing/smoke_tests.rs` for the full list.

## Bug Severity Policy

- **P0 (Critical)**: System crashes, data loss, security vulnerability, install failure on Tier 1
- **P1 (High)**: Feature broken, app crashes on launch, update fails, recovery fails
- **P2 (Medium)**: UI glitch, non-critical feature broken, performance regression
- **P3 (Low)**: Cosmetic issue, documentation gap, minor UX friction

## Hardware Validation

Must pass on at minimum:

### Tier 1 Reference Machines (must all pass)

| Machine | CPU | GPU | WiFi |
|---|---|---|---|
| Dell XPS 15 9530 | Intel Core i7-13700H | Intel Iris Xe | Intel AX211 |
| Lenovo ThinkPad X1 Carbon Gen 11 | Intel Core i7-1365U | Intel Iris Xe | Intel AX211 |
| Framework Laptop 13 (AMD) | AMD Ryzen 7 7840U | AMD Radeon 780M | Intel AX210 |
| Desktop: AMD Ryzen 9 7900X + RX 7900 XTX | AMD Ryzen 9 7900X | AMD RX 7900 XTX | Realtek RTW89 |
| Desktop: Intel Core i9-13900K + Arc A770 | Intel i9-13900K | Intel Arc A770 | Intel AX200 |

### Hardware Test Matrix (all must pass)

| Test | Pass Criteria |
|---|---|
| GPU renders desktop at native resolution | No artifacts, correct color |
| WiFi connects to WPA3 network | Connected within 10 s |
| Bluetooth pairs with a device | Pairs and transfers |
| Audio output (headphone jack + HDMI) | Sound plays correctly |
| Audio input (microphone) | Records without distortion |
| Suspend to RAM (S3) | Suspends and resumes correctly |
| External monitor (HDMI + DisplayPort) | Mirror and extend modes work |
| USB 3.x storage | Mounts and transfers at full speed |
| Printer discovery (IPP) | Discovers and prints a test page |

## Documentation

| Criterion | Target | Gate |
|---|---|---|
| Installation guide complete | Yes | BLOCKING |
| Troubleshooting guide covers top 10 issues | Yes | BLOCKING |
| Release notes document all changes | Yes | BLOCKING |
| Hardware compatibility matrix published | Yes | BLOCKING |

## Sign-Off Requirements

A release candidate becomes stable when:

1. All BLOCKING criteria above are met
2. Two maintainer approvals on the release PR
3. No open P0 or P1 bugs filed against the RC
4. Release notes reviewed and published
5. ISO signed with the SigmaOS release key (Ed25519)
6. SHA256 checksums published alongside ISO
