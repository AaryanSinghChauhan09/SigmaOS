# Requirements Document

## Introduction

This document captures the development requirements for the SigmaOS full-platform roadmap. SigmaOS is a sovereign, AI-native operating system targeting physical x86_64 hardware, a polished web-shell experience, full application completions, platform features, and a developer ecosystem. The roadmap is structured into five development phases (Phase 0–4) and a set of bug fixes across three severity levels (Critical, High, Medium). All phases and fixes are required to produce a production-grade, bootable, and secure operating system.

---

## Glossary

- **Kernel**: The SigmaOS freestanding x86_64 kernel binary, built without any host libc dependency.
- **IDT**: Interrupt Descriptor Table — the x86_64 hardware structure routing CPU exceptions and hardware IRQs to ISR handlers.
- **ISR**: Interrupt Service Routine — a handler function registered in the IDT for a specific vector.
- **TSS**: Task State Segment — x86_64 structure providing the kernel stack pointer used on privilege-level transitions.
- **Ring 3**: Unprivileged CPU protection ring used for userland processes.
- **CryptFS**: The SigmaOS encrypted root filesystem subsystem, backed by dm-crypt.
- **PCI_Scanner**: The SigmaOS component responsible for enumerating hardware devices via MMIO and PCI configuration space reads.
- **Web_Shell**: The browser-based SigmaOS desktop environment that simulates a full OS UI in-page.
- **Window_Manager**: The in-page compositing layer of Web_Shell managing draggable/resizable application windows and the taskbar.
- **SigmaNotes**: The bundled note-taking application with Markdown support.
- **SigmaCode**: The bundled code editor application backed by an embedded code editor engine.
- **SigmaTerm**: The bundled terminal emulator application with PTY support.
- **SigmaPaint**: The bundled raster painting application with layer support.
- **Neural_UI_Engine**: The SigmaOS component that proxies inference requests to the local AI daemon.
- **Enterprise_Dashboard**: The SigmaOS telemetry and enterprise metrics application.
- **sigmad_ai**: The local AI inference daemon listening on port 17392.
- **sigmad_clipboard**: The cross-application clipboard daemon.
- **bwrap**: Bubblewrap — the userland sandboxing tool used for zero-install package execution.
- **Notification_Center**: The UI component receiving and displaying system and application notifications.
- **Lock_Screen**: The security overlay that requires PIN or credential input before returning to the desktop.
- **App_Store**: The SigmaOS application registry and installation frontend.
- **SDK**: The SigmaOS application developer kit exposing platform APIs, manifest schema, and tooling.
- **CI**: Continuous Integration — the automated build and test pipeline.
- **ZeroTrust_Module**: The SigmaOS policy-enforcement component in `sigma_zerotrust.cpp` governing workload capability checks.
- **Firewall_Module**: The SigmaOS kernel network packet filtering component in `sigma_shield.cpp`.
- **Audit_Log**: The tamper-evident system audit trail recording security-relevant events with timestamps.
- **Go_Daemon**: The SigmaOS Go-language management daemon handling storage ejection operations.
- **TCP_Fuzzer**: The SigmaOS network stack fuzzing tool in `net/`.
- **Conntrack**: The connection-tracking subsystem maintaining counters for active network connections.
- **XSS**: Cross-Site Scripting — a class of injection vulnerability caused by unsanitized HTML output.

---

## Requirements

---

### Requirement 1: Interrupt Descriptor Table Initialization (Phase 0)

**User Story:** As a kernel developer, I want the IDT fully initialized with ISR stubs for all CPU exceptions, so that the kernel does not triple-fault on real x86_64 hardware when an exception occurs.

#### Acceptance Criteria

1. WHEN the Kernel starts on x86_64 hardware, THE Kernel SHALL call `sigma_idt_init()` before enabling interrupts.
2. THE Kernel SHALL register ISR stub handlers for CPU exception vectors 0 through 31 inclusive in the IDT.
3. WHEN a CPU exception fires with a registered vector, THE Kernel SHALL invoke the corresponding ISR stub without triple-faulting.
4. IF `sigma_idt_init()` fails to load the IDT descriptor, THEN THE Kernel SHALL emit a serial diagnostic message and halt.

