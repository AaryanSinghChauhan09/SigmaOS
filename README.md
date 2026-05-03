# Σ SigmaOS

Sovereign lattice kernel and **Zenith Desktop** web shell (`index.html`).

- **Repository**: https://github.com/AaryanSinghChauhan09/SigmaOS  
- **Zenith UI**: open `index.html` in a browser or serve the repo root with any static server (`npx serve`, Python `http.server`, etc.).

Modular front-end: `zenith_desktop.css` and ES modules under [`js/zenith/`](js/zenith/) (entry: `js/zenith/main.js`).

**Contributing:** [`CONTRIBUTING.md`](CONTRIBUTING.md) · **API docs:** `make docs-api` (Doxygen → `docs/api/html/`)

**Docs:** [100-item feature backlog](docs/FEATURE_ROADMAP_100.md) · [Competitive gaps (honest baseline)](docs/COMPETITIVE_GAPS.md) · [Wiki sources](docs/wiki/) (auto-sync to [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) when `WIKI_SYNC_TOKEN` is set — see `docs/wiki/README.md`)

**Website:** [GitHub Pages](https://aaryansinghchauhan09.github.io/SigmaOS/) (set Pages source to `/docs` on `main`; see [docs/SITE_FUTURE_IMPROVEMENTS.md](docs/SITE_FUTURE_IMPROVEMENTS.md))

**Future work (meta):** [docs/REPO_FUTURE_IMPROVEMENTS.md](docs/REPO_FUTURE_IMPROVEMENTS.md) · [Wiki: Future Improvements](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Future-Improvements)

**Roadmap (summary):** fundamentals first — build hygiene, tests, secure boot/sandboxing, networking, then UX/SDK. Full phased plan: [docs/wiki/Roadmap.md](docs/wiki/Roadmap.md). **Directory charter:** [docs/MODULAR_LAYOUT.md](docs/MODULAR_LAYOUT.md) · **Low-level kernel style:** [docs/LOW_LEVEL_CODING.md](docs/LOW_LEVEL_CODING.md)
