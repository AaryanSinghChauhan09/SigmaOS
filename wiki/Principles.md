# SigmaOS Design Principles

> SigmaOS is a **principle-driven** operating system. Every architectural decision, API choice, and implementation detail traces back to at least one named principle. This document catalogs all principles, their definitions, their application in SigmaOS, and concrete examples.

Principles are grouped thematically but each applies across all layers.

---

## Table of Contents

- [Software Engineering Fundamentals](#software-engineering-fundamentals)
- [Object-Oriented and Structural Design](#object-oriented-and-structural-design)
- [Systems Programming and OS Design](#systems-programming-and-os-design)
- [Memory and Concurrency](#memory-and-concurrency)
- [Testing and Quality](#testing-and-quality)
- [Modularity and Architecture](#modularity-and-architecture)
- [Community and Governance](#community-and-governance)
- [Distribution Engineering](#distribution-engineering)

---

## Software Engineering Fundamentals

---

### ABSTRACTION
**Definition:** Hide implementation details behind a stable interface so callers don't need to know how something works, only what it does.

**In SigmaOS:** Every layer boundary is an abstraction — the display layer exposes a `DisplayServer` trait; callers don't care if it's Wayland or X11 underneath.

**Example:** `sigma-display` exposes a unified `DisplayBackend` trait implemented by both `WaylandBackend` and `X11Backend`. Apps call `display.get_outputs()` without knowing which protocol is active.

---

### ARRANGE-ACT-ASSERT
**Definition:** Structure every test as three phases: set up state, perform the action under test, and verify the outcome.

**In SigmaOS:** All unit tests in Rust follow the AAA pattern with `// Arrange`, `// Act`, `// Assert` comments to make intent clear.

**Example:** In `sigma-pkg`, a test for dependency resolution arranges a mock package database, acts by calling `resolve_deps("sigma-shell")`, and asserts that the returned list contains all transitive deps in correct order.

---

### AVOID FRAGMENTATION
**Definition:** Prevent the proliferation of incompatible variants that dilute ecosystem cohesion and split maintenance effort.

**In SigmaOS:** A single official desktop flavor is maintained. Spin-offs are allowed as community projects but the official release is not fragmented into sub-editions.

**Example:** SigmaOS does not ship a KDE edition, GNOME edition, and Xfce edition simultaneously — it ships one curated desktop (`sigma-shell`) and documents how to swap it.

---

### BLACKBOX
**Definition:** Treat a component as a black box by interacting only through its defined interface, never reaching into internals.

**In SigmaOS:** Cross-component access is always through published APIs. Direct struct field access across crate boundaries is forbidden; only methods on public types are called.

**Example:** `sigma-net` never directly reads NetworkManager's internal connection objects — it uses the NetworkManager D-Bus API exclusively.

---

### BOY SCOUT RULE
**Definition:** Always leave the code cleaner than you found it — fix small issues opportunistically while working nearby.

**In SigmaOS:** Contributors are expected to fix obvious lint warnings, update outdated comments, and improve error messages in files they touch, even when not part of the primary change.

**Example:** A PR that adds IPv6 support to `sigma-net` also removes two dead code paths and updates three outdated doc comments found during the review.

---

### CLEAR ABSTRACTION
**Definition:** An abstraction must have a single, obvious purpose — if you can't explain it in one sentence, split it.

**In SigmaOS:** Trait definitions are reviewed for clarity. A trait that mixes "display management" with "input handling" is rejected and split into two.

**Example:** `sigma-wm` defines separate traits: `WindowPolicy` (where windows go) and `WindowDecorator` (how windows look). Mixing them into one `WindowManager` trait was explicitly rejected.

---

### CODE REUSABILITY
**Definition:** Write code once and use it in multiple contexts rather than duplicating logic.

**In SigmaOS:** Shared logic lives in the `sigma-common` crate, imported by all components that need it. No copy-pasted utilities.

**Example:** `sigma-common` contains `XdgPaths`, `DbusProxy`, `ConfigParser`, and `Logger` — all reused across `sigma-panel`, `sigma-notifications`, `sigma-search`, and a dozen other components.

---

### COMMAND-QUERY SEPARATION
**Definition:** A function either changes state (command) or returns data (query) — never both.

**In SigmaOS:** All service methods and trait functions are categorized as commands or queries. Mixed-purpose functions are split during code review.

**Example:** In `sigma-pkg`, `list_installed()` is a pure query. `install_package()` is a command. There is no `install_and_list()` function.

---

### CONTINUOUS IMPROVEMENT
**Definition:** Regularly revisit and improve the codebase, processes, and documentation — not just when something breaks.

**In SigmaOS:** Each release cycle includes a dedicated "cleanup sprint" before the RC phase, focused solely on refactoring, documentation, and technical debt reduction.

**Example:** After each milestone, a retrospective issue is filed listing three things to improve in the next phase. These become first-class tickets.

---

### DEFENSIVE PROGRAMMING
**Definition:** Assume inputs can be invalid, state can be inconsistent, and external systems can fail — handle all cases explicitly.

**In SigmaOS:** All external inputs (files, D-Bus calls, network responses) are validated before use. `unwrap()` is banned except in tests.

**Example:** `sigma-update` validates GPG signature, checks package hash, verifies version ordering, and confirms sufficient disk space before applying any update — each step can fail independently and does so gracefully.

---

### DEMETER LAW (Law of Demeter)
**Definition:** A module should only talk to its immediate collaborators — don't reach through an object to talk to another object's internals ("don't talk to strangers").

**In SigmaOS:** Components call methods on objects they directly hold, not on objects returned by those objects. Long method chains like `a.get_b().get_c().do_thing()` are rejected in code review.

**Example:** `sigma-panel`'s battery applet calls `power_manager.get_battery_level()` — it does not call `power_manager.get_acpi_device().get_battery().get_charge_percentage()`. The PowerManager is responsible for reading from ACPI; the applet only talks to the PowerManager.

---

### DESIGN PATTERN
**Definition:** Apply proven, named solutions to recurring design problems to improve communication and consistency.

**In SigmaOS:** Patterns are named in code comments. Common patterns used: Builder (for complex config structs), Observer (for event systems), Strategy (for pluggable backends), and Command (for undo-able operations).

**Example:** `sigma-shell` uses the **Observer** pattern for workspace events — components subscribe to `WorkspaceEvent` and react to window focus, close, and move events.

---

### DISCOVERABILITY
**Definition:** A system should make its capabilities easy to find without reading documentation.

**In SigmaOS:** `sigma-search` supports fuzzy search over all installed apps, settings, and commands. Tab completion is available everywhere. All CLI tools support `--help`.

**Example:** Running `sigma-ctl help` lists all available subcommands with one-line descriptions. Running `sigma-ctl help <subcommand>` shows full usage.

---

### DRY (Don't Repeat Yourself)
**Definition:** Every piece of knowledge should have a single, authoritative representation in the system.

**In SigmaOS:** Version numbers, package names, and configuration schema definitions are declared once and referenced everywhere. Build scripts read from `Cargo.toml`, not separate version files.

**Example:** The SigmaOS version string is defined in one place (`sigma-common/version.rs`) and auto-injected into the kernel cmdline, update manifests, and package metadata at build time.

---

### ENCAPSULATION
**Definition:** Bundle data and the functions that operate on it, and restrict direct access to internal state.

**In SigmaOS:** All Rust structs expose behavior through methods; fields are private by default. Pub fields require explicit justification in code review.

**Example:** `PackageDatabase` in `sigma-pkg` has no public fields. All operations go through `add()`, `remove()`, `query()`, `transaction()` methods that maintain invariants.

---

### FAIL-FAST
**Definition:** Detect and report errors as early as possible, close to where they originate.

**In SigmaOS:** Validation happens at input boundaries, not deep in call stacks. Configuration is validated at startup, not at first use.

**Example:** `sigma-init` validates all systemd unit files and sigma config files during boot before starting any services. Invalid config causes an immediate, informative boot failure rather than a silent runtime error.

---

### GALL'S LAW
**Definition:** A complex system that works evolved from a simple system that worked. Start simple.

**In SigmaOS:** The Phase 1 bootable base is intentionally minimal — a kernel, init, and shell. Desktop and AI features are added incrementally only after the base is stable.

**Example:** The Phase 2 target is a bootable ISO with only kernel + systemd + bash. No GUI. This validates the foundation before building complexity on top.

---

### IF IT AIN'T BROKE DON'T FIX IT
**Definition:** Avoid refactoring or replacing working components without a clear, justified reason.

**In SigmaOS:** Mature, battle-tested upstream components (btrfs, PipeWire, NetworkManager) are not replaced with custom alternatives without a demonstrated need.

**Example:** SigmaOS uses upstream NetworkManager rather than writing a custom network daemon, because NetworkManager works well and has 15 years of bug fixes.

---

### INFORMATION HIDING
**Definition:** Only expose what callers need; keep everything else private.

**In SigmaOS:** All crates default to `pub(crate)` visibility. Items are promoted to `pub` only when they cross crate boundaries.

**Example:** `sigma-audio`'s PipeWire session state is fully internal. External callers only see `AudioSession::list_sinks()`, `AudioSession::set_volume()`, etc.

---

### INSTRUMENTATION
**Definition:** Build observability into the system so that internal behavior can be measured and understood.

**In SigmaOS:** All system daemons emit structured logs via `sigma-journal`. Performance-critical paths emit tracing spans. Metrics are exposed via a Prometheus-compatible endpoint.

**Example:** `sigma-update` emits timing metrics for each update phase (download, verify, apply), so performance regressions can be detected across releases.

---

### INVERSION OF CONTROL
**Definition:** Depend on abstractions, not concrete implementations. Let a framework or container inject dependencies.

**In SigmaOS:** Service dependencies are passed as trait objects at construction time, not looked up globally. This makes testing trivial.

**Example:** `sigma-net`'s `DnsResolver` receives a `DnsBackend` trait object (DoH, systemd-resolved, or a mock) at construction, not a hardcoded implementation.

---

### KISS (Keep It Simple, Stupid)
**Definition:** Prefer the simplest solution that correctly solves the problem.

**In SigmaOS:** New dependencies are challenged. Complex algorithms are replaced with simpler ones when performance allows. Features that add complexity without proportional value are deferred.

**Example:** `sigma-panel`'s configuration format is TOML, not a custom DSL. The applet API is a simple Rust trait, not a plugin framework with dynamic loading.

---

### LEAST ASTONISHMENT
**Definition:** The system should behave the way a reasonable user or developer would expect.

**In SigmaOS:** Default behaviors follow established conventions (XDG paths, freedesktop standards, Wayland protocol norms). Deviations from convention are explicitly documented.

**Example:** `sigma-files` uses the same keyboard shortcuts as Nautilus (F2 to rename, Ctrl+H to show hidden files) because users coming from other Linux desktops already know them.

---

### LEAST PRIVILEGE
**Definition:** Every component, process, and user should operate with the minimum set of privileges necessary to perform its function — and nothing more.

**In SigmaOS:** All system daemons drop capabilities at startup to the minimal set required. No SUID binaries. Flatpak apps are granted only the permissions they explicitly declare and the user approves.

**Example:** `sigma-net` runs as a dedicated `sigma-net` system user with only `CAP_NET_ADMIN` and `CAP_NET_RAW`. It cannot read user files, cannot execute code, and cannot modify the system outside its D-Bus interface. A compromised `sigma-net` cannot escalate to root.

---

### NORMALISATION
**Definition:** Eliminate redundancy in data structures and configurations — each piece of information has one authoritative source, just as database normalization avoids repeating data across tables.

**In SigmaOS:** Configuration schemas are normalized — no setting is duplicated across multiple config files. Package metadata is stored once in the package database, not repeated in multiple index files.

**Example:** A package's version number is stored once in `sigma-pkg`'s database. The update manifest, the installed-packages list, and the rollback metadata all reference the database entry — they don't each store their own copy of the version string.

---

### NINETY-NINETY RULE
**Definition:** The first 90% of a feature takes 90% of the time; the last 10% takes the other 90% of the time. Budget accordingly.

**In SigmaOS:** Phase milestones are scheduled with explicit polish time built in. Features are not marked "done" until edge cases, error paths, and documentation are complete.

**Example:** `sigma-installer` entering "feature complete" state is not the same as "release ready." A 4-week hardening phase follows every feature-complete milestone.

---

### OFFENSIVE PROGRAMMING
**Definition:** Where an invariant must hold, assert it loudly rather than silently recovering — invalid state should crash, not corrupt.

**In SigmaOS:** `debug_assert!()` macros guard internal invariants in kernel-adjacent code. In debug builds, violated invariants cause an explicit panic with context.

**Example:** In `sigma-mm`, page allocations assert that the requested size is page-aligned. In release builds this is still checked via `Result<>`, but in debug builds an `assert!()` fires immediately.

---

### OPEN-CLOSED
**Definition:** Software entities should be open for extension but closed for modification.

**In SigmaOS:** Plugin interfaces (audio backends, display backends, AI backends) are designed so new implementations can be added without modifying existing code.

**Example:** Adding a new AI backend to `sigma-ai-assistant` requires only implementing the `AiBackend` trait and registering it — no existing code is modified.

---

### OPEN STANDARD
**Definition:** Prefer open, documented, community-governed standards over proprietary protocols.

**In SigmaOS:** Wayland over Mir, D-Bus over custom IPC, PipeWire over proprietary audio, OCI containers over proprietary formats.

**Example:** `sigma-store` uses the Flatpak runtime format and OCI image specification for all packaged apps, not a custom SigmaOS-specific container format.

---

### OPTIMISE FOR DELETION
**Definition:** Write code that is easy to remove — avoid tight coupling, prefer small modules, minimize the blast radius of deletion.

**In SigmaOS:** Each of the 149 component repositories can be removed from a build without affecting others (beyond their declared dependents). No component has hidden side effects on removal.

**Example:** Removing `sigma-calendar` from a build removes only the calendar app and its D-Bus service registration. No other component breaks.

---

### REFACTORING
**Definition:** Continuously improve code structure without changing external behavior.

**In SigmaOS:** Refactoring PRs are welcome and treated as first-class contributions. They must be accompanied by tests proving behavior is preserved.

**Example:** The `sigma-pkg` transaction engine was refactored from a mutable state machine to an immutable operation log in v0.2, improving testability without changing the public API.

---

### RULE OF THREE
**Definition:** When you've written the same code three times, extract it into a reusable abstraction.

**In SigmaOS:** Code review enforces this rule. Three occurrences of the same pattern trigger an extraction into `sigma-common` or a shared crate.

**Example:** When `sigma-panel`, `sigma-notifications`, and `sigma-search` all had hand-rolled D-Bus connection setup code, it was extracted into `sigma-common::dbus::ServiceConnection`.

---

### SECURE DEFAULTS
**Definition:** Every system should ship with security-conscious settings enabled out of the box — users should have to opt out of security, not opt in to it.

**In SigmaOS:** The firewall is on by default. SSH is off by default. Full-disk encryption is recommended prominently in the installer. DNS-over-HTTPS is enabled by default. No telemetry is collected. None of these require user configuration to be secure.

**Example:** When SigmaOS is installed, the nftables firewall blocks all inbound connections immediately. There is no "activate firewall" step. A user who never touches the firewall settings is still protected. Opening ports requires explicit action.

---

### SEPARATION OF CONCERNS
**Definition:** Each module, component, or layer has one area of responsibility and does not bleed into others.

**In SigmaOS:** The display server knows nothing about audio. The package manager knows nothing about the desktop theme. Boundaries are strict.

**Example:** `sigma-wm` handles only window placement and focus. It does not handle notifications (that's `sigma-notifications`) or app launching (that's `sigma-search`).

---

### SINGLE RESPONSIBILITY
**Definition:** A class or module should have only one reason to change.

**In SigmaOS:** All Rust modules are reviewed for single responsibility. A module that handles both parsing and networking is split.

**Example:** `sigma-update` is split into `update-fetcher` (download), `update-verifier` (signature check), `update-applier` (installation), and `update-daemon` (orchestration). Each changes independently.

---

### SOLID
**Definition:** The five principles — Single Responsibility, Open-Closed, Liskov Substitution, Interface Segregation, Dependency Inversion — applied together as a system.

**In SigmaOS:** SOLID is the baseline for all public API and trait design. PRs that violate SOLID principles are flagged in review.

**Example:** The `sigma-display` crate was redesigned to explicitly follow SOLID: each display protocol is a separate impl (SRP), new protocols extend rather than modify (OCP), all impls honor the `DisplayBackend` contract (LSP), fat interfaces were split (ISP), and all callers depend on traits not impls (DIP).

---

### SPEC-DRIVEN
**Definition:** Write the specification (interface, API, behavior contract) before the implementation.

**In SigmaOS:** All new components begin with a spec document in the `docs/` folder defining the API, behavior, error conditions, and test cases. Implementation follows.

**Example:** `sigma-ai-assistant` has a `spec/ai-backend-protocol.md` document defining the request/response format, error codes, and streaming behavior before a single line of implementation was written.

---

### TALK IS CHEAP SHOW ME THE CODE
**Definition:** Proposals must be backed by implementation or proof-of-concept — design discussions without code are time-bounded.

**In SigmaOS:** RFC issues are marked as needing a prototype within 30 days or they are closed. Abstract debates without code stall projects; SigmaOS defaults to implementation.

**Example:** The debate over custom init vs. systemd was resolved by writing a prototype custom init and benchmarking it. The results (no measurable benefit, high maintenance cost) ended the discussion.

---

### TRANSPARENCY
**Definition:** The system's behavior, configuration, and decision-making should be visible and understandable to the user.

**In SigmaOS:** All daemon activity is logged to `sigma-journal` and accessible to the user. Updates show exactly what will change. The AI assistant can explain any system action it takes.

**Example:** Before applying an update, `sigma-update` presents a human-readable changelog, lists all packages being changed, and requires user acknowledgment (unless running in automated mode).

---

### UNIFORM ACCESS
**Definition:** Access to an attribute should look the same whether it is stored or computed.

**In SigmaOS:** System information endpoints (battery level, CPU usage, network status) are accessed through the same D-Bus interface regardless of whether values are cached or computed live.

**Example:** `sigma-panel`'s battery widget calls `PowerManager.GetBatteryLevel()` — it doesn't know or care if this reads directly from the kernel or returns a cached value from 100ms ago.

---

### UNIFORM INFORMATION
**Definition:** Information about the system should be presented consistently regardless of source.

**In SigmaOS:** All system diagnostics, logs, and status information use the same structured format (JSON-LD) and are queryable through `sigma-ctl info`.

**Example:** CPU usage, disk I/O, network throughput, and GPU utilization are all accessible through `sigma-ctl info --format json` with the same schema.

---

### YAGNI (You Aren't Gonna Need It)
**Definition:** Don't add functionality until it is actually needed.

**In SigmaOS:** Features requested without a concrete use case are deferred. The AI assistant does not have a "schedule meeting" feature until there is a calendar app to schedule meetings in.

**Example:** `sigma-pkg` does not implement a dependency solver for optional weak dependencies. It was proposed but deferred until a real use case emerged.

---

### ZEN OF PYTHON
**Definition:** The guiding philosophy of Python's design — explicit is better than implicit, simple is better than complex, readability counts.

**In SigmaOS:** Applied to all Python tooling scripts. Python code in SigmaOS follows PEP 8, avoids clever one-liners, and prefers explicit error handling over magic.

**Example:** `sigma-build` scripts use explicit `subprocess.run()` with checked return codes rather than `os.system()`. Configuration is explicit TOML, not Python files executed for their side effects.

---

### ZERO-ONE-INFINITY
**Definition:** Allow none, one, or unlimited instances of a thing — arbitrary fixed limits (like "max 5 plugins") are a design smell.

**In SigmaOS:** Plugin counts, workspace counts, monitor counts, and notification counts are either zero, exactly one, or unlimited. Hardcoded limits like "max 4 workspaces" are rejected.

**Example:** `sigma-wm` supports an unlimited number of workspaces. There is no config key for "max workspaces" — you create as many as you need.

---

## Object-Oriented and Structural Design

---

### BOOCH METHOD
**Definition:** Grady Booch's approach to OO design — identify objects, define their responsibilities, and specify their collaborations.

**In SigmaOS:** Used during the design phase of each component. Component design docs list objects, their responsibilities (what they know, what they do), and their collaborators.

**Example:** `sigma-wm` was designed using the Booch method: Window objects know their geometry, own their surface buffers, and collaborate with WorkspaceManager and FocusTracker.

---

### COMPOSITION OVER INHERITANCE
**Definition:** Prefer composing objects from smaller pieces over inheriting from a base class.

**In SigmaOS:** Rust traits + struct composition are used everywhere. Deep inheritance hierarchies do not exist. Shared behavior is provided by composition.

**Example:** `sigma-panel` applets all implement the `Applet` trait. A `ClockApplet` wraps a `Timer` and a `TextRenderer` — it does not inherit from a base `TimedApplet` class.

---

### GRASP (General Responsibility Assignment Software Patterns)
**Definition:** A set of patterns for assigning responsibilities to classes/modules — including Creator, Controller, Low Coupling, High Cohesion, and Pure Fabrication.

**In SigmaOS:** GRASP patterns are used in code review to evaluate whether a new class or function has the right responsibility assignment.

**Example:** When `sigma-pkg` needed a component to coordinate a multi-step install, the Controller pattern from GRASP was applied to create `InstallCoordinator` — a Pure Fabrication that handles orchestration without domain logic.

---

### INHERITANCE
**Definition:** A type relationship where a subtype inherits behavior from a parent type.

**In SigmaOS:** Used sparingly and only where a true "is-a" relationship exists. Rust trait inheritance is used for type hierarchies (e.g., `AsyncBackend: Backend`).

**Example:** In `sigma-display`, `WaylandBackend` implements both `DisplayBackend` and `WaylandExtensions`, where `WaylandExtensions: DisplayBackend` — a valid trait inheritance expressing the "is-a Wayland backend" relationship.

---

### INTERFACE SEGREGATION
**Definition:** Clients should not be forced to depend on interfaces they don't use. Split fat interfaces into focused ones.

**In SigmaOS:** All traits are reviewed for segregation. A trait with 15 methods is likely a smell. Traits are split by usage pattern.

**Example:** The original `WindowManager` trait with 20 methods was split into `WindowLayoutPolicy`, `WindowFocusPolicy`, `WindowDecorationPolicy`, and `WindowEventSource` — each implemented independently.

---

### LISKOV SUBSTITUTION
**Definition:** Any implementation of a trait should be substitutable for any other implementation of that same trait without breaking the program.

**In SigmaOS:** All trait implementations are tested against the trait's behavioral contract via shared test suites. Implementors that violate postconditions are rejected.

**Example:** `sigma-display` has a `backend_conformance_tests!()` macro that every `DisplayBackend` implementation must pass, ensuring Wayland and X11 backends are truly substitutable.

---

### OOP (Object-Oriented Programming)
**Definition:** Organize software around objects that encapsulate data and behavior, communicate via messages.

**In SigmaOS:** Rust's struct + trait system provides OOP-style encapsulation and polymorphism. SigmaOS uses OOP patterns where they clarify design.

**Example:** `sigma-audio`'s audio graph is modeled as objects — `AudioNode`, `AudioSink`, `AudioSource`, `AudioMixer` — each encapsulating its state and processing logic.

---

### POLYMORPHISM
**Definition:** The same interface can have multiple implementations, and the correct one is selected at runtime or compile time.

**In SigmaOS:** Display backends, audio backends, AI backends, and package format backends are all polymorphic via Rust traits.

**Example:** `sigma-ai-assistant` dispatches to either `OllamaBackend`, `OpenAIBackend`, or `LocalLlamaBackend` through the `AiBackend` trait — the rest of the system doesn't know which is active.

---

## Systems Programming and OS Design

---

### ARCHITECTURE
**Definition:** The set of high-level structures and design decisions that govern a system — the "big picture."

**In SigmaOS:** The layered architecture (boot → kernel → system → desktop → app) is the primary architectural constraint. All components must fit within a layer and communicate only through defined interfaces.

**Example:** When `sigma-wm` needed access to GPU performance counters, it was explicitly routed through `sigma-display` (the display layer component) rather than accessing the DRM/KMS kernel API directly.

---

### BOOTLOADER INTEGRATION
**Definition:** The OS must cooperate correctly with the bootloader — passing correct kernel parameters, supporting dual-boot, and maintaining chainloading correctness.

**In SigmaOS:** `sigma-bootloader` manages GRUB2 and systemd-boot entries, ensuring that updates never leave the system unbootable and that fallback entries always exist.

**Example:** Every time a new kernel is installed, `sigma-bootloader` creates a signed boot entry AND preserves the previous kernel entry as a fallback. A failed boot automatically selects the fallback.

---

### CACHE CONSCIOUS DESIGN
**Definition:** Design data structures and access patterns to maximize CPU cache utilization.

**In SigmaOS:** Hot data paths (scheduler queues, window layout state, input event queues) use cache-line-aligned structs and access patterns that minimize cache misses.

**Example:** `sigma-sched`'s task queue is a cache-line-padded array-of-structs, not a linked list, to ensure sequential cache-line reads when iterating runnable tasks.

---

### CROSS COMPILATION
**Definition:** Build code on one architecture for execution on another.

**In SigmaOS:** The entire SigmaOS build system supports cross-compilation from x86_64 hosts to ARM64 targets. All components are cross-compile clean.

**Example:** A CI job builds the entire SigmaOS for `aarch64-unknown-linux-gnu` on `x86_64-unknown-linux-gnu` hosts to validate ARM64 support.

---

### DEADLOCK AVOIDANCE
**Definition:** Design concurrent systems so that circular resource dependencies (deadlocks) cannot occur.

**In SigmaOS:** Mutex acquisition order is globally documented. Lock hierarchies are enforced by convention and checked by CI tooling. Async-first design minimizes lock usage.

**Example:** `sigma-audio`'s PipeWire graph lock and the session lock are always acquired in a fixed order (graph first, session second). Violating this order fails CI.

---

### DEUTSCH LIMIT
**Definition:** (From L. Peter Deutsch) — the difficulty of a problem is proportional to its description length. Keep problem descriptions (specs, APIs) short.

**In SigmaOS:** Any API or protocol that cannot be described on one page is too complex and must be simplified. Spec documents have page limits.

**Example:** The `sigma-update` update manifest format was redesigned when the spec exceeded 5 pages. The redesign reduced it to a 1.5-page TOML schema.

---

### DIRECT DEVICE CONTROL
**Definition:** Performance-critical paths should control hardware directly rather than through abstraction layers.

**In SigmaOS:** GPU rendering in `sigma-shell` uses DRM/KMS directly (via `libdrm` bindings), bypassing X11 overhead. Audio latency-critical paths use ALSA directly where PipeWire latency is insufficient.

**Example:** `sigma-shell`'s page-flip path goes directly from the compositor to the DRM CRTC, with zero copies and no intermediate rendering server.

---

### EFFICIENT SCHEDULING
**Definition:** The scheduler should maximize system responsiveness for interactive workloads while ensuring fairness.

**In SigmaOS:** The kernel is configured with the EEVDF scheduler tuned for desktop workloads. `sigma-init` uses cgroup-based CPU allocation to give desktop processes priority over background tasks.

**Example:** `sigma-shell` and all GUI processes are placed in a high-priority cgroup slice. Background processes like `sigma-update`'s download phase run in a `background.slice` with CPU weight 50 (vs. 1000 for desktop).

---

### INTERRUPT HANDLING DISCIPLINE
**Definition:** Interrupt handlers must be minimal and fast — do only what must be done in interrupt context and defer the rest.

**In SigmaOS:** All custom kernel modules follow the top-half/bottom-half pattern: interrupt handlers acknowledge the hardware and enqueue work; a workqueue or tasklet does the actual processing.

**Example:** The `sigma-input` kernel module's interrupt handler reads the scancode and enqueues it to a lock-free ring buffer. The workqueue thread reads from the buffer and translates to input events.

---

### LAYERED DESIGN
**Definition:** Organize the system into layers where each layer provides services to the layer above and consumes services from the layer below.

**In SigmaOS:** The five-layer model (firmware, kernel, system, desktop, app) is the primary architectural constraint.

**Example:** See [Architecture.md](Architecture.md) for the full layer model. No component skips a layer.

---

### LOCALITY OF REFERENCE
**Definition:** Keep frequently accessed data close together in memory and time to improve cache performance.

**In SigmaOS:** Window state, input event queues, and rendering data are laid out to maximize spatial and temporal locality.

**Example:** `sigma-wm`'s window list is a `Vec<Window>` (not `Vec<Box<Window>>`) to keep window state contiguous in memory and avoid pointer chasing during layout calculations.

---

### LOW LEVEL CONTROL
**Definition:** The OS should provide mechanisms for direct hardware control when user/application needs require it.

**In SigmaOS:** `sigma-ctl` provides direct access to hardware controls — CPU governor, display brightness, fan curves, GPU power limits — without requiring custom kernel modules.

**Example:** `sigma-ctl power set-cpu-governor performance` directly writes to `/sys/devices/system/cpu/cpu*/cpufreq/scaling_governor` with proper permission checking.

---

### LOW LEVEL LANGUAGES
**Definition:** Use low-level languages (Rust, C) for performance-critical and safety-critical components.

**In SigmaOS:** Kernel, drivers, system daemons, and the compositor are written in Rust or C. Python is used only for build tooling, not runtime components.

**Example:** `sigma-shell` (compositor) is written in Rust. Its Python prototype was benchmarked at 12ms frame times; the Rust implementation achieves 0.8ms frame times.

---

### MANUAL MEMORY CONTROL
**Definition:** In performance-critical paths, allocate and free memory explicitly rather than relying on GC.

**In SigmaOS:** Rust's ownership model provides manual memory control without garbage collection. Custom allocators are used for latency-sensitive subsystems.

**Example:** `sigma-shell`'s render pipeline uses a pool allocator for frame resources to avoid heap allocation per frame. All allocations happen at compositor startup, not per-frame.

---

### MEMORY DISCIPLINE
**Definition:** Be rigorous about memory usage — minimize footprint, avoid leaks, use appropriate data structures.

**In SigmaOS:** Component memory usage is profiled at each release. Regression tests fail if a component exceeds its memory budget.

**Example:** `sigma-panel` has a 50MB RSS memory budget. If a new applet pushes it over, the CI fails until the memory usage is optimized.

---

### MEMORY SAFETY
**Definition:** The language and runtime should prevent use-after-free, buffer overflows, and data races.

**In SigmaOS:** Rust is the primary language precisely for its memory safety guarantees. Unsafe Rust blocks require explicit justification and security review.

**Example:** All `unsafe` blocks in SigmaOS components have a `// SAFETY:` comment explaining why the block is sound. Unsafe blocks without this comment fail CI.

---

### MINIMAL KERNEL PHILOSOPHY
**Definition:** The kernel should contain only what must be in the kernel — move everything possible to userspace.

**In SigmaOS:** The kernel config is explicitly minimized. Each enabled feature has a documented justification. The `sigma-kernel` kconfig review process is part of every release.

**Example:** `CONFIG_MODULES=n` is the default for the hardened build profile. Modular features that users don't need are compiled out entirely.

---

### MODULAR KERNEL DESIGN
**Definition:** The kernel should be composable from independent modules that can be loaded/unloaded.

**In SigmaOS:** All hardware-specific drivers are kernel modules (not built-in) to allow the same kernel image to run on diverse hardware.

**Example:** GPU drivers (`nouveau`, `amdgpu`, `i915`) are all loadable modules. The base kernel image is hardware-agnostic; the correct driver is loaded at boot by `udev`.

---

### PLUG-IN DRIVER MODEL
**Definition:** Hardware drivers should be pluggable — the kernel core should not know about specific hardware.

**In SigmaOS:** All drivers follow the Linux kernel's driver model. No hardware-specific code exists in the core kernel components of SigmaOS.

**Example:** `sigma-drivers` provides out-of-tree Rust drivers that follow the kernel's platform device model, registering themselves via the `Driver` trait without modifying core kernel code.

---

### PORTABILITY
**Definition:** The system should run on multiple hardware platforms without platform-specific hacks in the core.

**In SigmaOS:** Platform-specific code is isolated in well-defined HAL (Hardware Abstraction Layer) modules. The core system is portable across x86_64 and ARM64.

**Example:** `sigma-power`'s ACPI backend is compiled only on x86_64. ARM64 uses a Device Tree backend. The rest of `sigma-power` is identical on both platforms.

---

### POWER OF TEN
**Definition:** NASA's 10 rules for safety-critical code: simple control flow, bounded loops, no dynamic memory after init, minimum functions, etc.

**In SigmaOS:** Applied to kernel-level Rust modules and interrupt handlers. Not applied to userspace apps where flexibility is needed.

**Example:** `sigma-mm`'s physical page allocator follows Power of Ten: no recursion, no dynamic allocation after init, all loops have explicit bounds, all return values checked.

---

### PROCESS ISOLATION
**Definition:** Processes should be isolated from each other — a crash or compromise in one process should not affect others.

**In SigmaOS:** All sandboxed apps run in isolated namespaces (PID, mount, network, IPC). System daemons use separate user accounts.

**Example:** `sigma-browser` runs in a separate PID namespace with its own mount namespace. A browser crash cannot access `sigma-keyring`'s memory or filesystem.

---

### RACE-FREE DESIGN
**Definition:** Concurrent components must be designed so that race conditions are structurally impossible, not just unlikely.

**In SigmaOS:** Shared state is protected by ownership (Rust's `Arc<Mutex<>>` or `Arc<RwLock<>>` ) or by message passing. No shared mutable global state without synchronization.

**Example:** `sigma-sched`'s run queue uses lock-free atomics for the fast path (task insertion) and a mutex only for the slow path (queue rebalancing).

---

### SAFE CONCURRENCY
**Definition:** Use concurrency primitives and patterns that prevent data races and make concurrent code auditable.

**In SigmaOS:** Rust's `Send` and `Sync` bounds are the primary enforcement mechanism. `async/await` with Tokio is used for I/O-bound concurrency; `rayon` for CPU-bound parallelism.

**Example:** All D-Bus service handlers in SigmaOS use `async` functions. No blocking I/O is performed on the async executor thread. Blocking operations are offloaded to `spawn_blocking()`.

---

### SEPARATION OF MECHANISM AND POLICY
**Definition:** The kernel/low-level provides mechanisms; userspace decides the policy.

**In SigmaOS:** `sigma-sched` provides scheduling mechanisms (preemption, priority levels, CPU affinity). `sigma-init` decides policy (desktop processes get high priority, background processes get low priority).

**Example:** The kernel's Landlock LSM is a mechanism (filesystem access control rules). `sigma-sec` provides the policy daemon that generates per-app Landlock rule sets.

---

### VIRTUAL MEMORY ABSTRACTION
**Definition:** Provide a uniform virtual address space abstraction to userspace, hiding physical memory management.

**In SigmaOS:** All userspace processes see a flat 64-bit virtual address space. Physical memory management is entirely kernel-internal.

**Example:** `sigma-mm` configures the kernel's virtual memory layout with explicit ASLR entropy values and guards the kernel/userspace boundary via SMEP/SMAP.

---

### ZERO-COPY
**Definition:** Transfer data between components without copying — use shared memory, DMA, or reference passing.

**In SigmaOS:** The rendering pipeline from apps to the display uses DMA-BUF zero-copy buffer sharing. Audio from PipeWire to ALSA uses shared memory.

**Example:** When `sigma-browser` renders a frame, the GPU buffer is shared with `sigma-shell` via a DMA-BUF file descriptor. No pixel data is copied between the browser and the compositor.

---

## Testing and Quality

---

### DETERMINISTIC TESTING
**Definition:** Tests must produce the same result every time, regardless of execution order, timing, or environment.

**In SigmaOS:** All tests use fixed seeds for any randomness, mock clocks for time-dependent behavior, and dependency injection for external services.

**Example:** `sigma-sched`'s scheduler simulation tests use a deterministic mock clock that advances one tick per event, making test outcomes independent of real-world timing.

---

### FIRST (Fast, Isolated, Repeatable, Self-validating, Timely)
**Definition:** Unit test properties: Fast (< 1ms), Isolated (no external deps), Repeatable (same result always), Self-validating (no manual inspection), Timely (written before or with the code).

**In SigmaOS:** All unit tests are measured for run time. Tests exceeding 100ms are refactored to use mocks. Integration tests are in a separate test suite.

**Example:** `sigma-pkg`'s dependency resolution tests run in under 5ms total. They use in-memory mock package databases and do not touch the filesystem.

---

## Modularity and Architecture

---

### COMPONENT ISOLATION
**Definition:** Components should not share state, resources, or runtime environment without explicit contracts.

**In SigmaOS:** Each of the 149 components is a separate repository with its own Cargo.toml, build system, test suite, and release cycle.

**Example:** `sigma-notifications` has no direct code dependency on `sigma-panel`. They communicate only through the D-Bus notification spec (org.freedesktop.Notifications).

---

### LOOSE COUPLING
**Definition:** Components should depend on each other as little as possible, and only through stable interfaces.

**In SigmaOS:** Dependency graph is explicitly managed. A change to `sigma-common` is the only cross-cutting change that can affect all components.

**Example:** `sigma-audio` does not import `sigma-input`'s crate. Volume key events are delivered to `sigma-audio` via D-Bus signals from `sigma-input`, not via direct function calls.

---

### MAXIMISE COHESION
**Definition:** Everything in a module should be closely related to a single purpose.

**In SigmaOS:** Modules that grow to handle multiple concerns are split. PR review includes a "cohesion check" — if you can't describe the module in one sentence, it needs splitting.

**Example:** `sigma-update` was originally a monolithic daemon. It was split into four cohesive modules: fetcher, verifier, applier, and coordinator, each with a single clear purpose.

---

### MINIMISE COUPLING
**Definition:** Minimize the number and strength of dependencies between components.

**In SigmaOS:** The dependency graph is a DAG with no cycles. New dependencies require justification. The number of direct dependencies per component is tracked.

**Example:** `sigma-panel` has 6 direct dependencies (sigma-common, sigma-dbus, sigma-theme, libcairo, pango, glib). Adding a 7th requires a documented justification in the PR.

---

### MODULARITY
**Definition:** The system is composed of independent, interchangeable modules with well-defined interfaces.

**In SigmaOS:** 149 component repositories, each independently buildable and testable, are the embodiment of this principle.

**Example:** Replacing `sigma-wm` (the tiling window manager) with a different WM requires only implementing the `sigma-wm` D-Bus interface. All other components continue working.

---

### MULTIPLE REPOSITORY
**Definition:** Each independent component lives in its own repository for independent versioning, ownership, and release management.

**In SigmaOS:** All 149 components have separate repositories under the `AaryanSinghChauhan09` GitHub organization.

**Example:** `sigma-kernel` is on its own 6-week release cycle (tracking LTS kernels). `sigma-theme` has a separate release cycle driven by design work. They don't block each other.

---

### SINGLE REPOSITORY
**Definition:** When components are tightly coupled and must be developed together, they should share a repository.

**In SigmaOS:** `sigma-common` (shared utilities) and `sigma-types` (shared types) are co-located in `sigma-core` because they must always be in sync.

**Example:** `sigma-core` contains `sigma-common`, `sigma-types`, and `sigma-error` in a single Cargo workspace because they form a single logical unit that is always versioned together.

---

## Community and Governance

---

### COLLABORATION & SHARED RESPONSIBILITY
**Definition:** All maintainers share responsibility for the project's health — there are no silos.

**In SigmaOS:** Any maintainer can review and approve PRs in any component. Domain expertise is acknowledged but not gatekept.

**Example:** A kernel maintainer can review a `sigma-theme` PR for process/governance issues, even if they defer to the theme maintainer on design decisions.

---

### COMMUNITY-ORIENTED DEVELOPMENT
**Definition:** The project exists to serve its community — decisions are made with community benefit, not individual preference, as the primary criterion.

**In SigmaOS:** Major decisions (default desktop, default browser, licensing) are made through public RFC processes with community comment periods.

**Example:** The choice of GRUB2 vs. systemd-boot was an RFC with a 14-day comment period. Community feedback influenced the final decision.

---

### FOSS (Free and Open Source Software)
**Definition:** All software is freely available, modifiable, and distributable. Licenses are reviewed for compatibility.

**In SigmaOS:** All first-party components are Apache-2.0 or MIT dual-licensed. Third-party components must be FOSS-compatible. Non-FOSS components are explicitly opt-in only.

**Example:** The Wi-Fi firmware blobs (necessary for some Broadcom chipsets) are available in a separate `sigma-nonfree` package that is not installed by default and requires explicit user opt-in.

---

### GOVERNANCE
**Definition:** The rules and processes by which the project makes decisions, accepts contributions, and resolves disputes.

**In SigmaOS:** The GOVERNANCE.md document (in the main repository) defines the maintainer tier system, RFC process, voting rules, and code of conduct enforcement.

**Example:** Adding a new component to SigmaOS requires a Tier 3 RFC: a spec document, a prototype, and a 14-day community review. A simple majority of maintainer votes approves it.

---

### MERITOCRACY AND INCLUSIVITY
**Definition:** Contributions are judged on their technical merit; contributors are welcomed regardless of background.

**In SigmaOS:** All contributors are evaluated on the quality of their contributions. First-time contributors receive mentorship. The Code of Conduct enforces respectful collaboration.

**Example:** The top three contributors to `sigma-pkg` in v0.2 were first-time open source contributors. They received Contributor status after their third accepted PR.

---

### RESPECT
**Definition:** All community interactions are conducted with respect for fellow contributors, users, and the work itself.

**In SigmaOS:** The Code of Conduct (based on Contributor Covenant) is enforced. Maintainers lead by example. Disrespectful comments are removed and repeated violations result in bans.

**Example:** Code review comments are phrased as questions or suggestions ("Could this be simplified by...?") not commands ("Change this to..."). The style guide includes comment writing guidelines.

---

## Distribution Engineering

---

### PRACTICAL LICENSING
**Definition:** Choose licenses that achieve freedom goals while being practical for adoption and contribution.

**In SigmaOS:** Apache-2.0 for libraries (patent grants), MIT as secondary, GPL-2.0-only for kernel components that require GPL. No LGPL mixing issues.

**Example:** `sigma-common` is Apache-2.0 to allow both GPL and non-GPL projects to use it. `sigma-kernel` patches are GPL-2.0-only to match the Linux kernel.

---

### PRISTINE SOURCES
**Definition:** Keep upstream source code unmodified where possible; apply changes as documented patches.

**In SigmaOS:** All upstream dependencies are used at their released versions. Patches are stored in `patches/` directories as numbered `.patch` files with explanatory headers.

**Example:** SigmaOS's Firefox-based browser uses Firefox ESR as an unmodified base, with branding and config patches in `sigma-browser/patches/`. Upstream updates are applied cleanly.

---

### SOFTWARE SUPPORT LIKE DISTROS
**Definition:** Provide timely security updates and bug fixes for the lifetime of each release, as a distro would.

**In SigmaOS:** Each stable release has a 2-year support window. Security updates are delivered within 72 hours of a CVE fix becoming available.

**Example:** When a critical glibc CVE is fixed upstream, `sigma-update`'s security channel delivers the fix to all SigmaOS stable users within 72 hours.

---

### SOFTWARE UPDATE LIKE DISTROS
**Definition:** Deliver updates atomically, safely, and with rollback capability.

**In SigmaOS:** Updates use btrfs snapshots — the current system is snapshotted before an update, and the update is applied to a new subvolume. A failed update rolls back to the snapshot.

**Example:** `sigma-update apply` creates a new btrfs snapshot, applies all pending updates to it, and sets it as the default boot subvolume. If the user doesn't confirm on next boot, the old snapshot is restored.

---

### STABLE APIs
**Definition:** Published APIs must not break backward compatibility without a deprecation process.

**In SigmaOS:** All D-Bus interfaces, IPC protocols, and library APIs are semver-versioned. Breaking changes require a major version bump and a 6-month deprecation period.

**Example:** When `sigma-update`'s D-Bus interface gained a new method in v1.2, the old interface was preserved as deprecated with a 2-release window before removal.

---

### USER FREEDOM
**Definition:** Users should be able to run, study, modify, and distribute their software without artificial restrictions.

**In SigmaOS:** No telemetry, no forced cloud accounts, no locked bootloader. Users can modify their system without losing support. Binary formats are documented and open.

**Example:** SigmaOS ships without a Secure Boot requirement after initial setup. Users can disable Secure Boot and install unsigned kernels — the installer warns but does not prevent this.

---

### VERSION CONTROL
**Definition:** All source code, configuration, documentation, and scripts must be under version control.

**In SigmaOS:** All 149 component repositories use Git. Build scripts, kernel configs, and packaging specs are all version-controlled alongside source code.

**Example:** The kernel `.config` file for each supported platform is stored in `sigma-kernel/configs/` and versioned. Config changes are reviewed as carefully as code changes.

---

### THEIRS / XTHEIRS
**Definition:** When merging, prefer the upstream/external version of a file when there is a conflict, to minimize divergence from upstream.

**In SigmaOS:** When rebasing SigmaOS patches onto a new upstream version, `git merge -X theirs` is used for non-critical files to minimize the patch set.

**Example:** When rebasing `sigma-browser` Firefox patches onto a new ESR release, the build system files use `theirs` merge strategy to take Firefox's updated build system unchanged.

---

### WORSE IS BETTER
**Definition:** (From Richard Gabriel) — A slightly worse but simpler solution that ships is better than a perfect solution that doesn't.

**In SigmaOS:** When faced with the choice between a theoretically perfect but complex solution and a simpler good-enough solution, SigmaOS defaults to the simpler one.

**Example:** `sigma-dm` (display manager) uses `greetd` + a simple Rust greeter rather than building a custom display manager. greetd is not perfect for all use cases, but it works and ships.
