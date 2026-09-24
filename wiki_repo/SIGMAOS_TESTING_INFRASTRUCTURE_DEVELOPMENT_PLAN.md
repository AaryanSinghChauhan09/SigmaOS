# 🧪 SigmaOS Non-Graphical Testing Infrastructure (`testing_framework`) Strategic Development Plan

## Executive Summary & Design Vision

Testing infrastructure in modern operating systems provides reliable, reproducible, and granular verification across kernel syscalls, userland CLI tools, shell utilities, and desktop compositing models. Crucially, non-graphical test runners must execute flawlessly across diverse environments—including headless CI sandboxes without a Wayland/X11 compositor or GPU display.

Drawing inspiration from open-source test runners (**Omarchy Test Suite**, **FreeBSD `kyua` / `atf`**, **Linux Test Project `LTP`**, **Test Anything Protocol `TAP`**), the **SigmaOS Non-Graphical Testing Framework** (`scripts/verify.sh`, `./run_sigma_tests.sh`, `AGENTS_TESTS_MANAGEMENT.md`, `InspectionTestSuiteRunner`) provides a modular, TAP-compliant, headless-aware test execution pipeline in Native Safe Rust and POSIX Bash scripts.

---

## 1. Suite Map & Test Ownership Architecture

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                           `./run_sigma_tests.sh`                          │
│                   Master Non-Graphical Orchestrator                      │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│       `./test/all` (Runs both CLI and Shell Suites; Exits Non-Zero)       │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
               ┌──────────────────────┴──────────────────────┐
               ▼                                             ▼
┌───────────────────────────────┐             ┌─────────────────────────────┐
│          `./test/cli`         │             │        `./test/shell`       │
│  - Owns CLI Router & Help     │             │ - Runs `test/shell.d/*.sh`  │
│  - Group rendering & aliases  │             │ - Single area per suite file│
│  - Trailing `--help` guarantee│             │ - Dynamic auto-discovery    │
│  - Metadata header lint       │             │ - Modular plugin unit tests │
│  - Theme pipeline & sync stub │             │ - Migration idempotence     │
└───────────────────────────────┘             └─────────────────────────────┘
                                                             │
                                                             ▼
                                              ┌─────────────────────────────┐
                                              │      `base-test.sh`         │
                                              │ - TAP Protocol Assertions   │
                                              │ - Repo Root (`$ROOT`) Auto  │
                                              │ - `require_compositor` probe│
                                              │ - `run_node_test` JS Bridge │
                                              └─────────────────────────────┘
```

### 1.1 `./test/cli` Subsystem
- **Scope**: Owns the CLI command router (help screens, subgroup rendering, subcommand resolution, alias expansion, hidden commands, and the strict invariant that trailing `--help` never executes mutating commands).
- **Metadata Linter**: Scans all `bin/` executables for `# omarchy:summary=` metadata headers, verifying required fields and flagging deprecated flags.
- **Theme Pipeline**: Validates template color rendering (`omarchy-theme-set-templates`, `omarchy-theme-color`) against stub binaries and isolated fake `$HOME` directories.

### 1.2 `./test/shell` Subsystem (`test/shell.d/`)
- **Scope**: Auto-discovers and executes every `test/shell.d/*-test.sh` script (excluding `base-test.sh`).
- **Isolation**: Each file is an independent suite covering a dedicated component (e.g., shell plugins, CLI commands, config invariants, or active system migrations).
- **Shared Fixtures**: Reusable sample data and mock inputs live under `test/shell.d/fixtures/`.

---

## 2. The `base-test.sh` Contract & TAP Protocol

Every shell test script initiates with a standard prelude establishing repo root resolution and TAP assertion primitives:

```bash
#!/bin/bash
set -euo pipefail
source "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/base-test.sh"
```

### 2.1 TAP Assertion Primitives
- **`pass "description"`**: Prints `ok - description`.
- **`skip "description"`**: Prints `ok - description # SKIP`. Includes rationale for unexecuted checks. Returns exit status 0.
- **`fail "description" [detail]`**: Prints `not ok - description` to stderr with optional details, and **immediately exits the test file**. Early exits prevent cascading invalid assertion errors.
- **`require_command <cmd>`**: Verifies binary availability on `$PATH`, calling `skip` or `fail` if missing.

