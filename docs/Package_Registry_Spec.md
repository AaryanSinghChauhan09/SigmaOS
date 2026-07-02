# SigmaOS Package Registry Specification (sigpkg)

## Overview

The sigpkg registry is a content-addressed package registry for SigmaOS. Packages are signed with Dilithium-5, indexed in a Merkle tree, and distributed via a binary substituter protocol.

---

## REST API Endpoints

### Package Index

| Method | Path | Description |
|---|---|---|
| GET | `/v1/index` | Full index (TOML, Merkle root + entries) |
| GET | `/v1/index/{name}` | Package metadata by name |
| GET | `/v1/index/{name}/{version}` | Specific version metadata |
| GET | `/v1/search?q={query}` | Search by name/description |

**GET /v1/index/{name}/{version} Response:**
```json
{
  "name": "sigma-edit",
  "version": "1.2.0",
  "description": "SigmaOS text editor",
  "store_path": "sha256:abc123def456-sigma-edit-1.2.0",
  "nar_url": "nar/sha256:abc123.nar.zst",
  "nar_hash": "sha256:abc123...",
  "nar_size": 1234567,
  "dependencies": ["sigma-gui-1.0.0", "sigma-fonts-0.5.0"],
  "sig": "<base64-dilithium5>",
  "provenance_url": "prov/sha256:abc123.json"
}
```

### Upload

| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/v1/upload` | Bearer token (Dilithium-5 signed) | Upload a .spkg file |
| POST | `/v1/sign-token` | Admin key | Issue an upload token |

**POST /v1/upload Request:**
```
Content-Type: multipart/form-data
Authorization: Bearer <dilithium5-signed-token>

Fields:
  package: <.spkg binary>
  sig:     <.spkg.sig bundle>
  prov:    <provenance.json>
```

### Binary Cache (Substituter)

| Method | Path | Description |
|---|---|---|
| GET | `/nar/{hash}.narinfo` | Fetch narinfo for a store path |
| GET | `/nar/{hash}.nar.zst` | Fetch the compressed NAR archive |
| PUT | `/nar/{hash}.nar.zst` | Upload a NAR (authenticated) |

---

## Authentication: Dilithium-5 Signed Upload Tokens

Upload tokens are short-lived JWTs signed with a Dilithium-5 key:

```json
{
  "sub": "maintainer@sigmaos.dev",
  "pkg": "sigma-edit",
  "exp": 1720000000,
  "jti": "unique-nonce-abc123"
}
```

Signature: Dilithium-5 over the serialized header + payload.

---

## Index Format: TOML + Merkle Tree

```toml
# /sigma/store/index.toml
[meta]
version = 1
merkle_root = "sha256:aabbcc..."
generated_at = 1720000000

[[packages]]
name       = "sigma-edit"
version    = "1.2.0"
store_path = "sha256:abc123-sigma-edit-1.2.0"
nar_hash   = "sha256:abc123..."
sig        = "dilithium5:base64..."
```

The Merkle tree is built as a binary tree over all `(name, version, nar_hash)` tuples. Clients verify their downloaded package against the Merkle root from a signed index snapshot.

---

## Rate Limiting + Abuse Prevention

| Endpoint | Limit |
|---|---|
| GET /v1/search | 100 req/min per IP |
| GET /nar/*.nar.zst | 50 downloads/hour per IP; 500 MB/hour bandwidth |
| POST /v1/upload | 10 uploads/hour per token |
| POST /v1/sign-token | 5/hour per admin key |

Responses include `X-RateLimit-Remaining` and `Retry-After` headers.

---

## Mirror Sync Protocol

Mirrors poll the primary registry every 15 minutes:

```
GET /v1/sync/since?cursor=<last_merkle_root>
→ { "new_packages": [...], "new_merkle_root": "..." }
```

Mirrors verify the new Merkle root against the registry's Dilithium-5 signing certificate before accepting new packages.

---

## substituter Protocol Summary

```bash
# sigma-pkg queries substituter:
GET https://cache.sigmaos.dev/sha256:abc123.narinfo

# narinfo response:
StorePath: /sigma/store/sha256:abc123-sigma-edit-1.2.0
URL: nar/sha256:abc123.nar.zst
Compression: zstd
FileHash: sha256:def456...
FileSize: 1234567
Sig: sigma-cache-1:base64-dilithium5-sig

# sigma-pkg downloads, verifies, extracts:
GET https://cache.sigmaos.dev/nar/sha256:abc123.nar.zst
```
