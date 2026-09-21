# 🪝 SigmaOS Event Automation Hooks Subsystem (`automation_hooks`) Strategic Development Plan

## Executive Summary & Design Vision

System event hooks allow users, administrators, and automated services to register custom executable scripts that run automatically when key system events occur (theme changes, font switches, desktop login boot, low battery warnings, system package updates, or pacman refresh cycles).

Drawing direct architectural inspiration from **Omarchy Automation Hooks**, Linux `/etc/dhcp/dhclient-exit-hooks.d/` and ALPM hooks (`/usr/share/libalpm/hooks/`), and FreeBSD `devd.conf` action scripts, the **SigmaOS Automation Hooks Subsystem** (`SigmaHookEngine`, `TransactionalAutomationHook`, `SigmaHookCli`) provides a zero-dependency, modular, directory-based event hook runner written natively in Safe Rust and POSIX Bash.

---

## 1. Directory Structure & Hook Mapping Architecture

Hook scripts reside in `~/.config/sigma/hooks/<name>.d/`—where each event name maps to a dedicated directory containing executable scripts:

```text
~/.config/sigma/hooks/
├── battery-low.d/          # Low battery threshold trigger ($1 = percentage e.g., "15")
├── font-set.d/             # After desktop font switch ($1 = font name e.g., "Inter")
├── post-boot.d/            # After desktop session starts
├── post-update.d/          # During `sigma update`, after packages and migrations
├── pre-refresh-pacman.d/   # Before `sigma refresh pacman` re-syncs repository mirrors
└── theme-set.d/            # After desktop theme change ($1 = theme slug e.g., "tokyo-night")
```

```text
┌───────────────────────────────────────────────────────────────────────────┐
│              CLI Hook Management Tool (`sigma hook install ...`)          │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│              `SigmaHookEngine` Event Trigger & Directory Scanner          │
│     - Checks flat single-file fallback: `~/.config/sigma/hooks/<name>`    │
│     - Scans directory scripts: `~/.config/sigma/hooks/<name>.d/*`         │
│     - Validates executable permissions (`chmod +x`) & non-zero isolation  │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                     Sequential Hook Execution & Arguments                 │
│     - Passes positional parameters (`$1` = theme slug, font name, battery)│
│     - Logs output transcripts to `/tmp/sigma-hooks.log`                    │
└───────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Key Hook Installation & Execution Rules

### 2.1 CLI Installation Tool (`sigma hook install`)
- Users install custom hook scripts using the command:
  ```bash
  sigma hook install <event_name> <script_path>
  ```
  Which copies the script into `~/.config/sigma/hooks/<event_name>.d/` and marks it executable (`chmod +x`).

### 2.2 Execution Order & Fallback Files
- **Flat File First**: If a flat fallback file exists at `~/.config/sigma/hooks/<event_name>`, it executes first.
- **Directory Scripts Second**: All executable scripts in `~/.config/sigma/hooks/<event_name>.d/` execute in alphabetical lexical order.

### 2.3 Positional Arguments & Environment Context
- **`battery-low.d/`**: Receives remaining battery percentage in `$1` (e.g., `15`).
- **`font-set.d/`**: Receives new font name in `$1` (e.g., `JetBrains Mono`).
- **`theme-set.d/`**: Receives new theme slug in `$1` (e.g., `catppuccin`).
- **`post-update.d/`**: Receives updated package count in `$1`.

### 2.4 Error Isolation & Non-Zero Exit Handling
- If a hook script fails (exits non-zero), `SigmaHookEngine` logs the failure to `/tmp/sigma-hooks.log` and continues executing remaining hook scripts, preventing one faulty user script from blocking desktop event processing.

---

## 3. Phased Development Roadmap

### Phase 1: Native Hook Scanner & CLI Management (`sigma hook`) (Q4 2026)
- Build `SigmaHookEngine` directory scanner for `~/.config/sigma/hooks/<name>.d/`.
- Implement `sigma hook install <name> <script>` and `sigma hook list` CLI tools.
- Add support for flat fallback script files (`~/.config/sigma/hooks/<name>`).

### Phase 2: Positional Argument Router & Theme/Font Triggers (Q1 2027)
- Connect desktop theme switcher (`OmarchyThemeStudio`) to trigger `theme-set.d/` with `$1 = theme_slug`.
- Connect font manager to trigger `font-set.d/` with `$1 = font_name`.
- Add battery telemetry daemon trigger for `battery-low.d/`.

### Phase 3: Post-Update & Package Sync Hooks (Q2 2027)
- Wire `sigma update` pipeline to execute `post-update.d/` scripts post-migration.
- Wire package mirror synchronizer to execute `pre-refresh-pacman.d/` scripts.
- Implement `/tmp/sigma-hooks.log` transcript logging.

### Phase 4: Parallel Execution & CI Testing (Q3 2027+)
- Support optional parallel hook execution for non-blocking events.
- Integrate hook execution unit tests into `./scripts/verify.sh`.
- Conduct fuzz testing against hook argument parsers (`cargo fuzz`).

---

## 4. Verification & Testing Standards

All automation hook components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/automation/system_level.rs` (`TransactionalAutomationHook` & event triggers)
- `src/distro/omarchy.rs` (Theme & font switch hook triggers)
- `src/sigpkg/universal_engine.rs` (`post-update.d` & `pre-refresh-pacman.d` hook execution)
- `src/security/input_validation.rs` (hook script path & name sanitization)
