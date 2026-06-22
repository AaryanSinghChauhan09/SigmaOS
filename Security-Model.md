# Security Model

SigmaOS implements a Zero-Trust security model at every layer of the stack — from the bootloader to the web app permission system.

---

## Capability Rings

Every process, driver, and subsystem is assigned to a capability ring. Ring 0 is the most privileged; Ring 4 is the most restricted.

```
┌──────────────────────────────────────────────────────────┐
│  Ring 4 — Web Apps / PWAs (Chromium sandbox)             │
│  ┌────────────────────────────────────────────────────┐  │
│  │  Ring 3 — Go Daemons (sigmad-process, ai, sync)   │  │
│  │  ┌──────────────────────────────────────────────┐  │  │
│  │  │  Ring 2 — Driver Registry                   │  │  │
│  │  │  ┌──────────────────────────────────────┐   │  │  │
│  │  │  │  Ring 1 — SovereignVMM              │   │  │  │
│  │  │  │  ┌──────────────────────────────┐   │   │  │  │
│  │  │  │  │  Ring 0 — Zero-Trust Engine │   │   │  │  │
│  │  │  │  └──────────────────────────────┘   │   │  │  │
│  │  │  └──────────────────────────────────────┘   │  │  │
│  │  └──────────────────────────────────────────────┘  │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

---

## Zero-Trust Engine

The Zero-Trust Engine (Ring 0) runs a continuous threat scoring loop. Every inter-ring call is validated:

1. **Requestor identity** — is this caller who it claims to be?
2. **Capability token** — does this caller hold a valid, non-expired token for this resource?
3. **Threat score** — has the requestor's behavior exceeded the anomaly threshold?

If any check fails, the request is denied and an audit event is written to the immutable, CRC32C-checksummed kernel log.

### Worked Example: NVMe DMA Request

```
NVMe Driver (Ring 2) needs DMA buffer from VMM (Ring 1)

1. Driver calls sigma_capability_request(CAP_DMA_BUFFER, size=4096)
2. Ring 0 checks: driver holds CAP_DMA_REQUEST token? YES
3. Ring 0 checks: threat score for driver PID? Score=0 (clean)  
4. Ring 0 mints a temporal DMA capability: CAP_DMA_BUFFER, TTL=100ms, addr=0xFFA00000
5. VMM validates token signature (Dilithium-5) before mapping physical pages
6. After TTL expires, mapping is automatically revoked
```

---

## Post-Quantum Cryptography

All signatures and key exchanges in SigmaOS use NIST PQC Level 5 algorithms.

| Use Case | Algorithm | Key Size |
|----------|-----------|----------|
| Capability token signing | **Dilithium-5** | 4864-byte pk |
| Package signing (sigma-pkg) | **Dilithium-5** | 4864-byte pk |
| Key encapsulation (TLS) | **Kyber-1024** | 1568-byte ct |
| Kernel audit log MAC | **SHAKE-256** | 256-bit |

---

## Web App Capability System

Web apps access OS features via `navigator.sigmaos`. Every privileged API call requires a user-granted capability stored in `~/.sigmaos/capabilities.json`.

```json
[
  { "origin": "https://notes.app", "cap": "fs.read",    "granted": true },
  { "origin": "https://notes.app", "cap": "fs.write",   "granted": true },
  { "origin": "https://notes.app", "cap": "ai.complete","granted": true }
]
```

### Capability Flow

```
1. App calls navigator.sigmaos.fs.readFile("notes/hello.txt")
2. inject.js → chrome.runtime.sendMessage → native host → sigmad-fs
3. sigmad-fs checks capabilities.json for { origin, cap }
4. NOT FOUND → returns { ok: false, error: "cap not granted: fs.read" }
5. inject.js opens /settings/caps.html with request payload
6. User clicks "Allow Always" → capabilities.json updated
7. App retries → succeeds
```

Revoke any grant at any time via **Settings → Capabilities**.