---

### Requirement 2: Ring 3 Usermode Transition (Phase 0)

**User Story:** As a kernel developer, I want a functional `jump_to_usermode()` routine, so that the kernel can enter unprivileged user processes on real hardware.

#### Acceptance Criteria

1. WHEN the Kernel is ready to launch the first user process, THE Kernel SHALL invoke `jump_to_usermode()` to transition the CPU to Ring 3.
2. THE Kernel SHALL configure a valid TSS entry with a kernel stack pointer before executing the `iret` instruction into userland.
3. THE Kernel SHALL set up a separate per-process kernel stack in the TSS for each user process created.
4. IF the `iret` sequence encounters a general-protection fault, THEN THE Kernel SHALL handle the fault via the registered IDT ISR and log a diagnostic message.

---

### Requirement 3: Encrypted Root Filesystem Mount (Phase 0)

**User Story:** As a kernel developer, I want the encrypted root filesystem mounted at boot, so that all persistent data is protected by cryptographic storage on real hardware.

#### Acceptance Criteria

1. WHEN the Kernel completes memory initialization, THE Kernel SHALL call `sigma_cryptfs_mount_root()` to mount the encrypted root volume.
2. THE Kernel SHALL wire `sigma_cryptfs_mount_root()` to the dm-crypt layer to perform actual key derivation and block-device decryption.
3. IF the encrypted root volume cannot be decrypted due to an invalid key, THEN THE Kernel SHALL emit a serial error message and halt rather than mounting an unauthenticated filesystem.
4. THE CryptFS module SHALL write the derived key buffer before passing it to the dm-crypt layer.

---

### Requirement 4: PCI Bus Enumeration (Phase 0)

**User Story:** As a hardware engineer, I want real PCI bus scanning, so that the kernel can discover and initialize physical hardware devices on x86_64 machines.

#### Acceptance Criteria

1. WHEN the Kernel initializes the driver subsystem, THE PCI_Scanner SHALL enumerate all devices present in PCI configuration space using MMIO reads.
2. THE PCI_Scanner SHALL replace the `fake_dev` stub in `linux_shim.cpp` with real PCI configuration space access.
3. WHEN a PCI device is found, THE PCI_Scanner SHALL record the vendor ID, device ID, class code, and BAR values for each device.
4. IF a PCI configuration space read returns `0xFFFF` for the vendor ID, THEN THE PCI_Scanner SHALL treat the slot as empty and skip it.

---

### Requirement 5: In-Page Window Manager (Phase 1)

**User Story:** As a Web_Shell user, I want a proper in-page window manager with draggable and resizable windows, so that the desktop feels like a real OS rather than a collection of browser pop-ups.

#### Acceptance Criteria

1. THE Window_Manager SHALL render application windows as in-page elements without calling `window.open()`.
2. WHEN a user drags a window title bar, THE Window_Manager SHALL reposition the window to follow the pointer.
3. WHEN a user drags a window resize handle, THE Window_Manager SHALL resize the window to match the pointer position, with a minimum size of 200 × 150 pixels.
4. THE Window_Manager SHALL display a persistent taskbar showing all open application windows.
5. WHEN the user clicks a taskbar entry, THE Window_Manager SHALL bring the corresponding window to focus.

---

### Requirement 6: SigmaNotes Markdown Preview (Phase 1)

**User Story:** As a SigmaNotes user, I want a split-pane Markdown preview with a basic formatting toolbar, so that I can write and preview formatted notes without switching applications.

#### Acceptance Criteria

1. THE SigmaNotes application SHALL render a split-pane layout with an editable source pane and a read-only preview pane.
2. WHEN the user types in the source pane, THE SigmaNotes application SHALL update the preview pane within 500 milliseconds.
3. THE SigmaNotes application SHALL provide a toolbar with buttons for bold, italic, heading, and link formatting actions.
4. WHEN a toolbar button is activated, THE SigmaNotes application SHALL insert the corresponding Markdown syntax at the current cursor position.

---

### Requirement 7: SigmaCode IDE (Phase 1)

