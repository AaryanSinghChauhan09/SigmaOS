# GitHub Wiki Setup

This directory contains documentation that should be migrated to the GitHub Wiki.

## Wiki Pages to Create

The following pages are ready to be added to the GitHub Wiki:

### 1. ISO Build Guide
File: `docs/wiki/ISO-Build-Guide.md`
- Quick start with Docker, Arch, GitHub Actions
- Architecture overview
- Build process explanation
- QEMU testing instructions
- Troubleshooting

### 2. Hyprland Configuration
File: `docs/wiki/Hyprland-Configuration.md`
- SigmaOS Hyprland config structure
- Keybindings reference
- Autostart script
- Theming with Wallust
- Multi-monitor setup

### 3. Installer Guide
File: `docs/wiki/Installer-Guide.md`
- sigmaos-install usage
- Safety features (dry-run, root guard)
- Btrfs subvolume layout
- Post-installation steps
- Bootloader configuration

### 4. Package Management
File: `docs/wiki/Package-Management.md`
- Curated package set (116 packages)
- Essential packages (9 meta-packages)
- Arch repositories used
- Custom SigmaOS packages
- Updating the system

## Manual Setup Instructions

To create wiki pages:

1. Go to: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
2. Click "Add Page"
3. Copy content from the respective docs/wiki/*.md file
4. Save the page
5. Add to wiki sidebar for navigation

## Wiki Structure (Target)

```
Home
├── Getting Started
│   ├── ISO Build Guide
│   ├── Installation Guide
│   └── Quick Start
├── Configuration
│   ├── Hyprland Configuration
│   ├── Waybar Configuration
│   ├── Wallust Theming
│   └── Shell Configuration
├── Components
│   ├── Package Management
│   ├── Installer
│   └── Bootloader
└── Development
    ├── AGENTS.md (already in repo)
    ├── CONTRIBUTING.md (already in repo)
    └── Build System
```

## Cleanup Needed

The current wiki has hundreds of placeholder pages that should be:
- Consolidated if they cover similar topics
- Deleted if they're irrelevant stubs
- Migrated to docs/ if they're actual documentation

Following TRANSFER & CONSOLIDATE .MD/.TXT TO WIKI principle.
