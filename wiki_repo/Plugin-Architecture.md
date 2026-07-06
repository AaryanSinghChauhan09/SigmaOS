# Plugin Architecture

> SigmaOS v15.0 "Zenith" — Plugin System Reference

## Overview

SigmaOS supports a plugin architecture at multiple layers, allowing third-party extensions without modifying core kernel or system binaries.

---

## Plugin Layers

| Layer | Mechanism | Use Cases |
|---|---|---|
| Kernel | eBPF programs | Custom tracing, network filters, security policies |
| Init | `.service` drop-ins | Extend existing services |
| Package Manager | Repo plugins | Custom package sources, formats |
| Desktop | Zenith extensions | Applets, widgets, themes |
| AI Agent | Model plugins | Custom inference backends |
| Shell | `sigma-shell` functions | Custom commands, completions |

---

## Kernel Plugins — eBPF

SigmaOS supports loading eBPF programs for safe kernel extensibility:

```bash
# Load an eBPF program
sigma-bpf load /etc/sigma/bpf/my_filter.o --type xdp --interface eth0

# List loaded programs
sigma-bpf list

# Unload a program
sigma-bpf unload my_filter
```

eBPF programs are verified by the kernel verifier before loading. They cannot crash the kernel.

---

## Init Drop-in Files

Extend an existing service without modifying the original `.service` file:

```
/etc/sigma/services/nginx.service.d/
└── 10-custom-limit.conf
```

```ini
# 10-custom-limit.conf
[Service]
MemoryMax = 1G
CPUWeight = 80
```

Drop-ins are automatically merged with the base service definition.

---

## Package Manager Plugins

Custom package sources can be added as plugins:

```toml
# /etc/sigma/pkg-plugins/flatpak.toml
[plugin]
name = "flatpak"
type = "source"
exec = "/usr/lib/sigma-pkg/plugins/flatpak-bridge"
```

This allows `sigpkg install org.gnome.Builder` to transparently install from Flatpak.

---

## Zenith Desktop Extensions

Desktop extensions live in `/usr/share/zenith/extensions/` or `~/.zenith/extensions/`:

```
my-weather-applet/
├── manifest.json    — Extension metadata
├── index.js         — Main logic
└── style.css        — Styling
```

```json
{
  "name": "Weather Applet",
  "version": "1.0.0",
  "type": "panel-applet",
  "main": "index.js",
  "permissions": ["network", "location"]
}
```

Install via:
```bash
sigpkg install sigma-ext-weather-applet
# or manually:
zenith-ext install ./my-weather-applet/
```

---

## AI Agent Model Plugins

Swap out the default AI model or inference backend:

```toml
# /etc/sigma/ai-agent.toml
[model]
path = "/opt/models/llama-3-8b-q4.gguf"
backend = "llama.cpp"
context_size = 4096
```

Custom inference backends must implement the `SigmaInferenceBackend` trait:

```rust
pub trait SigmaInferenceBackend {
    fn load(&mut self, model_path: &str) -> Result<(), String>;
    fn infer(&self, prompt: &str, max_tokens: usize) -> Result<String, String>;
    fn clear_context(&mut self);
}
```

---

## Shell Plugins

Extend `sigma-shell` with custom functions and completions:

```bash
# ~/.sigmarc.d/my-functions.sh

sigma_deploy() {
    echo "Deploying $1 to $2..."
    sigma-ai "create a deployment plan for $1"
}
```

Auto-completion plugins:
```bash
# ~/.sigmarc.d/completions/my-tool.sh
_my_tool_completions() {
    COMPREPLY=($(compgen -W "start stop status" "${COMP_WORDS[1]}"))
}
complete -F _my_tool_completions my-tool
```

---

## Plugin Security

All plugins must declare permissions in their manifest. Denied by default:

- `network` — Internet/local network access
- `filesystem` — Read/write outside home directory
- `kernel` — eBPF program loading
- `secrets` — Sigma Vault access
- `display` — Zenith compositor access

Users are prompted to approve permissions on first install.