**User Story:** As a developer, I want a code editor embedded in SigmaCode with filesystem access and process spawn capability, so that I can write and run code from within the OS shell.

#### Acceptance Criteria

1. THE SigmaCode application SHALL embed a code editor engine providing syntax highlighting, line numbers, and multi-cursor editing.
2. WHEN the user opens a file from the filesystem, THE SigmaCode application SHALL load the file contents into the editor within 1 second.
3. WHEN the user saves an open file, THE SigmaCode application SHALL write the updated contents to the filesystem.
4. WHEN the user triggers a run action, THE SigmaCode application SHALL spawn a process for the open file and display its stdout and stderr output in an integrated terminal panel.

---

### Requirement 8: Notification Center (Phase 1)

**User Story:** As a Web_Shell user, I want a notification center accessible via a bell icon, so that I can view system and application notifications in a centralized panel.

#### Acceptance Criteria

1. THE Notification_Center SHALL display a bell icon in the system tray area of the taskbar.
2. WHEN `navigator.sigmaos.notification.show()` is called by an application, THE Notification_Center SHALL receive and queue the notification.
3. WHEN the user clicks the bell icon, THE Notification_Center SHALL slide out a panel listing all queued notifications in chronological order.
4. WHEN a new notification arrives while the panel is closed, THE Notification_Center SHALL increment a badge counter on the bell icon.

---

### Requirement 9: Lock Screen (Phase 1)

**User Story:** As a security-conscious user, I want a lock screen overlay triggered by Ctrl+L, so that I can secure my session without closing applications.

#### Acceptance Criteria

1. WHEN the user presses Ctrl+L, THE Lock_Screen SHALL render a full-screen overlay covering all application windows.
2. WHILE the Lock_Screen overlay is active, THE Lock_Screen SHALL display the current date and time, updating once per second.
3. WHEN the user submits the correct PIN or credential, THE Lock_Screen SHALL remove the overlay and restore the previous desktop state.
4. IF the user submits an incorrect PIN three consecutive times, THEN THE Lock_Screen SHALL impose a 30-second lockout before accepting further input.

---

### Requirement 10: SigmaTerm PTY Support (Phase 2)

**User Story:** As a developer, I want SigmaTerm to support a real PTY over WebSockets or SSE, so that interactive terminal applications run correctly inside the browser shell.

#### Acceptance Criteria

1. THE SigmaTerm application SHALL allocate a server-side PTY and multiplex its I/O over a WebSocket or SSE connection.
2. WHEN the user types a character in SigmaTerm, THE SigmaTerm application SHALL send the character to the PTY within 50 milliseconds.
3. WHEN the PTY produces output, THE SigmaTerm application SHALL render the output in the terminal view within 100 milliseconds of receipt.
4. WHEN the terminal window is resized, THE SigmaTerm application SHALL send a SIGWINCH signal to the PTY with the updated column and row dimensions.

---

### Requirement 11: SigmaNotes AI Integration (Phase 2)

**User Story:** As a SigmaNotes user, I want real AI-powered writing assistance via the sigmad-ai daemon, so that I can receive intelligent suggestions while composing notes.

#### Acceptance Criteria

1. WHEN the user requests an AI suggestion in SigmaNotes, THE SigmaNotes application SHALL send the request to `sigmad_ai` at `localhost:17392`.
2. WHEN `sigmad_ai` returns a response within 5 seconds, THE SigmaNotes application SHALL display the suggestion inline in the editor.
3. IF `sigmad_ai` does not respond within 5 seconds, THEN THE SigmaNotes application SHALL display a timeout error and allow the user to retry.
4. THE SigmaNotes application SHALL not block user editing while an AI request is pending.

---

### Requirement 12: SigmaPaint Layers Panel (Phase 2)

**User Story:** As a creative user, I want a layers panel in SigmaPaint, so that I can compose artwork with independent, non-destructive layers.

#### Acceptance Criteria

1. THE SigmaPaint application SHALL render a layers panel listing all layers in the current document.
2. WHEN the user creates a new layer, THE SigmaPaint application SHALL add the layer above the currently selected layer.
3. WHEN the user changes layer visibility, THE SigmaPaint application SHALL update the canvas composite within 100 milliseconds.
4. WHEN the user reorders layers by dragging, THE SigmaPaint application SHALL recomposite the canvas to reflect the new stacking order.

