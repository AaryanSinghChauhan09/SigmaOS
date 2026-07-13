# OSS Absorption: Docker — Container Engine

> **Status**: 📋 Planned | **Source Project**: Docker | **Target Shard**: `SigmaOS Container Subsystem`

---

## 1. Executive Summary

Docker revolutionized application packaging and deployment by introducing containerization using Linux cgroups and namespaces.

While SigmaOS targets daemonless runtime execution via Podman-inspired logic, it absorbs **Docker's OCI (Open Container Initiative) Image Layering and caching model**, ensuring full compatibility with existing Dockerfiles and container registries.

---

## 2. Key Features Absorbed

### 2.1 OCI Layer Caching (`sigma-image`)

SigmaOS implements a local image builder and cache manager that understands standard OCI container layers. This allows users to pull, build, and deploy containers from Docker Hub seamlessly, without requiring the resource-heavy background Docker daemon.

```bash
$ sigma container run alpine echo "Hello from isolated container!"
Σ [CONTAINER] Pulling layer sha256:82d2...
  Creating overlayfs rootfs...
  Spawning namespace sandbox...
  Hello from isolated container!
```

---

## 3. References & Standards

- Docker — `docker.com` (Apache-2.0 License)
- OCI Image Specification — `opencontainers.org`
