# Wiki: Omarchy Linux Desktop Parity & Distro Gap Closure Roadmap

## Abstract
This Wiki roadmap documents the clean-room implementation strategy and PR proposal specs for closing the gap between SigmaOS and modern desktop Linux distributions like Omarchy Linux.

## Roadmap Phases

### Phase 1: Compositor & HUD Parity
- Implement `OmarchyHyprlandCompositorLayoutEngineV75` for tiling layout handling.
- Implement `OmarchyWaybarStatusHudEngineV75` for status bar module synchronization.

### Phase 2: Terminal & Search Launchers
- Implement `OmarchyGhosttyPtyTerminalEngineV75` with CRT shader effects and GPU acceleration.
- Implement `OmarchyWalkerLauncherEngineV75` for fuzzy launcher searching and SSH execution.

### Phase 3: Dynamic Theme & Wallpaper Sync
- Implement `OmarchyWallustThemeSyncEngineV75` for LAB color clustering and GTK/Qt/Ghostty theme generation.
- Implement `OmarchyWallustChordThemeNavigatorEngineV75` for chord shortcut execution.

### Phase 4: Biometric Security & Priority Scheduler
- Implement `OmarchyHyprlockPamSecurityEngineV75` for PAM biometric locking.
- Implement `OmarchyAnanicyGameModeEngineV75` for auto-nice process priority adjustments.
- Implement `OmarchyFailClosedSudoPrivilegeShieldEngineV75` for fail-closed security enforcement.

### Phase 5: AI Workflows & Overlay HUD
- Implement `OmarchyHerdrAiSchedulerEngineV75` for local AI task orchestration and Quickshell overlays.

## Testing & Verification
- All phases are verified via `tests/test_omarchy_linux_v75_pr.rs` and `./run_sigma_tests.sh`.
