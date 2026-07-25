# OSS Absorption: Consul — Service Discovery & Configuration

> **Status**: 📋 Planned | **Source Project**: HashiCorp Consul | **Target Shard**: `SigmaOS Sovereign Mesh`

---

## 1. Executive Summary

HashiCorp Consul provides service discovery, health checking, KV configuration storage, and service mesh capabilities with built-in Connect (mTLS). It is widely used as the control plane for distributed systems.

SigmaOS absorbs Consul's **service catalog with health checking**, **KV configuration store**, and **intention-based access control** into `sigma-mesh`, creating a native service discovery and configuration backbone.

---

## 2. Key Features Absorbed

### 2.1 Service Catalog with Health Checks

Every service registered in `sigma-mesh` is continuously health-checked. Unhealthy services are automatically removed from DNS resolution and load-balancer pools.

```bash
$ sigma mesh services
Σ [MESH] Registered services:
  NAME         INSTANCES  HEALTHY  TAGS
  backend-api  3          3        [prod, v2.1]
  auth-svc     2          1        [prod]  ⚠ 1 failing
  cache-redis  1          1        [prod]
```

### 2.2 KV Configuration Store

A distributed, strongly-consistent key-value store provides runtime configuration to all services without file-based config deployment.

```bash
$ sigma mesh kv put config/rate-limit/max-rps 500
$ sigma mesh kv get config/rate-limit/max-rps
500

$ sigma mesh kv watch config/rate-limit/max-rps
Σ [MESH KV] Watching config/rate-limit/max-rps (Ctrl+C to stop)
  [12:01:05] Value changed: 500 → 1000
```

### 2.3 Intention-Based Access Control

Service-to-service communication is governed by explicit allow/deny intentions, enforced at the mesh layer via capability tokens.

```bash
$ sigma mesh intention create --allow backend-api → cache-redis
$ sigma mesh intention create --deny  auth-svc → cache-redis
$ sigma mesh intentions list
  ALLOW  backend-api  →  cache-redis
  DENY   auth-svc     →  cache-redis
```

---

## 3. References & Standards

- HashiCorp Consul — `consul.io` (MPL-2.0 / BSL-1.1)
