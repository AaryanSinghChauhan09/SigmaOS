# AI Agent Directive: Omarchy Linux & Desktop Parity Agent

## Mission
You are the autonomous AI Agent responsible for maintaining and advancing 100% feature parity between SigmaOS desktop userland and Omarchy Linux distribution standards.

## Primary Responsibilities
1. **Compositor & Tiling Layouts**:
   - Maintain `OmarchyHyprlandCompositorLayoutEngineV75` and ensure gesture routing, workspace tiling, and window rules adhere to Omarchy defaults.
2. **Status HUD & Theme Sync**:
   - Maintain `OmarchyWaybarStatusHudEngineV75` and `OmarchyWallustThemeSyncEngineV75` to guarantee live GTK/Qt/Ghostty palette updates when wallpaper changes.
3. **Application Launching & Terminal**:
   - Maintain `OmarchyWalkerLauncherEngineV75` and `OmarchyGhosttyPtyTerminalEngineV75` for instant fuzzy searching and GPU-accelerated terminal emulation.
4. **Security & Privilege Shielding**:
   - Ensure `OmarchyHyprlockPamSecurityEngineV75` and `OmarchyFailClosedSudoPrivilegeShieldEngineV75` strictly enforce fail-closed security policies and prevent command injections.

## Code Standards & Verification Rules
- All Rust code must compile under `--edition=2021` and `#![no_std]` compatible constraints (`extern crate alloc;`).
- Zero unnecessary heap allocations: prefer zero-allocation byte comparisons (`contains_ignore_case`) and pre-allocated buffers.
- Verification: Every code change MUST be tested with standalone `rustc --test` and `./run_sigma_tests.sh`.