---

### Requirement 13: Neural UI Engine Real Inference (Phase 2)

**User Story:** As a platform developer, I want the Neural UI Engine to perform real inference calls to `localhost:17392/v1/predict`, so that UI predictions are based on actual model output rather than mocked data.

#### Acceptance Criteria

1. WHEN the Neural_UI_Engine requires a prediction, THE Neural_UI_Engine SHALL issue an HTTP POST to `localhost:17392/v1/predict` with the inference payload.
2. WHEN the inference endpoint returns a 200 response, THE Neural_UI_Engine SHALL apply the returned prediction to the active UI context.
3. IF the inference endpoint returns a non-200 response, THEN THE Neural_UI_Engine SHALL log the error code and fall back to the last successful prediction.
4. IF the inference endpoint is unreachable, THEN THE Neural_UI_Engine SHALL disable predictive features and notify the user with a single non-blocking status indicator.

---

### Requirement 14: Enterprise Dashboard Live Telemetry (Phase 2)

**User Story:** As an enterprise operator, I want the Enterprise Dashboard to receive live telemetry data via SSE, so that I can monitor system health in real time without manual refresh.

#### Acceptance Criteria

1. THE Enterprise_Dashboard SHALL establish an SSE connection to the telemetry stream endpoint on startup.
2. WHEN a telemetry event arrives over SSE, THE Enterprise_Dashboard SHALL update the corresponding metric widget within 200 milliseconds.
3. IF the SSE connection drops, THEN THE Enterprise_Dashboard SHALL attempt to reconnect with an exponential backoff, starting at 1 second and capping at 30 seconds.
4. WHILE the SSE connection is disconnected, THE Enterprise_Dashboard SHALL display a visible "Disconnected" status indicator.

---

### Requirement 15: Zero-Install Package Execution (Phase 3)

**User Story:** As a power user, I want to run packages without installation using a bwrap sandbox chain with capability enforcement, so that I can execute untrusted software in a safe, isolated environment.

#### Acceptance Criteria

1. WHEN the user requests zero-install execution of a package, THE Kernel SHALL invoke `bwrap` with a minimal capability set derived from the package manifest.
2. THE Kernel SHALL enforce the declared capability list and deny system calls not covered by the manifest.
3. IF the package requests a capability not declared in its manifest, THEN THE Kernel SHALL deny the system call and record the violation in the Audit_Log.
4. WHEN the sandboxed process exits, THE Kernel SHALL release all sandbox resources within 500 milliseconds.

---

### Requirement 16: Cloud Sync (Phase 3)

**User Story:** As a user, I want a guided OAuth flow and visible sync status, so that I can synchronize my files to a cloud provider and monitor the sync state from the taskbar.

#### Acceptance Criteria

1. WHEN the user initiates Cloud Sync setup, THE Web_Shell SHALL present a wizard that guides the user through OAuth authorization with the selected cloud provider.
2. WHEN OAuth authorization completes, THE Web_Shell SHALL store the access and refresh tokens in the system credential vault.
3. THE Web_Shell SHALL display a sync-status indicator in the topbar showing one of: Idle, Syncing, Error, or Disconnected.
4. WHEN a sync operation fails, THE Web_Shell SHALL update the sync-status indicator to Error and display a human-readable reason on hover.

---

### Requirement 17: SigmaAI Assistant (Phase 3)

**User Story:** As a user, I want a Spotlight-style natural language search powered by the AI assistant, so that I can find files, apps, and system functions by typing intent rather than exact names.

#### Acceptance Criteria

1. WHEN the user activates the search overlay, THE SigmaAI assistant SHALL display a centered input bar accepting natural language queries.
2. WHEN the user submits a query, THE SigmaAI assistant SHALL return ranked results within 2 seconds.
3. THE SigmaAI assistant SHALL include results from installed applications, filesystem paths, and system settings in each result set.
4. IF `sigmad_ai` is unavailable, THEN THE SigmaAI assistant SHALL fall back to a local text-match search and indicate that AI ranking is unavailable.

