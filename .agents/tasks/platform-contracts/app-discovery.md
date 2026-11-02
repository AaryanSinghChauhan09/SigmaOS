# App Discovery Contract

> **Shared SigmaOS platform contract.** Define the app catalog schema, install/uninstall flow, update policy, and compatibility metadata for the SigmaOS app ecosystem.

---

## Schema

AppManifest { name: String; app_id: String; version: SemVer; categories: [String]; keywords: [String]; exec: String; icon: Path; desktop_entry: Path; supported_backends: [Backend]; min_runtime: SemVer; }

## Flow

- 1. Catalog scan: SigmaOS-apps scans /usr/share/applications/*.desktop and registered manifests.
- 2. Search: fuzzy-match by name/keyword/category (omafind-inspired algorithm).
- 3. Install: resolve dependencies, verify signature, install via package-manager.
- 4. Update: check catalog for newer version; stage + apply transactionally.
- 5. Uninstall: remove package + clean user config (with --keep-config option).

## Failure Modes and Recovery

App not found, dependency unresolvable, signature invalid, install interrupted (rollback).

## Open Questions

Should app stores support paid apps? Current: no — FOSS only. Plugin marketplace (SigmaOS-plugin-marketplace) handles community/third-party.
