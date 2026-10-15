# AI Agent Maintenance Instructions

## Purpose
This document provides instructions for AI agents to maintain SigmaOS by creating component-specific GitHub Wiki pages and agent files based on open source competitors (Linux, BSD, Windows, etc.).
## Mandatory Rule: Component Wiki Pages Created by AI Agents with Open Source Competitor Inspiration

AI agents MUST strictly adhere to the following rule:
1. **Exclusive Component Wiki Creation**: AI agents are permitted to create and maintain ONLY component-specific GitHub wiki pages. No other types of wiki pages (such as session-specific logs, temporary work files, PR-specific logs, date-stamped notes, or generic scratch pages) shall ever be created on the GitHub Wiki.
2. **Open Source Competitor Inspiration**: Every component wiki page MUST take inspiration for current architecture and future development directly from leading open-source competitors (Linux kernel, FreeBSD, OpenBSD, NetBSD, DragonFly BSD, illumos, systemd, etc.).
3. **Consolidation & Uniqueness**: Exactly ONE wiki page shall exist per OS component (e.g., `Scheduler-Component-Guide.md`, `Memory-Management-Component-Guide.md`, `Filesystem-VFS-Component-Guide.md`). When modifying, improving, or adding ideas to a component, the AI agent MUST update the existing component wiki page rather than creating duplicate or fragmented pages.
4. **Future Development Roadmap**: Each component wiki page must document concrete future development roadmaps, feature gap analyses against Linux/BSD distributions, architectural diagrams, implementation priorities, and testing strategies.


## AI Agent Guidelines

### 1. Component Wiki Page Creation

**When to Create:**
- When implementing a new component in SigmaOS
- When significantly improving an existing component
- When adding missing critical components from Linux/BSD

**CRITICAL RULE: One Wiki Page Per Component**
- AI agents MUST create only ONE wiki page per OS component
- All component-specific documentation must be consolidated into a single canonical page
- Do NOT create duplicate or redundant wiki pages for the same component
- If a component wiki page already exists, UPDATE it instead of creating a new one
- Future development ideas for the component should be documented in the component's single wiki page by taking inspiration from open source competitors (Linux, BSD, Windows, etc.)
- Research open source competitors and document future development plans in the component's wiki page

**Wiki Page Naming Convention:**
- Format: `COMPONENT_NAME-Component-Guide.md`
- Examples: `Scheduler-Component-Guide.md`, `Memory-Management-Component-Guide.md`

**Wiki Page Structure:**
```markdown
# COMPONENT_NAME Component Guide

## Overview
[Brief description of the component and its purpose]

## Linux Inspiration
[List relevant Linux kernel/user-space components and their features]

## BSD Inspiration
[List relevant BSD components and their features]

## Current SigmaOS Status
[Current implementation status, what works, what's missing]

## Critical Missing Features
[List critical features that SigmaOS is missing compared to Linux/BSD]

## Future Development Plan (Open Source Competitor Analysis)
[List future improvements based on research of Linux, BSD, Windows, and other open source competitors]
- Competitor A: Feature X - How to implement in SigmaOS
- Competitor B: Feature Y - How to implement in SigmaOS
- Competitor C: Feature Z - How to implement in SigmaOS

## Implementation Priority
[HIGH/MEDIUM/LOW priority for each missing feature]

## Key Files to Create/Improve
[List specific source files that need to be created or improved]

## Testing Strategy
[How to test the component implementation]

## Dependencies
[What other components this depends on]

## Success Criteria
[What defines successful implementation]

## Open Source Competitors Analysis
[Compare with Linux, BSD, Windows implementations]

## Future Enhancements
[Potential future improvements]
```

### 2. Agent File Creation

**Location:** `Agents/COMPONENT_AGENT.md`

**CRITICAL RULE: One Agent File Per Component**
- AI agents MUST create only ONE agent file per OS component
- Agent files must use the naming convention: `COMPONENT_AGENT.md` (e.g., `MEMORY_AGENT.md`, `SCHEDULER_AGENT.md`)
- Do NOT create duplicate or redundant agent files for the same component
- If an agent file already exists for a component, UPDATE it instead of creating a new one
- Agent files should document research on open source competitors and future development plans
- Even for components not yet implemented in SigmaOS, create agent files for critical OS components
- Research should cover Linux, BSD, Windows, and other open source competitors

**When to Create:**
- One agent file per OS component
- Even for components not yet implemented in SigmaOS
- Focus on critical components from Linux/BSD

**Current Agent Files Status:**
As of the latest update, the following agent files have been created:
- AUDIO_AGENT.md
- BOOT_BOOTLOADER_AGENT.md
- CONTAINER_RUNTIME_AGENT.md
- FILESYSTEM_AGENT.md
- GRAPHICS_GPU_AGENT.md
- INPUT_ACCESSIBILITY_AGENT.md
- INTERRUPT_MANAGEMENT_AGENT.md
- LOGGING_AGENT.md
- NETWORKING_STACK_AGENT.md
- PACKAGE_MANAGEMENT_AGENT.md
- POWER_MANAGEMENT_AGENT.md
- PROCESS_MANAGEMENT_AGENT.md
- STORAGE_AGENT.md
- SYSTEM_SERVICES_INIT_AGENT.md
- VIRTUALIZATION_AGENT.md
- WAYLAND_COMPOSITOR_AGENT.md
- HYPRLAND_AGENT.md
- MANGOHUD_AGENT.md
- LUTRIS_AGENT.md
- HEROIC_AGENT.md
- GAMESCOPE_AGENT.md
- WLOGOUT_AGENT.md
- WAYPAPER_AGENT.md
- SOFTWARE_MANAGER_AGENT.md
- UPDATE_MANAGER_AGENT.md
- BACKUP_TOOL_AGENT.md
- DRIVER_MANAGER_AGENT.md
- SYSTEM_TOOLS_AGENT.md
- PHOTOCRAFT_AGENT.md
- FILMCRAFT_AGENT.md
- VECTORCRAFT_AGENT.md
- PDFCRAFT_AGENT.md
- DESIGNCRAFT_AGENT.md
- EFFECTCRAFT_AGENT.md
- LIGHTCRAFT_AGENT.md
- ARTCRAFT_AGENT.md
- SYSTEM_MONITOR_AGENT.md (NEW)
- TASK_SCHEDULER_AGENT.md (NEW)

