# SigmaOS Utils Consolidation Plan

## Status: PLANNED

## Objective

Consolidate 8 utility repositories into a unified `SigmaOS-utils` repository following **CONSOLIDATE GITHUB REPOS** and **MODULARITY** principles.

## Source Repositories

All 8 repositories exist on GitHub:

1. **SigmaOS-file-writer** - Minimal distraction-free text writer
2. **SigmaOS-folder-color-switcher** - Colour-code folders in the file manager
3. **SigmaOS-renamer-file** - Single & batch file renamer
4. **SigmaOS-domain-blocker** - Block domains system-wide via /etc/hosts
5. **SigmaOS-chatroom** - LAN chat server & client
6. **SigmaOS-automate** - Task automation engine (hooks, rules, schedules)
7. **SigmaOS-audio-tuner** - Audio tuning utilities
8. **SigmaOS-asdcontrol-git** - Hard drive control utilities

## Target Repository

- **Name**: `SigmaOS-utils`
- **URL**: https://github.com/AaryanSinghChauhan09/SigmaOS-utils
- **Structure**: Monorepo with subdirectories for each utility
  - `file-writer/` - Text writer
  - `folder-color/` - Folder color switcher
  - `renamer/` - File renamer
  - `domain-blocker/` - Domain blocker
  - `chatroom/` - LAN chat
  - `automate/` - Task automation
  - `audio-tuner/` - Audio utilities
  - `hard-drive/` - Hard drive control

## Consolidation Approach

Following the successful pattern from SigmaOS-themes:

1. Create empty SigmaOS-utils repository
2. Clone to /tmp/SigmaOS-utils
3. Create root Cargo.toml workspace
4. Copy content from each source repository
5. Update COMPONENT.toml in each subdirectory
6. Normalize inspirations to SigmaOS
7. Validate with cargo build --offline and cargo test --offline
8. Create Wiki documentation
9. Update main SigmaOS COMPONENTS-FOCUSED.toml
10. Push to GitHub

## Non-Destructive Approach

Following **IF IT AIN'T BROKE, DON'T FIX IT**:

- Source repositories remain available on GitHub
- No deletions performed during this phase
- Original repos can be archived after migration verification
- Users can continue using original repos until migration is complete

## Component Metadata Updates

Each subdirectory will have updated COMPONENT.toml with:
- `repo` pointing to the consolidated SigmaOS-utils repository
- `name` prefixed for clarity (e.g., `sigmaos-file-writer`)
- `inspirations` set to "SigmaOS"
- Clear dependency relationships (if any)

## Testing Requirements

Following **TEST CODE USING CI/CD** and **DETERMINISTIC TESTING**:

- `cargo build --offline` must succeed for all utilities
- `cargo test --offline` must pass with AAA pattern
- COMPONENT.toml files must be valid TOML
- Integration tests for cross-utility dependencies (if any)

## Benefits

1. **Simpler Maintenance** - One repo instead of 8
2. **Consistent Build** - Unified workspace
3. **Easier Discovery** - All utilities in one place
4. **Follows Arch KISS** - Keep It Simple, Stupid
5. **Follows Omarchy** - Modular but consolidated where logical

## Next Steps

1. Create SigmaOS-utils repository
2. Clone and set up workspace structure
3. Copy content from 8 source repositories
4. Update metadata and normalize inspirations
5. Validate build and tests
6. Create Wiki documentation
7. Update main registry
8. Document completion

## Rollback Plan

If consolidation fails:
1. Keep original 8 repos intact
2. Delete SigmaOS-utils
3. Revert COMPONENTS-FOCUSED.toml changes
4. No data loss - source repos intact