### 2.2 Granularity & Failure Isolation
- `./test/shell` runner continues executing remaining files even when an individual test file fails.
- Failure granularity is per-file within a run, and per-assertion within a file.
- Unfailed runs containing skips are reported as completed without failures, distinguishing unreached coverage from full pass assertions.

---

## 3. Compositor-Dependent Probing (`require_compositor`)

Tests requiring Quickshell or Wayland compositing (`Hyprland`, `Zenith`) must execute safely in headless CI sandboxes without core-dumping or failing the build.

### 3.1 `compositor_reachable` Socket Probe
- Checks `$WAYLAND_DISPLAY` and verifies the actual Unix domain socket exists under `$XDG_RUNTIME_DIR`.
- Probes compositor responsiveness via IPC (`hyprctl -j monitors` or `zenith-ctl status` with timeout retries).
- If unreachable, calls `skip "no active compositor"` and exits 0 cleanly.

### 3.2 Core Dump Debris Guard
- When a compositor is reachable, `require_compositor` sets `ulimit -c 0` to prevent core dump file debris if Quickshell or a client process aborts mid-run.

---

## 4. Node.js JavaScript Unit Testing Bridge (`run_node_test`)

Quickshell model plugins (`shell/plugins/menu/MenuModel.js`, `bar/BarModel.js`) maintain dual compatibility: QML imports them natively, while Node.js loads them as CommonJS modules via `module.exports`.

`run_node_test` bridges Bash and Node.js testing:

```bash
run_node_test <<'JS'
const menu = requireFromRoot('shell/plugins/menu/MenuModel.js');
const parsed = menu.parseMenuJsonc('{ "items": { "root": { "label": "Go" } } }');
assertEqual(parsed.length, 1, 'menu parses JSONC with trailing commas');
JS
```

The JS prelude provides TAP-compatible assertion helpers (`pass`, `fail`, `assert`, `assertEqual`, `assertDeepEqual`), `$ROOT` resolution, and `requireFromRoot(relativePath)`.

---

## 5. Key Testing Conventions

1. **Background Process Isolation**: Redirect background fixture output (`> /dev/null 2>&1 &`) and install `trap 'kill $(jobs -p) 2>/dev/null || true' EXIT` to prevent hanging output pipes.
2. **Stub Execution**: Build temporary `bin/` scratch directories containing stub binaries (`sudo`, `tmux`, `gsettings`) that record invocations to call log files. Prepend scratch `bin/` to `$PATH`.
3. **Isolated `$HOME` Environment**: Point `HOME` at a temporary directory (`mktemp -d`) and set `OMARCHY_PATH="$ROOT"` so test state never mutates the developer machine.
4. **Migration Idempotence**: Test migrations against fake `$HOME` state, verifying two consecutive executions leave user customizations intact and produce identical results.
5. **Invariant Assertions**: Pin exact structural invariants (e.g., widget adjacency) rather than full snapshot comparisons to avoid fragile churn failures.

---

## 6. Phased Implementation Roadmap

### Phase 1: TAP Assertion Library & `base-test.sh` Contract (Q4 2026)
- Standardize `base-test.sh` TAP assertion functions (`pass`, `skip`, `fail`, `require_command`) across all shell tests.
- Implement repo root `$ROOT` auto-discovery.

### Phase 2: Headless Compositor Probe & `run_node_test` JS Bridge (Q1 2027)
- Deploy `compositor_reachable` socket probe and `require_compositor` guard.
- Build `run_node_test` Node.js CommonJS testing bridge for QML model logic.

### Phase 3: CLI Router Linter & Theme Test Suite (Q2 2027)
- Build `./test/cli` metadata linter for `# omarchy:summary=` headers.
- Implement stub binary environment for theme pipeline and sync testing.

### Phase 4: Migration Idempotence & CI Integration (Q3 2027+)
- Implement isolated `$HOME` migration test runner verifying idempotence.
- Integrate non-graphical test runner suite into `./scripts/verify.sh` and GitHub Actions CI matrices.

---

## 7. Verification Standard

Execute the non-graphical test suite runner:
```bash
./scripts/verify.sh
```
