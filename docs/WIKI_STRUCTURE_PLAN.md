# SigmaOS Wiki Structure Plan

**Version:** 1.0
**Status:** Draft
**Last Updated:** 2025-01-22
**Purpose:** Define the Arch Linux-inspired wiki structure for SigmaOS

---

## 1. Overview

SigmaOS Wiki follows an Arch Linux-inspired information architecture with:
- **One page per topic** - No duplicate pages
- **Task-oriented titles** - Clear, searchable titles
- **Cross-references** - Links between related pages
- **AI agent maintenance instructions** - Per-component guidance
- **Versioned instructions** - Last verified version tracking

---

## 2. Wiki Hierarchy

### 2.1 Home

**Home.md**
- SigmaOS overview
- Quick start links
- Latest news
- Community resources

---

### 2.2 Getting Started

**Getting-Started.md**
- What is SigmaOS?
- System requirements
- Download options
- Installation overview

**Downloading-Images.md**
- Official download locations
- Mirror selection
- Verification (checksums, signatures)
- Alternative download methods

**Verifying-Downloads.md**
- Checksum verification
- GPG signature verification
- What to do if verification fails

**Installing-SigmaOS.md**
- Pre-installation checklist
- Boot from ISO
- Installation wizard
- Manual installation
- Post-installation steps

**First-Boot.md**
- Initial configuration
- User account creation
- Network setup
- Desktop environment selection
- First login

**Recovery.md**
- Emergency shell access
- Boot parameters
- Recovery mode
- Chroot repair
- System rollback

---

### 2.3 User Guide

**Zenith-Desktop.md**
- Zenith compositor overview
- Window management
- Keyboard shortcuts
- Customization
- Troubleshooting

**Applications.md**
- Installing applications
- Application catalog
- Recommended applications
- Running applications
- Managing applications

**sigpkg.md**
- Package management overview
- Installing packages
- Removing packages
- Updating packages
- Searching packages
- Package repositories

**Updates-and-Rollback.md**
- System updates
- Update frequency
- Automatic updates
- Rollback procedure
- Recovery from failed updates

**Networking.md**
- Network configuration
- Wired networks
- Wireless networks
- Network troubleshooting
- Firewall configuration

**Audio-and-Bluetooth.md**
- Audio setup
- Bluetooth setup
- Audio troubleshooting
- Bluetooth troubleshooting

**Displays-and-Graphics.md**
- Display configuration
- Multiple monitors
- Graphics drivers
- Display troubleshooting

**Power-Management.md**
- Power settings
- Suspend and resume
- Battery management
- Power troubleshooting

**Accessibility.md**
- Screen reader configuration
- Keyboard navigation
- High contrast mode
- Text scaling
- Accessibility troubleshooting

**Troubleshooting.md**
- Common issues
- System logs
- Diagnostic tools
- Getting help
- Reporting bugs

---

### 2.4 Administration

**Users-and-Groups.md**
- User management
- Group management
- Permissions
- Sudo configuration
- User policies

**Services.md**
- Service management
- Enabling/disabling services
- Service status
- Service logs
- Custom services

**Storage.md**
- Disk management
- Partitioning
- Filesystems
- Mount points
- Storage troubleshooting

**Backups.md**
- Backup strategies
- System snapshots
- User data backups
- Backup automation
- Restore procedures

**Security-Policies.md**
- Security overview
- Firewall rules
- Sandbox policies
- Access control
- Security auditing

**Firewall.md**
- Firewall configuration
- Rule management
- Default policies
- Firewall troubleshooting

**Logs-and-Diagnostics.md**
- System logs
- Journal viewing
- Log rotation
- Diagnostic tools
- Log analysis

---

### 2.5 Development

**Build-Environment.md**
- Development prerequisites
- Setting up build environment
- Cross-compilation
- Build tools
- Build troubleshooting

**Architecture.md**
- System architecture overview
- Component interaction
- Kernel architecture
- Userland architecture
- Architecture decisions

**Kernel-Development.md**
- Kernel structure
- Building the kernel
- Kernel modules
- Kernel debugging
- Kernel contribution guide

**Driver-Development.md**
- Driver framework
- Writing drivers
- Driver testing
- Driver debugging
- Driver contribution guide

**Package-Recipes.md**
- sigpkg format
- Writing package recipes
- Building packages
- Package testing
- Package contribution guide

**SDK.md**
- SDK overview
- SDK installation
- SDK tools
- SDK examples
- SDK documentation

**Testing.md**
- Test framework
- Writing tests
- Running tests
- Test coverage
- Test contribution guide

**Contributing.md**
- Contribution workflow
- Code of conduct
- Pull request process
- Code review
- Recognition

---

### 2.6 Reference

**System-Calls.md**
- System call reference
- POSIX compatibility
- SigmaOS extensions
- System call usage
- System call troubleshooting

**File-Hierarchy.md**
- Filesystem hierarchy
- Directory structure
- Standard paths
- Custom paths
- Filesystem conventions

**Configuration-Files.md**
- System configuration
- User configuration
- Configuration syntax
- Configuration examples
- Configuration troubleshooting

**Environment-Variables.md**
- Environment variables
- System variables
- User variables
- Variable precedence
- Variable troubleshooting

**Feature-Flags.md**
- Feature flag reference
- Available flags
- Feature selection
- Feature testing
- Feature documentation

**Package-Format.md**
- sigpkg format specification
- Manifest schema
- Signing specification
- Format compatibility
- Format examples

