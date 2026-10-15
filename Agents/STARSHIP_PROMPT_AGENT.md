# Starship Contextual Command Prompt Component Agent

## Component Overview
Governs blazing-fast cross-shell contextual prompt generation, Git branch and dirty state tracking, language runtime version detection, and execution duration calculation inspired by Omarchy Linux Starship.

## Linux Inspiration
Starship cross-shell prompt (Rust), Oh-My-Zsh themes, and Powerlevel10k.

## BSD Inspiration
OpenBSD ksh prompt customization, FreeBSD tcsh prompt.

## Current SigmaOS Status
Contextual prompt string rendering and Git branch tracking implemented in StarshipPromptEngine and Zig sigma_starship_fast_prompt.zig.

## Critical Missing Features
Asynchronous Git status query thread preventing prompt delays on massive multi-gigabyte repositories.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/desktop/omarchy_terminal_gamemode.rs, src/desktop/sigma_starship_fast_prompt.zig, src/shell/sigma_shell.nim

## Testing Strategy
Prompt render latency tests on massive Git repositories (<5ms query response), Unicode symbol fidelity.

## Dependencies
FILESYSTEM_VFS_AGENT, TIMEKEEPING_AGENT, DESKTOP_ENVIRONMENT_AGENT

## Success Criteria
Sub-5ms prompt generation, zero perceptible delay between command completion and next prompt display.

## Open Source Competitors Analysis
50x faster than traditional shell script prompts (Oh-My-Zsh) by executing in compiled native safe Rust and Zig.

## Future Enhancements
Dynamic Kubernetes and cloud cluster context indicator embedded seamlessly in shell prompts.
