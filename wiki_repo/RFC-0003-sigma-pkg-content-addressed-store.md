# RFC-0003: sigpkg Content-Addressed Package Store

- **RFC number**: 0003

- **Author(s)**: SigmaOS Project

- **Subsystem**: sigma-pkg / packaging

- **Status**: Accepted

- **Date proposed**: 2026-07-01

- **Tracking issue**: #sigpkg-store

- **Implementation PR**: sigma-pkg/sigma_pkg_install.py

---

## Summary

Define the on-disk layout and cryptographic verification scheme for the
SigmaOS sovereign package store at `/sigma/store/`. Every installed package
occupies an immutable, content-addressed directory. Identical files across
packages share a single on-disk copy via hardlinks (deduplication).

---

## Detailed Design

### Store path format

```
/sigma/store/<derivation-hash>-<name>-<version>/
```

`<derivation-hash>` is the SHA-256 of a deterministic JSON object containing:

- `name`, `version`, `sha256` of source archive

- sorted `makedepends`, sorted `hardening` flags

- `build_style`, `cmake_args`, `tool_flags`

This mirrors [GNU Guix's store path scheme](https://guix.gnu.org/manual/en/html_node/The-Store.html).

### Content-addressed deduplication

Files with identical SHA-256 hashes are stored once and hardlinked:

```
/sigma/store/cas/<sha256-first-2-chars>/<sha256-rest>  → actual file
/sigma/store/<hash>-pkg-1.0/bin/tool → hardlink to cas entry
/sigma/store/<hash>-pkg-2.0/bin/tool → same hardlink (if content identical)
```

### Signature verification chain

```
package.spkg
  └── manifest.json  (package metadata + file list + per-file hashes)
  └── manifest.json.dilithium5.sig  (Dilithium-5 signature by maintainer key)
  └── files/...
```

Verification steps:

1. Verify `manifest.json.dilithium5.sig` against maintainer keyring in `/sigma/etc/sigma-pkg/keyrings/`

2. Verify SHA-256 of each file in `files/` matches `manifest.json`

3. Verify derivation hash matches `manifest.json.derivation_hash`

### Database schema

`/sigma/var/pkg/installed.json` — JSON object, keyed by package name:
```json
{
  "sigma-core": {
    "name": "sigma-core",
    "version": "15.0.0",
    "install_path": "/sigma/store/abc123-sigma-core-15.0.0",
    "sha256": "...",
    "depends": [],
    "installed_at": 1751000000.0,
    "pinned": false,
    "auto": false
  }
}
```

### Atomic install / rollback

1. Download `.spkg` to `/sigma/var/pkg/cache/`

2. Verify signatures

3. Extract to `/sigma/store/<hash>-<name>-<ver>/` (atomic: temp dir + rename)

4. Register in database

5. On failure: remove temp dir, do not update database → system unchanged

---

## Security Considerations

- Store directory is read-only after install (`chmod 555`)

- Only `sigma-pkg` daemon (runs as `sigma-pkg` user) can write to store

- `sigma_pledge` restricts sigma-pkg to `wpath cpath rpath inet dns` only
