# 📟 SigmaOS: Sovereign Orchestrator v3.0 (Modular)

The **Sovereign Orchestrator** has been fully modularized to ensure maximum maintainability and extensibility. It now features advanced automation and personalization engines.

## 🚀 Automation & Onboarding

### Autonomous Setup
```bash
./s-cli auto
```
The **Singularity Engine** automatically detects your hardware and environment (Bare-Metal/VM/Browser) and deploys the optimal shard profile for a "Ready-to-Use" experience.

### Interactive Onboarding
```bash
./s-cli setup
```
Starts the **Zenith Wizard**, guiding you through the initial lattice initialization, identity generation, and shard selection.

## 🎨 Customization & Personalization

### System Configuration
```bash
./s-cli config
```
Interface for hot-swapping kernel policies, UI themes, and system personalities. 

## 🏗️ Modular Architecture
The orchestrator source code is now partitioned into the `orchestrator/` directory:
- `main.cpp`: Core command routing and banner management.
- `auto_profile.cpp`: Autonomous deployment logic.
- `security_audit.cpp`: Hardened lattice verification.

---
*Automation. Customization. Sovereignty.*
