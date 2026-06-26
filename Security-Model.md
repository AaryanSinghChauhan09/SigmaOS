# Security Model

SigmaOS is designed with security as a first-class constraint, not an afterthought. This page explains the full trust model: from how capabilities are declared and enforced, to how processes are sandboxed, to how zero-trust workload identity works.

---

## 1. Capability System

Every SigmaOS app must declare its capabilities in a `manifest.json` before any platform API call is permitted.

```json
{
  "name": "VideoEditor",
  "version": "1.0.0",
  "capabilities": [
    "process:spawn",
    "fs:/home/user/Videos",
    "fs:/tmp",
    "bin:ffmpeg",
    "net:none"
  ]
}
```

### Capability Categories

| Prefix | What it grants |
|---|---|
| `process:spawn` | Permission to call `navigator.sigmaos.process.spawn()` |
| `fs:<path>` | Read/write access to a specific filesystem path |
| `bin:<name>` | Execute a specific binary inside a sandbox |
| `net:none` | Explicitly declare no network access (for offline tools) |
| `net:host` | Full outbound network (requires user approval) |
| `hw:camera` | Access to `/dev/video*` |
| `hw:audio` | Access to ALSA/PulseAudio sockets |
| `clipboard:read` | Read from the shared clipboard daemon |
| `clipboard:write` | Write to the shared clipboard daemon |
| `ai:complete` | Access to the local `sigmad-ai` inference endpoint |

Capabilities not declared in the manifest are **denied at the extension layer** before the request ever reaches a daemon. An undeclared capability cannot be granted at runtime without re-installing the app with an updated manifest.

---

## 2. Extension Permission Gate (background.js)

The SigmaOS Chrome extension is the first enforcement point. Every `navigator.sigmaos.*` call flows through it.

```
Web App → chrome.runtime.sendMessage({ method: "process.spawn", ... })
              │
              ▼
    background.js: checkPermission(origin, "process:spawn")
              │
       ┌──────┴──────┐
     GRANTED       DENIED
       │               │
       ▼               ▼
  Forward to      Reject Promise
  native host     (PermissionDeniedError)
```

Critically, permission state is held **in memory** (`Map<requestId, resolve>`), not in `chrome.storage`. This prevents the serialization bug where storing a Promise resolver in session storage would silently lose the function reference and hang the API call forever.

---

## 3. Bubblewrap Sandboxing

Every process spawned through `sigmad-process` runs inside a **bubblewrap (bwrap) container** — a lightweight namespacing wrapper using Linux kernel primitives:

- **PID namespace**: The sandboxed process sees only itself and its children. It cannot enumerate or signal other system processes.
- **Mount namespace**: Only the paths declared in the `fs:` capabilities are bind-mounted into the container. The rest of the filesystem is invisible.
- **Network namespace**: By default, only loopback is available. `net:host` capability adds a veth pair to the host network.
- **User namespace**: The process runs as an unprivileged UID inside the container, mapped from a sub-UID range.
- **seccomp filter**: A syscall allowlist is generated from the capability set. `execve` of unlisted binaries, raw socket creation, and `ptrace` are blocked.

Example bwrap invocation for an app with `["bin:ffmpeg", "fs:/tmp", "net:none"]`:

```bash
bwrap \
  --ro-bind /usr/bin/ffmpeg /usr/bin/ffmpeg \
  --bind /tmp /tmp \
  --unshare-net \
  --unshare-pid \
  --seccomp 3 \
  -- ffmpeg "$@"
```

---

## 4. Zero-Trust Workload Identity (sigma_zerotrust)

For the native kernel layer, SigmaOS implements a zero-trust policy engine based on **SPIFFE/SPIRE-style workload identities**.

Each process that registers with the kernel is assigned a SPIFFE URI:

```
spiffe://sigma.os/workload/<exec_path>
```

Every capability check (IPC call, filesystem access, network flow) verifies:

1. **Identity**: Does this PID have a registered workload identity?
2. **Revocation**: Has this workload been revoked? If yes, deny immediately — no exceptions.
3. **Policy**: Does a ALLOW rule exist for this (src_spiffe → dst_spiffe, capability) tuple?

Revocation is **checked on every capability request**, not just at authentication time. A workload revoked at runtime is denied all subsequent requests without requiring a process restart.

---

## 5. Audit Log

Every security-relevant event is written to the SigmaOS audit log with a real monotonic timestamp:

```
[1719400234.512] ALLOW  pid=1042 spiffe=spiffe://sigma.os/workload/ffmpeg cap=fs:/tmp
[1719400234.513] DENY   pid=1099 spiffe=spiffe://sigma.os/workload/unknown cap=net:host  reason=not_registered
[1719400234.514] REVOKE pid=1042 reason=manual_revocation
[1719400234.515] DENY   pid=1042 spiffe=spiffe://sigma.os/workload/ffmpeg cap=fs:/tmp  reason=revoked
```

Timestamps are sourced from the kernel monotonic clock — never hardcoded or simulated values. This makes the audit log forensically useful for ordering events and investigating incidents.

---

## 6. Cryptographic Attestation (Zenith)

The Zenith desktop adds a hardware attestation layer using **Kyber-1024** (post-quantum key encapsulation):

- On boot, Zenith generates a fresh Kyber-1024 keypair bound to the current hardware fingerprint (TPM or CPU serial where available).
- Each shard in the persistent shard matrix is encrypted with the session-bound key.
- Attestation tokens are required for cross-shard communication, preventing injection of rogue shards.

---

## Threat Model Summary

| Threat | Mitigation |
|---|---|
| Malicious PWA calling system APIs | Extension capability gate — denied before reaching daemon |
| Compromised app escaping its sandbox | bwrap namespaces + seccomp filter |
| Revoked process continuing to operate | Zero-trust revocation check on every capability request |
| Buffer overflow in kernel policy engine | `snprintf`/`sigma_strlcpy` everywhere, no `strcpy`/`sprintf` |
| XSS in the web shell injecting payloads | `textContent` DOM insertion, no raw `innerHTML` from untrusted input |
| Extension hanging on permission request | In-memory Promise map — no `chrome.storage` for resolver functions |

---

*See also: [Architecture Overview](Architecture-Overview) · [API Reference](API-Reference)*
