# Implementation Plan: SigmaOS Full-Platform Roadmap

## Overview

This plan converts the SigmaOS roadmap design into incremental coding tasks. Implementation follows the phase dependency graph: Phase 0 kernel foundations must land first, followed by Phase 1 shell polish, then Phase 2 app completions, Phase 3 platform features, Phase 4 ecosystem, and finally the bug-fix batches ordered by severity. Languages used: C/C++ for kernel and system modules, Go for daemons, JavaScript/TypeScript for web shell and applications, with property-based tests using theft/rapidcheck (C), fast-check (JS), and gopter (Go).

---

## Tasks

- [ ] 1. Phase 0 — Kernel Stability: Foundations
  - [ ] 1.1 Implement IDT initialization (`sigma_idt.h` / `sigma_idt.cpp`)
    - Define `idt_entry_t` packed struct (offset_low, selector, IST, type_attr, offset_mid, offset_high)
    - Implement `sigma_idt_init()` populating 32 exception entries
    - Write ISR stub assembly for vectors 0–31 (`arch/x86_64/isr_stubs.asm`)
    - Call `sigma_idt_load()` to flush descriptor to CPU; emit serial diagnostic and halt on failure
    - _Requirements: 1.1, 1.2, 1.3, 1.4_

  - [ ] 1.2 Write property test for ISR handler invocation (Property 1)
    - **Property 1: ISR Handler Invocation for All Exception Vectors**
    - **Validates: Requirement 1.3**
    - Use `theft` to generate exception vectors 0–31 and verify corresponding handler is registered
    - _Test file: `tests/unit/kernel/idt_isr_property_test.c`_

  - [ ] 1.3 Implement Ring 3 usermode transition (`sigma_usermode.h` / `sigma_usermode.cpp`)
    - Define `tss_t` packed struct with `rsp0` kernel stack pointer
    - Implement `sigma_tss_init(uint64_t kernel_stack)` to configure TSS GDT descriptor
    - Write `jump_to_usermode(entry_point, user_stack)` assembly routine using `iret`
    - Allocate per-process kernel stack and update TSS on each process creation
    - Register GP fault ISR to handle `iret` errors; log diagnostic
    - _Requirements: 2.1, 2.2, 2.3, 2.4_

  - [ ] 1.4 Write property test for per-process TSS stack isolation (Property 2)
    - **Property 2: Per-Process TSS Kernel Stack Isolation**
    - **Validates: Requirement 2.3**
    - Use `theft` to generate 10–100 mock process structures and assert all `rsp0` values are unique
    - _Test file: `tests/unit/kernel/tss_isolation_property_test.c`_

  - [ ] 1.5 Implement CryptFS key derivation and root mount (`sigma_cryptfs.cpp`)
    - Implement `sigma_cryptfs_derive_key()` using PBKDF2 (≥100,000 iterations, AES-256)
    - Write full 32-byte derived key buffer before returning; zero buffer and return error on failure
    - Wire `sigma_cryptfs_mount_root()` to dm-crypt block-device decryption
    - Emit serial error and halt if decryption key is invalid (no unauthenticated mount)
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 37.1, 37.2, 37.3, 37.4_

  - [ ] 1.6 Write property test for CryptFS key derivation determinism (Property 17)
    - **Property 17: CryptFS Key Derivation Determinism**
    - **Validates: Requirement 37.4**
    - Use `theft` to generate passphrases (1–256 chars) and salts (16–64 bytes); call `sigma_cryptfs_derive_key()` twice per pair and assert identical 256-bit outputs
    - _Test file: `tests/unit/kernel/cryptfs_key_determinism_property_test.c`_

  - [ ] 1.7 Implement real PCI bus enumeration replacing `fake_dev` stub (`linux_shim.cpp` / `pci_scanner.h`)
    - Implement `pci_read_config(bus, slot, func, offset)` using MMIO CONFIG_ADDRESS/CONFIG_DATA I/O ports
    - Iterate buses 0–255, devices 0–31, functions 0–7; skip slots with vendor ID `0xFFFF`
    - Populate `pci_device_t` with vendor_id, device_id, class_code, subclass, and BAR[0–5]
    - Replace `fake_dev` stub call with `pci_scan_devices(devices, max)`
    - _Requirements: 4.1, 4.2, 4.3, 4.4_

  - [ ] 1.8 Write property tests for PCI scanner (Properties 3 & 4)
    - **Property 3: PCI Device Field Capture Completeness** — Validates: Requirement 4.3
    - **Property 4: Empty PCI Slot Exclusion** — Validates: Requirement 4.4
    - Use `theft` to generate mock PCI config tables; assert field capture and empty-slot filtering
    - _Test file: `tests/unit/kernel/pci_scanner_property_test.c`_

