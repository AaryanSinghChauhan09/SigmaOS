# Distro Absorption: Fedora RPM Format & SELinux Hardening

> **Status**: ✅ Absorbed | **Target Shards**: `RPMAbsorber.shard`, `SovereignSELinux.shard` | **Source Distro**: Fedora Linux

---

## 1. Executive Summary

Fedora is known for its focus on modern technology, the RPM package format (`.rpm` / DNF), and strict security policies via Security-Enhanced Linux (SELinux). Transitioning from Fedora requires supporting RPM packages and adapting SELinux's Role-Based Access Control (RBAC) and Type Enforcement policies.

The `RPMAbsorber.shard` handles RPM archive extraction and dependency analysis, while `SovereignSELinux.shard` converts complex SELinux policy rules into native microkernel access tokens.

---

## 2. Technical Features & Absorption Strategy

### 2.1 RPM Package Translator (`RPMAbsorber.shard`)
- **Fedora Concept**: Package archives (`.rpm`) contain compiled binaries, resource files, and system scripts, configured via `.spec` files.
- **Sovereign Implementation**: `RPMAbsorber` parses RPM file formats, extracts resources, and generates dependency graphs to automatically run packages inside sandboxed execution rooms.

### 2.2 SELinux Policy Conversion (`SovereignSELinux.shard`)
- **Fedora Concept**: SELinux enforces security policies using Type Enforcement (TE), Role-Based Access Control (RBAC), and Multi-Level Security (MLS) to restrict application actions.
- **Sovereign Implementation**: The `SovereignSELinux` engine parses Fedora’s policy files and maps their subjects, roles, and types to the capability tokens used by the SigmaOS microkernel. This provides SELinux-level security with microkernel-enforced memory isolation.

---

## 3. Shard Architecture

```
┌─────────────────────────────────────────────────────────┐
│               FEDORA ABSORPTION MATRIX                  │
├─────────────────────────────────────────────────────────┤
│  ┌───────────────────────┐   ┌───────────────────────┐  │
│  │   RPMAbsorber.shard   │   │SovereignSELinux.shard │  │
│  │ (RPM Archive Extractor)│   │ (Policy to Token Map) │  │
│  └───────────┬───────────┘   └───────────┬───────────┘  │
│              └─────────────┬─────────────┘              │
│              ┌─────────────▼─────────────┐              │
│              │      SELinux Sandbox      │              │
│              │    (Enforced Access Room) │              │
│              └───────────────────────────┘              │
└─────────────────────────────────────────────────────────┘
```

---

## 4. Usage & Commands

To list and execute Fedora absorption:

```powershell
$ sigma distro list
Σ [INFO] Sovereign Linux Distro Absorption Registry:
  * Fedora       -> RPMAbsorber.shard           [Active]  (RPM-spec absorption pipeline)
  ...

$ sigma distro absorb fedora
Σ [INFO] Starting Deep-Lattice absorption of 'fedora' paradigm...
Σ [INFO]   -> Loading RPMAbsorber.shard & SovereignSELinux.shard...
Σ [INFO]   -> Compiling RPM spec parser...
Σ [SUCCESS] Fedora RPM absorber and SELinux engine absorbed successfully!
```

---

## 5. References & Standards
- RPM File Format and Packaging Specifications
- SELinux Architecture and Policy Enforcement Specifications
- Fedora packaging guidelines and DNF architecture
