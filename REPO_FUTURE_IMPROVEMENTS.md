# Repository — future improvements

Structured suggestions beyond the numbered [FEATURE_ROADMAP_100.md](./FEATURE_ROADMAP_100.md). Use this for **meta-work**: process, quality gates, governance, and cross-cutting engineering. Product features stay in the 100-item backlog; **GitHub Pages** ideas stay in [SITE_FUTURE_IMPROVEMENTS.md](./SITE_FUTURE_IMPROVEMENTS.md).

---

## Kernel & platform

- [ ] **Shard manifest:** generate `KERNEL_SHARDS` (or CMake/Ninja) from a declarative list to wire `drivers/` and `fs/` without manual Makefile drift.
- [ ] **Boot narrative:** one documented path (firmware → loader → kernel entry) with diagrams checked into `docs/`.
- [ ] **Panic / crash:** uniform panic macro, optional `minidump` to serial or buffer; document in wiki.
- [ ] **SMP:** formalize CPU bring-up and IPI story; tests under QEMU with `-smp`.
- [ ] **Freestanding purity audit:** grep for STL headers in `kernel/` and track reduction over time ([LOW_LEVEL_CODING.md](./LOW_LEVEL_CODING.md)).

## Security & supply chain

- [ ] **Threat model** doc (assets, adversaries, out-of-scope) in `docs/`, linked from the wiki [Security Posture](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Security-Posture).
- [ ] **Signed releases:** GitHub Release artifacts + checksum file + optional cosign/sigstore.
- [ ] **SBOM** or dependency inventory for host toolchains and any vendored blobs.
- [ ] **Security policy:** `SECURITY.md` with disclosure contact and supported branches.

## Testing & CI

- [ ] **QEMU integration job:** boot `sigmaos.bin`, timeout, assert golden strings on serial (allow flaky retry).
- [ ] **Coverage:** gcov/llvm-cov for host-compiled unit tests where applicable.
- [ ] **Scheduled workflows:** weekly `main` build + issue on failure.
- [ ] **PR template:** checklist (tests, docs, changelog snippet).

## Documentation & discoverability

- [ ] **CHANGELOG.md** following Keep a Changelog; link from README and Releases.
- [ ] **Architecture decision records (ADRs)** in `docs/adr/` for major choices.
- [ ] **Auto-link** Doxygen HTML from README/Pages when `docs-api` artifact is published to a stable URL.
- [ ] **Glossary:** `sigma_*` terms and “shard” vocabulary for new contributors.

## Developer experience

- [ ] **Dev container** or **Nix flake** for reproducible kernel build on contributor machines.
- [ ] **pre-commit** config (optional): clang-format, black, markdownlint locally.
- [ ] **Issue labels** taxonomy documented in `CONTRIBUTING.md` (`area/*`, `good first issue`).

## Zenith & web

- [ ] **E2E smoke** (Playwright or similar) for critical Zenith paths — run only if Node deps acceptable.
- [ ] **Bundle size / a11y** budget for `js/zenith/` documented in CI comment.

## Community & releases

- [ ] **Quarterly** lightweight release notes even before “1.0” (what built, what broke, what’s next).
- [ ] **Good first issues** curated from refactors and doc tasks.
- [ ] **Code of Conduct** (Contributor Covenant or equivalent) if community grows.

## Metrics (honest progress)

- [ ] **Dashboard** (wiki table or `docs/STATUS.md`) updated manually each milestone: build green, tests count, backlog items “done” with links to PRs.

---

## Keeping this file in sync with GitHub

| Goal | Action |
| :--- | :--- |
| **Repository (source of truth)** | Edit **this file** in `docs/REPO_FUTURE_IMPROVEMENTS.md` and open a PR to [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS). |
| **GitHub Wiki** | The wiki page [Future Improvements](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Future-Improvements) is generated from **`docs/wiki/Future-Improvements.md`**. After merging to `main`, run **Wiki sync** (push to `docs/wiki/**` triggers [`.github/workflows/wiki-sync.yml`](../.github/workflows/wiki-sync.yml) if `WIKI_SYNC_TOKEN` is set). |
| **Local git** | `git add docs/REPO_FUTURE_IMPROVEMENTS.md docs/wiki/Future-Improvements.md` → `git commit` → `git push origin main` |

This document is **not** automatically synced to the wiki full-text (to avoid duplication drift). The wiki holds a **summary + links**; detailed checklists live here in the repo.
