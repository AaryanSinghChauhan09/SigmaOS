# SigmaOS Repository Consolidation Plan

## Overview

Following **CONSOLIDATE similar repos**, **OPTIMISE FOR DELETION**, and **NO DEMO & NO STUB** principles, the hub script identified **1,100+ scaffold repositories** across 60+ families that should be consolidated.

## Consolidation Strategy

### Principles Applied
- **Reversible archival preferred** (archive, don't delete)
- **Verify each group isn't unique work** before consolidation
- **Archive placeholder/scaffold repos** that contain no actual implementation
- **Keep only necessary components** with real working code

### Major Consolidation Families

#### 1. Arch Linux Infrastructure (410 repos)
- **Canonical**: `SigmaOS-arch` (12KB)
- **Scope**: 410 scaffold repos for Arch infrastructure
- **Action**: Archive or consolidate - SigmaOS uses standard Arch repos (UPSTREAM FIRST)
- **Examples**: `SigmaOS-arch-3d-kernel`, `SigmaOS-arch-ai-kernel`, `SigmaOS-arch-am`, ...

#### 2. Mint-Inspired Repos (182 repos)
- **Canonical**: `SigmaOS-mint` (4KB)
- **Scope**: 182 scaffold repos
- **Action**: Archive - violates Arch-based architecture and NORMALISE OS IDENTITY
- **Examples**: `SigmaOS-mint-accessibility`, `SigmaOS-mint-antivirus`, ...

#### 3. Arch Linux Dotfiles (148 repos)
- **Canonical**: `SigmaOS-archlinux-dotfiles` (6KB)
- **Scope**: 148 scaffold repos
- **Action**: Consolidate or archive - use standard Arch configurations
- **Examples**: `SigmaOS-archlinux-aide`, `SigmaOS-archlinux-alsa`, ...

#### 4. Data Analyzer (111 repos)
- **Canonical**: `SigmaOS-data-analyzer` (4KB)
- **Scope**: 111 scaffold repos
- **Action**: Consolidate - most are stubs
- **Examples**: `SigmaOS-data-access-check`, `SigmaOS-data-access-control`, ...

#### 5. Emacs Themes (105 repos)
- **Canonical**: `SigmaOS-emacs` (7KB)
- **Scope**: 105 scaffold repos for theme directories
- **Action**: Consolidate into single theme repo
- **Examples**: `SigmaOS-emacs-3d-ambition-theme-directory`, ...

#### 6. Omarchy-Inspired (85 repos)
- **Canonical**: `SigmaOS-omarchy` (25KB)
- **Scope**: 85 scaffold repos
- **Action**: Archive - violates HIDE ANY REFERENCE TO ANY OTHER PROJECTS
- **Examples**: `SigmaOS-omarchy-ar`, `SigmaOS-omarchy-as`, ...

#### 7. Data Science Tools (40 repos)
- **Canonical**: `SigmaOS-tools` (871KB)
- **Scope**: 40 scaffold repos
- **Action**: Consolidate - use SigmaOS-utils or single tools repo
- **Examples**: `SigmaOS-tools-airbyte`, `SigmaOS-tools-bigquery`, ...

#### 8. System Components (30 repos)
- **Canonical**: `SigmaOS-system` (2.7MB)
- **Scope**: 30 scaffold repos
- **Action**: Consolidate into SigmaOS-utils or dedicated repo
- **Examples**: `SigmaOS-system-admin`, `SigmaOS-system-apis`, ...

#### 9. Kernel Components (29 repos)
- **Canonical**: `SigmaOS-kernel` (40KB)
- **Scope**: 29 scaffold repos
- **Action**: Archive - SigmaOS uses stock Arch kernel
- **Examples**: `SigmaOS-kernel-apis`, `SigmaOS-kernel-call`, ...

#### 10. Theme Components (13 repos)
- **Canonical**: `SigmaOS-theme` (6KB)
- **Scope**: 13 scaffold repos
- **Action**: Already consolidated into `SigmaOS-themes`
- **Examples**: `SigmaOS-theme-gtk3`, `SigmaOS-theme-icon`, ...

## Implementation Plan

### Phase 1: Archive Conflicting Repos
- Archive all Omarchy-inspired repos (violates identity normalization)
- Archive all Mint-inspired repos (violates Arch-based architecture)
- Archive all custom kernel repos (violates UPSTREAM FIRST)

### Phase 2: Consolidate Scaffold Families
- Consolidate data analyzer repos into single `SigmaOS-data-tools`
- Consolidate Emacs theme repos into `SigmaOS-emacs-themes`
- Consolidate system component repos into `SigmaOS-utils`
- Consolidate data science tools into `SigmaOS-data-science`

### Phase 3: Verify Unique Work
- Check each consolidation family for unique implementations
- Preserve any repos with actual working code
- Archive only true scaffolds/stubs

### Phase 4: Update Registry
- Remove archived repos from `COMPONENTS-FOCUSED.toml`
- Add consolidated repos to registry
- Update dependency graph

## Safety Measures

1. **Dry-run first**: Use `./scripts/sigma_hub.sh consolidate --dry-run`
2. **Reversible archival**: Use `gh repo edit --archived true` instead of deletion
3. **Verification**: Manual code review before each archival
4. **Rollback plan**: Keep archive list for restoration if needed

## Expected Outcome

- **Before**: 1,100+ repos (mostly scaffolds)
- **After**: ~50 repos (all with actual working code)
- **Principles satisfied**: CONSOLIDATE, OPTIMISE FOR DELETION, NO DEMO & NO STUB

## Current Status

- ✅ Consolidation plan identified via hub script
- ✅ Archived 4 theme repos (SigmaOS-theme, SigmaOS-theme-icon, SigmaOS-theme-gtk3, SigmaOS-theme-desktop-environment)
- ✅ Archived 4 apt/dpkg repos (SigmaOS-packages-tools, SigmaOS-packages-kit, SigmaOS-daemon-apt, SigmaOS-apturl)
- ✅ Archived 1 player-library repo (SigmaOS-player-library)
- ✅ Updated repo descriptions to remove external references (SigmaOS-bootloader, SigmaOS-chatroom, SigmaOS-audio-tuner, SigmaOS-asdcontrol-git)
- ⏳ Pending: Continue with other consolidation families
- ⏳ Pending: Registry updates

## Repositories Archived (51 total)

1. **Theme duplicates (4):**
   - SigmaOS-theme → SigmaOS-themes (canonical)
   - SigmaOS-theme-icon → SigmaOS-themes
   - SigmaOS-theme-gtk3 → SigmaOS-themes
   - SigmaOS-theme-desktop-environment → SigmaOS-themes

2. **apt/dpkg violations (4):**
   - SigmaOS-packages-tools (violates Arch-based architecture)
   - SigmaOS-packages-kit (violates Arch-based architecture)
   - SigmaOS-daemon-apt (violates Arch-based architecture)
   - SigmaOS-apturl (violates Arch-based architecture)

3. **Player duplicate (1):**
   - SigmaOS-player-library → SigmaOS-player (canonical)

4. **Dev tools duplicate (1):**
   - SigmaOS-dev-tools → SigmaOS-dev (canonical)

5. **Utils consolidation (4):**
   - SigmaOS-folder-color-switcher → SigmaOS-utils
   - SigmaOS-file-writer → SigmaOS-utils
   - SigmaOS-renamer-file → SigmaOS-utils
   - SigmaOS-archive-manager2 → SigmaOS-utils

6. **Arch infrastructure placeholders (10):**
   - SigmaOS-archwiki (use official Arch Wiki)
   - SigmaOS-archmanweb (use official Arch Man Pages)
   - SigmaOS-archlinux-repro (use official Arch repro)
   - SigmaOS-archivetools (use standard tools)
   - SigmaOS-archbbs (use official Arch BBS)
   - SigmaOS-arch-tools (use standard Arch tools)
   - SigmaOS-arch-signoff (use official process)
   - SigmaOS-arch-security-tracker (use official tracker)
   - SigmaOS-arch-repro-status (use official status)
   - SigmaOS-arch-release-promotion (use official process)

7. **Arch kernel/build placeholders (10):**
   - SigmaOS-arch-rebuild-order (use standard tools)
   - SigmaOS-arch-historical-archive (use standard tools)
   - SigmaOS-arch-boxes (use standard tools)
   - SigmaOS-arch-directx-kernel (use stock Arch kernel)
   - SigmaOS-arch-desktop-kernel (use stock Arch kernel)
   - SigmaOS-arch-log (use standard tools)
   - SigmaOS-arch-deps (use standard tools)
   - SigmaOS-arch-lock (use standard tools)
   - SigmaOS-arch-llvm (use standard tools)
   - SigmaOS-arch-dependency-manager (use standard tools)

8. **Arch config placeholder (1):**
   - SigmaOS-conf-archlinux-org (use official Arch config)

9. **Arch language/tool placeholders (15):**
   - SigmaOS-arch-defi-kernel, SigmaOS-arch-laptop-kernel, SigmaOS-arch-default-kernel
   - SigmaOS-arch-decentralization-kernel, SigmaOS-arch-javascript-kernel
   - SigmaOS-arch-list, SigmaOS-arch-lisp, SigmaOS-arch-ksh, SigmaOS-arch-kotlin
   - SigmaOS-arch-db, SigmaOS-arch-database, SigmaOS-arch-java, SigmaOS-arch-dash
   - SigmaOS-arch-iso, SigmaOS-arch-dart (use standard Arch tools/stock kernel)

Generated with [Devin](https://devin.ai)