- [ ] 2. Checkpoint — Phase 0 kernel tests pass
  - Ensure all kernel unit tests pass, `nm` shows no glibc symbols, ask the user if questions arise.

- [ ] 3. Phase 1 — Shell Polish: Window Manager
  - [ ] 3.1 Implement `WindowManager` class with window registry (`webshell/windowManager.js`)
    - Create `WindowManager` with `Map<id, WindowConfig>` registry and `zIndexCounter`
    - Implement `createWindow(config)` rendering in-page `<div>` elements (no `window.open()`)
    - Implement `focusWindow(id)` incrementing z-index and tracking `focusedWindow`
    - Implement `closeWindow(id)` removing element and updating taskbar
    - _Requirements: 5.1, 5.5_

  - [ ] 3.2 Implement drag and resize handlers with minimum size constraint
    - Add `enableDrag(windowId)` binding `mousedown` on title bar, `mousemove` and `mouseup` on document
    - Track pointer offset so window follows cursor without jumping
    - Add `enableResize(windowId)` binding resize handle; constrain to min 200×150 px
    - _Requirements: 5.2, 5.3_

  - [ ] 3.3 Write property tests for window drag and resize (Properties 5 & 6)
    - **Property 5: Window Drag Position Translation** — Validates: Requirement 5.2
    - **Property 6: Window Resize Minimum Constraints** — Validates: Requirement 5.3
    - Use `fast-check` to generate drag deltas in [-1000, 1000] and resize dimensions including (0×0, 199×149, 200×150)
    - _Test file: `tests/unit/webshell/windowManager.property.test.js`_

  - [ ] 3.4 Implement persistent taskbar with focus management (`webshell/taskbar.js`)
    - Render taskbar `<div>` pinned at bottom of viewport
    - Update taskbar entry list on window create/close
    - Click handler calls `focusWindow(id)` and updates active highlight
    - _Requirements: 5.4, 5.5_

  - [ ] 3.5 Write unit tests for WindowManager
    - Test window create, focus, close, taskbar sync, and z-index ordering
    - _Test file: `tests/unit/webshell/windowManager.test.js`_

- [ ] 4. Phase 1 — Shell Polish: SigmaNotes, SigmaCode, Notification Center, Lock Screen
  - [ ] 4.1 Implement SigmaNotes split-pane Markdown preview (`apps/sigmaNotes.js`)
    - Create two-pane layout (editable source + read-only preview) with CSS `display:grid`
    - Integrate `marked.js` for Markdown-to-HTML rendering in preview pane
    - Add 500ms debounce on source `input` event calling `renderMarkdown()`
    - Implement formatting toolbar: bold (`**`), italic (`_`), heading (`#`), link (`[]()`); insert at cursor position
    - _Requirements: 6.1, 6.2, 6.3, 6.4_

  - [ ] 4.2 Write unit tests for SigmaNotes
    - Test debounce timing (≤500ms), toolbar insertions, cursor position, Markdown render correctness
    - _Test file: `tests/unit/apps/sigmaNotes.test.js`_

  - [ ] 4.3 Implement SigmaCode IDE with Monaco/CodeMirror, filesystem access, and process spawn (`apps/sigmaCode.js`)
    - Initialize Monaco or CodeMirror editor in container with syntax highlighting, line numbers, multi-cursor
    - Implement `openFile(path)` calling `navigator.sigmaos.fs.readFile(path)` and loading content within 1s
    - Implement `saveFile()` writing editor content to filesystem via `navigator.sigmaos.fs.writeFile()`
    - Implement `runFile()` calling `navigator.sigmaos.process.spawn()` and piping stdout/stderr to integrated terminal panel
    - _Requirements: 7.1, 7.2, 7.3, 7.4_

  - [ ] 4.4 Write unit tests for SigmaCode
    - Test open/save round-trip, process spawn stdout display, and error handling for missing files
    - _Test file: `tests/unit/apps/sigmaCode.test.js`_

  - [ ] 4.5 Implement Notification Center with bell icon, queue, and badge counter (`webshell/notificationCenter.js`)
    - Create bell icon in system tray; maintain FIFO `queue` array and `badgeCount`
    - Implement `navigator.sigmaos.notification.show(config)` API pushing to queue and incrementing badge when panel hidden
    - Build slide-out panel listing notifications chronologically; clear badge on open
    - _Requirements: 8.1, 8.2, 8.3, 8.4_

  - [ ] 4.6 Write unit tests for Notification Center
    - Test FIFO ordering, badge increment/reset, and panel toggle behavior
    - _Test file: `tests/unit/webshell/notificationCenter.test.js`_

  - [ ] 4.7 Implement Lock Screen overlay with Ctrl+L, clock, PIN verification, and lockout (`webshell/lockScreen.js`)
    - Bind `keydown` listener for Ctrl+L; render full-screen overlay at maximum z-index
    - Display live date/time updating every 1 second via `setInterval`
    - Implement `verifyCredential(input)` checking PIN; remove overlay on success
    - Track `failedAttempts`; impose 30-second lockout after 3 consecutive failures
    - _Requirements: 9.1, 9.2, 9.3, 9.4_

  - [ ] 4.8 Write unit tests for Lock Screen
    - Test Ctrl+L binding, clock updates, correct PIN unlock, three-strike lockout timing
    - _Test file: `tests/unit/webshell/lockScreen.test.js`_

