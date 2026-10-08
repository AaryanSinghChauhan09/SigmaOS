# AI Agent Maintenance Instructions

## Purpose
This document provides instructions for AI agents to maintain SigmaOS by creating component-specific GitHub Wiki pages based on open source competitors (Linux, BSD, Windows, etc.).

## AI Agent Guidelines

### 1. Component Wiki Page Creation

**When to Create:**
- When implementing a new component in SigmaOS
- When significantly improving an existing component
- When adding missing critical components from Linux/BSD

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

**When to Create:**
- One agent file per OS component
- Even for components not yet implemented in SigmaOS
- Focus on critical components from Linux/BSD

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
- **Linux Kernel:** `https://github.com/torvalds/linux`
- **FreeBSD:** `https://github.com/freebsd/freebsd-src`
- **OpenBSD:** `https://github.com/openbsd/src`
- **NetBSD:** `https://github.com/NetBSD/src`
- **DragonFlyBSD:** `https://github.com/DragonFlyBSD/DragonFlyBSD`
- **systemd:** `https://github.com/systemd/systemd`
- **GNOME:** `https://github.com/GNOME`
- **KDE:** `https://github.com/KDE`

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
- Zero external dependencies where possible
- Safe Rust over unsafe
- Clear separation of concerns
- Modular design
- Document trade-offs

**Documentation Requirements:**
- Update FEATURE_STATUS.toml with component status
- Update CAPABILITY_MATRIX.toml with capabilities
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
