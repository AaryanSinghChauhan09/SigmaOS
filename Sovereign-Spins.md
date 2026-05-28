# SigmaOS Spins — Profession-Based Editions

SigmaOS ships a **sovereign core** — a minimal, secure, non-POSIX base OS — and layers **profession-optimized spins** on top, similar to how Debian Edu, SteamOS, CAINE, and Fedora CoreOS tailor Linux to specific audiences.

Each spin is installable independently or upgrading from the core with:

```bash
sigma-spin install <name>
```

---

## 🗂 Available Spins

| Spin | Audience | Inspired By |
|------|----------|------------|
| [**SigmaOS Dev**](Spin-Dev.md) | Developers & Engineers | Clear Linux, Fedora Silverblue |
| [**SigmaOS Creative**](Spin-Creative.md) | Designers, Artists, Musicians | Ubuntu Studio, Fedora Design Suite |
| [**SigmaOS Gaming**](Spin-Gaming.md) | Gamers | SteamOS, Garuda Linux |
| [**SigmaOS Edu**](Spin-Edu.md) | Students & Teachers | Debian Edu, Endless OS |
| [**SigmaOS Science**](Spin-Science.md) | Researchers & Data Scientists | Scientific Linux, NixOS |
| [**SigmaOS Business**](Spin-Business.md) | Enterprise & Productivity | Ubuntu LTS, openSUSE Leap |
| [**SigmaOS Secure**](Spin-Secure.md) | Security, Forensics, Recovery | CAINE, Kali, SystemRescue |

---

## 🏗 Architecture

```
┌─────────────────────────────────────────────┐
│           SigmaOS Sovereign Core            │
│   (Kernel · WASI · VFS · Crypto · HAL)      │
└────────────────────┬────────────────────────┘
                     │
     ┌───────────────┼───────────────┐
     ▼               ▼               ▼
  Dev Spin      Gaming Spin     Secure Spin
  Creative    Science Spin    Business Spin
  Edu Spin
```

Each spin **inherits** the sovereign core but ships its own:
- **Package manifest** — curated set of pre-installed tools
- **Default profile** — AppArmor/sandbox policies tuned for the use-case
- **Desktop layout** — Zenith WM configured for the profession
- **OmniPackage repos** — curated software repositories for that spin

---

## 📦 Installing Spins

```bash
# List available spins
sigma-spin list

# Install a spin over the running core
sigma-spin install gaming

# Verify spin integrity (Dilithium-5 attestation)
sigma-spin verify gaming

# Switch between installed spins
sigma-spin switch dev
```

---

## 🔧 Building a Custom Spin

Sovereign developers can create their own spins using the **Spin Manifest** format:

```toml
# my-spin.toml
[spin]
name = "SigmaOS Industrial"
base = "sigma-core"
version = "1.0.0"

[packages]
include = ["gcc", "openplc", "modbus-utils", "mosquitto"]
exclude = ["steam", "blender"]

[profile]
sandbox_ring = "RING_2_SERVICE"
network_policy = "restricted"
```

```bash
sigma-spin build my-spin.toml
```

---

## 📚 See Also

- [All-In-One Roadmap](All-In-One-Roadmap.md)
- [OmniPackage Manager](OmniPackage-Manager.md)
- [Flagship Use Cases](Flagship-Use-Cases.md)
- [Sovereign Sandbox](Sovereign-Sandbox.md)