- [ ] 5. Checkpoint — Phase 1 shell tests pass
  - Ensure all web shell unit tests pass and lint-security passes with no `innerHTML` warnings. Ask the user if questions arise.

- [ ] 6. Phase 2 — App Completions: SigmaTerm PTY
  - [ ] 6.1 Implement Go server-side PTY allocator and WebSocket multiplexer (`sigmad-process/pty_server.go`)
    - Allocate PTY using `github.com/creack/pty`; store `PTYSession{id, pid, masterFd, cols, rows}`
    - Stand up WebSocket endpoint `/pty` via `gorilla/websocket`; forward `master` output to WebSocket
    - Handle `resize` messages: call `pty.Setsize()` with new cols/rows and send SIGWINCH to PTY process
    - _Requirements: 10.1, 10.4_

  - [ ] 6.2 Implement SigmaTerm frontend client with xterm.js (`apps/sigmaTerm.js`)
    - Initialize `xterm.js` terminal and connect `WebSocket('ws://localhost:17393/pty')`
    - Forward key input to WebSocket within 50ms (`sendInput` on `terminal.onData`)
    - Render PTY output to terminal within 100ms of receipt
    - Send `{type:'resize', cols, rows}` JSON message on terminal resize event
    - _Requirements: 10.1, 10.2, 10.3, 10.4_

  - [ ] 6.3 Write unit tests for SigmaTerm
    - Test input forwarding, output rendering, resize signal, and WebSocket reconnection logic
    - _Test file: `tests/unit/apps/sigmaTerm.test.js`_