---

### Requirement 18: Cross-App Clipboard (Phase 3)

**User Story:** As a user, I want a shared clipboard daemon so that I can copy content in one application and paste it in another seamlessly.

#### Acceptance Criteria

1. THE sigmad_clipboard daemon SHALL start at session initialization and remain resident for the duration of the user session.
2. WHEN an application writes to the clipboard, THE sigmad_clipboard daemon SHALL store the payload and broadcast a clipboard-updated event to all registered applications.
3. WHEN an application reads from the clipboard, THE sigmad_clipboard daemon SHALL return the most recently written payload within 100 milliseconds.
4. THE sigmad_clipboard daemon SHALL support plain text, rich text, and image MIME types.

---

### Requirement 19: App Developer SDK (Phase 4)

**User Story:** As an app developer, I want a documented SDK with JSDoc API docs, a manifest schema, and a template repository, so that I can build and publish SigmaOS applications with clear guidance.

#### Acceptance Criteria

1. THE SDK SHALL expose all public platform APIs with JSDoc annotations covering parameters, return types, and examples.
2. THE SDK SHALL include a machine-readable manifest schema that validators can parse to verify application metadata correctness.
3. THE SDK SHALL provide a template repository containing a minimal working application that compiles and runs on SigmaOS without modification.
4. WHEN a developer validates an app manifest against the schema, THE SDK SHALL return a list of all validation errors with field paths and human-readable messages.

---

### Requirement 20: App Store Backend (Phase 4)

**User Story:** As a user, I want to browse and install apps from a live registry, so that I can discover new SigmaOS applications without manually editing static JSON files.

#### Acceptance Criteria

1. THE App_Store SHALL query a live registry endpoint to retrieve the available application list on every open.
2. WHEN the registry endpoint returns updated metadata for an application, THE App_Store SHALL display the updated version, description, and icon within 3 seconds.
3. WHEN the user initiates an application install, THE App_Store SHALL download, verify the signature, and install the application.
4. IF application signature verification fails, THEN THE App_Store SHALL abort the installation and display an error message identifying the package as untrusted.

---

### Requirement 21: GitHub and CI Hygiene (Phase 4)

**User Story:** As a contributor, I want working issue templates, a passing CI workflow, and a tagged release, so that the project is welcoming to new contributors and has a verifiable release artifact.

#### Acceptance Criteria

1. THE CI workflow SHALL reference only file paths that exist in the repository.
2. WHEN a pull request is opened, THE CI workflow SHALL execute the full test suite and report pass/fail status on the pull request within 10 minutes.
3. THE CI workflow SHALL not have any test blocks permanently commented out.
4. WHEN a `v0.1.0` tag is pushed, THE CI workflow SHALL produce a signed release artifact and publish it to the GitHub Releases page.

---

### Requirement 22: Documentation Wiki (Phase 4)

**User Story:** As a contributor or evaluator, I want a complete wiki covering architecture, API reference, build guide, app tutorial, and security model, so that I can understand, build, and extend SigmaOS without requiring direct author support.

#### Acceptance Criteria

1. THE Wiki SHALL include an architecture document describing the kernel layer, HAL, scheduler, filesystem, network stack, and security subsystems.
2. THE Wiki SHALL include an API reference document covering all public syscalls, IPC mechanisms, and SDK entry points.
3. THE Wiki SHALL include a build guide that produces a bootable SigmaOS image from source on a supported Linux host.
4. THE Wiki SHALL include an app tutorial that walks a developer through creating, signing, and publishing a minimal SigmaOS application.
5. THE Wiki SHALL include a security model document describing the trust boundaries, capability model, zero-trust enforcement, and cryptographic attestation mechanisms.

---

## Bug Fix Requirements

### Requirement 23: PID 1 Watchdog Loop (Critical Bug 1)

**User Story:** As a kernel developer, I want PID 1 to remain alive indefinitely, so that the kernel does not panic on real hardware due to the init process exiting.

#### Acceptance Criteria

