# 🔤 SigmaOS Icon Font Migration & SHA-256 Hash Verification (`font_migration`) Strategic Development Plan

## Executive Summary & Design Vision

System font assets—especially specialized icon and symbol fonts like `omarchy.ttf`—provide critical UI glyphs for desktop panels, status applets, and terminal prompts. During system refactoring, font assets migrate from static userland configuration trees (`config/omarchy.ttf`) into dedicated system packages (`omarchy-settings` or `omarchy-fonts`).

To prevent font corruption, missing glyphs, or fake mocked binaries during system upgrade migrations, the **SigmaOS Icon Font Migration Subsystem** (`SigmaFontMigrationEngine`, `FontSha256Validator`) utilizes exact-content SHA-256 cryptographic verification against original stock font binaries.

---

## 1. Stock Icon Font Specifications & Cryptographic Signature

| Attribute | Value & Specification |
| :--- | :--- |
| **Original Asset Path** | `config/omarchy.ttf` (at commit `babfafa5^`, prior to settings package migration) |
| **Asset Format** | TrueType Font Binary (`.ttf`) |
| **SHA-256 Checksum** | `e55e67119e82f56f92d90cbf54b7ccc1b2946b32c535a29370439d7ef5215966` |
| **Verification Strategy** | Migration tests exercise the **real font binary** to validate the exact-content SHA-256 guard without mocking `sha256sum`. |

---

## 2. Architecture & Migration Verification Protocol

```text
┌───────────────────────────────────────────────────────────────────────────┐
│              Legacy System State (`config/omarchy.ttf`)                   │
│          SHA-256: `e55e67119e82f56f92d90cbf54b7ccc1b2946b32c5...`          │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               `SigmaFontMigrationEngine` Verification Guard                │
│     - Calculates SHA-256 checksum of existing font asset                  │
│     - Verifies exact match against expected `e55e6711...` hash             │
│     - Prevents migration if font asset is corrupted or tampered            │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               Package-Backed Font Asset Relocation & Symlink              │
│     - Installs package asset: `/usr/share/fonts/TTF/omarchy.ttf`          │
│     - Creates compatibility symlink: `~/.local/share/fonts/omarchy.ttf`   │
│     - Triggers font cache refresh: `fc-cache -f`                          │
└───────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Key Migration Rules & Testing Principles

### 3.1 Unmocked `sha256sum` Verification
- Migration tests use the real `config/omarchy.ttf` binary rather than mock stubs to verify that `sha256sum` verification accurately identifies valid vs corrupted font assets.

### 3.2 Idempotent Relocation
- Running font migration scripts multiple times is strictly idempotent—if `/usr/share/fonts/TTF/omarchy.ttf` already matches the expected SHA-256 digest, no redundant file copies occur.

### 3.3 Font Cache Refresh (`fc-cache`)
- Following font file relocation, `SigmaFontMigrationEngine` invokes `fc-cache -f -v` to ensure Fontconfig immediately exposes updated icon glyphs to Wayland applications, terminal emulators, and status bars without requiring desktop re-login.

---

## 4. Phased Development Roadmap

### Phase 1: SHA-256 Validator & Real-Asset Verification (Q4 2026)
- Build `FontSha256Validator` verifying `config/omarchy.ttf` against digest `e55e67119e82f56f92d90cbf54b7ccc1b2946b32c535a29370439d7ef5215966`.
- Add unmocked `sha256sum` verification test runner.

### Phase 2: Package-Backed Font Relocation & Symlink Engine (Q1 2027)
- Deploy font asset migration scripts re-pointing `~/.config/omarchy/` font paths to system `/usr/share/fonts/TTF/`.
- Add Fontconfig `fc-cache` auto-refresh trigger post-migration.

### Phase 3: Desktop Font Fallback & Icon Alignment (Q2 2027)
- Configure font fallbacks in `OmarchyGhosttyTerminalEngine` and `OmarchyStarshipPromptConfigEngine`.
- Ensure status bar glyphs render correctly with exact-hash verified icon fonts.

### Phase 4: CI Integration & Integrity Benchmarks (Q3 2027+)
- Integrate font asset SHA-256 migration checks into `./scripts/verify.sh`.
- Measure Fontconfig cache refresh latency (< 100ms).

---

## 5. Verification & Testing Standards

All font migration components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/distro/omarchy.rs` (Font configuration & theme integration)
- `src/security/input_validation.rs` (Font file path & checksum sanitization)