**Agent File Structure:**
```markdown
# COMPONENT Component Agent

## Component Overview
[Brief description]

## Linux Inspiration
[Linux components and features]

## BSD Inspiration
[BSD components and features]

## Current SigmaOS Status
[Current implementation status]

## Critical Missing Features
[List missing features]

## Implementation Priority
[HIGH/MEDIUM/LOW priority]

## Key Files to Create/Improve
[Specific files needed]

## Testing Strategy
[Testing approach]

## Dependencies
[Component dependencies]

## Success Criteria
[Success metrics]

## Open Source Competitors Analysis
[Competitor comparison]

## Future Enhancements
[Future improvements]
```

### 3. Research Requirements

**Open Source Competitors to Research:**
- **Linux Kernel:** https://github.com/torvalds/linux
- **FreeBSD:** https://github.com/freebsd/freebsd-src
- **OpenBSD:** https://github.com/openbsd/src
- **NetBSD:** https://github.com/NetBSD/src
- **DragonFlyBSD:** https://github.com/DragonFlyBSD/DragonFlyBSD
- **systemd:** https://github.com/systemd/systemd
- **GNOME:** https://github.com/GNOME
- **KDE:** https://github.com/KDE

**Research Focus:**
- Architecture and design patterns
- Key algorithms and data structures
- API design and interfaces
- Security considerations
- Performance optimizations
- Known limitations and trade-offs

### 4. Implementation Guidelines

**Code Style:**
- Follow existing SigmaOS code style
- Use Rust idioms and patterns
- Write comprehensive unit tests
- Add inline documentation comments
- Use unsafe only when necessary with justification

**Architecture Principles:**
- Standard libraries & standard functions (`std`, libc, runtime functions in Rust, Zig, Nim, and C) are permitted and MUST be used as and when required for robust concurrency, memory management, and I/O efficiency
- The rule of non-usage of standard library & functions is officially DROPPED
- Safe abstractions over unsafe operations
- Clear separation of concerns and modular subsystem boundaries
- Document trade-offs and performance characteristics

**Documentation Requirements:**
- Update FEATURE_STATUS.toml with component status
- Update CAPABILITY_MATRIX.tomL with capabilities
- Update WHAT_IS_WORKING_AND_NOT_WORKING.md with progress
- Create/update wiki pages for component documentation

### 5. Workflow

**For New Components:**
1. Create `Agents/COMPONENT_AGENT.md` with research
2. Implement component in `src/` directory
3. Write comprehensive unit tests
4. Update documentation (FEATURE_STATUS, CAPABILITY_MATRIX, wiki)
5. Commit and push to main branch
6. Create GitHub Wiki page for component

**For Existing Components:**
1. Review existing implementation
2. Identify gaps compared to Linux/BSD
3. Implement missing features
4. Update documentation
5. Commit and push
6. Update GitHub Wiki page

### 6. Critical Components List

**Must Have Agent Files (even if not implemented):**
- Bootloaders
- Scheduler
- Memory Management
- Filesystems
- Networking
- Security
- Desktop Environment
- Systemd/Init
- Package Management
- Drivers
- Virtualization
- Container Runtime
- System Monitoring
- Logging
- IPC
- Timekeeping
- Cryptography
- Internationalization (i18n)
- Accessibility

### 7. Quality Standards

**Code Quality:**
- All code must pass `cargo check`
- All tests must pass
- No clippy warnings
- Proper error handling
- Memory safety guarantees

**Documentation Quality:**
- Clear and concise
- Up-to-date with implementation
- Includes examples where appropriate
- References to standards (RFCs, specifications)

**Testing Quality:**
- Unit tests for all functions
- Integration tests for component interactions
- Edge case coverage
- Performance benchmarks where applicable

### 8. Git Workflow

**Branch Policy:**
- Only main branch exists
- All commits go directly to main
- No feature branches
- No pull requests (direct commits)

**Commit Message Format:**
```
Brief summary of changes

Detailed description if needed

Generated with [Devin](https://devin.ai)

Co-Authored-By: Devin <158243242+devin-ai-integration[bot]@users.noreply.github.com>
```

**Sync with GitHub:**
- Commit all changes
- Push to origin/main immediately
- Update GitHub Wiki after each major change

### 9. Continuous Improvement

**Review Cycle:**
- Periodically review existing components
- Compare with latest Linux/BSD releases
- Identify new features to implement
- Update agent files with new research

**Feedback Loop:**
- Monitor GitHub issues for component requests
- Update priorities based on user feedback
- Revise implementation plans based on testing results

### 10. Safety and Security

**Security Considerations:**
- Audit all unsafe code
- Validate all user input
- Use secure coding practices
- Follow principle of least privilege
- Test for common vulnerabilities

**Safety Checklist:**
- No buffer overflows
- No use-after-free
- No double-free
- No integer overflows
- Proper bounds checking
- Safe error handling

## Conclusion

Follow these guidelines to ensure SigmaOS grows with high-quality, well-documented components that match or exceed Linux/BSD capabilities. Always prioritize:
1. Correctness and safety
2. Performance
3. Compatibility with Linux/BSD
4. Clear documentation
5. Comprehensive testing
