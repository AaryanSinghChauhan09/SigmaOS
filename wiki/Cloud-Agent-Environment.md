# Cloud Agent Environment

SigmaOS's **Omarchy Cloud Agent Environment Engine** manages AI agent deployments across cloud providers — inspired by the Omarchy `cloud-agent-environment` branch and extended with multi-cloud, secret management, and manifest generation.

---

## Architecture Comparison

| Metric | Omarchy cloud-agent-environment | Linux Mint (n/a) | SigmaOS Cloud Agent Environment |
|---|---|---|---|
| Core language | Shell / YAML | — | Safe Rust (`#![no_std]`) |
| Cloud providers | 1 (manual) | — | AWS, GCP, Azure, Hetzner, Fly.io, Cloudflare, SigmaCloud |
| Secret management | Plaintext env | — | Secret-flagged vars excluded from manifests |
| Manifest format | Manual fly.toml | — | **Auto-generated** per-provider manifests |
| Resource limits | None | — | CPU (millicores) + Memory (MiB) per target |
| External deps | Shell, YAML libs | — | **Zero** |
| Multi-target | No | — | Yes — unlimited targets |

---

## Architectural Highlights

- **Multi-cloud targets** — register targets on AWS, GCP, Azure, Hetzner, Fly.io, Cloudflare, or SigmaCloud
- **Global + per-target env vars** — global vars shared across all targets; target-specific vars override
- **Secret isolation** — `is_secret: true` vars are tracked but never written into generated manifests
- **Manifest generator** — produces deployment manifests in Fly.toml / Kubernetes-compatible format
- **Resource limits** — CPU in millicores (1000m = 1 vCPU), memory in MiB
- **Target listing** — enumerate all registered targets for CI/CD automation
- **SigmaCloud-native** — first-class `SigmaCloud` provider for future SigmaOS hosting platform

---

## API & Usage

```rust
use sigmaos::ai::omarchy_cloud_agent_environment::{
    OmarchyCloudAgentEnvironment, CloudProvider
};

let mut env = OmarchyCloudAgentEnvironment::new();

// Global env (shared across all targets)
env.set_global_var("SIGMAOS_VERSION", "0.1.0", false);
env.set_global_var("OPENAI_API_KEY", "sk-...", true);  // secret — not in manifests

// Register targets
env.add_target("sigma-prod", CloudProvider::Hetzner, "hel1");
env.add_target("sigma-edge", CloudProvider::Cloudflare, "global");
env.add_target("sigma-dev", CloudProvider::Fly, "ams");

// Per-target vars
env.set_target_var("sigma-prod", "LOG_LEVEL", "warn", false);
env.set_limits("sigma-prod", 2000, 1024); // 2 vCPU, 1 GiB

// Generate deployment manifest
let manifest = env.generate_manifest("sigma-prod").unwrap();
// # SigmaOS Cloud Agent Manifest — sigma-prod
// # Provider: Hetzner  # Region: hel1
// [env]
//   SIGMAOS_VERSION = "0.1.0"
//   LOG_LEVEL = "warn"
// [resources]
//   cpu = "2000m"  memory = "1024Mi"

// List all targets
let names = env.list_targets();
// ["sigma-prod", "sigma-edge", "sigma-dev"]
```

---

## Supported Cloud Providers

| Provider | API Name | Use Case |
|---|---|---|
| AWS | `Aws` | Enterprise, large-scale |
| GCP | `Gcp` | ML workloads, TPU access |
| Azure | `Azure` | Enterprise Windows compat |
| Hetzner | `Hetzner` | Cost-efficient EU hosting |
| Fly.io | `Fly` | Edge, low-latency global |
| Cloudflare | `Cloudflare` | Workers, edge compute |
| SigmaCloud | `SigmaCloud` | Native SigmaOS platform |

---

## Testing

```bash
rustc --test src/ai/omarchy_cloud_agent_environment.rs \
  --edition=2021 --cfg 'feature="standalone_test"' \
  -o build/test_cloud_agent && ./build/test_cloud_agent
# test result: ok. 1 passed; 0 failed
```

---

## Related Components

- [AI and ML Engine](AI-and-ML-Engine.md) — agent runtime
- [Multi-Agent Provider Orchestration](Multi-Agent-Provider-Orchestration.md) — local agents
- [Grok AI Panel Agent](Grok-AI-Panel-Agent.md) — UI integration
