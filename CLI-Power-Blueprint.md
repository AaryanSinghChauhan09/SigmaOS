# 🐚 SigmaOS CLI Power, Tooling, & Parity Blueprint

This document details the master plan, workflows, and implementations to achieve complete operational parity between the **Zenith Desktop (GUI)** and the **Sigma-Shell (CLI)** in SigmaOS.

By ensuring that 100% of desktop, administrative, and configuration tasks can be driven headless via highly optimized commands, SigmaOS becomes the ultimate powerhouse for developers, server administrators, and automated build pipelines.

***

## 🗺️ Parity Matrix: GUI vs. CLI

| Domain | Zenith Desktop (GUI) Action | Sovereign CLI Equivalents | Status |
| :--- | :--- | :--- | :--- |
| **Package Management** | Software Center / Store installer | `sigpkg install <pkg>`, `sigpkg remove <pkg>` | **Implemented** |
| **System Tracing** | Visual Activity & Resource Monitor | `sigtrace start`, `sigtrace stop` | **Implemented** |
| **Telemetry Exporter** | Grafana / Dashboard telemetry widgets | `sigmetrics export`, `sigmetrics show` | **Implemented** |
| **Standards Compliance** | Settings -> Standards tab | `sigstandards verify <path>`, `sigstandards posix` | **Implemented** |
| **Real-Time Scheduler** | Control Panel -> Performance modes toggles | `sigsched rt`, `sigsched hpc`, `sigsched show` | **Implemented** |

***

## 🛠️ 1. Native CLI Commands & Syntax Specification

To support these workflows directly, the microkernel shell parses and validates the following structured system commands:

### 1.1 Package Management: `sigpkg`

*   **Install Package**: `sigpkg install <package_name>`
    *   Triggers the Universal Package Manager to resolve dependencies and verify Dilithium-5 signatures.
*   **Uninstall Package**: `sigpkg remove <package_name>`
    *   Safe removal and dependency pruning.

### 1.2 System Observability: `sigtrace`

*   **Trace Event**: `sigtrace trace <event> <payload>`
    *   Registers dynamic eBPF-like tracing hooks inside kernel context spans.

### 1.3 Telemetry Exporter: `sigmetrics`

*   **Prometheus Export**: `sigmetrics export`
    *   Formats current memory allocations, CPU loads, and queue states to raw text endpoints.

### 1.4 Standard Verification: `sigstandards`

*   **FHS Compliance**: `sigstandards verify <path_name>`
    *   Evaluates standard path routes against the FHS specification.

### 1.5 Performance Scheduling Profile: `sigsched`

*   **Toggles Profiles**: `sigsched rt` or `sigsched hpc`
    *   Tunes preemption limits and context-switch bypass channels.

***

## 📅 2. Integration & Synchronization

These core commands are built directly into `src/shell/command.rs` using standard traits. They execute with zero temporary memory allocations and compile fully under `#![no_std]` targets.