1. THE Kernel init process (PID 1) SHALL enter an infinite wait loop after launching all registered services rather than exiting after a bounded number of iterations.
2. WHEN all registered services have been started, THE Kernel init process SHALL call `hlt` in a tight loop to yield CPU while remaining in the running state.
3. IF a registered service exits with a non-zero code, THEN THE Kernel init process SHALL log the service name and exit code before attempting a restart.

---

### Requirement 24: Memory-Safe String Operations in ZeroTrust Module (Critical Bug 2)

**User Story:** As a security engineer, I want all string operations in the ZeroTrust module to be memory-safe, so that buffer overflows cannot corrupt kernel memory or be exploited.

#### Acceptance Criteria

1. THE ZeroTrust_Module SHALL use bounded string copy operations with explicit size limits in place of all `strcpy` and `sprintf` calls.
2. WHEN the ZeroTrust_Module copies a string into a fixed-size buffer, THE ZeroTrust_Module SHALL truncate the input to `buffer_size - 1` bytes and null-terminate the result.
3. THE ZeroTrust_Module SHALL use `snprintf` with an explicit size argument for all formatted string writes.
4. IF a string operation would exceed the buffer capacity, THEN THE ZeroTrust_Module SHALL record a security event in the Audit_Log identifying the call site.

---

### Requirement 25: Revoked Workload Policy Enforcement (Critical Bug 3)

**User Story:** As a security engineer, I want revoked workloads to fail all policy checks, so that unauthorized processes cannot continue operating after their credentials are revoked.

#### Acceptance Criteria

1. WHEN the ZeroTrust_Module evaluates a policy check for a workload, THE ZeroTrust_Module SHALL verify the workload's revocation status before granting any capability.
2. IF the workload's identity token is present in the revocation list, THEN THE ZeroTrust_Module SHALL deny the capability request and return a revocation error code.
3. THE ZeroTrust_Module SHALL consult the revocation list on every capability check, not only on initial authentication.
4. WHEN a workload is revoked while running, THE ZeroTrust_Module SHALL deny all subsequent capability requests from that workload without requiring a restart.

---

### Requirement 26: Browser Extension API Promise Resolution (Critical Bug 4)

**User Story:** As an app developer, I want all browser extension API calls to resolve or reject their Promises, so that applications do not hang indefinitely waiting for responses.

#### Acceptance Criteria

1. WHEN a browser extension API call is dispatched, THE Web_Shell extension SHALL resolve or reject the corresponding Promise within 10 seconds.
2. IF the kernel-side handler does not respond within 10 seconds, THEN THE Web_Shell extension SHALL reject the Promise with a timeout error.
3. THE Web_Shell extension background script SHALL not leave any Promise permanently unresolved.
4. WHEN the extension background script is restarted, THE Web_Shell extension SHALL reject all pending Promises that were outstanding at the time of restart.

---

### Requirement 27: Freestanding Kernel Build (Critical Bug 5 and 6)

**User Story:** As a kernel developer, I want the kernel binary to link as a freestanding binary without any host libc dependency, so that the kernel is self-contained and bootable on real hardware.

#### Acceptance Criteria

1. THE Kernel build system SHALL pass `-nostdlib`, `-nostdinc`, and `-ffreestanding` flags when compiling all kernel source files.
2. THE Kernel SHALL not include any hosted standard library headers such as `<stdlib.h>`, `<stdio.h>`, or `<string.h>` in freestanding kernel code.
3. WHEN the kernel binary is linked, THE Kernel build system SHALL link against the SigmaOS sovereign libc implementation rather than the host glibc.
4. THE CI workflow SHALL verify the kernel binary contains no host libc symbols by running `nm` on the output and failing if any `glibc` version symbols are present.

---

### Requirement 28: Service Array Bounds Protection in Init (High Bug 7)

**User Story:** As a kernel developer, I want the service registration array to be guarded against overflow, so that registering more services than the array capacity does not corrupt adjacent memory.

#### Acceptance Criteria

1. THE Kernel init system SHALL define `MAX_SERVICES` as a compile-time constant and enforce it in `sigma_init_register()`.
2. WHEN `sigma_init_register()` is called and `service_count` equals `MAX_SERVICES`, THE Kernel init system SHALL return an error code and not write to the array.
3. THE Kernel init system SHALL log a warning message when the service limit is reached, identifying the service name that was rejected.

