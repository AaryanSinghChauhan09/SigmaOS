# OSS Absorption: Caddy — Automatic HTTPS Web Server

> **Status**: 📋 Planned | **Source Project**: Caddy | **Target Shard**: `SigmaOS Auto-TLS Gateway`

---

## 1. Executive Summary

Caddy is a powerful, enterprise-ready, open-source web server written in Go with automatic HTTPS built in. By default, it provisions and rotates TLS certificates through Let's Encrypt without any user configuration.

SigmaOS absorbs Caddy's **automatic certificate provisioning model** and integrates it into `sigma-gateway`, ensuring that any internally hosted service can get a trusted TLS certificate with zero manual steps.

---

## 2. Key Features Absorbed

### 2.1 Zero-Config TLS for Local Services

When a local developer registers a service in `sigma-gateway`, it automatically uses ACME to provision a real public TLS certificate (or a trusted local CA certificate for LAN access).

```bash
$ sigma gateway add dev.sigma.local --backend localhost:3000
Σ [GATEWAY] Route registered: dev.sigma.local → :3000
  Provisioning TLS certificate via ACME...
  Certificate issued: dev.sigma.local (expires 90d)
```

---

## 3. References & Standards

- Caddy — `caddyserver.com` (Apache-2.0)
