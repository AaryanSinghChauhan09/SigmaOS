# SigmaOS Dev Tools Consolidation Plan

## Status: PLANNED

## Objective

Consolidate 5 dev tool repositories into a unified `SigmaOS-dev` repository following **CONSOLIDATE GITHUB REPOS** and **MODULARITY** principles.

## Source Repositories

All 5 repositories exist on GitHub:

1. **SigmaOS-dev-tools** - Developer toolchain helpers
2. **SigmaOS-fish** - Fish config & plugins
3. **SigmaOS-agent-cli** - AI agent CLI interface
4. **SigmaOS-quickshell** - Quick shell command runner
5. **SigmaOS-workspace** - Workspace management

## Target Repository

- **Name**: `SigmaOS-dev`
- **URL**: https://github.com/AaryanSinghChauhan09/SigmaOS-dev
- **Structure**: Monorepo with subdirectories for each dev tool
  - `toolchain/` - Developer toolchain helpers
  - `fish/` - Fish config & plugins
  - `agent-cli/` - AI agent CLI interface
  - `quickshell/` - Quick shell command runner
  - `workspace/` - Workspace management

## Non-Destructive Approach

Following **IF IT AIN'T BROKE, DON'T FIX IT**:

- Source repositories remain available on GitHub
- No deletions performed during this phase
- Original repos can be archived after migration verification
