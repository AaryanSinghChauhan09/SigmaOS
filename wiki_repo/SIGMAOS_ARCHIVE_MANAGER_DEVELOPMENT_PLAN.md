# 📦 SigmaOS Multi-Format Archive Manager & Compression Suite (`archive_manager`) Strategic Development Plan

## Executive Summary & Design Vision

Archive managers in modern operating systems are vital utilities enabling users and system services to compress, inspect, encrypt, split, and extract files across diverse archive formats. System applications, package managers (`sigpkg`), software update engines, and backup utilities depend on high-throughput, secure archive processing.

Drawing inspiration from open-source archive tools (**KDE Ark**, **GNOME File Roller / Engrampa**, **7-Zip**, **PeaZip**) and BSD system libraries (**FreeBSD `libarchive` / `bsdtar`**), the **SigmaOS Archive Manager** (`ArchiveCompressorUtility`, `SevenZipEngine`, `SovereignArchivalSuite`) provides a zero-dependency, Safe Rust streaming compression and extraction engine supporting `.tar.zst`, `.tar.xz`, `.tar.gz`, `.tar.bz2`, `.7z`, `.zip`, `.cpio`, `.apk`, and `.deb` archives with path-traversal sandboxing.

---

## 1. Multi-Distro & Multi-OS Archive Inspirations

### 1.1 FreeBSD `libarchive` & `bsdtar` (Universal Format Auto-Detection)
- **Inspirations**:
  - **Magic-Header Auto-Detection**: Inspecting leading magic byte signatures (`7z\xBC\xAF\x27\x1C`, `PK\x03\x04`, `\x28\xB5\x2F\xFD` for Zstd, `\xFD7zXZ\x00` for XZ) to auto-select decompressors regardless of file extension mismatches.
  - **POSIX Tar & CPIO Stream Parser**: Zero-allocation streaming file entry header parsing (`ustar`, `pax`, `cpio-odc`, `cpio-newc`).
- **SigmaOS Integration**: Magic signature format detection in `src/productivity/utility_suite.rs` and `src/sigpkg/universal_adapter.rs`.

### 1.2 KDE Ark & PeaZip (Multi-Volume Split & Encrypted Archives)
- **Inspirations**:
  - **AES-256 Multi-Volume Spanning**: Splitting large compressed payloads (`.7z.001`, `.7z.002` or `.z01`, `.z02`) across user-defined volume size limits (e.g., 4GB, 700MB, or custom MB byte chunk limits).
  - **AES-256 Header & Payload Encryption**: Password-protected archive creation with PBKDF2/Argon2 key derivation.
- **SigmaOS Integration**: `SevenZipEngine` multi-volume split archive builder with `ArchiveVolume` chunking in `src/productivity/utility_suite.rs`.

### 1.3 Arch Linux & Alpine Linux Package Compression (`zstd` & `xz`)
- **Inspirations**:
  - **Parallel Multithreaded Zstandard (`zstd -T0`)**: Ultra-fast decompression throughput (> 1.5 GB/s) used in Arch `.pkg.tar.zst` and Alpine `.apk` (APK v3) package decompression.
  - **High-Ratio LZMA2 (`xz -9`)**: Maximum compression ratio for system distribution ISO images and source tarballs.
- **SigmaOS Integration**: `ArchPacmanDeltaSyncEngine` and package decompression pipelines in `src/package/`.

### 1.4 Path Traversal & Zip Slip Security Sandboxing (OpenBSD `unveil` & Landlock)
- **Inspirations**:
  - **Zip Slip Vulnerability Mitigation**: Rejecting embedded directory traversal paths (`../../etc/shadow`, Absolute paths `/usr/bin/malicious`) during extraction.
  - **Symlink Sanity Checks**: Ensuring symlinks in archives do not escape the designated target extraction directory boundary.
- **SigmaOS Integration**: `validate_path` and `validate_filename` in `src/security/input_validation.rs` and OpenBSD `unveil` extraction guards in `src/security/pledge.rs`.

---

## 2. Core Architectural Subsystems

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                 Zenith GUI Archive Manager & `sigma-sh` CLI               │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                 Archive Manager Format Router & Detector                  │
│     - Magic Signature Detector (7z, Zip, Tar, Zstd, XZ, Gzip, Bzip2, Cpio) │
│     - Format Extension Normalizer (.tar.zst, .tgz, .txz, .7z.001)         │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               Streaming Decompressor & Compression Engine                 │
│     - Parallel Zstd & LZMA2 Compression Thread Pool                       │
│     - Encrypted Multi-Volume Split Archive Manager (`ArchiveVolume`)      │
│     - In-Memory Preview & Single File Selective Extraction                │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               Path Traversal & Extraction Security Guard                  │
│     - Zip Slip & Traversal Guard (`..` and absolute path rejection)        │
│     - Symlink Target Boundary Verification (`unveil` sandbox)             │
│     - Permission & POSIX Mode Sanitizer                                  │
└───────────────────────────────────────────────────────────────────────────┘
```

### 2.1 Extraction Safety Guarantee
- Every extracted file entry path is canonicalized and validated against the target destination directory prefix.
- Absolute paths (`/usr/bin/...`) or paths containing parent directory segments (`../../`) trigger immediate extraction aborts with `SecurityError::PathTraversalAttempt`.

### 2.2 Throughput Performance Target
- Decompression throughput target for `.tar.zst`: **> 1,000 MB/s**.
- Compression throughput for multi-threaded `.7z` / `.zip`: **> 150 MB/s**.

---

## 3. Phased Development Roadmap

### Phase 1: Native Magic Detection & Tar/Zip/7z Engine (Q4 2026)
- Stabilize magic header signature detection in `src/productivity/utility_suite.rs`.
- Complete `.tar`, `.zip`, and `.7z` basic archive creation and entry enumeration.
- Enforce strict Zip Slip path traversal extraction guards.

### Phase 2: Parallel Zstd/LZMA2 & Split Multi-Volume Archives (Q1 2027)
- Implement `SevenZipEngine` multi-volume volume chunking (`ArchiveVolume`) with custom size limits.
- Integrate multithreaded Zstandard (`zstd`) decompression pipeline in `src/package/`.
- Add AES-256 header and content payload password encryption support.

### Phase 3: Zenith GUI Archive Manager & Context Menu Integration (Q2 2027)
- Build Zenith GTK/Wayland GUI Archive Manager interface (file list, drag-and-drop extraction, in-memory file preview).
- Register Thunar custom context menu actions (`Extract Here`, `Extract To...`, `Create Compressed Archive...`).
- Add archive integrity checksum verification (`CRC32`, `SHA-256`).

### Phase 4: Package Manager Integration & Benchmarks (Q3 2027+)
- Connect `ArchiveCompressorUtility` directly with `sigpkg` for zero-overhead package extraction.
- Include archive compression/decompression benchmark suite in `scripts/tech_media_benchmark_suite.sh`.
- Conduct fuzz testing against archive format parsers (`cargo fuzz`).

---

## 4. Verification & Testing Standards

All archive manager components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/productivity/utility_suite.rs` (`SevenZipEngine` multi-volume creation & `ArchiveCompressorUtility`)
- `src/sigpkg/universal_adapter.rs` (package archive magic header parsing)
- `src/security/input_validation.rs` (path traversal Zip Slip rejection)
- `src/security/pledge.rs` (unveil extraction boundary sandboxing)