- [ ] 7. Phase 2 — App Completions: SigmaNotes AI, SigmaPaint Layers, Neural UI, Enterprise Dashboard
  - [ ] 7.1 Implement SigmaNotes AI HTTP client with 5-second timeout (`apps/sigmaNotesAI.js`)
    - Add `NotesAIClient` fetching `POST localhost:17392` with `AbortController` timeout of 5000ms
    - Display inline suggestion in editor on success; show timeout error with retry button on `AbortError`
    - Ensure AI fetch is non-blocking: editor input events never await the fetch directly
    - _Requirements: 11.1, 11.2, 11.3, 11.4_

  - [ ] 7.2 Write unit tests for SigmaNotes AI
    - Test success path, 5-second timeout rejection, retry mechanism, and non-blocking behavior
    - _Test file: `tests/unit/apps/sigmaNotesAI.test.js`_

  - [ ] 7.3 Implement SigmaPaint layers panel and canvas composite engine (`apps/sigmaPaint.js`)
    - Add `layers[]` array; render layers panel UI listing layer name, visibility toggle, drag handle
    - Implement `createLayer()` inserting new layer above `activeLayerIndex` with blank `ImageData`
    - Implement `toggleVisibility(layerId)` flipping `visible` and calling `recomposite()` within 100ms
    - Implement `reorderLayer(from, to)` via drag events; call `recomposite()` immediately after reorder
    - Implement `recomposite()` clearing canvas and blending all visible layers in order
    - _Requirements: 12.1, 12.2, 12.3, 12.4_

  - [ ] 7.4 Write unit tests for SigmaPaint
    - Test layer create/reorder/toggle, recomposite ordering, and 100ms visibility latency
    - _Test file: `tests/unit/apps/sigmaPaint.test.js`_

  - [ ] 7.5 Implement Neural UI Engine with real inference and fallback (`webshell/neuralUIEngine.js`)
    - Implement `predict(uiContext)` issuing `POST localhost:17392/v1/predict`
    - On HTTP 200: apply prediction and cache as `lastPrediction`
    - On non-200: log error code and return `lastPrediction`
    - On network error: set `available = false`, show single non-blocking status indicator
    - _Requirements: 13.1, 13.2, 13.3, 13.4_

  - [ ] 7.6 Write unit tests for Neural UI Engine
    - Test 200 path, non-200 fallback, unreachable endpoint disablement, and status indicator
    - _Test file: `tests/unit/webshell/neuralUIEngine.test.js`_

  - [ ] 7.7 Implement Enterprise Dashboard SSE connection manager and metric widgets (`apps/enterpriseDashboard.js`)
    - Connect `EventSource('/api/telemetry/stream')` on startup; parse and render metric widgets within 200ms per event
    - On `onerror`: show "Disconnected" indicator and schedule reconnect with exponential backoff (1s→30s cap)
    - Reset `reconnectDelay` to 1s on successful reconnection
    - _Requirements: 14.1, 14.2, 14.3, 14.4_

  - [ ] 7.8 Write unit tests for Enterprise Dashboard
    - Test SSE metric update, disconnection indicator, and exponential backoff timing
    - _Test file: `tests/unit/apps/enterpriseDashboard.test.js`_

- [ ] 8. Checkpoint — Phase 2 app tests pass
  - Ensure all Phase 2 unit tests pass and latency assertions are met. Ask the user if questions arise.

- [ ] 9. Phase 3 — Platform Features: Zero-Install Sandbox
  - [ ] 9.1 Implement Go sandbox manager parsing package manifest and building bwrap command (`sigmad-sandbox/main.go`)
    - Define `PackageManifest{Name, Command, Capabilities[]}`; parse from JSON file path argument
    - Build `bwrap` invocation array: `--ro-bind /usr /usr`, `--ro-bind /lib /lib`, `--tmpfs /tmp`, `--unshare-all`, `--die-with-parent`
    - Map capabilities (`network`→`--share-net`, `filesystem.read`→`--ro-bind /home /home`) from manifest
    - Invoke `exec.Command(cmd[0], cmd[1:]...)` and stream stdout/stderr to caller
    - _Requirements: 15.1, 15.2_

  - [ ] 9.2 Add seccomp syscall filtering and capability violation audit logging
    - Generate seccomp profile from manifest capability list using `libseccomp` bindings
    - Deny any syscall not covered by declared capabilities with `EPERM`
    - On capability violation: write structured entry to audit log (event_type, subject, object, action, result)
    - On sandbox exit: release all resources within 500ms (`context.WithTimeout`)
    - _Requirements: 15.2, 15.3, 15.4_

  - [ ] 9.3 Write property tests for sandbox enforcement (Properties 7 & 8)
    - **Property 7: Sandbox Syscall Enforcement** — Validates: Requirement 15.2
    - **Property 8: Capability Violation Audit Logging** — Validates: Requirement 15.3
    - Use `gopter` to generate syscall lists and manifests; assert denial and log entry creation
    - _Test file: `tests/unit/kernel/sandbox_enforcement_property_test.go`_

  - [ ] 9.4 Write integration tests for zero-install sandbox
    - Test with manifests requesting no capabilities, network-only, and filesystem.read
    - Verify untrusted syscalls are blocked and violations are logged
    - _Test file: `tests/integration/sandbox_enforcement_test.go`_

