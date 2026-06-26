# ☁️ SigmaOS Cloud Branch — `release/cloud`

> **Sovereign Cloud Computing: Your infrastructure, your silicon, your rules.**

The `release/cloud` branch targets **cloud-native and hyperscale deployment**, absorbing best practices from:
- **CoreOS / Fedora CoreOS** (immutable OS, container-first)
- **Ubuntu Server Cloud Image** (cloud-init, minimal footprint)
- **Alpine Linux** (smallest container base)
- **NixOS** (reproducible declarative cloud configs)
- **Bottlerocket** (AWS purpose-built container OS)

---

## 🏗 Cloud Deployment Modes

| Mode | Description | Target |
|------|-------------|--------|
| **Bare-Metal Cloud** | SigmaOS directly on physical cloud hardware | Hetzner, OVH, Equinix |
| **VM Image** | qcow2/vmdk cloud image with cloud-init | AWS, GCP, Azure, DigitalOcean |
| **Container Runtime** | Minimal SigmaOS base for OCI containers | Docker, Podman, containerd |
| **Kubernetes Node** | SigmaOS as a K8s worker node OS | EKS, GKE, AKS, self-hosted |

---

## 🚀 Cloud-Init Sovereign Implementation

Absorbed from **Ubuntu's cloud-init** and **CoreOS Ignition** concepts:

```cpp
/* sigma_cloud_init.cpp — Zero-dependency cloud configuration */
struct SigmaCloudConfig {
    char hostname[64];
    char ssh_authorized_key[512];
    char user[32];
    char password_hash[128]; /* SHA-256, sovereign impl */
    bool enable_networking;
    u8   static_ip[4];
    u8   gateway[4];
    u8   dns[4];
};
```

### Supported cloud-init directives (YAML parsed by sovereign parser):
- `hostname:` → sets `/etc/hostname`
- `users:` → creates system users
- `ssh_authorized_keys:` → writes to `~/.ssh/authorized_keys`
- `runcmd:` → executes at first boot
- `packages:` → installs via `sigma-pkg`
- `write_files:` → writes arbitrary files

---

## 📦 Cloud-Specific Shards

| Shard | Source File | Description |
|-------|-------------|-------------|
| Cloud Init Engine | `sigma_cloud_init.cpp` | First-boot config processor |
| Instance Metadata | `sigma_imds.cpp` | AWS/GCP IMDS client (raw TCP) |
| Object Storage | `sigma_s3_client.cpp` | Sovereign S3 protocol client |
| Cloud Firewall | `sigma_cloud_fw.cpp` | VPC security group enforcement |
| Auto Scaler Hook | `sigma_autoscale.cpp` | Scale-in/out lifecycle handler |
| Telemetry Agent | `sigma_cloud_telemetry.cpp` | Metrics push (no third-party SDK) |

---

## 🔐 Sovereign Cloud Security

Absorbed from **HashiCorp Vault** principles and **AWS Nitro Enclaves**:

- **Instance Identity** — TPM 2.0 attestation via `sigma_tpm.cpp`
- **Secrets at Rest** — ChaCha20-Poly1305 (sovereign impl, no OpenSSL)
- **Secrets in Transit** — TLS 1.3 via sovereign `sigma_tls.cpp`
- **IMDSv2 Enforcement** — Token-based metadata access (prevents SSRF)

---

## 🌐 Networking Stack for Cloud

```
┌─────────────────────────────────────┐
│  sigma_dhcp.cpp   (DHCP client)     │
│  sigma_arp.cpp    (ARP table)       │
│  sigma_net_ipv4.cpp (IPv4 routing)  │
│  sigma_net_tcp.cpp  (TCP/IP stack)  │
│  sigma_e1000.cpp  (virtio-net NIC)  │
└─────────────────────────────────────┘
```

---

## 📊 Cloud Image Build Pipeline

```bash
# Build minimal cloud image (no GUI, no debug symbols)
make PROFILE=cloud ARCH=x86_64 image

# Output: sigmaos-cloud-amd64.qcow2 (~32MB)
# Supports: cloud-init, virtio-blk, virtio-net
```

---

*Branch: `release/cloud` | Minimum image size target: 32MB*
