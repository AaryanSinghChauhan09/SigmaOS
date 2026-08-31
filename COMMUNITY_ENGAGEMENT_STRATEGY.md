# SigmaOS Community Engagement Strategy

> Building a vibrant, global contributor community around an OS project
> requires coordinated outreach across forums, academic programs, hackathons,
> and social channels. This document outlines the full strategy.

***

## Table of Contents

1.  [Community Philosophy](#community-philosophy)
2.  [Contributor Personas](#contributor-personas)
3.  [Onboarding Pipeline](#onboarding-pipeline)
4.  [GSoC & Academic Programs](#gsoc--academic-programs)
5.  [Hackathons](#hackathons)
6.  [Forums & Communication Channels](#forums--communication-channels)
7.  [Content Strategy](#content-strategy)
8.  [Mentorship Program](#mentorship-program)
9.  [Recognition System](#recognition-system)
10. [India-Specific Outreach](#india-specific-outreach)
11. [Metrics & Goals](#metrics--goals)

***

## Community Philosophy

SigmaOS is built on three core community values:

**1. Radical Transparency**
Every design decision, architecture debate, and roadmap change happens in public.
No private Slack, no closed meetings. GitHub Discussions and the mailing list are
the only official channels.

**2. Beginner-Friendly Kernel Development**
Most OS projects are notoriously hostile to newcomers. SigmaOS specifically
invests in documentation, mentorship, and "good first issue" tags to change this.

**3. Global Inclusivity**
Documentation, release notes, and community calls will be available in English,
Hindi, Spanish, Mandarin, and Portuguese. GSoC mentors span time zones.

***

## Contributor Personas

### Persona A: The OS Student

*   Profile: University student (2nd–4th year CS), learning about kernels
*   Motivation: Hands-on systems programming experience
*   Entry point: "good first issue" tags, student guide
*   Needs: Patient mentors, clear documentation, small tractable tasks

### Persona B: The Embedded Developer

*   Profile: Professional embedded developer, knows C/C++ and bare-metal
*   Motivation: Wants to contribute drivers, explore Rust on hardware
*   Entry point: `docs/DRIVER_ABSORPTION_PRIORITY_PLAN.md`
*   Needs: Driver API docs, QEMU test environment, hardware access

### Persona C: The Security Researcher

*   Profile: Security-focused developer or academic
*   Motivation: Interested in pledge/unveil, formal verification, CVE research
*   Entry point: `docs/Security.md`, `src/security/`
*   Needs: Threat model docs, reproducible QEMU environments

### Persona D: The Distribution Packager

*   Profile: Maintains packages for another distro, wants sigpkg compatibility
*   Motivation: Multi-distro package format support
*   Entry point: `docs/PACKAGE_MANAGEMENT_MATURITY_ROADMAP.md`
*   Needs: sigpkg spec documentation, testing toolchain

### Persona E: The India Tech Ecosystem Builder

*   Profile: Developer at Indian startup or government digital project
*   Motivation: Aadhaar/UPI/DPDP compliance, India Stack integration
*   Entry point: `docs/INDIAN_COMPLIANCE_INTEGRATION_ROADMAP.md`
*   Needs: Compliance test cases, government approval documentation

***

## Onboarding Pipeline

### Step 1: Discovery (Week 0)

*   Visitor finds SigmaOS via GitHub, Hacker News, or conference talk
*   Reads `README.md` → links to `CONTRIBUTING.md`

### Step 2: Environment Setup (Week 1)

*   `docs/Building-SigmaOS.md` – complete build guide
*   `.devcontainer/devcontainer.json` – one-click GitHub Codespaces setup
*   First QEMU boot in 5 minutes: `make qemu`

### Step 3: First Contribution (Week 2–4)

*   Browse `good first issue` tag on GitHub
*   Join Matrix (`#sigmaos-dev`) or Discord
*   Submit first PR (usually documentation or a small driver stub)
*   Get review within 48 hours (guaranteed by maintainer rotation)

### Step 4: Regular Contributor (Month 2+)

*   Assigned a mentor from the core team
*   Invited to bi-weekly contributor sync call
*   Listed in `CONTRIBUTORS.md`

### Step 5: Trusted Contributor (Month 6+)

*   Write access to feature branches
*   Can review and approve PRs in their area
*   Invited to roadmap planning calls

### Step 6: Maintainer

*   Nominated by existing maintainers
*   Responsible for a subsystem (kernel, security, network, etc.)
*   Voting rights on governance decisions

***

## GSoC & Academic Programs

### Google Summer of Code (GSoC)

SigmaOS will apply for GSoC 2027 as a mentoring organisation.

#### Project Ideas for GSoC 2027

**Project 1: RISC-V port**

*   Difficulty: Hard
*   Duration: 350 hours
*   Mentor: TBD
*   Description: Port the SigmaOS kernel to RISC-V 64 (rv64gc). Implement HAL,
    interrupt controller, memory management. Target: boot to shell on QEMU virt.

**Project 2: WASM sandbox for package isolation**

*   Difficulty: Medium
*   Duration: 175 hours
*   Mentor: TBD
*   Description: Run sigpkg build scripts in a WASM sandbox (`src/compatibility/wasm_sandbox.rs`).
    Prevent build scripts from escaping to the host filesystem.

**Project 3: Formal verification of pledge**

*   Difficulty: Hard
*   Duration: 350 hours
*   Mentor: TBD
*   Description: Use Verus or Prusti to formally verify the pledge system call
    implementation in `src/security/sigma_pledge.rs`.

**Project 4: ZFS-compatible snapshot tool**

*   Difficulty: Medium
*   Duration: 175 hours
*   Mentor: TBD
*   Description: Implement a `sigsnap` CLI tool for managing CoW snapshots.

**Project 5: GUI installer**

*   Difficulty: Medium
*   Duration: 175 hours
*   Mentor: TBD
*   Description: Build a Qt/GTK-free graphical installer using SigmaOS's own UI
    toolkit (`src/ui/toolkit.rs`).

#### GSoC Application Timeline

*   **November 2026:** Submit mentor organisation application
*   **January 2027:** Publish project ideas page
*   **February 2027:** Student application window opens
*   **March 2027:** Mentor slots assigned
*   **May–August 2027:** Coding period

### Other Academic Programs

| Program | Organisation | Application Deadline |
|---------|-------------|---------------------|
| Outreachy | Software Freedom Conservancy | Rolling (twice yearly) |
| MLH Fellowship | Major League Hacking | Rolling |
| Linux Foundation Mentorship | LF | Quarterly |
| Season of KDE | KDE | January annually |

***

## Hackathons

### SigmaOS Annual Hackathon

**Cadence:** Annual (first announced for 2027)
**Format:** 48-hour online hackathon, optional local meetup hubs in India/EU/US
**Tracks:**

1.  **Driver Track** – Write a new hardware driver
2.  **App Track** – Port an open-source application to SigmaOS
3.  **Security Track** – Find vulnerabilities, write formal proofs
4.  **Compliance Track** – Implement new compliance module
5.  **Performance Track** – Improve benchmark results

**Prizes:**

*   Track winners: Sponsored hardware (SBC, development board)
*   Best in show: 1 month paid contributor stipend
*   All participants: SigmaOS contributor badge and T-shirt

### External Hackathon Participation

SigmaOS maintains a "hackathon kit" for teams working on SigmaOS at external events:

*   `docs/Building-SigmaOS.md` – fast setup guide (target: 15 minutes)
*   Pre-built QEMU image available at releases page
*   Curated "hackathon issues" tagged on GitHub for 4–8 hour scope

***

## Forums & Communication Channels

### Official Channels

| Channel | Purpose | Audience |
|---------|---------|----------|
| GitHub Discussions | Technical Q\&A, RFCs | All contributors |
| GitHub Issues | Bug reports, feature requests | All users |
| Matrix (`#sigmaos-dev`) | Real-time dev discussion | Contributors |
| Matrix (`#sigmaos-user`) | User support | Users |
| Mailing list (`dev@sigmaos.dev`) | Long-form discussion, announcements | Core team + interested |
| Discord (`SigmaOS Community`) | Social, voice calls | Everyone |

### Forum Strategy

**Stack Overflow tag:** `sigmaos` – monitor and answer within 24 hours.
**Reddit:** `r/SigmaOS` – weekly project updates, AMA with maintainers quarterly.
**Hacker News:** Submit release announcements; engage thoughtfully in comments.
**Lobsters:** Submit technical deep-dives.

### Response Time SLAs

| Channel | Expected response |
|---------|-----------------|
| GitHub Issues (security) | 24 hours |
| GitHub Issues (bug) | 72 hours |
| GitHub Issues (feature) | 1 week |
| Matrix #dev | 4 hours during business hours |
| Stack Overflow | 48 hours |
| Discord | Best effort |

***

## Content Strategy

### Blog (`blog.sigmaos.dev`)

**Publishing cadence:** Bi-weekly
**Article types:**

*   "How we implemented X" – deep technical dives
*   "SigmaOS vs \[Linux distro]" – honest comparison articles
*   "Contributor spotlight" – interview a contributor
*   "Month in review" – progress updates

**Example articles:**

1.  "How SigmaOS implements OpenBSD pledge in Rust"
2.  "Zero-dependency kernel: why we wrote our own allocator"
3.  "Building a rolling-release OS: lessons from Arch"
4.  "SigmaOS performance: how we beat Linux io\_uring benchmarks"

### Video Content (`youtube.com/SigmaOS`)

**Videos:**

*   Demo videos for each release
*   "Contributing to SigmaOS" tutorial series
*   Conference talk recordings
*   Weekly dev stream (Fridays 18:00 IST)

### Conference Talks

Target conferences:
| Conference | Audience | Submission timeline |
|-----------|---------|---------------------|
| FOSDEM | European OSS community | October annually |
| Linux Conf AU | Pacific FOSS | July annually |
| RustConf | Rust community | April annually |
| DEF CON | Security | March annually |
| India FOSS | Indian FOSS | March annually |
| USENIX OSDI | Academic OS | January annually |

***

## Mentorship Program

### Structure

Each new contributor is matched with a mentor based on:

*   Area of interest (kernel/security/network/tools)
*   Time zone proximity
*   Language preference

### Mentor Responsibilities

*   Weekly 30-minute check-in (can be async)
*   Review mentee's PRs within 24 hours
*   Help mentee understand architecture decisions
*   Connect mentee with other community members

### Mentee Journey (3-month program)

*   Month 1: Environment setup, first PR, read architecture docs
*   Month 2: Meaningful contribution (new feature or non-trivial bug fix)
*   Month 3: Code review of others' PRs, write documentation

### Becoming a Mentor

After 6 months as an active contributor, anyone can apply to become a mentor.
Requirements:

1.  10+ merged PRs
2.  Familiarity with the subsystem they'll mentor on
3.  Availability of 2 hours/week

***

## Recognition System

### Contributor Levels

| Level | Requirement | Benefits |
|-------|------------|---------|
| Contributor | 1 merged PR | Listed in CONTRIBUTORS.md |
| Active Contributor | 10 merged PRs | Discord role, stickers |
| Trusted Contributor | 50 merged PRs + 6 months | Branch write access |
| Maintainer | Nominated + approved | Subsystem ownership, voting |
| Core Team | Maintainer + 1 year | Release signing key |

### Hall of Fame

The `CONTRIBUTORS.md` file includes:

*   Total PR count
*   Primary contribution area
*   Notable contributions highlighted

### Swag

*   Stickers sent to all Trusted Contributors (globally)
*   T-shirts for Maintainers and above
*   Conference sponsorship for core team talks

***

## India-Specific Outreach

### Target Communities

*   IIT/NIT/IIIT open-source clubs
*   Fossasia India chapter
*   Mozilla India community
*   Python India / PyCon India network
*   IndiaFOSS conference community

### Programs

1.  **SigmaOS India Contributor Program** – Structured 3-month program for Indian
    students, with focus on India Stack compliance modules.
2.  **Workshops at IITs** – "Build your first kernel module" hands-on workshop.
3.  **DPDP Act Compliance Bounty** – Paid bounty for implementing India DPDP
    compliance checks.
4.  **IITB-Monash Research Academy** – Explore academic collaboration on formal
    verification of SigmaOS security subsystems.

***

## Metrics & Goals

### 2026 Goals

*   GitHub stars: 5 000
*   Contributors: 150
*   Monthly active contributors: 30
*   GSoC slots applied: 5
*   Forum posts answered within SLA: 90%

### 2027 Goals

*   GitHub stars: 20 000
*   Contributors: 500
*   Monthly active contributors: 100
*   GSoC accepted projects: 5
*   Hackathon participants: 300

### Tracking Dashboard

Metrics are tracked in `src/community/mod.rs` (community health metrics module)
and displayed on the web dashboard at `web_ui/index.html`.

***

*Last updated: 2026-08-04*
