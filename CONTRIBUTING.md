# Contributing to SigmaOS

Thank you for helping improve SigmaOS. This document points to the conventions and automation used in this repository.

## Where to start

- Read the **[Developer Guide](docs/wiki/Developer-Guide.md)** (published copy: [GitHub Wiki — Developer Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Developer-Guide)).
- For the browser shell, see **[Zenith Desktop](docs/wiki/Zenith-Desktop.md)** and `js/zenith/`.
- For scope and honesty about what is implemented vs. aspirational, see `docs/COMPETITIVE_GAPS.md` and **[Roadmap](docs/wiki/Roadmap.md)**.

## C and C++ (kernel / core)

- Format with **clang-format** using the repo root [`.clang-format`](.clang-format).
- Pull requests are checked for style on **changed** `.cpp` / `.h` / `.hpp` files (see `.github/workflows/sigma_style.yml`).
- Prefer **`sigma_status`** + [`include/sigma_result.h`](include/sigma_result.h) over exceptions (kernel uses `-fno-exceptions`). See [`docs/LOW_LEVEL_CODING.md`](docs/LOW_LEVEL_CODING.md).
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

- Modules live under `js/zenith/`; the entry point is `js/zenith/main.js`.
- Inline `onclick` handlers in `index.html` require corresponding exports on `window` in `main.js`.

## Documentation and wiki

- Wiki **source** pages live in **`docs/wiki/`**.
- Pushes to the default branch sync the wiki when **`WIKI_SYNC_TOKEN`** is configured (see `docs/wiki/README.md`).
- Large design matrices and backlogs stay in `docs/`; the wiki links to them where useful.
- Cross-cutting **future improvements** (CI, security process, tooling): [`docs/REPO_FUTURE_IMPROVEMENTS.md`](docs/REPO_FUTURE_IMPROVEMENTS.md).

## Issues and PRs

Use the GitHub issue templates for bugs and features. In PRs, describe motivation, testing performed, and any user-visible or build changes.
