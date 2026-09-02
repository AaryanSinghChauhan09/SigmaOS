# 🚀 SigmaOS Linux Mint Parity Roadmap & Architecture Plan

This document details the architectural roadmap to achieve full feature parity with **Linux Mint** (Cinnamon / LMDE Edition) while preserving SigmaOS's zero-dependency, safe Rust, microkernel-based sovereignty.

***

## 1. Executive Summary

Linux Mint is widely recognized as the benchmark for beginner-friendly, out-of-the-box usable desktop operating systems. To match Linux Mint's user experience, SigmaOS systematically addresses seven core architectural pillars:

1.  **Cinnamon & MATE Desktop Experience:** Panel applets, GTK themes (`Mint-Y`), window management.
2.  **Hardware Enablement (HWE):** Driver detection manager, NVIDIA PRIME hybrid GPU offloading.
3.  **Multimedia Engine:** Audio DSP session mixing, hardware-accelerated AV1/H.264 video rendering.
4.  **Update Safety & System Rollback:** 5-tier Update Manager and Timeshift snapshot restores.
5.  **Beginner-Friendly System Utilities:** Software Store, MintStick USB ISO flasher, MintMenu launcher.
6.  **Network & VPN Security:** WireGuard driver integration and PIA VPN split-tunneling.
7.  **Documentation Sovereignty:** Synchronized Wiki guides and contribution standards.

***

## 2. Technical Architecture & Module Mapping

| Linux Mint Component | SigmaOS Architecture Module | Implementation Details |
|---|---|---|
| **Cinnamon Applets & Panels** | `src/productivity/mint_competitor.rs` | `CinnamonAppletEngine`, `CinnamonApplet` |
| **Mint-Y / Mint-X Themes** | `src/compatibility/mint_linux.rs` | `CinnamonThemeEngine`, `CinnamonPreset` |
| **MATE Desktop Suite** | `src/desktop/mate_betsy.rs` | `MateBetsyDesktopEnvironment`, `MarcoWindowManager`, `CajaFileManager` |
| **Driver Manager** | `src/compatibility/mint_linux.rs` | `MintDriverManager`, `MintDriverInfo` |
| **NVIDIA PRIME Switching** | `src/productivity/mint_competitor.rs` | `SovereignNvidiaPrimeEngine`, `NvidiaPrimeProfile` |
| **Update Manager** | `src/compatibility/mint_linux.rs` | `MintUpdateManager`, `MintUpdateLevel` (Levels 1-5) |
| **Timeshift Snapshots** | `src/compatibility/mint_linux.rs` | `MintTimeshiftEngine`, `MintTimeshiftSnapshot` |
| **Software Manager** | `src/compatibility/mint_linux.rs` | `MintSoftwareManager`, `MintAppMetadata` |
| **MintStick USB Flasher** | `src/productivity/mint_competitor.rs` | `SovereignMintStickEngine`, `MintStickMode` |
| **MintMenu Application Menu** | `src/productivity/mint_competitor.rs` | `SovereignMintMenuValaEngine`, `MintMenuItem` |
| **Cinnamon L10n Translations** | `src/productivity/mint_competitor.rs` | `CinnamonTranslationEngine` (GNU gettext parity) |

***

## 3. Implementation Verification & Test Strategy

All Linux Mint parity components are continuously verified through the automated test runner `./run_sigma_tests.sh` and dedicated unit tests in `src/compatibility/mint_linux.rs` and `src/productivity/mint_competitor.rs`:

*   `test_mint_update_manager`: Verifies level 1-5 update classification and package staging.
*   `test_mint_timeshift_restore_points`: Verifies Btrfs/RSYNC snapshot creation and point-in-time rollback.
*   `test_mint_software_manager_with_reviews`: Verifies application catalog queries and safety ratings.
*   `test_sovereign_mintstick_engine`: Verifies bootable ISO block streaming and USB drive formatting.
*   `test_nvidia_prime_applet`: Verifies PRIME render offload environment generation and dynamic GPU power state switching.
*   `test_mintmenu_vala_engine`: Verifies fuzzy application search, category filtering, and favorite toggles.

***

## 4. Quality Gate Integration

To ensure production readiness, all modifications must satisfy the SigmaOS Quality Gate script:

```bash
./scripts/sigma_quality_check.sh
./run_sigma_tests.sh
```

These scripts confirm:

*   0 open TODO/stub markers
*   0 critical security alerts
*   100% test pass rate across 212 core algorithm tests, 11 inspection tests, 2 compatibility tests, and hardware harness tests
*   1:1 synchronization between `wiki/` and `wiki_repo/`
