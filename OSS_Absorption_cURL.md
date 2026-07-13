# OSS Absorption: cURL & QUIC — Modern Network Transports

> **Status**: 🔄 Active | **Source Projects**: cURL, quinn (QUIC), nghttp2 | **Target Shard**: `SigmaOS Network Transport Layer`

---

## 1. Executive Summary

cURL is the undisputed king of command-line data transfer and the underlying `libcurl` powers thousands of applications. However, modern network protocols (HTTP/3, QUIC) are shifting networking complexity from the kernel (TCP) to userspace (UDP-based QUIC).

SigmaOS absorbs the **universal transfer interface** of cURL and integrates it with **Rust-native QUIC implementations** (`quinn`) to provide `sigma-fetch`, a universally available, cryptographically modern transport layer.

---

## 2. Key Features Absorbed

### 2.1 The `sigma-fetch` API

Instead of applications each bringing their own HTTP/TLS libraries (leading to bloat and security vulnerabilities), SigmaOS provides a system-level transfer API.

```bash
# Unified command-line transfer tool
$ sigma fetch https://example.com/data.tar.gz --progress
Σ [FETCH] Establishing HTTP/3 (QUIC) connection...
  TLS 1.3 negotiated (X25519)
  Downloading: [██████████] 100% (45MB/s)
```

### 2.2 Userspace QUIC Integration

QUIC replaces TCP+TLS with a single, multiplexed, encrypted-by-default UDP protocol. SigmaOS utilizes the Rust `quinn` library to provide this natively to all applications via the IPC bus, bypassing the legacy TCP stack entirely for modern connections.

```rust
// userland/net/transport.rs
// SPDX-License-Identifier: MIT

pub async fn secure_transfer(url: &str) -> Result<Vec<u8>> {
    // Transparently negotiates HTTP/3 over QUIC if supported,
    // falling back to HTTP/2 over TCP.
    let client = SigmaNetClient::new();
    let response = client.get(url).send().await?;
    
    Ok(response.bytes().await?)
}
```

---

## 3. References & Standards

- cURL — `curl.se` (MIT/X derivative)
- quinn (Rust QUIC implementation) — `github.com/quinn-rs/quinn` (MIT / Apache-2.0)
- HTTP/3 & QUIC IETF Standards
