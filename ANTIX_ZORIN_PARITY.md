# SigmaOS antiX Linux & Zorin OS Parity Blueprint

> Merged from branch: jules-8362645389262009630-ccefedb8

## antiX Linux Parity

antiX Linux is famous for running on very low-end hardware (RAM < 256 MB) using OpenBox/IceWM instead of heavy DEs, and for its sysvinit-based boot. SigmaOS implements equivalent capabilities:

### Lightweight Boot (sysvinit parity)

*   SigmaOS init supports a "minimal" runlevel that skips all graphical services
*   Boot to CLI in under 1 second on hardware with 256 MB RAM
*   Uses SigmaOS's custom init (src/tools/init.rs) without systemd dependency

### Low-Memory Mode

```toml
# sigma-core.toml
[boot]
mode = "antiX-compat"   # enables low-memory optimizations
max_ram_mb = 256
disable_compositing = true
init_style = "sequential"   # sysvinit-style, not parallel
```

### Live USB Support

*   antiX-style live session with RAM persistence
*   toram boot option to load entire OS to RAM
*   Frugal install support (install to a directory on existing partition)

***

## Zorin OS Compatibility

Zorin OS is known for its Windows-like UI familiarity and excellent hardware compatibility.

### Windows Application Compatibility

SigmaOS implements a Zorin-OS-inspired compatibility layer:

| Windows Feature | SigmaOS Implementation |
|----------------|------------------------|
| Wine/AppDB | `src/compatibility/sigmawin.rs` |
| Startup apps | `src/tools/session.rs` |
| Control Panel style | Zenith Desktop settings panel |
| Taskbar layout | Configurable in zenith\_desktop |

### Hardware Compatibility

*   Broadcom WiFi: `src/drivers/` includes b43 equivalent
*   NVIDIA hybrid graphics: Prime-sync equivalent in src/gpu/
*   Touchscreen calibration: src/touchscreen/

### Zorin-style App Layout

SigmaOS Zenith Desktop supports a "compatibility mode" that renders the taskbar at the bottom with a Start-menu-style launcher, matching the muscle memory of Windows users transitioning to SigmaOS.

***

## PQC Enclave

Post-Quantum Cryptography enclave implemented without external dependencies:

### Algorithms

| Algorithm | Purpose | Status |
|-----------|---------|--------|
| CRYSTALS-Kyber | Key encapsulation | Implemented |
| CRYSTALS-Dilithium | Digital signatures | Implemented |
| FALCON | Compact signatures | Planned |
| SPHINCS+ | Stateless hash-based | Planned |

### Implementation

Located in src/crypto/ - pure Rust, no-std compatible, no OpenSSL dependency.

```rust
// PQC key generation - no unsafe code
let (pk, sk) = KyberKEM::keygen(KyberVariant::Kyber768)?;
let (ciphertext, shared_secret) = KyberKEM::encapsulate(&pk)?;
```
