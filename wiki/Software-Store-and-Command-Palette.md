# Software Store and Command Palette

**Capability state: Prototype.** Catalog and launcher models are not integrated software installation or desktop action services. See the [shared status vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

`src/desktop/mint_software_store.rs` is a catalog/search and permission-model prototype inspired by Linux Mint Software Manager. Its seeded listings are illustrative and deliberately mark versions, sizes, ratings, and developer verification as unknown or unavailable. The component does not fetch repository metadata, verify package signatures, download packages, install/remove files, or enforce sandbox permissions.

Install, remove, and batch-install methods fail clearly when no transactional backend exists, and leave installed state unchanged. Delta-size estimation returns unavailable without measured package-diff data.

`src/desktop/launcher.rs` implements an in-memory keyboard-palette query model. An empty query ranks the eight most-used applications first, breaking equal-use ties by name for predictable keyboard navigation; non-empty queries rank app/window/action/clipboard matches and basic arithmetic. It does not execute commands, launch apps, switch compositor windows, or connect to a system clipboard service. Clipboard data remains in memory only; callers can clear history explicitly.

The launcher regression suite covers UTF-8-safe previews, stable unique history IDs through eviction and clearing, zero-capacity behavior, usage-ranked empty queries, result limits, stable tie ordering, multi-mode query behavior, and calculator behavior:

```sh
rustc --edition=2021 --test src/desktop/launcher.rs -o /tmp/sigmaos_launcher_tests
/tmp/sigmaos_launcher_tests
```

The software-store module tests can be run independently with:

```sh
rustc --edition=2021 --test src/desktop/mint_software_store.rs -o /tmp/sigmaos_store_tests
/tmp/sigmaos_store_tests
```

## Design references

- **Arch Linux:** keep package recipes, dependencies, provenance, and system changes inspectable. Never show catalog metadata as proof a package is installed or trusted.
- **Linux Mint:** make onboarding, permissions, update impact, and recovery understandable. Present unknown metadata honestly and provide a clear way to remove sensitive clipboard history.
- **Omarchy:** make keyboard-first application discovery and documented commands easy to reach. Bind palette results to real service APIs before making an action executable.

## Roadmap

1. Define signed repository metadata, reproducible package recipes, architecture policy, and provenance.
2. Build a transactional package backend with signature/dependency verification, staged writes, cancellation, rollback, and installed-state reconciliation.
3. Wire the command palette to app launch, compositor window focus, clipboard, and privileged system-action APIs. Do not pass shell command strings to a shell.
4. Add user-visible permission review, clipboard retention controls, accessibility review, keyboard-only workflows, and actionable error reporting.
5. Validate clean install, package update/removal rollback, offline recovery, and desktop first-login on the supported QEMU target before claiming desktop readiness.

Package installation, sandbox enforcement, clipboard-service integration, command execution, and a usable graphical desktop are not currently supported. Treat Linux and BSD distributions as design references, and promote each capability only after runtime wiring and relevant test evidence exist.
