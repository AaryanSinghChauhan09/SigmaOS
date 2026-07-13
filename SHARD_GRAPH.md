# SHARD GRAPH

> **Component**: Lattice Architecture | **Status**: Living Document

This document defines the Shard Dependency Graph — the complete picture of how all SigmaOS shards relate, depend on, and communicate with each other via sigma-bus.

---

## Full Dependency Graph

```mermaid
graph TD
    subgraph "Stage 1: Genesis"
        S01["S01_Genesis<br/>sigma_types, sigma_libc"]
    end

    subgraph "Stage 2: Core"
        S03["S03_Orchestrator<br/>Boot sequencing"]
        S05["S05_Memory<br/>Allocator, VMM"]
        S06["S06_Scheduler<br/>EEVDF, RT"]
    end

    subgraph "Stage 3: Security"
        S08["S08_Security<br/>MAC, Crypto, PQC"]
        S09["S09_Audit<br/>IMA, Forensic Log"]
    end

    subgraph "Stage 4: Registry"
        S10["S10_Registry<br/>Shard Catalog"]
    end

    subgraph "Stage 5: Infrastructure"
        S11["S11_IPC<br/>sigma-bus"]
        S12["S12_VFS<br/>Filesystem Layer"]
        S13["S13_Drivers<br/>HAL, Device Mgr"]
        S14["S14_Network<br/>TCP/IP Stack"]
    end

    subgraph "Stage 6: Services"
        S15["S15_Telemetry<br/>Profiler"]
        S16["S16_SelfHealing<br/>Watchdog, Recovery"]
        S17["S17_AI<br/>Neural Core"]
        S18["S18_Package<br/>sigpkg"]
    end

    subgraph "Stage 7: User-Facing"
        S20["S20_Desktop<br/>Zenith"]
        S21["S21_Shell<br/>sigma-shell"]
        S22["S22_Apps<br/>User Applications"]
    end

    subgraph "Stage 8: Finality"
        S33["S33_Finality<br/>System Ready"]
    end

    %% Dependencies
    S01 --> S03
    S01 --> S05
    S01 --> S06
    S01 --> S08

    S03 --> S10
    S05 --> S10
    S06 --> S10
    S08 --> S10
    S09 --> S10

    S10 --> S11
    S10 --> S12
    S10 --> S13
    S10 --> S14

    S11 --> S15
    S11 --> S16
    S11 --> S17
    S12 --> S18

    S14 --> S20
    S15 --> S20
    S18 --> S20
    S11 --> S21

    S20 --> S22
    S21 --> S22

    S22 --> S33
    S16 --> S33
    S15 --> S33
```

---

## IPC Channel Map

Shards communicate exclusively via named sigma-bus channels:

| Channel Name | Publisher | Subscribers | Message Type |
|---|---|---|---|
| `sigma.boot.stage` | S03_Orchestrator | All | Boot stage transition |
| `sigma.memory.pressure` | S05_Memory | S06, S16, S17 | OOM/pressure notification |
| `sigma.sched.tune` | S17_AI | S06_Scheduler | EEVDF slice recommendation |
| `sigma.security.alert` | S08_Security | S09, S16, S20 | Security event |
| `sigma.metrics.vitals` | S15_Telemetry | S20, S17, S16 | 10Hz metrics snapshot |
| `sigma.net.packet` | S14_Network | S08 (firewall) | Packet filter pipeline |
| `sigma.pkg.install` | S18_Package | S12_VFS | Package file write |
| `sigma.desktop.notify` | S20_Desktop | (display) | User notification |
| `sigma.shell.cmd` | S21_Shell | S17_AI | NL-CLI → command |
| `sigma.heal.crash` | S16_SelfHealing | S10_Registry | Shard crash report |
| `sigma.focus.state` | S20_Desktop | S06, S14 | Focus mode toggle |

---

## Boot Order (Topological Sort)

```
Boot Order    Shard                 Dependencies
─────────────────────────────────────────────────
  1           S01_Genesis           (none)
  2           S05_Memory            S01
  3           S06_Scheduler         S01
  4           S03_Orchestrator      S01
  5           S08_Security          S01
  6           S09_Audit             S08
  7           S10_Registry          S03, S05, S06, S08
  8           S11_IPC               S10
  9           S13_Drivers           S10
 10           S12_VFS               S10
 11           S14_Network           S10
 12           S15_Telemetry         S11
 13           S16_SelfHealing       S11
 14           S17_AI                S11
 15           S18_Package           S12
 16           S20_Desktop           S14, S15, S18
 17           S21_Shell             S11
 18           S22_Apps              S20, S21
 99           S33_Finality          All
```

---

## Shard Size Footprint

| Shard | Binary Size | RAM (Idle) | RAM (Active) |
|---|---|---|---|
| S01_Genesis | 8 KB | 32 KB | 32 KB |
| S05_Memory | 64 KB | 256 KB | 2 MB |
| S06_Scheduler | 48 KB | 128 KB | 512 KB |
| S08_Security | 96 KB | 512 KB | 2 MB |
| S10_Registry | 32 KB | 64 KB | 128 KB |
| S11_IPC | 24 KB | 128 KB | 4 MB |
| S14_Network | 128 KB | 1 MB | 8 MB |
| S15_Telemetry | 16 KB | 256 KB | 4 MB |
| S20_Desktop | 2 MB | 32 MB | 128 MB |
| **Total (minimal)** | **~420 KB** | **~2.4 MB** | **~16 MB** |
| **Total (full)** | **~2.5 MB** | **~35 MB** | **~149 MB** |

---

## Graph Validation Rules

1. **No cycles**: Dependency graph must be a DAG (Directed Acyclic Graph)
2. **No orphans**: Every shard (except S01) must have at least one dependency
3. **Single root**: Only S01_Genesis has zero dependencies
4. **Single sink**: Only S33_Finality has zero dependents
5. **Category ordering**: Core → Security → Registry → Infrastructure → Services → UI → Finality

**Validation command**: `sigma registry validate-graph`
