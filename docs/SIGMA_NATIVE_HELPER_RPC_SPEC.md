# SigmaOS Native Helper RPC Protocol & Security Sandbox Portals Specification (v1.0)

## Executive Summary

This specification defines the technical architecture, JSON-RPC 2.0 API contracts, security sandbox portals, multi-user profile policy model, session snapshotting formats, and implementation backlog for the **Sigma Native Helper (`sigma-native`)** subsystem.

The `sigma-native` daemon bridges Web UI / WebExtensions (such as Zenith Desktop and Sigma Apps) with sovereign userland and kernel capabilities using an authenticated, Ed25519-signed RPC channel over stdio / UNIX domain sockets.

---

## 1. Native Helper RPC Protocol (`sigma-native`)

### 1.1 Messaging Envelope Format

All RPC requests and responses follow JSON-RPC 2.0 with Ed25519 digital signatures in the `headers` field to prevent unauthorized command injection.

```json
{
  "jsonrpc": "2.0",
  "id": "req-1024-9842",
  "headers": {
    "profile_id": "usr-default-001",
    "signature": "3b2a8d...ed25519_hex",
    "timestamp": 1727481200,
    "nonce": "8f3b0c1e4d"
  },
  "method": "portal.file.open",
  "params": {
    "title": "Select Sovereign Image",
    "filters": ["*.png", "*.jpg", "*.iso"],
    "multiple": false
  }
}
```

Response format:

```json
{
  "jsonrpc": "2.0",
  "id": "req-1024-9842",
  "result": {
    "handle": "file-handle-8d1e",
    "path": "/home/sovereign/Documents/boot.iso",
    "size_bytes": 1073741824,
    "mime_type": "application/x-iso9660-image"
  },
  "error": null
}
```

---

## 2. Sandbox Portals Specification

Portals mediate access between sandboxed Web/WASM applications and host OS primitives, following least-privilege capability negotiation.

### 2.1 File Portal (`portal.file`)

- `portal.file.open`: Opens a file picker dialog and returns scoped file handle.
- `portal.file.save`: Opens a save-as dialog and grants write permissions.
- `portal.file.readChunk`: Reads chunked binary data from an authorized file handle.

### 2.2 Device Portal (`portal.device`)

- `portal.device.listUsb`: Lists permitted USB devices mapped to the sandbox.
- `portal.device.claimGpu`: Allocates a virtual GPU queue context (VirtIO-GPU / Vulkan).

### 2.3 PTY & Terminal Portal (`portal.pty`)

Provides a zero-copy PTY bridge connecting `xterm.js` frontend instances with userland shells:

```json
{
  "method": "portal.pty.spawn",
  "params": {
    "command": "/bin/sh",
    "args": ["-l"],
    "cols": 80,
    "rows": 24,
    "env": { "TERM": "xterm-256color" }
  }
}
```

---

## 3. Multi-User Profiles, Accounts & Policy Management

SigmaOS profiles provide encrypted user-space isolation and enterprise MDM policy enforcement.

### 3.1 Profile Structure

Each profile is anchored in an encrypted store (`~/.local/share/sigmaos/profiles/<profile_id>/`):

```json
{
  "profile_id": "usr-enterprise-402",
  "display_name": "Sovereign Engineering",
  "account_type": "EnterpriseProfile",
  "encryption": {
    "cipher": "AES-256-GCM-Kyber1024",
    "key_derivation": "Argon2id"
  },
  "policy": {
    "allow_unverified_shards": false,
    "web_extension_whitelist": ["*@sigmaos.org"],
    "max_wasm_memory_mb": 4096,
    "allowed_sys_portals": ["file", "pty", "device.gpu"]
  }
}
```

### 3.2 Account Modes

1. **Owner Profile**: Full administrative and kernel attestation rights.
2. **Standard User Profile**: Scoped filesystem & portal capabilities.
3. **Guest Mode**: Ephemeral in-memory profile purged immediately on session exit.
4. **Enterprise MDM Profile**: Enforces signed policy manifests and SSO auth tokens.

---

## 4. Session Snapshots Specification

Session snapshots capture and restore full workspace state (open tabs, windows, PTY streams, and IndexedDB state).

### 4.1 Snapshot Manifest Format

```json
{
  "snapshot_version": "1.0",
  "timestamp": "2026-09-28T12:00:00Z",
  "profile_id": "usr-default-001",
  "workspace": {
    "theme": "gold",
    "windows": [
      {
        "id": "terminal-win",
        "title": "OmniShell v5.1",
        "zIndex": 105,
        "rect": { "x": 100, "y": 80, "width": 700, "height": 500 },
        "active_tab": "tab-1",
        "pty_session_id": "pty-9012"
      }
    ]
  }
}
```

---

## 5. Implementation Roadmap & Backlog

| Phase | Duration | Core Deliverables | Success Metrics |
| :--- | :--- | :--- | :--- |
| **Phase A (Quick Wins)** | Months 0–3 | `sigma-native` RPC spec, xterm.js PTY bridge, Session Snapshots UI v1 | Session restore < 2s |
| **Phase B (Foundation)** | Months 3–9 | Helper daemon, Signed PWA/Bundle store, Per-app Proxy routing | Portal IPC latency < 5ms |
| **Phase C (Advanced)** | Months 9–18 | WASM/WASI runtime, Bubblewrap/AppArmor sandbox enforcement, Content-addressed snapshots | 0 critical security audit gaps |

---
