---
name: zenith-gui
description: Control Zenith Desktop GUI apps via CLI — layout, theme, settings, files, browser, dashboard.
---

# zenith-gui

Maps every Zenith Desktop GUI operation to `sigma-agent gui` / `sigma_zenith_cli_exec()` commands.
Use this skill when the user wants to do something they would normally click in the GUI.

## When to use

- Change layout, theme, or workspace
- Launch or close apps (settings, files, browser, terminal, dashboard)
- Read/write settings (language, accessibility, network)
- Semantic file search, tree view, open files
- Browser navigation
- System dashboard metrics and AI diagnosis

## CLI (preferred)

```bash
sigma-agent gui status
sigma-agent gui layout tile
sigma-agent gui theme cyber
sigma-agent gui launch settings
sigma-agent gui settings set ui.theme obsidian
sigma-agent gui files search "architecture docs"
sigma-agent gui browser open https://sigmaos.local
sigma-agent gui dashboard query "why is CPU high"
```

## Kernel API

```c
sigma_zenith_cli_exec("launch settings", output, sizeof(output));
sigma_zenith_settings_set("ui.theme", "cyber", status, sizeof(status));
sigma_zenith_app_launch(SIGMA_ZENITH_APP_FILES, status, sizeof(status));
```

## Natural language (via sigma-agent chat)

```bash
sigma-agent chat "switch to tile layout and open zenith settings"
sigma-agent chat "open browser to sigmaos.local"
sigma-agent chat "find architecture documentation"
```

## GUI ↔ CLI mapping

| GUI action | CLI command |
|------------|-------------|
| Applications → Settings | `gui launch settings` |
| Files semantic search | `gui files search "<query>"` |
| Panel → workspace N | `gui workspace N` |
| Settings → Theme | `gui theme obsidian\|cyber\|paper` |
| WM → Tile layout | `gui layout tile` |
| Browser URL bar | `gui browser open <url>` |
| Dashboard AI query | `gui dashboard query "<prompt>"` |

Pairs with `computer-use` for accessibility-tree fallback when CLI mapping is insufficient.
