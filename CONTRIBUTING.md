# Contributing to SigmaOS

Thank you for helping improve SigmaOS. This document points to the conventions and automation used in this repository.

## Where to start

- Read the **[Developer Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/BuildGuide)** (published copy: [GitHub Wiki — Developer Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Developer-Guide)).
- For the browser shell, see **[Zenith Desktop](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Zenith-Desktop)** and `zenith_desktop.js`.

- For scope and honesty about what is implemented vs. aspirational, see `https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/INDUSTRIAL_GAP_RESOLUTION` and **[Roadmap](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Home)**.

## C and C++ (kernel / core)

- Format with **clang-format** using the repo root [`LICENSE`](LICENSE).

- Pull requests are checked for style on **changed** `.cpp` / `.h` / `.hpp` files (see `.github/workflows/ci.yml`).

- Prefer **`sigma_status`** + [`include/sigma_types.h`](include/sigma_types.h) over exceptions (kernel uses `-fno-exceptions`). See [`https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture`](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture).

- Host smoke test: `make check-host` (compiles `tests/cpp_host` against headers).

- Static analysis: `cppcheck` and `clang-tidy` run in existing quality workflows (see `.github/workflows/sigma_quality.yml`).

Generate API HTML locally:

```bash

sudo apt install doxygen graphviz   # optional: graphviz for diagrams
doxygen Doxyfile

# Open docs/api/html/index.html

```

## Python (host scripts)

- Format with **Black** using [`pyproject.toml`](pyproject.toml) (`scripts/`, `tools/`, root `fix_includes.py`). The `userland/` tree is excluded until you opt in.

- CI: `.github/workflows/python_quality.yml` (`black --check`, `pytest`).

## JavaScript (Zenith Desktop)

- Modules live under `zenith_desktop.js`; the entry point is `zenith_desktop.jsmain.js`.

- Inline `onclick` handlers in `index.html` require corresponding exports on `window` in `main.js`.

## Documentation and wiki

- Wiki **source** pages live in **`docs/wiki/`**.

- Pushes to the default branch sync the wiki when **`WIKI_SYNC_TOKEN`** is configured (see `docs/wiki/README.md`).

- Large design matrices and backlogs stay in `docs/`; the wiki links to them where useful.

- Cross-cutting **future improvements** (CI, security process, tooling): [`docs/REPO_FUTURE_IMPROVEMENTS.md`](docs/REPO_FUTURE_IMPROVEMENTS.md).

## Issues and PRs

Use the GitHub issue templates for bugs and features. In PRs, describe motivation, testing performed, and any user-visible or build changes.

## Development Setup & Verification

To verify the boot process locally in QEMU:
1. Ensure you have `build-essential`, `nasm`, and `qemu-system-x86` installed.
2. Run `./qemu-boot.sh` from the repository root.
   - This script will build the kernel.
   - It will launch QEMU headlessly and pipe the serial output to `serial.log`.
   - It will automatically verify that the `"SOVEREIGN BOOT"` message appears, validating a successful boot.
