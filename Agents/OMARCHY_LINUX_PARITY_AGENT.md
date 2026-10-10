# Omarchy Linux Parity & AI Agent Autonomous Development Directives

## Overview
This document defines AI agent directives and execution guidelines for maintaining 100% feature parity with Omarchy Linux (Omakase Hyprland desktop, Ghostty terminal, Walker launcher, Waybar status HUD, hyprlock screen locker, hypridle power management, and Neovim Omakase studio) in SigmaOS.

## Core Component Mappings

| Omarchy Component | SigmaOS Native Parity Engine | Location |
| :--- | :--- | :--- |
| Hyprland Compositor | `Zenith` DRM/KMS Wayland Compositor | `src/desktop/zenith.rs` |
| 12 Omakase Theme Presets | `OmarchyOmakaseThemeManager` | `src/distro/omarchy_omakase_ultimate_parity.rs` |
| Walker / fuzzel Launcher | `OmarchyCommandPalette` (ASCII Zero-Heap Matcher) | `src/tools/omarchy_command_palette.rs` |
| Ghostty / Fish Shell | `IntegratedTerminal` & `OmarchyShell` | `src/productivity/terminal.rs` |
| hyprlock / hypridle | `OmarchyHyprlockScreenLocker` & `OmarchyHypridleEngine` | `src/distro/omarchy_expanded_parity.rs` |
| Neovim Omakase Studio | `OmarchyNeovimPresetStudioEngine` | `src/distro/omarchy_inspiration.rs` |
| PKGBUILD / AUR / Omakub | `SovereignOmarchyPrProposalMasterSuite` | `src/package/omarchy_pr_proposal_engine.rs` |

## Autonomous Agent Execution Guidelines
1. **Zero External Dependencies:** Implement all Omarchy desktop features in pure `#![no_std]` / `std` Rust without binding to C++ or heavy external libraries.
2. **Sub-Millisecond Responsiveness:** Guarantee launcher queries, theme switches, and shell prompt renders execute in $< 1\text{ ms}$.
3. **Pull Request Proposal Format:** Export all package transpilation, theme presets, and Omakub recipe conversions using `format_as_pull_request_submission`.