---

### Requirement 29: Complete Kernel Core Source Files (High Bug 8)

**User Story:** As a build engineer, I want all declared kernel core source files to exist on disk, so that the build system produces a non-empty, complete kernel binary.

#### Acceptance Criteria

1. THE Kernel build system SHALL compile all source files listed in `CMakeLists.txt` without "file not found" errors.
2. WHEN the build completes, THE Kernel binary SHALL contain object code from every declared source file, verified by `nm` showing at least one symbol per translation unit.
3. IF a declared source file is missing, THE CI workflow SHALL fail the build step and report the missing file path.

---

### Requirement 30: CI Test Suite Activation (High Bug 9)

**User Story:** As a project maintainer, I want all CI tests to run on every pull request, so that regressions are caught automatically before code is merged.

#### Acceptance Criteria

1. THE CI workflow SHALL execute all test cases that exist in the repository without any test block being commented out.
2. WHEN a test fails in the CI workflow, THE CI workflow SHALL mark the pipeline as failed and report the failing test name and output.
3. THE CI workflow SHALL achieve a test execution time below 10 minutes for the full suite on standard CI runners.

---

### Requirement 31: Firewall Real Packet Inspection (High Bug 10)

**User Story:** As a security engineer, I want the firewall to inspect actual network packets rather than mocked data, so that firewall rules have real enforcement effect.

#### Acceptance Criteria

1. WHEN a network packet arrives at the Firewall_Module, THE Firewall_Module SHALL evaluate the configured rules against the actual packet header fields.
2. THE Firewall_Module SHALL not use hardcoded or mocked packet data in any production code path.
3. WHEN the Firewall_Module drops a packet, THE Firewall_Module SHALL increment the corresponding rule's drop counter using the actual packet's source and destination fields.
4. THE Firewall_Module SHALL process inbound packets within 1 millisecond per packet under normal system load.

---

### Requirement 32: Audit Log Real Timestamps (High Bug 11)

**User Story:** As a security auditor, I want all audit log entries to carry real system timestamps, so that the log accurately reflects when security events occurred.

#### Acceptance Criteria

1. THE Audit_Log SHALL record the real wall-clock time obtained from the system clock for every log entry.
2. THE Audit_Log SHALL not contain hardcoded or static timestamp values in any production code path.
3. WHEN an audit entry is written, THE Audit_Log SHALL include the timestamp in ISO 8601 format with at least millisecond precision.

---

### Requirement 33: Go Daemon Ejection Error Handling (High Bug 12)

**User Story:** As a storage engineer, I want the Go daemon's eject handler to report failure when the unmount operation fails, so that callers know whether the device was safely ejected.

#### Acceptance Criteria

1. WHEN the Go_Daemon `handleEject` function is called, THE Go_Daemon SHALL execute the underlying unmount system call.
2. IF the unmount system call returns an error, THEN THE Go_Daemon SHALL propagate the error to the caller with a non-success response code.
3. THE Go_Daemon SHALL not return a success response from `handleEject` unless the unmount system call succeeded.

---

### Requirement 34: Separate WiFi and Bluetooth Build Targets (High Bug 13)

**User Story:** As a build engineer, I want WiFi and Bluetooth compiled into separate binaries, so that deployments requiring only one subsystem are not forced to carry the other.

#### Acceptance Criteria

1. THE Kernel build system SHALL define separate CMake targets for the WiFi driver module and the Bluetooth driver module.
2. WHEN building a WiFi-only configuration, THE Kernel build system SHALL not include Bluetooth object code in the output binary.
3. WHEN building a Bluetooth-only configuration, THE Kernel build system SHALL not include WiFi object code in the output binary.

---

### Requirement 35: XSS Prevention in Web Shell (Medium Bug 14)

**User Story:** As a security engineer, I want all dynamic HTML insertion in the web shell to be sanitized, so that malicious content cannot execute scripts in the user's browser session.

#### Acceptance Criteria

