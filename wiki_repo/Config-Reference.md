# SigmaOS Config.toml Reference

The `sigma.toml` configuration file acts as the single source of truth for the entire SigmaOS build, boot, identity, network, theme, and package configuration lifecycle.

## File Location
Create a `sigma.toml` file at the root of your project codebase directory.

## Full Schema Structure

```toml
[profile]
preset  = "desktop"          # desktop | server | edge | embedded | airgapped
arch    = "x86_64"           # x86_64 | aarch64 | riscv64gc
board   = "generic-uefi"     # generic-uefi | raspi4 | beaglev | cloud-vm
name    = "SigmaOS"          # custom distro name
version = "15.0.0"

[kernel]
scheduler     = "eevdf"         # cfs | eevdf | realtime
allocator     = "buddy"         # buddy | slab | tlsf
ipc           = "capability"    # message-passing | shared-mem | capability
network_stack = "full"          # full | minimal | none
features      = ["audit", "aslr", "capabilities", "namespaces"]
filesystems   = ["ext-like", "log-structured"]

[network]
profile   = "home"           # home | travel | airgapped | tor-only | mesh
dns       = "doh"            # system | doh | doh+dnsec
firewall  = "sigma-shield"   # none | sigma-shield | strict
wifi      = true
bluetooth = true

[theme]
palette = "phosphor-green"   # phosphor-green | ocean | amber | monochrome
font    = "jetbrains-mono"   # jetbrains-mono | berkeley-mono | inter
density = "comfortable"      # compact | comfortable | spacious
mode    = "dark"             # dark | light | auto

[security]
signing_key         = ""     # path to verification key
secure_boot         = false
capability_profiles = true

[packages]
include = ["sigma-shell", "sigma-core", "sigma-tui"]
exclude = []
```

## Validate Configuration
You can validate your configuration file syntax and schema matching using the built-in CLI validation tools:

```bash
sigma config validate
```

If the validation is successful, the terminal will report a `SUCCESS` status. If there are missing fields or sections, it will print detail errors.
