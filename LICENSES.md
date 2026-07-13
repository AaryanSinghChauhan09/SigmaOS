# SigmaOS License & Compliance Attribution

This document lists the licensing terms and compliance attributes for third-party files, drivers, and libraries imported or adapted into the SigmaOS project.

## Core License
SigmaOS is licensed under the **GPL-2.0-or-later** license (GNU General Public License, Version 2 or any later version).

## Third-Party Components & Attributions

### 1. Mainline Linux Kernel Subsystems (torvalds/linux)
- **Files/Drivers**: GPU Mode setting, Network adapters (E1000, VirtIO), Scheduler concepts.
- **License**: GPL-2.0-only
- **Attribution**: Copyright (c) Torvalds et al.
- **Compliance Note**: All adapted code is kept in separate drivers or modular subsystems under GPL-2.0 compliance.

### 2. Fedora / RPM Integration Patterns
- **Files/Drivers**: Package manifests, atomic update specs.
- **License**: GPL-2.0-or-later
- **Attribution**: Copyright (c) Red Hat, Inc. and Fedora Project Contributors.

### 3. Nix-style Package Specifications
- **Files/Drivers**: Declarative packaging models, build blueprints.
- **License**: MIT
- **Attribution**: Copyright (c) NixOS Contributors.

### 4. Musl Libc Primitives
- **Files/Drivers**: Memory primitives, string manipulation concepts.
- **License**: MIT
- **Attribution**: Copyright (c) Rich Felker et al.

---

For full text of the GPL-2.0 license, see the [LICENSE](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/LICENSE) file in the root directory.
