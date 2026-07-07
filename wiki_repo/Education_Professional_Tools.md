# Education & Professional Tools Roadmap

## 1. Pre-Bundled Software Suites
SigmaOS bundles professional software packages, making them immediately available offline:
- **Mathematical Modeling**: GeoGebra, Scilab, Octave.
- **E-Learning Platforms**: Moodle, flashcard engines, offline training tools.
- **Professional Systems**: ERPNext (Enterprise Resource Planning), Koha (Library Management), GNUCash, QGIS (Geographic Information System).

## 2. Desktop Integration & Zenith Panel Shortcuts
- Pre-configured Zenith desktop icons and launcher shortcuts.
- System services configured to enable background engines (like ERPNext backend) only when active.
- Unified configuration maps so that education packages share locale preferences.

## 3. Sandboxed Executions
To protect the core system, all educational and professional tools execute inside isolated `sigma_sandbox.rs` MicroVMs.
- Home directory file mounts are limited to the user's workspace.
- Databases run in locked SQLite or PostgreSQL instances bound to isolated network namespaces.

## 4. Roadmap Phases
- **Phase 1 (0–3m)**: Define packaging profiles for GeoGebra and Octave.
- **Phase 2 (3–6m)**: Configure local server containers for ERPNext and Koha services.
- **Phase 3 (6–9m)**: Design Zenith GUI launchers and menu trees for professional applications.
- **Phase 4 (9–12m)**: Implement automated backup recovery engines for data directories.

## 5. Contributor Guidelines
- Enforce reproducible builds for all packaged suites.
- Provide descriptive user guides for offline tools.
