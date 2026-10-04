# Desktop

**Capability state: Prototype.** SigmaOS has no verified boot-to-graphical-session path. This page is the canonical home for desktop status, UX references, validation, and the desktop roadmap. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The repository includes desktop-related configuration and UI models, including a command-palette/launcher model. These are not an integrated desktop session: there is no verified compositor-to-display path, login flow, window management runtime, or supported desktop application set. Proposed names, shortcuts, settings, and shell commands in older plans are not available SigmaOS commands.

The launcher model supports in-memory filtering/ranking and has standalone tests. It does not launch applications or connect to a running desktop. See [Software Store and Command Palette](Software-Store-and-Command-Palette.md) for its specific status.

`src/desktop/shortcuts.rs` contains a shortcut manager model, but the main branch does not export it from `src/desktop/mod.rs` or connect it to compositor input. Its shortcut list is therefore not a live desktop feature. `Super+K` for shortcut help and `Super+Space` for the launcher are design targets; they require a graphical event path before they can be described as integrated.

## Design references

- **Omarchy:** its [hotkey guide](https://github.com/basecamp/omarchy/blob/quattro/manual/07-hotkeys.md) documents a menu at `Super+Space` and shortcut help at `Super+K`; the [navigation guide](https://github.com/omacom/omarchy/blob/quattro/manual/04-navigation.md) makes the menu a central entry point.
- **Linux Mint:** its [user documentation](https://linuxmint.com/documentation.php) and desktop settings make application discovery and shortcut configuration visible to users.
- **Arch Linux:** explicit configuration and concise, task-focused documentation.

These are design references, not SigmaOS capability claims.

## Validation

The standalone shortcut model can be checked with:

```sh
rustc --edition=2021 --test src/desktop/shortcuts.rs -o /tmp/sigmaos_shortcuts_tests
/tmp/sigmaos_shortcuts_tests
```

This checks in-memory matching and search behavior only. No graphical-session or compositor-input integration test has passed.

## Roadmap

1. Establish a real display, input, and session path on the supported QEMU profile.
2. Export the shortcut manager through the desktop module, keep modifier matching exact, and provide searchable shortcut help. Connect it to compositor input only when a real session path exists.
3. Add a keyboard-operable launcher connected to actual installed applications; report unavailable apps clearly.
4. Build first-session setup for display, input, networking, help, and recovery.
5. Add accessible window management, settings, notifications, and power controls only as backed services become available.
6. Validate clean install, keyboard-only use, screen scaling, session failure, and recovery before calling the desktop supported.

**Completion evidence:** a clean installation reaches a usable graphical session, core workflows work with keyboard and pointer, accessibility behavior is checked, and failure/recovery steps are reproducible.

## Related pages

- [Boot and installation](01-Installation.md)
- [Packaging and updates](09-Packaging.md)
- [Audio and graphics](Audio-and-Graphics.md)
- [Software Store and Command Palette](Software-Store-and-Command-Palette.md)
