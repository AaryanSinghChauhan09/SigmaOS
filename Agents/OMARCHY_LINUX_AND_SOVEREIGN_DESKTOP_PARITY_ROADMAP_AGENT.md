# OMARCHY LINUX AND SOVEREIGN DESKTOP PARITY ROADMAP AGENT DIRECTIVE

## Directive Purpose
This agent directive governs autonomous AI agents responsible for developing, maintaining, and verifying feature parity between **SigmaOS** and **Omarchy Linux** (including Wayland compositing, terminal user experience, dynamic Omakase themes, Walker launcher, Hyprlock/Hypridle security, and Herdr local AI task scheduling).

---

## Standard Operating Procedures for AI Agents

1. **Clean-Room Implementation Rule:**
   - All desktop components, compositing logic, and launcher engines must be implemented in clean-room `#![no_std]` or standard Rust code within `src/desktop/`, `src/theming/`, and `src/distro/omarchy_linux_missing_components_v60_pr.rs`.
   - Never copy external proprietary source code.

2. **PR Format Rule:**
   - All proposed additions must be formatted with structured PR proposals (`OmarchyPrEngineResult`) containing explicit `pr_id`, `title`, `component_name`, `status`, and `details`.

3. **Zero-Allocation Hot Paths:**
   - Search routines (such as `WalkerLauncher::search` or `contains_ignore_case`) must run zero-allocation slice comparisons to preserve sub-millisecond execution speeds.

4. **Testing & Verification Requirement:**
   - Each module must include standalone `#[cfg(test)]` unit tests.
   - Run compilation and test execution via:
     ```bash
     mkdir -p build && rustc --test --edition=2021 src/distro/omarchy_linux_missing_components_v60_pr.rs -o build/omarchy_v60_test && ./build/omarchy_v60_test
     ```

---

## Target Subsystems & Components

- **GPU Terminal:** `Ghostty` & `Alacritty` PTY parser with Sixel graphics acceleration.
- **Wayland Compositor:** `Hyprland` master-stack layout engine & touchpad gesture handler.
- **Application Launcher:** `Walker` fuzzy launcher with zero-allocation ASCII matching.
- **Dynamic Theming:** `Wallust` wallpaper palette extraction & Omakase preset synchronizer.
- **Desktop Security:** `Hyprlock` PAM biometric lockscreen & `Hypridle` multi-tier idle governor.
- **AI Task Orchestration:** `Herdr` local LLM task scheduler & Ollama REST router.
- **Privilege Elevation:** Fail-Closed PTY-isolated graphical sudo shield.