1. THE Web_Shell SHALL not assign untrusted string values directly to `element.innerHTML`.
2. WHEN the Web_Shell inserts user-supplied or external content into the DOM, THE Web_Shell SHALL use `element.textContent` or a DOM sanitization function that strips script-bearing elements.
3. IF content must be rendered as HTML, THEN THE Web_Shell SHALL pass it through an allowlist-based sanitizer before insertion.
4. THE CI workflow SHALL run a static analysis rule that flags any new `innerHTML` assignments with non-literal values as a build warning.

---

### Requirement 36: Seeded TCP Fuzzer Randomness (Medium Bug 15)

**User Story:** As a test engineer, I want the TCP fuzzer to use a seeded pseudo-random generator, so that fuzz runs are reproducible and corpus-driven rather than always deterministic.

#### Acceptance Criteria

1. THE TCP_Fuzzer SHALL accept a seed value as a command-line argument or environment variable.
2. WHEN a seed value is provided, THE TCP_Fuzzer SHALL initialize the pseudo-random generator with that seed, producing a reproducible packet sequence.
3. WHEN no seed value is provided, THE TCP_Fuzzer SHALL seed the pseudo-random generator with a high-entropy value derived from the system entropy source.
4. THE TCP_Fuzzer SHALL log the seed value used at the start of each fuzz session to facilitate reproduction of failures.

---

### Requirement 37: CryptFS Key Derivation Implementation (Medium Bug 16)

**User Story:** As a kernel developer, I want the `sigma_cryptfs.cpp` key derivation function to actually write the derived key into the key buffer, so that encryption operations use the real derived key rather than uninitialized memory.

#### Acceptance Criteria

1. WHEN `sigma_cryptfs_derive_key()` is called, THE CryptFS module SHALL compute the key from the provided passphrase and salt using a defined key-derivation function.
2. THE CryptFS module SHALL write the full derived key into the output key buffer before returning.
3. IF key derivation fails, THEN THE CryptFS module SHALL zero the output key buffer and return an error code.
4. THE CryptFS module SHALL accept any non-empty passphrase and produce a 256-bit derived key deterministically for a given passphrase and salt pair.

---

### Requirement 38: SIGMA_PROFILE Flag Consumption (Medium Bug 17)

**User Story:** As a developer, I want the `SIGMA_PROFILE` build flag to actually enable profiling instrumentation, so that passing the flag produces measurable profiling output.

#### Acceptance Criteria

1. WHEN the Kernel is compiled with `SIGMA_PROFILE` defined, THE Kernel SHALL enable per-function timing instrumentation at function entry and exit points.
2. WHEN the profiled Kernel runs, THE Kernel SHALL write profiling samples to a designated output buffer or file.
3. THE Kernel build system SHALL document the `SIGMA_PROFILE` flag and its expected output format in the build guide.

---

### Requirement 39: CI Workflow Valid File Paths (Medium Bug 18)

**User Story:** As a build engineer, I want all CI workflow file references to point to files that actually exist in the repository, so that CI runs do not fail due to missing path references.

#### Acceptance Criteria

1. THE CI workflow files SHALL reference only paths that exist in the repository at the time of the workflow commit.
2. THE CI workflow SHALL validate path references as part of the pipeline lint step.
3. IF a CI workflow references a non-existent path, THE CI pipeline SHALL fail with a descriptive error message identifying the invalid path.

---

### Requirement 40: Connection Tracking Counter Decrement (Medium Bug 19)

**User Story:** As a network engineer, I want the conntrack counter to be decremented when a connection closes, so that the connection table does not grow unboundedly and exhaust kernel memory.

#### Acceptance Criteria

1. WHEN a tracked connection transitions to the CLOSED state, THE Conntrack module SHALL decrement the active connection counter by exactly one.
2. WHEN a tracked connection entry is removed from the table, THE Conntrack module SHALL decrement the active connection counter before freeing the entry.
3. THE Conntrack module SHALL maintain a counter value that equals the number of entries currently in the connection table at all times.
4. IF the Conntrack counter reaches a configurable maximum, THE Conntrack module SHALL reject new connection tracking entries and log a warning.