- [ ] 10. Phase 3 — Platform Features: Cloud Sync, SigmaAI Assistant, Cross-App Clipboard
  - [ ] 10.1 Implement Cloud Sync OAuth wizard and credential vault (`webshell/cloudSync.js`)
    - Build multi-step OAuth wizard UI; open provider auth URL in popup window
    - Listen for `message` event with `oauth-callback` type; extract tokens
    - Store access and refresh tokens in credential vault via `navigator.sigmaos.credentials.store()`
    - _Requirements: 16.1, 16.2_

  - [ ] 10.2 Implement sync-status topbar indicator and error display (`webshell/cloudSync.js`)
    - Add topbar status pill element with CSS classes for `Idle`, `Syncing`, `Error`, `Disconnected`
    - Implement `setStatus(status, error)` updating class and `title` attribute for hover error text
    - _Requirements: 16.3, 16.4_

  - [ ] 10.3 Write unit tests for Cloud Sync
    - Test OAuth callback handling, token storage, and status indicator state transitions
    - _Test file: `tests/unit/webshell/cloudSync.test.js`_

  - [ ] 10.4 Implement SigmaAI Assistant Spotlight overlay with AI search and text-match fallback (`webshell/sigmaAIAssistant.js`)
    - Create centered overlay `<div>` shown on activation hotkey; auto-focus text input
    - On query submit: `POST localhost:17392/search`; rank and display results (apps, filesystem, settings) within 2s
    - On AI failure (`catch`): set `aiAvailable = false`, fallback to `textMatchSearch()` across indexed sources; show "AI ranking unavailable" badge
    - _Requirements: 17.1, 17.2, 17.3, 17.4_

  - [ ] 10.5 Write unit tests for SigmaAI Assistant
    - Test overlay show/hide, ranked result display, 2s latency, text-match fallback, and unavailability indicator
    - _Test file: `tests/unit/webshell/sigmaAIAssistant.test.js`_

  - [ ] 10.6 Implement `sigmad-clipboard` Go daemon with IPC broadcast and MIME support (`sigmad-clipboard/main.go`)
    - Initialize daemon at session startup; register on D-Bus or Unix socket
    - Implement `Write(ClipboardData)` storing payload and broadcasting `clipboard-updated` event to all registered clients (non-blocking channel sends)
    - Implement `Read()` returning most recent payload under RLock within 100ms
    - Support MIME types: `text/plain`, `text/html`, `image/*`
    - _Requirements: 18.1, 18.2, 18.3, 18.4_

  - [ ] 10.7 Write property test for clipboard round-trip fidelity (Property 9)
    - **Property 9: Clipboard Round-Trip Fidelity**
    - **Validates: Requirements 18.3, 18.4**
    - Use `gopter` to generate text strings (ASCII, Unicode), HTML snippets, base64 images; Write then Read and assert identical content and MIME type
    - _Test file: `tests/unit/kernel/clipboard_roundtrip_property_test.go`_

  - [ ] 10.8 Write integration tests for clipboard daemon IPC
    - Test multi-client broadcast, concurrent reads, and 100ms read latency guarantee
    - _Test file: `tests/integration/clipboard_daemon_test.go`_

- [ ] 11. Checkpoint — Phase 3 platform tests pass
  - Ensure all Phase 3 unit and integration tests pass. Ask the user if questions arise.

- [ ] 12. Phase 4 — Ecosystem: App Developer SDK
  - [ ] 12.1 Write JSDoc annotations for all public platform APIs (`sdk/api/`)
    - Annotate `navigator.sigmaos.fs.*`, `navigator.sigmaos.process.*`, `navigator.sigmaos.notification.*`, and `navigator.sigmaos.credentials.*`
    - Include `@param`, `@returns`, `@throws`, and `@example` tags for every public function
    - _Requirements: 19.1_

  - [ ] 12.2 Create machine-readable JSON Schema for app manifest validation (`sdk/schema/manifest.schema.json`)
    - Define schema with required fields: `name` (kebab-case), `version` (semver), `entry`, `capabilities` enum array
    - Add optional fields: `icon`, `author.name`, `author.email` (format: email), `dependencies`, `files`, `signature`, `publicKey`
    - _Requirements: 19.2_

  - [ ] 12.3 Build manifest schema validator with structured error reporting (`sdk/validate.js`)
    - Implement `validateManifest(manifest)` using AJV against the JSON Schema
    - Return array of `{field: string, message: string}` for all validation errors
    - Return empty array on valid manifest
    - _Requirements: 19.4_

  - [ ] 12.4 Create template repository with minimal working app (`sdk/template/`)
    - Create `index.html`, `manifest.json`, and `app.js` that compile and run on SigmaOS without modification
    - Include `npm` or `make` build step and README with quickstart instructions
    - _Requirements: 19.3_

  - [ ] 12.5 Write unit tests for SDK validator
    - Test valid manifests, each required-field missing, invalid capability value, and invalid semver
    - _Test file: `tests/unit/sdk/manifestValidator.test.js`_

