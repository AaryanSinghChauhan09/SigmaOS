# AI Agent Archival & Compression Management Guidelines

## 1. Overview & Architecture
This document details AI agent protocols for maintaining compression codecs (`gzip`, `bzip2`, `xz`, `zstd`), archive formats (`tar`, `cpio`, `zip`), streaming package extraction pipelines, and delta update strategies in SigmaOS (`src/package/universal.rs`, `src/sigpkg/universal_oop_system.rs`).

---

## 2. Operational Directives for AI Agents

### 2.1 Compression Codec Implementations
- **Streaming Decompression**: Decompressors for `.deb` (ar/xz/zstd), `.rpm` (cpio/zstd), `.pkg.tar.zst`, and `.apk` (tar.gz) must operate in streaming mode with bounded RAM buffer bounds.
- **Delta Compression Engines**: `ZstdChunkedDeltaStrategy` and `DnfDeltaRpmStrategy` must verify SHA-256 / Dilithium-5 checksums before and after patch application.

### 2.2 Archive Extraction Safety
- **Path Traversal Guard**: AI agents must sanitize all archive file header entries, panicking or rejecting any paths containing relative directory traversal elements (`../`, `..\\`, absolute root overrides).
- **Hardlink & Symlink Resolution**: Symlinks in tar/zip archives must be restricted to target paths inside the designated extraction destination directory.

---

## 3. Related Files
- `src/package/universal.rs`
- `src/sigpkg/universal_oop_system.rs`
- `docs/AI_AGENT_CODECS_MANAGEMENT.md`
- `docs/LINUX_DISTRO_PARITY_CHECKLIST.md`
