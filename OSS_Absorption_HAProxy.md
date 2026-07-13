# OSS Absorption: HAProxy — High-Performance Load Balancing

> **Status**: 📋 Planned | **Source Project**: HAProxy | **Target Shard**: `SigmaOS Network Load Balancer`

---

## 1. Executive Summary

HAProxy is a free, very fast and reliable solution offering high availability, load balancing, and proxying for TCP and HTTP-based applications.

SigmaOS absorbs HAProxy's **non-blocking event loops** and **dynamic weight-based backend routing algorithms**, embedding them directly into `sigma-net-balancer` to handle local and clustered traffic distribution.

---

## 2. Key Features Absorbed

### 2.1 Low-Overhead Network Balance

Instead of context-switching to userspace to balance requests, SigmaOS utilizes HAProxy-inspired routing loops implemented natively inside the kernel's networking layer via eBPF/XDP.

```bash
# Register a service load balancer
$ sigma net balancer create web-service --backends 10.0.0.2:80,10.0.0.3:80
Σ [NET] Load balancer initialized. Alg: Round-Robin.
  Incoming traffic on port 80 balanced across 2 backends.
```

---

## 3. References & Standards

- HAProxy — `haproxy.org` (GPL-2.0 / LGPL-2.1)
