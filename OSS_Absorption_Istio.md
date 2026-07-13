# OSS Absorption: Istio — Service Mesh Control Plane

> **Status**: 📋 Planned | **Source Project**: Istio (CNCF) | **Target Shard**: `SigmaOS Sovereign Mesh`

---

## 1. Executive Summary

Istio is the leading service mesh solution, providing traffic management, security (mTLS), and observability for microservice architectures without requiring application code changes. It uses Envoy as its sidecar data plane proxy and provides a rich control plane API.

SigmaOS absorbs Istio's **mTLS-by-default sidecar model** and **policy-based traffic management** into `sigma-mesh`, achieving mutual authentication between all inter-service calls using SigmaOS capability tokens instead of X.509 certificates.

---

## 2. Key Features to Absorb

### 2.1 mTLS Between All Services (Zero-Trust)

Every service-to-service call in SigmaOS is automatically encrypted and mutually authenticated. There is no "trust the internal network" assumption.

```bash
$ sigma mesh mtls status
Σ [MESH] mTLS configuration:
  Default policy:  STRICT (all plaintext blocked)
  backend-api:     STRICT ✓
  auth-svc:        STRICT ✓
  legacy-service:  PERMISSIVE (migration mode, plain allowed)
```

### 2.2 Traffic Shifting for Canary Deployments

`sigma-mesh` can split traffic between service versions by percentage, enabling safe canary releases without DNS changes.

```toml
# /etc/sigma/mesh/traffic/backend-api.toml
[[route]]
destination = "backend-api-v1"
weight = 90

[[route]]
destination = "backend-api-v2"
weight = 10   # 10% canary
```

```bash
$ sigma mesh traffic apply backend-api
Σ [MESH] Traffic split applied:
  backend-api-v1: 90% of requests
  backend-api-v2: 10% of requests (canary)
```

---

## 3. References & Standards

- Istio — `istio.io` (Apache-2.0)
