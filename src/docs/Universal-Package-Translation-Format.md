# SigmaOS: Universal Package Format Translation & Slackware Crusher

This document details how SigmaOS ingests, parses, and translates foreign Linux/BSD packages into secure native capability-based environments.

---

## 🌎 Multi-Distro Manifest Translation

SigmaOS does not force developers to rewrite their applications for its native capability API. Instead, the SigmaPkg manager exposes adapters for:

```
  [ Debian Control ]   [ Pacman PKGBUILD ]   [ Snapcraft YAML ]   [ Flatpak JSON ]
          \                    |                     |                  /
           \                   |                     |                 /
            v                  v                     v                v
      +----------------------------------------------------------------------+
      |                      Universal Package Adapter                       |
      +----------------------------------------------------------------------+
                                         |
                                         v
                      +--------------------------------------+
                      |      SigmaOS Native Package         |
                      |   - Capability Gate Permissions      |
                      |   - PQC Signature Seals              |
                      +--------------------------------------+
```

### 1. Translating Sandbox Metadata
Container formats like Snap and Flatpak list requested device permissions (e.g., Plugs and finish-args).
* **Apt Control Files:** Parsed for dependencies and package metadata.
* **PKGBUILD Scripts:** Extracted for raw versioning constraints.
* **Snapcraft & Flatpak Manifests:** Permissions like `network`, `--share=network`, and `--filesystem=home` are directly mapped to SigmaOS native `Permission::NetworkTcp`, `Permission::FileRead`, and `Permission::FileWrite` Capability gates.

---

## 🏴‍☠️ Slackware Legacy Dependency Harvester (Slackware Crusher)

Legacy Slackware `.tgz`/`.txz` packages contain files without explicit metadata regarding shared library dependencies, often leading to "dependency hell".

The **Slackware Crusher Engine** resolves this limitation:
1. **ELF Extraction:** Parses installed binaries and libraries to extract required `SONAME` dependencies (e.g., `libc.so.6`, `libssl.so.3`).
2. **Registry Mapping:** Queries the SigmaOS `library_provider_registry` to discover which package provides each requested library.
3. **Transaction Construction:** Automatically builds a complete, dependency-aware transaction chain, converting primitive archives into secure, robustly resolved packages.
