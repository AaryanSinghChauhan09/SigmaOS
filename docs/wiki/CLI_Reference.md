# SigmaOS Command Line Interface (CLI) Reference

The SigmaOS CLI (`s-cli`) is built on a scalable namespace architecture. Instead of flat lists of commands, every operation is categorized into logical subsystems. This ensures the environment remains modular, intuitive, and completely free of bloat.

---

## 🧠 `s-assist` (Intelligence & AI)

Interact directly with the Sigma Assistant for system insights and dynamic optimization.

* `s-assist status` - Renders the real-time health dashboard (CPU, Memory, Network, Battery).

* `s-assist suggest` - Triggers AI-driven recommendations based on current telemetry.

* `s-assist optimize <task>` - Auto-tunes the system for specific workloads (e.g., `gaming`, `video editing`).

* `s-assist explain` - Provides transparency logs explaining why the AI made a specific system adjustment.

## 🔧 `s-func`, `s-lib`, `s-comp`, `s-driver` (Modularisation)

Manage predefined functions, libraries, components, and hardware drivers.

* `s-func list` - Show all predefined core micro-functions.

* `s-func swap <name>` - Replace a core function with an optimized version.

* `s-lib audit` - Scan libraries for bloat or vulnerabilities.

* `s-lib replace <lib>` - Swap a heavy framework with a lightweight equivalent.

* `s-lib sandbox <lib>` - Force a library to run in isolation.

* `s-comp prune` - Remove unused third-party components.

* `s-driver load <device>` - Load a driver on-demand.

* `s-driver audit` - Check driver dependencies.

## ⚙️ `s-auto` (Automations)

* `s-auto backup nightly` - Schedule automated nightly snapshots.

* `s-auto prune weekly` - Auto-remove unused dependencies.

* `s-auto profile switch work 9am` - Auto-switch profiles based on time/location.

* `s-auto rollback detect` - Auto-rollback on crash detection.

* `s-auto monitor <component>` - Watchdog for third-party modules.

## 📦 `s-deps` (Dependency Management)

Ensure the OS remains free of dependency bloat.

* `s-deps prune` - Remove unused dependencies.

* `s-deps tree` - Visualize the dependency impact of installed software.

* `s-deps explain <package>` - Explain why a specific dependency is required.

* `s-deps reduce <lib>` - Replace a heavy library with a modular equivalent.

## ⚡ `s-perf` (Performance Tuning)

* `s-perf boost` - Temporarily maximize performance for heavy workloads.

* `s-perf cache adaptive` - Enable predictive app pre-loading.

* `s-perf isolate <process>` - Isolate a resource-heavy process in a micro-VM.

* `s-perf profile gaming` - Fast boot with minimal background services.

## 🌐 `s-net` & `s-sec` (Networking & Security)

* `s-net secure` - Enforce internal Zero-Trust subsystem authentication.

* `s-sec audit` - Run a comprehensive security audit.

* `s-sec encrypt <file>` - Encrypt a file with Quantum-Safe keys.

* `s-sec firewall enable` - Enable strict firewall rules.

## 🎨 `s-theme`, `s-dash`, `s-profile`, `s-ui` (Personalization)

* `s-theme set dark` - Switch aesthetic theme.

* `s-theme dynamic` - Enable time-based environmental themes.

* `s-dash create gaming` - Create a specific dashboard profile.

* `s-profile switch work` - Instantly swap to the Work profile.

* `s-ui adaptive` - Enable adaptive UI formatting based on device type (desktop, tablet, VR).

## 🚀 `s-install` & `s-access` (Ease of Use)

* `s-help <command>` - Contextual help.

* `s-onboard wizard` - Guided setup for new users.

* `s-install simple <app>` - One-click app installation.

* `s-install audit` - Check installation footprint/impact before executing.

* `s-access voice enable` - Enable voice commands.

* `s-access shortcut list` - List accessibility shortcuts.

## 🐞 `s-dev` (Developer Tools)

* `s-dev test <module>` - Run unit tests on a specific module.

* `s-dev lint` - Check coding style consistency.

* `s-dev ci` - Trigger the CI/CD pipeline locally.

## 👥 `s-community` (Community & Governance)

* `s-community share <theme>` - Share a theme or profile with the Sovereign Lattice.

* `s-community fetch` - Download community-curated templates.

* `s-community rate <profile>` - Rate shared profiles.

---
*By utilizing this modular namespace approach, SigmaOS guarantees a 1:1 parity between the Zenith GUI and the Terminal, scaling infinitely without overwhelming the user.*

