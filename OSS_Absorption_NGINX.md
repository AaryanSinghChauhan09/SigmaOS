# OSS Absorption: NGINX — Event-Driven Architecture

> **Status**: 📋 Planned | **Source Project**: NGINX | **Target Shard**: `SigmaOS Network Gateway`

---

## 1. Executive Summary

NGINX popularized the asynchronous, event-driven approach to handling network connections, allowing a single thread to handle tens of thousands of concurrent HTTP requests efficiently, fundamentally outperforming process-per-connection models like Apache.

SigmaOS absorbs the **asynchronous event loop** and **Reverse Proxy** concepts, embedding them directly into the `sigma-net` stack to provide a zero-configuration HTTP gateway for local services.

---

## 2. Key Features Absorbed

### 2.1 The Kernel-level Gateway

Instead of running NGINX in userspace and context-switching for every packet, SigmaOS leverages eBPF (like Cloudflare's pingora) and the kernel's io_uring equivalent to parse HTTP headers and route traffic dynamically.

If an incoming request hits port 80/443, the kernel looks at the Host header and forwards the connection directly to the sandboxed application's file descriptor without waking up an intermediate proxy daemon.

### 2.2 Declarative Proxy Rules

Local web developers or system administrators can map domains to containers via simple configuration, and the system handles the rest (including ACME/Let's Encrypt certificates).

```toml
# /etc/sigma/gateway.toml
[[route]]
domain = "app.local"
target = "container:webapp_1"
port = 3000
tls = "auto"
```

---

## 3. References & Standards

- NGINX — `nginx.org` (BSD-2-Clause)
- Pingora — Cloudflare's Rust proxy
