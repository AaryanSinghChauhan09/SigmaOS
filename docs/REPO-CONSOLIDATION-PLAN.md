# SigmaOS Component Consolidation Plan

## Analysis of Existing SigmaOS Repositories

Based on examination of GitHub repositories, the following SigmaOS-* repos have real code:

### Theming (3 repos)
- **SigmaOS-theme** - Theming, Design Tokens & Color Palettes (Rust)
- **SigmaOS-theme-icon** - Icon theme pack
- **SigmaOS-theme-gtk3** - GTK3 theme engine
- **SigmaOS-theme-desktop-environment** - Desktop-shell theme

**Arch/Omarchy Approach**: These should be consolidated into a single "SigmaOS-themes" repo with subdirectories for icons, GTK3, and desktop themes. Omarchy Linux uses a similar pattern with separate theme packages but they're all in one ecosystem.

### Desktop (3 repos)
- **SigmaOS-greeter** - Login greeter (Python/Rust)
- **SigmaOS-desktop-portal** - XDG desktop portal backend
- **SigmaOS-deskland** - Web browser-based desktop (Rust/HTML)

**Arch/Omarchy Approach**: 
- Greeter and desktop-portal are essential - keep separate
- Deskland is experimental web-based desktop - could be consolidated into a "SigmaOS-experimental" repo or kept if it has clear purpose

### Package Management (3 repos)
- **SigmaOS-packages-tools** - Package install/remove CLI (Python/Rust)
- **SigmaOS-packages-kit** - apt/dpkg transaction kit
- **SigmaOS-daemon-apt** - Privileged apt D-Bus daemon
- **SigmaOS-apturl** - apt:// URL handler

**Arch/Omarchy Approach**: Arch uses pacman, not apt. These appear to be Debian/Ubuntu-inspired. For an Arch-based SigmaOS, these should be:
- DELETE packages-kit, daemon-apt, apturl (Debian-specific)
- RENAME packages-tools to "SigmaOS-pacman-tools" or consolidate into main SigmaOS repo

### Applications (3 repos)
- **SigmaOS-office** - Office suite (Python/JS/HTML/CSS)
- **SigmaOS-player** - Media player (Python/Rust)
- **SigmaOS-player-library** - Media player library

**Arch/Omarchy Approach**: 
- Office suite: Use LibreOffice from Arch repos, not custom build
- Media player: Use mpv/VLC from Arch repos, not custom build
- RECOMMENDATION: DELETE these repos unless they provide unique SigmaOS-specific value

### Utilities (7 repos)
- **SigmaOS-file-writer** - Text editor (Python/Rust)
- **SigmaOS-folder-color-switcher** - Folder colors (Python/Rust)
- **SigmaOS-renamer-file** - File renamer (Python/Rust)
- **SigmaOS-domain-blocker** - Hosts blocker (Python/Rust)
- **SigmaOS-chatroom** - LAN chat (Python/Rust)
- **SigmaOS-automate** - Task automation (Python/Rust)
- **SigmaOS-audio-tuner** - Audio tuning
- **SigmaOS-asdcontrol-git** - Hard drive control

**Arch/Omarchy Approach**: These are small utilities. Omarchy has similar tools. They should be:
- Consolidated into a single "SigmaOS-utils" repo with separate subdirectories
- Or kept as separate repos if they're substantial enough

### Development Tools (5 repos)
- **SigmaOS-dev-tools** - Developer toolchain helpers (Python/Rust)
- **SigmaOS-fish** - Fish config & plugins (Python/Rust)
- **SigmaOS-agent-cli** - AI agent CLI
- **SigmaOS-quickshell** - Quick shell runner
- **SigmaOS-workspace** - Workspace management (Rust)
- **SigmaOS-skills** - AI agent skills

**Arch/Omarchy Approach**:
- Fish config: Should be in main SigmaOS repo (dotfiles)
- Dev tools: Consolidate into main SigmaOS repo or single "SigmaOS-dev" repo
- Agent tools: Keep separate if they're substantial
- Skills: Consolidate into single "SigmaOS-skills" repo

### System Components (3 repos)
- **SigmaOS-rollback** - BTRFS snapshot management
- **SigmaOS-permissions** - App permission model
- **SigmaOS-migration** - System migration tools
- **SigmaOS-media-codecs** - Media codec support
- **SigmaOS-office-interop** - Office interoperability

**Arch/Omarchy Approach**:
- Rollback: Essential for Arch system - keep
- Permissions: Could be consolidated with main SigmaOS
- Migration: Could be consolidated with main SigmaOS
- Media codecs: Use Arch packages, not custom repo
- Office interop: Use Arch packages, not custom repo

## Consolidation Plan

