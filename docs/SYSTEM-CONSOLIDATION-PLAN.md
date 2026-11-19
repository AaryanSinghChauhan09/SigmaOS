# SigmaOS System Components Consolidation Plan

## Status: PLANNED

## Objective

Consolidate 6 system component repositories into a unified `SigmaOS-system` repository following **CONSOLIDATE GITHUB REPOS** and **MODULARITY** principles.

## Source Repositories

All 6 repositories exist on GitHub:

1. **SigmaOS-greeter** - Slick login greeter with user list & a11y
2. **SigmaOS-rollback** - System rollback: BTRFS snapshot management, pre-update snapshots, restore on failure
3. **SigmaOS-permissions** - App permission model: network, filesystem, camera, microphone, notification access
4. **SigmaOS-migration** - System migration: import from other distros, data transfer, dual-boot setup
5. **SigmaOS-media-codecs** - Media codec support: GStreamer plugins, FFmpeg, hardware decode, codec licensing
6. **SigmaOS-office-interop** - Office interoperability: ODF, OOXML, PDF import/export, font compatibility

## Target Repository

- **Name**: `SigmaOS-system`
- **URL**: https://github.com/AaryanSinghChauhan09/SigmaOS-system
- **Structure**: Monorepo with subdirectories for each system component
  - `greeter/` - Login greeter
  - `rollback/` - BTRFS snapshot management
  - `permissions/` - App permission model
  - `migration/` - System migration tools
  - `media-codecs/` - Media codec support
  - `office-interop/` - Office interoperability

## Notes

These are shell-based components (not Rust), so consolidation will be simpler:
- No Cargo workspace needed
- Direct directory structure
- Each component is independent
