# Docker Rootless Engine

SigmaOS's **Omarchy Docker Rootless Engine** implements a zero-dependency, no-root-group container runtime configuration engine — inspired by the Omarchy `docker-no-group` branch and surpassing it with Podman-OCI parity and systemd user-unit generation.

---

## Architecture Comparison

| Metric | Omarchy docker-no-group | Linux Mint (n/a) | SigmaOS Docker Rootless Engine |
|---|---|---|---|
| Core language | Shell | — | Safe Rust (`#![no_std]`) |
| Docker group | Removes it | — | Never needed — rootless by default |
| Rootless runtime | dockerd-rootless.sh | — | dockerd-rootless + Podman/OCI |
| Config generation | Shell script | — | Programmatic `daemon.json` + systemd unit |
| subuid/subgid | Manual | — | **Auto-generated** per-user mapping |
| Network isolation | Default bridge | — | `slirp4netns` (no root bridge) |
| External deps | docker, shell | — | **Zero** |

---

## Architectural Highlights

- **Rootless-first** — never installs the `docker` group; uses user namespaces instead
- **Dual runtime** — supports `dockerd-rootless` and Podman OCI runtimes with `crun`
- **Programmatic daemon.json** — generates per-mode `daemon.json` with `userns-remap`, logging, and `no-new-privileges`
- **subuid/subgid generator** — emits the correct `/etc/subuid` and `/etc/subgid` entries (100000–165535)
- **Systemd user unit** — generates a complete `[Unit]/[Service]/[Install]` user unit for `dockerd-rootless.sh`
- **Run command builder** — produces rootless-compatible `docker run` commands with `--userns=keep-id` and `slirp4netns`
- **Security posture** — `no-new-privileges`, JSON-file log driver, max 10 MB per log file, 3 files max

---

## API & Usage

```rust
use sigmaos::system::omarchy_docker_rootless_engine::{
    OmarchyDockerRootlessEngine, DockerMode
};

let mut engine = OmarchyDockerRootlessEngine::new();
// Default: rootless mode

// Register containers
engine.register_container("sigma-web", "sigmaos/web:latest");
engine.register_container("sigma-db", "postgres:16-alpine");

// Generate daemon.json
let daemon_json = engine.generate_daemon_json();
// {"userns-remap":"default","no-new-privileges":true,...}

// subuid/subgid entries
let subuid = engine.generate_subuid_entry("aaryan");
// "aaryan:100000:65536"

// Systemd user unit
let unit = engine.generate_systemd_user_unit();
// [Unit] Description=Docker Application Container Engine (Rootless)...

// Rootless run command
let cmd = engine.get_run_command("sigma-web").unwrap();
// docker run --rm --network slirp4netns --userns=keep-id ...

// Switch to Podman
engine.set_mode(DockerMode::PodmanRootless);
```

---

## Security Model

```
Traditional:            SigmaOS Rootless:
┌─────────────┐        ┌─────────────────────┐
│  root       │        │  user (uid=1000)     │
│  dockerd    │  ───▶  │  dockerd-rootless    │
│  docker grp │        │  uid 100000–165535   │
└─────────────┘        │  slirp4netns         │
                       └─────────────────────┘
```

---

## Testing

```bash
rustc --test src/system/omarchy_docker_rootless_engine.rs \
  --edition=2021 --cfg 'feature="standalone_test"' \
  -o build/test_docker && ./build/test_docker
# test result: ok. 1 passed; 0 failed
```

---

## Related Components

- [Container Virtualization](Container-Virtualization.md) — OCI/Podman architecture
- [Init and Services](Init-and-Services.md) — systemd user unit integration
- [Security and Hardening](Security-and-Hardening.md) — privilege isolation