### Phase 1: Delete Debian/Ubuntu-Inspired Repos (4 repos)
- DELETE: SigmaOS-packages-kit (apt-specific)
- DELETE: SigmaOS-daemon-apt (apt-specific)
- DELETE: SigmaOS-apturl (apt-specific)
- DELETE: SigmaOS-media-codecs (use Arch packages)
- DELETE: SigmaOS-office-interop (use Arch packages)

### Phase 2: Consolidate Theming (4 repos → 1 repo)
- CREATE: SigmaOS-themes
- MOVE: SigmaOS-theme → SigmaOS-themes/core
- MOVE: SigmaOS-theme-icon → SigmaOS-themes/icons
- MOVE: SigmaOS-theme-gtk3 → SigmaOS-themes/gtk3
- MOVE: SigmaOS-theme-desktop-environment → SigmaOS-themes/desktop
- DELETE: Original repos after migration

### Phase 3: Consolidate Small Utilities (7 repos → 1 repo)
- CREATE: SigmaOS-utils
- MOVE: SigmaOS-file-writer → SigmaOS-utils/file-writer
- MOVE: SigmaOS-folder-color-switcher → SigmaOS-utils/folder-colors
- MOVE: SigmaOS-renamer-file → SigmaOS-utils/renamer
- MOVE: SigmaOS-domain-blocker → SigmaOS-utils/domain-blocker
- MOVE: SigmaOS-chatroom → SigmaOS-utils/chatroom
- MOVE: SigmaOS-automate → SigmaOS-utils/automate
- MOVE: SigmaOS-audio-tuner → SigmaOS-utils/audio-tuner
- MOVE: SigmaOS-asdcontrol-git → SigmaOS-utils/asdcontrol
- DELETE: Original repos after migration

### Phase 4: Consolidate Development Tools (5 repos → 1 repo)
- CREATE: SigmaOS-dev
- MOVE: SigmaOS-dev-tools → SigmaOS-dev/tools
- MOVE: SigmaOS-fish → SigmaOS-dev/shell-config
- MOVE: SigmaOS-agent-cli → SigmaOS-dev/agent-cli
- MOVE: SigmaOS-quickshell → SigmaOS-dev/quickshell
- MOVE: SigmaOS-workspace → SigmaOS-dev/workspace
- DELETE: Original repos after migration

### Phase 5: Delete Application Repos (3 repos)
- DELETE: SigmaOS-office (use LibreOffice from Arch)
- DELETE: SigmaOS-player (use mpv from Arch)
- DELETE: SigmaOS-player-library (use mpv from Arch)

### Phase 6: Consolidate System Components (5 repos → 1 repo)
- CREATE: SigmaOS-system
- MOVE: SigmaOS-rollback → SigmaOS-system/rollback
- MOVE: SigmaOS-permissions → SigmaOS-system/permissions
- MOVE: SigmaOS-migration → SigmaOS-system/migration
- DELETE: Original repos after migration

### Phase 7: Keep Essential Separate Repos (6 repos)
- KEEP: SigmaOS (main distro)
- KEEP: SigmaOS-greeter (login greeter)
- KEEP: SigmaOS-desktop-portal (XDG portal)
- KEEP: SigmaOS-deskland (experimental web desktop)
- KEEP: SigmaOS-skills (AI skills)
- KEEP: SigmaOS-agent-cli (if substantial enough)

## Final Repository Count

**Before**: ~50 SigmaOS-* repositories
**After**: ~7 repositories

- SigmaOS (main distro)
- SigmaOS-themes (consolidated theming)
- SigmaOS-utils (consolidated utilities)
- SigmaOS-dev (consolidated dev tools)
- SigmaOS-system (consolidated system tools)
- SigmaOS-greeter (login greeter)
- SigmaOS-desktop-portal (XDG portal)
- SigmaOS-deskland (experimental web desktop)
- SigmaOS-skills (AI skills)

## Arch Linux Inspiration

Arch Linux follows these principles:
1. **KISS** - Keep It Simple, Stupid
2. **Modular** - Each package has a single purpose
3. **Upstream First** - Don't fork when upstream is good
4. **User-Centric** - Focus on what users actually need

Our consolidation follows these principles by:
- Not forking Arch packages (office, media, codecs)
- Consolidating similar tools into logical groups
- Keeping only what provides unique SigmaOS value

## Omarchy Linux Inspiration

Omarchy Linux uses multi-repo structure with:
- Separate repos for each major component
- Clear purpose for each repo
- Consistent naming convention

Our consolidation follows this by:
- Keeping logical groupings (themes, utils, dev, system)
- Clear naming (SigmaOS-themes, SigmaOS-utils, etc.)
- Consistent structure within each repo

## Next Steps

1. Get user approval for consolidation plan
2. Create new consolidated repos
3. Move code from old repos to new repos
4. Delete old repos
5. Update COMPONENTS-FOCUSED.toml
6. Update scripts/sigma_hub.sh to reflect new structure
