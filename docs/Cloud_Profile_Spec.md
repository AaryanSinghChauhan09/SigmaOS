# SigmaOS Cloud/Headless Profile Specification

## Overview

The SigmaOS cloud profile is a minimal, headless image designed for virtual machines, containers, and serverless workloads. It boots via cloud-init, uses an immutable root filesystem, and exposes OCI-compatible container management via `sigma-pod`.

---

## Boot Sequence

```
UEFI (OVMF) → sigma-boot.efi
  │
  ▼ sigma-init (cloud variant)
  │  1. Parse cloud-init user-data from:
  │     - IMDS (http://169.254.169.254/latest/user-data for AWS/GCP)
  │     - Config drive (NoCloud datasource, ISO 9660)
  │  2. Apply cloud-init modules:
  │     - hostname, ssh-keys, users, write-files, runcmd
  │  3. Start system shards: sigma-net, sigma-sshd, sigma-otel-collector
  │  4. Start sigma-pod runtime
  ▼
  sigma-pod ready (container workloads accepted)
```

---

## Immutable Root: dm-verity + OSTree A/B

The root filesystem is mounted read-only with dm-verity:

```
sigma-boot.efi sets up:
  dm-verity device → /dev/dm-0 (verified read-only root)
    ├── root_hash: 64-char hex, embedded in kernel cmdline
    └── hash_device: /dev/sda2 (verity hash tree partition)

VFS mounts:
  /           → dm-verity (read-only SigmaFS)
  /etc/sigma  → overlayfs (writable upper layer on /data)
  /tmp        → tmpfs
  /data       → /dev/sda4 (persistent data partition)
```

OSTree A/B deployment: see [OSTRee-updates.md](integrations/OSTRee-updates.md).

---

## sigma-pod: OCI Container Lifecycle

`sigma-pod` is the SigmaOS OCI container runtime, implementing the same lifecycle as `containerd` + `runc`:

| Command | Action |
|---|---|
| `sigma-pod create <id> <image>` | Pull image (if needed), create OCI bundle |
| `sigma-pod start <id>` | Start container process (via sigma-shim) |
| `sigma-pod stop <id>` | Send SIGTERM → SIGKILL after grace period |
| `sigma-pod delete <id>` | Remove OCI bundle, release resources |
| `sigma-pod exec <id> <cmd>` | Execute command in running container |
| `sigma-pod ps` | List containers with SVID identities |

### sigma-pod Spec (sigma-pod.toml)

```toml
[pod]
name  = "web-server"
image = "sigmaos/nginx:1.26"

[[containers]]
name    = "nginx"
image   = "sigmaos/nginx:1.26"
command = ["nginx", "-g", "daemon off;"]
ports   = [{ host = 8080, container = 80 }]
pledge  = "stdio rpath inet"
unveil  = "/usr/share/nginx/html:/r,/var/log/nginx:/wc"

[resources]
cpu_quota     = "0.5"  # 50% of one CPU
memory_limit  = "128m"
```

---

## Network: VirtIO-net + Cloud Metadata

```bash
# DHCP via virtio-net (auto-configured by sigma-net shard)
ip addr show eth0
# → 10.0.2.15/24 (QEMU user-mode) or cloud-assigned IP

# Cloud metadata access (IMDS)
sigma-metadata get instance-id
sigma-metadata get public-ipv4
sigma-metadata get user-data
```

The `sigma-metadata` tool queries the cloud IMDS endpoint via sigma-curl and caches responses with a 60-second TTL.

---

## Observability: OTel Traces + Prometheus Metrics

The cloud profile ships with OTel and Prometheus pre-configured:

```bash
# OTel traces: sigma-bus IPC spans exported to Jaeger
# Prometheus metrics available at :8888/metrics

curl http://localhost:8888/metrics | grep sigma_
# sigma_bus_ipc_calls_total{shard="sigma-net"} 12345
# sigma_pod_containers_active 3
# sigma_cpu_usage_percent{core="0"} 42.1
```

### Grafana Cloud Integration

```bash
# Push metrics to Grafana Cloud
sigma-otel-collector configure \
  --grafana-endpoint https://prometheus-prod.grafana.net/api/prom/push \
  --grafana-token $GRAFANA_CLOUD_TOKEN
```

---

## Cloud Image Sizes

| Image | Size | Profile |
|---|---|---|
| sigma-cloud-minimal.img | ~50 MB | sigma-init + sigma-pod + sigma-net |
| sigma-cloud-standard.img | ~150 MB | + sigma-otel + sigma-sshd + dev tools |
| sigma-cloud-gpu.img | ~500 MB | + Mesa + CUDA/ROCm runtime |