**Release-Policy.md**
- Release cadence
- Versioning scheme
- Support policy
- Security updates
- EOL policy

---

### 2.7 Compatibility

**Linux-Compatibility.md**
- Linux compatibility level
- Supported Linux features
- Compatibility limitations
- Binary compatibility
- Source compatibility

**FreeBSD-Compatibility.md**
- FreeBSD compatibility level
- Supported FreeBSD features
- Compatibility limitations
- Binary compatibility
- Source compatibility

**OpenBSD-Compatibility.md**
- OpenBSD compatibility level
- Supported OpenBSD features
- Compatibility limitations
- Binary compatibility
- Source compatibility

**POSIX-Status.md**
- POSIX compliance level
- POSIX features supported
- POSIX limitations
- POSIX test results
- POSIX roadmap

**Supported-Hardware.md**
- Hardware support tiers
- Supported platforms
- Hardware requirements
- Known issues
- Hardware testing

---

### 2.8 Project

**Roadmap.md**
- Development roadmap
- Milestone definitions
- Feature plans
- Timeline
- Progress tracking

**Architecture-Decisions.md**
- ADR index
- ADR summaries
- ADR links
- ADR contribution
- ADR process

**Security-Advisories.md**
- Security advisories
- CVE tracking
- Security updates
- Security reporting
- Security response

**Governance.md**
- Project governance
- Decision making
- Roles and responsibilities
- Voting process
- Conflict resolution

**Release-Criteria.md**
- Release criteria
- Quality gates
- Testing requirements
- Documentation requirements
- Release process

**Known-Limitations.md**
- Current limitations
- Known issues
- Workarounds
- Future improvements
- Limitation tracking

---

## 3. AI Agent Maintenance Instructions

Each wiki page must include a section at the end:

```markdown
---

## AI Agent Maintenance

### Persona Assignment
- **Primary:** [Sentinel/Palette/Bolt]
- **Secondary:** [Persona]

### Maintenance Tasks
- [ ] Update status references in PROJECT_STATUS.md
- [ ] Verify all internal links resolve
- [ ] Update examples with latest syntax
- [ ] Add troubleshooting for new issues
- [ ] Document new features as they are added

### Known Issues
- Issue 1: [description]
- Issue 2: [description]

### Edge Cases
- Edge case 1: [description]
- Edge case 2: [description]

### Related Components
- [Component 1](../../src/path/to/component)
- [Component 2](../../src/path/to/component)

### Last Verified
- **Version:** [version]
- **Date:** [date]
- **Verified by:** [agent name]
```

---

## 4. Cross-Reference Guidelines

### 4.1 Internal Links

Use relative paths for internal wiki links:

```markdown
See [Installation](Installing-SigmaOS.md) for details.
```

### 4.2 Code References

Link to source code using relative paths:

```markdown
Implementation in [src/kernel/scheduler.rs](../../src/kernel/scheduler.rs)
```

### 4.3 Documentation References

Link to canonical docs in `docs/`:

```markdown
See [Architecture Decisions](../docs/ARCHITECTURE_DECISIONS.md) for details.
```

---

## 5. Page Templates

### 5.1 Overview Page Template

```markdown
# [Page Title]

**Last Updated:** [Date]
**Status:** [Stable/Draft/Deprecated]
**Persona:** [Sentinel/Palette/Bolt]

## Overview
[Brief description of the topic]

## Prerequisites
- [Prerequisite 1]
- [Prerequisite 2]

## Instructions
[Step-by-step instructions]

## Examples
[Usage examples]

## Troubleshooting
[Common issues and solutions]

## References
- [Related page 1](Related-Page-1.md)
- [Related documentation](../docs/Document.md)

## AI Agent Maintenance
[AI agent maintenance section]
```

### 5.2 Reference Page Template

```markdown
# [Page Title]

**Last Updated:** [Date]
**Status:** [Stable/Draft/Deprecated]
**Persona:** [Bolt]

## Overview
[Brief description of the reference]

## Reference
[Detailed reference information]

## Examples
[Usage examples]

## See Also
- [Related reference 1](Related-Reference-1.md)
- [Related documentation](../docs/Document.md)

## AI Agent Maintenance
[AI agent maintenance section]
```

---

## 6. Migration Strategy

### 6.1 Phase 1: Structure Definition
- Define wiki hierarchy
- Create page templates
- Document AI agent instructions

### 6.2 Phase 2: Content Migration
- Migrate existing wiki content
- Consolidate duplicate pages
- Add cross-references
- Add AI agent instructions

### 6.3 Phase 3: CI Integration
- Implement wiki generation from `docs/`
- Add broken-link checks
- Add stale copy detection
- Automate synchronization

### 6.4 Phase 4: Cleanup
- Remove legacy wiki copies
- Remove duplicate pages
- Finalize structure
- Publish to GitHub Wiki

---

## 7. Success Metrics

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| One page per topic | 100% | 0% | ❌ |
| All pages have AI agent instructions | 100% | 0% | ❌ |
| All internal links resolve | 100% | Unknown | ⚠️ |
| No duplicate content | 100% | Unknown | ⚠️ |
| All pages last verified date | 100% | 0% | ❌ |

---

## 8. References

- [PROJECT_STATUS.md](PROJECT_STATUS.md) - Project status
- [DOCUMENTATION_SOURCE_POLICY.md](DOCUMENTATION_SOURCE_POLICY.md) - Documentation policy
- [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md) - Architecture decisions
- [Arch Linux Wiki](https://wiki.archlinux.org/) - Inspiration source
