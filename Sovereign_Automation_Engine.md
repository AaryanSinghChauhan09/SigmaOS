# Σ Sovereign Automation Engine (SigmaScript)

The **Sovereign Automation Engine** is a native Zenith-grade kernel module designed for the high-speed execution of automation scripts within the SigmaOS ecosystem. By providing a Ring 0 scripting environment, SigmaOS eliminates the overhead of external interpreters and provides direct silicon-level mission orchestration.

## SigmaScript Architecture

SigmaScript is a token-based interpreted language designed for zero-dependency automation.

### 1. Native CLI Integration
The engine dispatches commands directly to the **Sovereign Unified CLI**, allowing scripts to leverage any registered kernel shard (e.g., AI, Personalizer, Distro Slinger).

### 2. Line-by-Line Execution
Scripts are processed line-by-line with support for comments (`#`). This ensures deterministic execution and minimal memory footprint.

## CLI Command: `sigma-run`

The Automation Engine is managed via the unified `sigma-run` command:

```bash
# Execute the standard system stabilization script
sigma-run standard_boot

# View automation audit stats
sigma-run
```

### Example: `standard_boot.sigma`
```bash
# Σ SIGMAOS: Standard Mission Stabilization
sigma-uname -a
sigma-personalize theme ZENITH_DARK
sigma-ai audit
sigma-ls /
sigma-echo [AUTO]: System Stabilized.
```

## Architectural Specifications

| Feature | Specification | Standard |
| :--- | :--- | :--- |
| Interpreter | C11 Token-Based | Zenith |
| Dispatch Backend | Sovereign Unified CLI | Industrial |
| Audit Capabilities | Real-time Heartbeat | Sovereign |
| Dependency | Zero | Absolute |

---
**Σ SIGMAOS: AUTOMATION IS SOVEREIGN.**
