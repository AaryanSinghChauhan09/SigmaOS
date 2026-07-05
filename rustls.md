# rustls Pure-Rust TLS Integration

## Overview

SigmaOS uses [rustls](https://github.com/rustls/rustls) (MIT / Apache-2.0) as the sole TLS library across the entire stack. OpenSSL is explicitly excluded — it introduces C code, legacy cipher support, and a large attack surface.

---

## Why rustls over OpenSSL

| Concern | OpenSSL | rustls |
|---|---|---|
| Language | C (memory-unsafe) | Pure Rust (memory-safe) |
| Legacy ciphers | RC4, MD5, SSLv3 — hard to disable | TLS 1.2+ only, no legacy negotiation |
| License | OpenSSL dual-license (complex) | MIT + Apache-2.0 (clean) |
| Audit | Multiple CVEs annually | Formally audited 2019, 2023 |
| Binary size | ~4MB | ~400KB |
| FIPS concern | Requires special build | Custom crypto provider model |

---

## Integration Points

| Component | How rustls is used |
|---|---|
| `sigma-curl` | HTTPS client for package downloads and REST APIs |
| `sigma-ssh` | TLS session wrapping for SSH-over-TLS tunnel mode |
| `sigma-pkg` | Registry HTTPS: mutual TLS for package uploads |
| `sigma-trustd` | TLS for remote attestation channels |
| `sigma-otel-collector` | gRPC/TLS export of traces and metrics |

---

## Cargo.toml Snippet (Pinned)

```toml
[dependencies]
rustls          = { version = "=0.23.7", features = ["tls12"] }
rustls-pki-types = "=1.7.0"
webpki-roots    = "=0.26.3"
rustls-pemfile  = "=2.1.2"

# For Kyber-1024 hybrid KEX (custom crypto provider)

sigma-pqcrypto  = { path = "../../crypto/sigma-pqcrypto" }
```

---

## TLS 1.3 + Kyber-1024 Hybrid KEX

SigmaOS implements a **custom rustls crypto provider** that extends the standard TLS 1.3 ECDHE key exchange with a post-quantum Kyber-1024 KEM component (X25519Kyber1024Draft00, draft-tls-westerbaan-xyber768d00 style).

```rust
// crypto/sigma-pqcrypto/src/kyber_provider.rs

use rustls::crypto::{CryptoProvider, KeyExchangeAlgorithm};
use rustls::NamedGroup;

/// Post-quantum hybrid group: X25519 + Kyber-1024
pub struct X25519Kyber1024;

impl rustls::crypto::ActiveKeyExchange for X25519Kyber1024 {
    fn complete(
        self: Box<Self>,
        peer_pub_key: &[u8],
    ) -> Result<rustls::crypto::SharedSecret, rustls::Error> {
        // 1. Extract X25519 public key (first 32 bytes)
        // 2. Extract Kyber-1024 ciphertext (remaining bytes)
        // 3. Derive X25519 shared secret + Kyber-1024 shared secret
        // 4. Concatenate and hash with HKDF-SHA384
        let (x25519_peer, kyber_ct) = peer_pub_key.split_at(32);
        let x25519_ss = sigma_x25519::agree(&self.x25519_priv, x25519_peer)?;
        let kyber_ss  = sigma_kyber::decapsulate(&self.kyber_key, kyber_ct)?;
        let hybrid_ss = sigma_hkdf::extract_and_expand(
            &[&x25519_ss, &kyber_ss],
            b"SigmaOS Hybrid TLS 1.3",
        );
        Ok(rustls::crypto::SharedSecret::from(hybrid_ss.as_slice()))
    }

    fn pub_key(&self) -> &[u8] {
        &self.pub_key_bytes
    }

    fn group(&self) -> NamedGroup {
        // Custom group code: 0x6399 (unassigned, for internal use)
        NamedGroup::Unknown(0x6399)
    }
}
```

---

## sigma-curl Integration

```rust
// tools/sigma-curl/src/lib.rs (TLS client setup)

use rustls::{ClientConfig, RootCertStore};
use std::sync::Arc;

pub fn build_tls_config() -> Arc<ClientConfig> {
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    Arc::new(config)
}
```

---

## Exit Criteria

- `sigma-pkg install sigma-edit` downloads over HTTPS using rustls; cert verification passes.

- `sigma-curl https://registry.sigmaos.dev/v1/index` returns HTTP 200.

- TLS 1.3 is negotiated; TLS 1.0/1.1 connections are rejected.

- Kyber-1024 hybrid KEX is logged in debug output when connecting to a PQC-enabled server.
