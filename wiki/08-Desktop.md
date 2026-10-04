# Desktop and Keyboard Workflows

**Capability state: Prototype.** `main` has testable compositor, launcher, shortcut, workspace, capture, and settings models, but no verified boot-to-graphical-session path. No launcher action, scratchpad, audio control, screenshot, or power profile is a supported system operation until a runtime backend is connected and tested.

## Current capability on `main`

- `src/desktop/shortcuts.rs` provides exact modifier matching, searchable shortcut listings, checked user registration, and atomic rebinding. `src/desktop/mod.rs` exports the manager.
- `src/compositor/zenith_core.rs` owns a default shortcut manager and accepts layout-translated key events through `dispatch_translated_shortcut`. Raw Wayland keycodes still require a real keyboard-layout translator; the compositor has no verified display/input session or action UI.
- `src/desktop/launcher.rs` filters applications, windows, clipboard items, calculator queries, and built-in system action entries. System actions use typed IDs instead of shell strings, remain unavailable until a service reports support, and reboot/power-off require explicit confirmation. The action handler is an interface; no host command runner or SigmaOS service integration is supplied.
- `src/tools/omarchy_command_palette.rs` is a compatibility re-export of the desktop palette, so there is one command catalog implementation.
- `src/desktop/omarchy_dynamic_workspace_suite.rs` includes pure layout calculations. Audio and power start unknown/empty; screenshot, OCR, and scratchpad requests fail or remain unavailable without real device/window backends.
- `src/desktop/onboarding_wizard.rs` keeps codec installation, theme/layout application, and developer setup unavailable until their package and settings backends exist.

The feature inventory should distinguish these prototypes from a working desktop. Unit tests validate models and boundaries; they do not establish a bootable graphical session, installed application launch, or hardware control.

## Omarchy design references

Omarchy makes the menu, shortcut help, common controls, and CLI discoverable through one keyboard-centered workflow. SigmaOS may adopt the navigation pattern while keeping each action tied to a typed service and visible availability state. See the [Omarchy hotkey guide](https://omarchy.org/manual/hotkeys/), [navigation guide](https://omarchy.org/manual/navigation/), [CLI manual](https://omarchy.org/manual/omarchy-cli/), and [top bar guide](https://omarchy.org/manual/the-top-bar/).

## Validation

```sh
rustc --edition=2021 --test src/desktop/launcher.rs -o /tmp/sigmaos_launcher_tests
/tmp/sigmaos_launcher_tests
rustc --edition=2021 --test src/desktop/shortcuts.rs -o /tmp/sigmaos_shortcut_tests
/tmp/sigmaos_shortcut_tests
rustc --edition=2021 --test src/desktop/omarchy_dynamic_workspace_suite.rs -o /tmp/sigmaos_workspace_tests
/tmp/sigmaos_workspace_tests
cargo check --lib
```

These checks cover in-memory filtering, typed action gating, shortcut matching/rebinding, workspace geometry, and fail-closed behavior. They do not validate QEMU graphics, keyboard layout translation, session startup, or physical controls.

## Roadmap and acceptance gates

1. **Boot and session:** build the kernel for the intended target, boot to a shell in QEMU, and establish display/input/session startup. Until this passes, desktop features remain prototypes.
2. **Keyboard path:** integrate a real layout-aware input translator; test press/release, modifier state, focus, key repeat, and keyboard-only navigation. `Super+Space` and `Super+K` must open actual menu/help surfaces.
3. **Single command catalog:** connect app discovery to installed application metadata and typed action handlers. Unsupported actions stay disabled with a reason. Test missing apps, denied permissions, service failure, and confirmation for destructive actions; never execute palette strings through a shell.
4. **Workspaces and controls:** connect tiling, workspace movement, scratchpads, audio, network, display, media, and power to real compositor/service state. Unknown device state must remain unknown. Validate on the named QEMU profile before adding physical hardware.
5. **Developer workflow and configuration:** add reusable workspace profiles, package-backed developer setup, theme preview/apply/rollback, and persisted shortcut changes only after package, settings, and rollback services exist.
6. **Accessibility and recovery:** verify keyboard-only operation, visible shortcut help, focus order, scaling, accessible labels, session restart, and recovery after failed configuration.

**Integrated gate:** each menu item invokes a tested runtime service and reports its real result. **Supported desktop gate:** a clean install boots into a usable session, core workflows pass QEMU integration tests, supported devices are named, and recovery steps work. Module tests alone do not pass either gate.

## Related pages

- [Boot and installation](01-Installation.md)
- [Packaging and updates](09-Packaging.md)
- [Audio and graphics](Audio-and-Graphics.md)
- [Software Store and Command Palette](Software-Store-and-Command-Palette.md)
- [Testing](Testing.md)
