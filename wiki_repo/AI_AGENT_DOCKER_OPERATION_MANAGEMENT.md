# 🐳 AI Agent Docker & OCI Container Operation Management in SigmaOS

## Executive Summary
Container operations in SigmaOS provide lightweight, isolated application execution environments compliant with the Open Container Initiative (OCI) runtime and image specifications. Autonomous AI Agents (**Bolt ⚡**, **Palette 🎨**, and **Sentinel 🛡️**) managing containerized microservices, build pipelines, and system workloads must manage Docker / containerd runtime daemons, overlay filesystem layers, cgroups v2 resource limits, and network bridges safely while adhering to zero-trust security boundaries.

---

## 1. OCI Container Runtime Architecture

SigmaOS implements a native container management engine (`SigmaContainer`) supporting Docker API compatibility:

```
+-------------------------------------------------------------+
|               SigmaOS Docker Operation Engine               |
+-------------------------------------------------------------+
                              │
         ┌────────────────────┴────────────────────┐
         ▼                                         ▼
 [OCI Image Layer Store]                [Container Execution]
 • OverlayFS / Btrfs CoW                • `runc` / MicroVM Isolation
 • Content-Addressable Storage          • Linux Namespaces (PID, Mount, Net)
 • Image Digest Signature Verification  • cgroups v2 Resource Quotas
```

### Key Components
- **`SigmaContainerDaemon`**: Manages container lifecycles (`create`, `start`, `pause`, `stop`, `remove`).
- **Overlay2 Storage Driver**: Merges read-only lower image layers with a mutable upper container layer.
- **CNI Bridge & veth Pairs**: Configures virtual ethernet pairs connecting container network namespaces to the host `sigma0` bridge.

---

## 2. Isolation & Resource Enforcement

Containers operating under AI Agent supervision MUST enforce multi-layered isolation primitives:

1. **Linux Namespaces**:
   - `CLONE_NEWPID`: Process ID space isolation.
   - `CLONE_NEWNET`: Private network stack with dedicated virtual interfaces.
   - `CLONE_NEWMNT`: Private mount points prevents host filesystem leakage.
   - `CLONE_NEWUSER`: UID/GID mapping (maps container root `0` to unprivileged host user).

2. **cgroups v2 Quotas**:
   - `cpu.max`: Restricts maximum CPU quota (e.g. `50000 100000` for 0.5 CPU cores).
   - `memory.max`: Hard memory limit with out-of-memory (OOM) killer protection.
   - `io.max`: Throttle disk read/write IOPS and bandwidth.

---

## 3. Security & Sandboxing Policy

- **Rootless Containers**: Run container daemons under unprivileged user namespaces by default.
- **Seccomp BPF Filtering**: Enforce restrictive syscall filter profiles blocking `ptrace`, `kexec_load`, and raw socket manipulation.
- **Cap Drop**: Drop all ambient Linux capabilities except essential rights (`CAP_NET_BIND_SERVICE`).

---

## 4. AI Agent Operational Guidelines

1. **Bolt ⚡ (Performance Optimization)**:
   - Use multi-stage Docker builds and cached OverlayFS layers to accelerate container startup times.
   - Monitor container cgroup memory limits to prevent unexpected host OOM thrashing.

2. **Palette 🎨 (UX & Visibility)**:
   - Provide clear CLI/GUI dashboard status indicators for container health, port mappings, and resource usage.

3. **Sentinel 🛡️ (Security & Compliance)**:
   - Scan container image layers for CVE vulnerabilities and verify image signatures using Sigstore/Cosign before spawning container instances.
