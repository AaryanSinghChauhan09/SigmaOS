# Package Ecosystem Roadmap — sigpkg

This document outlines the unified package management system for SigmaOS, designed to unify .deb, .rpm, Flatpak, and Snap into a single cohesive experience.

---

## Phase 1: sigpkg Core

### Current Status
- Basic package manager stub exists
- No unified package format

### Target State
- **Unified Package Manager (sigpkg)**
  - Single command for all software: `sigpkg install <package>`
  - Automatically handles .deb, .rpm, Flatpak, Snap
  - Dependency resolution with SAT solver
  - Transaction rollback on failure
  - PQC-signed packages (Dilithium-3)
  - SBOM integration (CycloneDX)

### Architecture
```
sigpkg CLI
    ↓
sigpkg Core (dependency resolver)
    ↓
┌──────────┬──────────┬──────────┬──────────┐
│  .deb    │  .rpm    │ Flatpak  │  Snap    │
│ Adapter  │ Adapter  │ Adapter  │ Adapter  │
└──────────┴──────────┴──────────┴──────────┘
```

### Implementation Tasks
- [ ] Design sigpkg package format (metadata + payload)
- [ ] Implement SAT solver for dependency resolution
- [ ] Build .deb adapter (using dpkg/apt libraries)
- [ ] Build .rpm adapter (using rpm/yum libraries)
- [ ] Build Flatpak adapter (using flatpak libraries)
- [ ] Build Snap adapter (using snapd libraries)
- [ ] Add Dilithium-3 package signing
- [ ] Implement transaction rollback
- [ ] Add SBOM generation and verification

### Estimated Timeline: 3-4 months

---

## Phase 2: Repositories

### Current Status
- No central repository
- No mirrors

### Target State
- **Central Repository Infrastructure**
  - Primary repo: repo.sigmaos.org
  - Global mirrors (India, US, EU, APAC)
  - CDN integration for fast downloads
  - PQC-signed repository metadata
  - Automatic mirror synchronization

- **Repository Structure**
  ```
  ├── stable/          # Production packages
  ├── testing/         # Pre-release testing
  ├── unstable/        # Development builds
  ├── security/        # Security updates only
  └── backports/       # Backported features
  ```

- **Package Metadata**
  - DID-based package identity
  - Dilithium-3 signatures
  - SBOM (CycloneDX)
  - Security advisories
  - Compatibility matrix

### Implementation Tasks
- [ ] Set up repository server infrastructure
- [ ] Implement repository signing (Dilithium-3)
- [ ] Create mirror synchronization system
- [ ] Build CDN integration
- [ ] Design repository web UI
- [ ] Add package search API
- [ ] Implement automatic security updates

### Estimated Timeline: 2-3 months

---

## Phase 3: Dependency Resolution

### Current Status
- Basic dependency tracking
- No conflict detection

### Target State
- **AI-Assisted Dependency Resolution**
  - SAT solver for complex dependency graphs
  - AI-powered conflict detection and resolution
  - Automatic suggestion of alternatives
  - Dependency health scoring
  - Vulnerability-aware resolution

### Features
```bash
# Traditional install
sigpkg install libreoffice

# AI-assisted install with conflict resolution
sigpkg install libreoffice --ai-resolve
# → "Detected conflict: libssl1.1 vs libssl3"
# → "Suggested: Upgrade libssl1.1 to libssl3 (compatible with 98% of packages)"
# → "Alternative: Use libreoffice-stable (requires libssl1.1)"

# Vulnerability-aware install
sigpkg install nginx --secure-only
# → "nginx 1.24.0 has 2 CVEs (CVE-2024-1234, CVE-2024-5678)"
# → "Recommended: nginx 1.25.0 (no CVEs)"
```

### Implementation Tasks
- [ ] Integrate SAT solver (pubgrub or similar)
- [ ] Build conflict detection engine
- [ ] Add AI model for resolution suggestions
- [ ] Integrate vulnerability database (NVD, CVE)
- [ ] Implement dependency health scoring
- [ ] Add interactive conflict resolution UI

### Estimated Timeline: 2 months

---

## Phase 4: Package Building

### Current Status
- Manual build process
- No CI/CD

### Target State
- **Automated Package Building**
  - CI/CD pipeline for package builds
  - Reproducible builds (hermetic environment)
  - Multi-architecture builds (x86_64, ARM64, RISC-V)
  - Automatic testing before publishing
  - Automatic SBOM generation

### Build Pipeline
```
Source Code → Build → Test → Sign → Publish → Mirror
     ↓         ↓      ↓      ↓        ↓        ↓
  Git      Docker  Unit   Dilithium  sigpkg   CDN
             Image  Tests  Signature  Repo
```

### Implementation Tasks
- [ ] Set up build infrastructure (GitLab CI or GitHub Actions)
- [ ] Create build recipes for 100+ essential packages
- [ ] Implement reproducible builds
- [ ] Add multi-architecture builds
- [ ] Integrate Dilithium-3 signing
- [ ] Build package testing framework
- [ ] Add automatic SBOM generation

### Estimated Timeline: 3-4 months

---

## Phase 5: Package Migration Tools

### Current Status
- No migration tools

### Target State
- **Migration from Other Distros**
  - Ubuntu/Debian → SigmaOS package converter
  - Fedora/RHEL → SigmaOS package converter
  - Flatpak → native sigpkg converter
  - Snap → native sigpkg converter

### Features
```bash
# Convert .deb to sigpkg
sigpkg-convert --from deb --to sigpkg libreoffice.deb

# Convert entire system
sigpkg-migrate --from ubuntu --to sigmaos
# → Analyzes installed packages
# → Converts to sigpkg equivalents
# → Preserves configuration
# → Rolls back on failure
```

### Implementation Tasks
- [ ] Build .deb → sigpkg converter
- [ ] Build .rpm → sigpkg converter
- [ ] Build Flatpak → sigpkg converter
- [ ] Build Snap → sigpkg converter
- [ ] Create system migration tool
- [ ] Add configuration preservation
- [ ] Implement rollback on failure

### Estimated Timeline: 2 months

---

## Dependencies

- Core System (for build infrastructure)
- Security (for package signing)
- AI Automation (for AI-assisted resolution)

---

## Success Metrics

- 10,000+ packages in repository
- 95% of Ubuntu packages have sigpkg equivalent
- <5% dependency conflicts in typical install
- <30s average install time for complex packages
- 100% packages PQC-signed
- 100% packages have SBOM

---

## Next Steps

1. Design sigpkg architecture and package format
2. Set up repository infrastructure
3. Build first 100 essential packages
4. Implement migration tools for Ubuntu/Debian
5. Launch public beta repository

---

## See Also

- [Core System Roadmap](Core_System.md)
- [Security Roadmap](Security.md)
- [AI Automation Roadmap](AI_Automation.md)
