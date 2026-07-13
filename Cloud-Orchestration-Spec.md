# SigmaOS Cloud Orchestration Layer — Architecture Spec

> **Status**: Planning | **Target**: v0.6-cloud | **Codename**: Sigma Nebula

---

## Vision

**Sigma Nebula** is the cloud-native orchestration layer for SigmaOS. It enables SigmaOS nodes to operate as a sovereign cloud platform — without dependence on Kubernetes, Docker, or AWS-proprietary control planes.

The system provides:
- **Sovereign Container Runtime**: Not dependent on OCI/containerd internals
- **Distributed Scheduling**: Multi-node workload orchestration
- **Service Mesh**: Zero-trust inter-service networking
- **Policy-as-Code**: GitOps-based deployment pipeline
- **Observability**: Native OpenTelemetry integration

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Management Plane                       │
│   sigma-ctl CLI │ Web Dashboard │ GitOps Controller     │
├─────────────────────────────────────────────────────────┤
│                   Control Plane                          │
│  sigma-scheduler │ sigma-registry │ sigma-cert-manager  │
├─────────────────────────────────────────────────────────┤
│                   Data Plane                             │
│  sigma-proxy │ sigma-lb │ sigma-dns │ sigma-ingress     │
├─────────────────────────────────────────────────────────┤
│                   Node Agents                            │
│  sigma-node-agent │ sigma-cni │ sigma-csi │ sigma-cri   │
├─────────────────────────────────────────────────────────┤
│             SigmaOS Kernel (Cloud Profile)               │
│  cgroup v2 │ namespaces │ eBPF │ IOMMU │ RDMA           │
└─────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. sigma-node-agent

The node agent runs on every SigmaOS node and is responsible for:

- Registering the node with the control plane
- Reporting resource availability (CPU, RAM, GPU, RDMA)
- Running the container runtime (sigma-cri)
- Enforcing network policies via eBPF
- Health checking and self-healing

```toml
# /etc/sigma-nebula/node.toml
[node]
name = "worker-01"
control_plane = "https://sigma-ctl.cluster.local:6443"
labels = { zone = "us-east-1", gpu = "true" }
taints = []

[resources]
cpu_cores = 32
ram_gb = 128
gpu_vram_gb = 24
storage_gb = 2000

[network]
cni = "sigma-cni"
pod_cidr = "10.244.0.0/24"
```

### 2. sigma-scheduler

Multi-resource aware bin-packing scheduler with:

- **Resource types**: CPU, RAM, GPU, RDMA, storage IOPS, network bandwidth
- **Scheduling policies**: BestFit, MostPacked, Spread, AntiAffinity
- **Priority classes**: Critical (preemptible targets), High, Normal, Low, BestEffort
- **Gang scheduling**: Atomic placement for distributed ML training jobs

```rust
pub struct SchedulingRequest {
    pub workload_id: WorkloadId,
    pub cpu_millicores: u32,
    pub ram_mb: u32,
    pub gpu_count: u8,
    pub priority: SchedulingPriority,
    pub affinity: Vec<AffinityRule>,
    pub node_selector: BTreeMap<String, String>,
}

pub enum SchedulingDecision {
    Scheduled { node_id: NodeId },
    Pending { reason: String },
    Unschedulable { reason: String },
}
```

### 3. sigma-cri (Container Runtime Interface)

SigmaOS-native container runtime using kernel namespaces + cgroups:

```rust
pub trait ContainerRuntime {
    fn create_container(&mut self, spec: ContainerSpec) -> Result<ContainerId, RuntimeError>;
    fn start_container(&mut self, id: ContainerId) -> Result<(), RuntimeError>;
    fn stop_container(&mut self, id: ContainerId, grace_s: u32) -> Result<(), RuntimeError>;
    fn delete_container(&mut self, id: ContainerId) -> Result<(), RuntimeError>;
    fn exec(&mut self, id: ContainerId, cmd: &[&str]) -> Result<ExecSession, RuntimeError>;
    fn logs(&self, id: ContainerId) -> Result<LogStream, RuntimeError>;
    fn stats(&self, id: ContainerId) -> Result<ContainerStats, RuntimeError>;
}
```

Isolation stack per container:

| Layer | Mechanism |
|-------|-----------|
| Process isolation | Linux namespaces (pid, mount, net, ipc, uts) |
| Resource limits | cgroup v2 (sigma-cgroups) |
| Network isolation | sigma-cni (eBPF-based) |
| Filesystem | Overlay FS + read-only root |
| Security | sigma-shield (per-container firewall) + MAC |
| Syscall filter | Seccomp-style sigma-pledge |

### 4. sigma-cni (Container Network Interface)

eBPF-based networking with:
- L3 routing without iptables overhead
- **Latency**: < 10 µs pod-to-pod on same node
- **Throughput**: Line rate (10/25/100 GbE)
- Network policies enforced in kernel eBPF (zero-copy path)
- Service discovery via internal DNS (sigma-dns)

### 5. sigma-proxy (Service Mesh)

Zero-trust service mesh with:
- Automatic mTLS between all services
- Circuit breaking and retries
- Traffic splitting (canary, A/B)
- Rate limiting per service
- Distributed tracing (sigma-trace → OpenTelemetry)

---

## Deployment Manifests

SigmaOS uses TOML-based workload manifests (not YAML):

```toml
# workload.toml
[metadata]
name = "web-frontend"
namespace = "production"
version = "v2.1.0"
labels = { tier = "frontend", env = "prod" }

[spec]
replicas = 3
update_strategy = "rolling"   # rolling | recreate | blue-green
revision_history = 5

[[spec.containers]]
name = "nginx"
image = "sigma-store.io/nginx:1.25.0"
cpu_millicores = 500
ram_mb = 256
env = { NGINX_WORKER_PROCESSES = "auto" }
ports = [{ container = 80, protocol = "tcp" }]

[spec.health_check]
http_path = "/healthz"
port = 80
initial_delay_s = 5
interval_s = 10
timeout_s = 3
failure_threshold = 3

[spec.scaling]
min_replicas = 2
max_replicas = 20
cpu_target_percent = 70
```

---

## GitOps Pipeline

```
Developer pushes → GitHub → sigma-gitops-controller detects change
                                     │
                            Validates manifest against schema
                                     │
                            Runs policy checks (sigma-policy)
                                     │
                            Plans diff (current vs desired state)
                                     │
                            sigma-scheduler assigns nodes
                                     │
                            sigma-cri pulls image + starts container
                                     │
                            sigma-proxy updates routing table
                                     │
                            OpenTelemetry metrics emitted
```

---

## Multi-Region Architecture

```
Region A (Primary)            Region B (Secondary)
┌─────────────────────┐      ┌─────────────────────┐
│ Control Plane       │◄────►│ Control Plane        │
│  sigma-scheduler    │      │  sigma-scheduler     │
├─────────────────────┤      ├─────────────────────┤
│ Worker Nodes (x10)  │      │ Worker Nodes (x5)   │
└─────────────────────┘      └─────────────────────┘
         │                             │
         └──────────┬──────────────────┘
                sigma-gfs (Distributed FS)
                sigma-db (Distributed State)
```

Global routing: Anycast DNS with health-aware failover (< 30 s RTO).

---

## Observability Stack

| Component | Technology | Endpoint |
|-----------|-----------|---------|
| Metrics | OpenTelemetry → sigma-prometheus | `/sigma/metrics` |
| Traces | sigma-trace (distributed) | Jaeger-compatible |
| Logs | sigma-log (structured JSON) | Loki-compatible |
| Events | sigma-audit (tamper-evident) | `/sigma/audit` |
| Alerts | sigma-alert manager | PagerDuty/webhook |

---

## Pricing Model (Foundation-operated Cloud)

| Tier | Resources | Monthly Price |
|------|-----------|-------------|
| Dev | 2 CPU / 4 GB RAM / 20 GB | Free |
| Standard | 8 CPU / 16 GB RAM / 100 GB | $49 |
| Pro | 32 CPU / 64 GB RAM / 500 GB | $199 |
| Enterprise | Custom | Contact foundation |

*Revenue funds the SigmaOS Foundation operations.*

---

## Roadmap

| Milestone | Target | Description |
|-----------|--------|-------------|
| M1 | 2028 Q1 | sigma-cri on single node |
| M2 | 2028 Q2 | Multi-node scheduling |
| M3 | 2028 Q3 | sigma-cni eBPF networking |
| M4 | 2028 Q4 | GitOps controller |
| M5 | 2029 Q1 | sigma-proxy service mesh |
| M6 | 2029 Q2 | Multi-region support |
| M7 | 2029 Q3 | Public cloud launch |