- [ ] 13. Phase 4 — Ecosystem: App Store Backend, GitHub/CI Hygiene, Documentation Wiki
  - [ ] 13.1 Implement App Store live registry client with download and signature verification (`webshell/appStore.js`)
    - Implement `fetchApps()` querying `https://registry.sigmaos.org/api/v1/apps`; refresh UI within 3s
    - Implement `install(appId)`: download package → verify RSA signature via `crypto.subtle` → extract
    - On signature failure: abort installation and display untrusted-package error message
    - _Requirements: 20.1, 20.2, 20.3, 20.4_

  - [ ] 13.2 Write unit tests for App Store
    - Test registry fetch, valid-signature install, invalid-signature abort, and offline fallback to cached list
    - _Test file: `tests/unit/webshell/appStore.test.js`_

  - [ ] 13.3 Audit and repair CI workflow file paths and uncomment all test blocks (`.github/workflows/`)
    - Scan all workflow YAML files for `run:` steps referencing non-existent paths; fix each broken reference
    - Uncomment any commented-out test blocks in CI YAML files
    - Add `pipeline-lint` job that runs a path-existence check script and fails with descriptive errors on invalid paths
    - _Requirements: 21.1, 21.2, 21.3, 30.1, 30.2, 39.1, 39.2, 39.3_

  - [ ] 13.4 Configure signed release workflow for v0.1.0 tag (`.github/workflows/sigma_release.yml`)
    - Add `on: push: tags: ['v*']` trigger to release workflow
    - Add GPG signing step for release artifact using repository secret
    - Publish signed artifact to GitHub Releases page
    - _Requirements: 21.4_

  - [ ] 13.5 Write all five Documentation Wiki pages (`wiki_repo/`)
    - `architecture.md`: kernel layer, HAL, scheduler, filesystem, network stack, security subsystems
    - `api-reference.md`: all public syscalls, IPC mechanisms, SDK entry points
    - `build-guide.md`: step-by-step instructions producing a bootable SigmaOS image from source; document `SIGMA_PROFILE` flag and profiling output format
    - `app-tutorial.md`: create, sign, and publish a minimal SigmaOS application
    - `security-model.md`: trust boundaries, capability model, zero-trust enforcement, cryptographic attestation
    - _Requirements: 22.1, 22.2, 22.3, 22.4, 22.5, 38.3_

- [ ] 14. Checkpoint — Phase 4 ecosystem complete
  - Ensure SDK validator tests pass, App Store tests pass, CI pipeline lint passes, and all five wiki pages are written. Ask the user if questions arise.

