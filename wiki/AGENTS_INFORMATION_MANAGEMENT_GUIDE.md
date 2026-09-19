# SigmaOS AI Agent Information Management & Knowledge Base Guide

This guide defines information retrieval protocols, persistent memory recording procedures, wiki synchronization, and context prioritization hierarchies for AI coding agents developing SigmaOS.

---

## 1. Information Prioritization Hierarchy

When resolving conflicts or retrieving information, AI agents must adhere to the following hierarchy:

1. **User Directives (Highest Priority):** The explicit, current instructions provided by the user in the active turn supersede all repository defaults or historical memories.
2. **Repository Source Code State (Ground Truth):** Source files (`src/`, `klib/`, `kernel/`), configuration files, and `Cargo.toml` represent the source of truth for current system implementation.
3. **Persistent Memory Context (Passive Historical Context):** Memories recorded via `initiate_memory_recording` provide intent and historical learnings ("why" a pattern was established), but must yield if codebase files have evolved.

---

## 2. Knowledge Base & Information Retrieval Protocol

### 2.1 Knowledgebase Lookup Tool
Agents should proactively call `knowledgebase_lookup` when:
* Encountering unexpected build, compiler, or dependency errors.
* Seeking specific instructions for bootstrapping tools or testing crates.
* Investigating platform-specific BSD or Linux API translation behaviors.

### 2.2 Offline Distro Knowledge Base Engine
SigmaOS embeds an offline Arch-Wiki style knowledge base engine in `src/distro/wiki_ideas_implementation.rs` (`ArchWikiKnowledgeBaseEngine`). Agents can query or expand this engine for offline troubleshooting articles regarding systemd, Btrfs, ZFS, and OpenBSD pledge/unveil.

---

## 3. Persistent Memory Recording Directives

Before completing any major task or submitting a pull request, agents must invoke `initiate_memory_recording`.

### 3.1 What to Record
* **Subsystem Architecture:** Key structs, public functions, module locations, and `rustc --test` / `cargo test` execution commands.
* **Security & Vulnerability Patterns:** Root causes of security findings (e.g., IPv4 leading-zero SSRF octal differentials, path traversal colon boundary checks) and prevention methods.
* **CI/CD Configuration:** Workflow file parameters, action commit SHA pins, and token permission requirements.

### 3.2 What NOT to Record
* Routine routine work without architectural insights (e.g. "fixed typo in comment").
* Temporary build artifacts or transient debug outputs.

---

## 4. Documentation & Wiki Asset Synchronization

* **In-Tree Troff Man Pages:** Maintain troff manual pages in `docs/man/man1/` (e.g., `sigma-sh.1`) and `docs/man/man8/` (e.g., `sigma-pkg.8`) whenever CLI commands or system services are added or modified.
* **Wiki Sync Utility:** Run `./scripts/sync_wiki.sh` after modifying markdown documentation to automatically synchronize documentation across `WIKI/`, `wiki/`, and `wiki_repo/`.
