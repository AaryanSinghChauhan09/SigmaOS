---
name: sigmaos
description: Self-tailoring agentic skill for navigating, diagnosing, configuring, and extending the SigmaOS & Omarchy Linux ecosystem.
---

# SigmaOS & Omarchy Agent Skill

This skill instructs autonomous AI coding agents (`claude`, `codex`, `opencode`, `agy`, etc.) how to interact with SigmaOS.

## Core Directives & Conventions

1. **Defensive Storage Operations**:
   - Always run installers and destructive partition tools with `--dry-run` by default.
   - Never write to host root disk (`findmnt -no SOURCE /`).
   - Destructive operations strictly require dual confirmation (`--destructive --confirm-disk <DEV>`).

2. **Shell-First Architecture**:
   - Prefer native shell tools in `bin/` over ad-hoc scripts:
     - `sigmaos-update`: System package, dotfile, and Wallust theme updates.
     - `sigmaos-doctor`: Run 73-point system diagnostics audit.
     - `sigmaos-snapshot`: Manage Btrfs snapshots and subvolume rollbacks.
     - `sigmaos-pkgmgr`: Pacman and AUR operations (`install`, `search`, `clean`, `orphans`).
     - `sigmaos-network`: NetworkManager CLI wrapper (`wifi`, `ethernet`, `vpn`, `dns`).
     - `sigmaos-audio`: PipeWire / WirePlumber audio and volume controller.
     - `sigmaos-bluetooth`: BlueZ device discovery and connection manager.
     - `sigmaos-theme`: Wallust palette switcher with built-in themes.
     - `sigmaos-theme-install`: Community themes installer and previewer.
     - `sigmaos-hyprshot`: Screenshot tool with region/window/output, swappy annotations, and clipboard.
     - `sigmaos-tpm`: TPM2 security manager, SecureBoot status, and LUKS auto-unlock.
     - `sigmaos-dev-stack`: Provision developer runtime stacks (Mise, Docker, Rails, Laravel, Node.js).
     - `sigmaos-default-editor`: System-wide editor switcher (`nvim`, `code`, `cursor`, `zed`, `helix`, etc.).
     - `sigmaos-default-terminal`: Default terminal switcher (`foot`, `alacritty`, `ghostty`, `kitty`).
     - `sigmaos-crash-diagnostics`: Inspect `systemd-coredump` traces.
     - `sigmaos-local-ai`: Manage Ollama and LM Studio local GPU inference.
     - `omarchy`: Universal 100% drop-in command dispatcher for all sovereign tooling.

3. **Desktop Session & Compositor**:
   - Compositor: Hyprland with dynamic tiling, fractional scaling, and overshot bezier animations.
   - Shell Panels: Quickshell (QML reactive shell) with Waybar fallback.
   - Config directory: `~/.config/hypr`, `~/.config/quickshell`, `~/.config/waybar`, `~/.config/wallust`.

4. **Self-Healing & Diagnostics**:
   - When a crash occurs, query `sigmaos-crash-diagnostics latest` to extract stack traces.
   - Re-run `./run.sh --doctor` to verify system health.