- [ ] 15. Critical Bug Fixes
  - [ ] 15.1 Fix PID 1 watchdog loop in `sigma_init.cpp`
    - Replace bounded iteration loop with `while(1)` infinite loop calling `__asm__ volatile("hlt")`
    - Add service exit handler: log `service_name` + exit code on non-zero exit, then restart service
    - _Requirements: 23.1, 23.2, 23.3_

  - [ ] 15.2 Write unit tests for PID 1 watchdog
    - Test that init never returns from main loop, service restart on non-zero exit, and exit code logging
    - _Test file: `tests/unit/kernel/sigma_init_watchdog_test.c`_

  - [ ] 15.3 Replace unsafe string operations in `sigma_zerotrust.cpp` with bounded equivalents
    - Replace every `strcpy` with `strncpy(dest, src, sizeof(dest) - 1)` followed by null-termination
    - Replace every `sprintf` with `snprintf(buf, sizeof(buf), fmt, ...)`
    - Add security audit log entry when any copy would exceed buffer capacity (before truncation)
    - _Requirements: 24.1, 24.2, 24.3, 24.4_

  - [ ] 15.4 Write property tests for ZeroTrust bounded string operations (Properties 10 & 11)
    - **Property 10: ZeroTrust Bounded String Copy** — Validates: Requirements 24.1, 24.2
    - **Property 11: Buffer Overflow Security Event Logging** — Validates: Requirement 24.4
    - Use `theft` to generate strings (0–1000 bytes) and buffers (16–512 bytes); verify canary integrity, null termination, and audit log entry on overflow
    - _Test file: `tests/unit/kernel/zerotrust_bounded_copy_property_test.c`_

  - [ ] 15.5 Implement revocation list consultation on every capability check in `sigma_zerotrust.cpp`
    - Add revocation list lookup before any capability grant in the policy check function
    - Return dedicated revocation error code when workload ID is in revocation list
    - Ensure runtime revocation is enforced: no restart required; check runs on every subsequent request
    - _Requirements: 25.1, 25.2, 25.3, 25.4_

  - [ ] 15.6 Write property tests for revocation list enforcement (Properties 12 & 13)
    - **Property 12: Revocation List Consultation on Every Check** — Validates: Requirements 25.1, 25.3
    - **Property 13: Runtime Revocation Enforcement** — Validates: Requirement 25.4
    - Use `theft` to generate workload IDs and revocation list states; assert every-check consultation and immediate denial
    - _Test file: `tests/unit/kernel/zerotrust_revocation_property_test.c`_

  - [ ] 15.7 Fix browser extension API Promise resolution in `background.js`
    - Add 10-second `setTimeout` to every outbound API call that rejects the Promise with a timeout error on expiry
    - Audit `background.js` for any Promises without a rejection path and add `.catch()` or `reject()` branches
    - On extension restart: iterate all pending Promises in `pendingCalls` map and reject each one
    - _Requirements: 26.1, 26.2, 26.3, 26.4_

  - [ ] 15.8 Write unit tests for extension Promise resolution
    - Test 10s timeout rejection, response-before-timeout resolve, restart-time pending rejection
    - _Test file: `tests/unit/webshell/extensionPromises.test.js`_

  - [ ] 15.9 Add freestanding build flags to `CMakeLists.txt` and remove hosted stdlib includes
    - Add `-nostdlib -nostdinc -ffreestanding` to kernel compile flags in `CMakeLists.txt`
    - Remove any `#include <stdlib.h>`, `#include <stdio.h>`, or `#include <string.h>` from kernel source files; replace with sovereign libc equivalents
    - Link kernel binary against sovereign libc (`-lsigma_libc`) instead of host glibc
    - Add CI step: `nm build/sigma-kernel | grep -q GLIBC && exit 1 | | exit 0`
    - _Requirements: 27.1, 27.2, 27.3, 27.4_

- [ ] 16. Checkpoint — Critical bug fixes verified
  - Ensure all critical bug fix tests pass and `nm` verification confirms no glibc symbols. Ask the user if questions arise.

## Task Dependency Graph

```
1 -> 2
2 -> 3, 6, 9, 12, 15
3 -> 4
4 -> 5
5 -> 6
6 -> 7
7 -> 8
8 -> 9
9 -> 10
10 -> 11
11 -> 12
12 -> 13
13 -> 14
14 -> 15
15 -> 16
```

```json
{
  "waves": [
    {"wave": 1, "tasks": ["1"]},
    {"wave": 2, "tasks": ["2"]},
    {"wave": 3, "tasks": ["3", "6", "9", "12", "15"]},
    {"wave": 4, "tasks": ["4", "7", "10", "13"]},
    {"wave": 5, "tasks": ["5", "8", "11", "14", "16"]}
  ]
}
```

## Notes

- Phase 0 kernel tasks (task 1) must complete before any subsequent phases.
- Checkpoints (tasks 2, 5, 8, 11, 14, 16) are validation gates; all children of the preceding phase must pass before advancing.
- Property-based tests (PBT) are co-located with their implementation sub-tasks and must pass before the parent task is marked complete.
- Languages: C/C++ for kernel modules, Go for daemons, JavaScript/TypeScript for web shell and applications.
- Property test libraries: `theft`/`rapidcheck` (C), `fast-check` (JS), `gopter` (Go).
