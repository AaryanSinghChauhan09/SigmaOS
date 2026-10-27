# PR Specification: Omarchy Linux & SigmaOS Desktop Gap Closure

## Abstract
This proposal specifies the clean-room implementation of 10 core Omarchy Linux desktop and userland subsystems within SigmaOS (`src/distro/omarchy_linux_missing_components_v75_pr.rs`). It bridges the feature gap between SigmaOS and modern Linux desktop distributions by delivering automated tiling, status HUD, GPU terminal, fuzzy launching, theme sync, biometric locking, process auto-niceness, AI task scheduling, fail-closed privilege protection, and chord hotkeys.

## Detailed Architecture

### Subsystems Implemented
1. **`OmarchyHyprlandCompositorLayoutEngineV75`**:
   - Tiling layouts (`dwindle`, `master`), workspace swipe gestures, animation curves, and window rules.
2. **`OmarchyWaybarStatusHudEngineV75`**:
   - Status bar module orchestration with live CSS reload and IPC event handling.
3. **`OmarchyGhosttyPtyTerminalEngineV75`**:
   - GPU PTY terminal emulator configuration, font rendering, custom CRT shader pipeline.
4. **`OmarchyWalkerLauncherEngineV75`**:
   - Fuzzy application search, emoji finder, calculator, SSH host launcher, cliphist clipboard daemon integration.
5. **`OmarchyWallustThemeSyncEngineV75`**:
   - Wallpaper color extraction and dynamic palette generation for GTK, Qt, and terminal windows.
6. **`OmarchyHyprlockPamSecurityEngineV75`**:
   - PAM module authentication, fingerprint biometrics, idle power governor.
7. **`OmarchyAnanicyGameModeEngineV75`**:
   - eBPF-assisted auto-nice process priority tuner and gaming mode trigger.
8. **`OmarchyHerdrAiSchedulerEngineV75`**:
   - Local AI workflow orchestration and Quickshell QML overlay rendering.
9. **`OmarchyFailClosedSudoPrivilegeShieldEngineV75`**:
   - Fail-closed privilege escalation guard, input sanitization, and Polkit policy gate.
10. **`OmarchyWallustChordThemeNavigatorEngineV75`**:
    - Chord keybinding shortcuts for theme cycling and desktop state management.

## Verification
- Unit test suite: `tests/test_omarchy_linux_v75_pr.rs`
- Master test runner: `./run_sigma_tests.sh`
- Expected Parity Score: 100%
