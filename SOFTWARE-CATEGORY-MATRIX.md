# SigmaOS Software Category Matrix

Canonical category breakdowns for the operating-system, office-suite, data, and
tooling families SigmaOS targets. This page consolidates the category taxonomies
kept uniform across the GitHub repository and this wiki
(**UNIFORM INFORMATION IN GITHUB REPO & GITHUB WIKI**, **AVOID FRAGMENTATION**).

Related canonical pages:

- BSD subsystem parity: [[BSD-Universal-Synthesis-and-Parity]]
- Linux distro parity: [[Linux-Universal-Distro-Parity]]
- Craft suite (20 modules): [[Craft-Creative-Suite-Overview]]
- Sub-repository catalog (150 repos): [[Sub-Repositories-and-Components]]

---

## BSD

`BSD = DragonFly; Free; Open`

| BSD | Wiki Page |
| :--- | :--- |
| DragonFly BSD | [[DragonFlyBSD-Ecosystem-Parity]] |
| FreeBSD | [[FreeBSD-Ecosystem-Parity]] |
| OpenBSD | [[OpenBSD-Ecosystem-Parity]] |

Debian / niche BSD family also tracked by the parity matrix:
`Ghost; Nomad; Midnight; Hardened; Liberty; Fury; Desktop; Karma; Star`
(see [[BSD-Universal-Synthesis-and-Parity]]).

---

## Craft

`Craft = 3D; Ani; Art; CAD; CraftLibs; Deck; Design; Effect; Fig; Film; Font;
Grid; Light; PDF; Photo; Print; Sound; Vector; Voice; Word`

All 20 modules are documented with full tables in
[[Craft-Creative-Suite-Overview]] and linked from [[Home]].

---

## Linux

`Linux = Arch; Cachy; Debian; ElementaryOS; EndeavourOS; Fedora; Gentoo; Kali;
Mint; Omarchy; Parrot; Qubes; ReactOS; RedoxOS; Slackware; SUSE; Ubuntu`

Per-distro parity pages:

| Distro | Status |
| :--- | :--- |
| Arch Linux | [[Arch-Linux-Ecosystem-Parity]] |
| CachyOS | [[CachyOS-Ecosystem-Parity]] |
| Debian | [[Debian-Ecosystem-Parity]] |
| ElementaryOS | tracked in [[Linux-Universal-Distro-Parity]] |
| EndeavourOS | tracked in [[Linux-Universal-Distro-Parity]] |
| Fedora | tracked in [[Linux-Universal-Distro-Parity]] |
| Gentoo | tracked in [[Linux-Universal-Distro-Parity]] |
| Kali | tracked in [[Linux-Universal-Distro-Parity]] |
| Linux Mint | tracked in [[Linux-Universal-Distro-Parity]] |
| Omarchy | tracked in [[Linux-Universal-Distro-Parity]] |
| Parrot | tracked in [[Linux-Universal-Distro-Parity]] |
| Qubes | tracked in [[Linux-Universal-Distro-Parity]] |
| ReactOS | [[PR_PROPOSAL_REACTOS_MISSING_COMPONENTS_PARITY]] |
| RedoxOS | tracked in [[Linux-Universal-Distro-Parity]] |
| Slackware | tracked in [[Linux-Universal-Distro-Parity]] |
| SUSE | tracked in [[Linux-Universal-Distro-Parity]] |
| Ubuntu | tracked in [[Linux-Universal-Distro-Parity]] |

Debian / BSD family: `Ghost; Nomad; Midnight; Hardened; Liberty; Fury;
Desktop; Karma; Star` (see BSD section above).

---

## DOS

`DOS = Free; MS; BASIC; ReactOS`

| System | Notes |
| :--- | :--- |
| FreeDOS | Open-source DOS target for legacy software and firmware setup tools |
| MS-DOS | Historical compatibility reference (no code shipped) |
| BASIC | GW-BASIC / QBasic-era interpreter compatibility notes |
| ReactOS | [[PR_PROPOSAL_REACTOS_MISSING_COMPONENTS_PARITY]] |

---

## Kernel

`KERNEL = Linux; GNU; Minix; ReactOS; XNU`

| Kernel | Notes |
| :--- | :--- |
| Linux | Primary syscall/ABI parity reference (see [[41-Linux-BSD-Kernel-Security-And-Virtualization-AI-Agent-Roadmap]]) |
| GNU (Hurd) | Multi-server design lessons for the modular microkernel |
| Minix | Microkernel pedagogy; capability & process isolation reference |
| ReactOS | NT-driver-model compatibility reference (see DOS section) |
| XNU | Mach-O / BSD-layer hybrid reference (Mach messaging, archives) |

---

## LibreOffice

`LIBREOFFICE = Writer; Calc; Impress; Draw; Math; Base`

| App | Craft counterpart |
| :--- | :--- |
| Writer | [[WordCraft-Typographic-Document-Processor]] |
| Calc | [[GridCraft-Reactive-Spreadsheets]] |
| Impress | [[DeckCraft-Presentations]] |
| Draw | [[VectorCraft-Vector-Illustration]] / [[ArtCraft-Digital-Painting]] |
| Math | [[FigCraft-Technical-Diagrams]] formula tooling |
| Base | embedded datastore (planned; see [[Sub-Repositories-and-Components]]) |

---

## OpenOffice

`OPENOFFICE = Writer; Calc; Impress; Draw; Math; Base`

Same six applications as LibreOffice — SigmaOS targets **open standard and
interoperability** (ODF 1.3, ODF/DOCX/XLSX round-trip) through the same Craft
counterparts listed above. One implementation, two upstream families
(**DON'T REPEAT YOURSELF**).

---

## RDBMS

`RDBMS = PostgreSQL; DuckDB; DBearer; Apache; Superset; Metabase`

| System | Role in SigmaOS |
| :--- | :--- |
| PostgreSQL | Primary transactional datastore (SQL:2016 parity) |
| DuckDB | Embedded analytical (OLAP) engine for local telemetry |
| DBearer | Schema-aware bearer/data access tooling |
| Apache (Drill/Arrow) | Vectorized in-memory analytics interchange |
| Superset | Dashboard / visualization front-end |
| Metabase | Lightweight no-SQL-query BI front-end |

---

## Tools

`Tools = Blender; Cline; CodexCLI; Command Code; CrewAI; DeepSeek Coder;
Docker; Hermes Agent; LangChain; Notepad++; OpenClaw; VLC; Warp`

| Tool | Category |
| :--- | :--- |
| Blender | 3D creation suite (parity target of [[3DCraft-Modeling-and-Rendering]]) |
| Cline | Agentic coding assistant |
| CodexCLI | Terminal coding agent |
| Command Code | Command-driven code assistant |
| CrewAI | Multi-agent orchestration |
| DeepSeek Coder | Code model integration |
| Docker | Container runtime (see container subsystem engines) |
| Hermes Agent | On-device agent runtime |
| LangChain | Agent/tool chain framework |
| Notepad++ | Text editing (parity target of [[WordCraft-Typographic-Document-Processor]] editing mode) |
| OpenClaw | Open agent tooling |
| VLC | Media playback (parity target of player subsystem) |
| Warp | Modern terminal (parity target of [[27-SigmaOS-CLI-Development-Plan]]) |

---

*Governed by the periodic PR-only component-rule system
([[component-rules-README]]). Update this page whenever a category gains or
loses a member so the repository and wiki stay uniform.*
