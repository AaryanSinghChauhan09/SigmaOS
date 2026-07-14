# SigmaOS Documentation Index

Welcome to the `docs/` directory — the canonical source of engineering documentation for the SigmaOS project.

This directory contains design documents, specifications, coding standards, and build guides. Finalized high-level pages are also mirrored to the [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki).

---

## Quick Navigation

### 🏗️ Building & Setup

| Document | Description |
| -------- | ----------- |
| [Building-from-Source.md](Building-from-Source.md) | Full toolchain setup and build instructions |
| [Coding-Standards.md](Coding-Standards.md) | Code style rules for Rust, Nim, Zig, C |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Documentation-specific contribution guide |

### 📐 Design Specifications

| Document | Description |
| -------- | ----------- |
| [design/sigmapkg.md](design/sigmapkg.md) | Sigma Package Manager architecture |
| [pr_template.md](pr_template.md) | Pull request template reference |

### 📋 Audits & Backlogs

| Document | Description |
| -------- | ----------- |
| [doc_audit_backlog.md](doc_audit_backlog.md) | Tracked stubs, TODO docs, and gaps |

---

## Documentation Architecture

```
docs/
├── README.md                   ← this file
├── CONTRIBUTING.md             ← docs contribution guide
├── Building-from-Source.md     ← toolchain & build
├── Coding-Standards.md         ← coding style rules
├── doc_audit_backlog.md        ← tracked doc gaps
├── pr_template.md              ← PR template reference
└── design/
    └── sigmapkg.md             ← package manager design
```

---

## Key Principles

SigmaOS follows a **wiki-first documentation strategy**:

1. **In-repo `docs/`**: Engineering specs, design decisions, build guides, and contribution guidelines — content that lives close to the code.
2. **GitHub Wiki (`wiki_repo/`)**: High-level conceptual documentation, roadmaps, guides, and user-facing pages.
3. **Module READMEs**: Every module directory under `modules/` contains a `README.md` explaining purpose, API, and roadmap.

---

## Related Resources

- 🌐 [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) — Full documentation portal
- 📜 [ARCHITECTURE.md](../ARCHITECTURE.md) — Repository-root architecture overview
- 🔒 [SECURITY.md](../SECURITY.md) — Security policy and vulnerability reporting
- 🤝 [CONTRIBUTING.md](../CONTRIBUTING.md) — Root contribution guide
- ❓ [FAQ.md](../FAQ.md) — Frequently asked questions
