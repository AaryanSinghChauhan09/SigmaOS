# Σ SIGMAOS: SHARD AUTONOMY & SILICON PURGE

[![Autonomy](https://img.shields.io/badge/Autonomy-TOTAL-blue?style=for-the-badge)]()

**Σ SIGMAOS** grants you **Complete Control** over every professional tool in the OS.

## 🧩 SHARD-ON-DEMAND (SOD)

- Every system and domain component (AI, DS, DSA) is a **Sovereign Shard**.
- Use the **Shard Store** UI to toggle tools near the silicon.
- **OOPS Hierarchy**: Every shard inherits from the `SigmaShard` base class for encapsulated logic.

## ⚙️ THE SILICON PURGE

- To eliminate all unnecessary code from your active environment, use the **PURGE TRIGGER**.
- This physically removes the tool definitions from the system memory (`SigmaStore.js`).
- This is not a "Disable" function; it is a **Sovereign Removal** for pure performance and security.

---

## How Shards Work

```text
Boot Sequence
    └─► Kernel Init (kernel/main.c)
            └─► shard_core.c loaded
                    └─► SOD Registry populated
                            └─► UI requests Shard activation
                                    └─► Shard `.c` compiled + injected
                                            └─► Shard runs in isolated domain
```

---

## Shard Lifecycle

| State | Description |
| ------- | ----------- |
| `DORMANT` | Registered but not loaded into memory |
| `LOADING` | Being compiled/injected by shard_core |
| `ACTIVE` | Running in its isolated memory domain |
| `PAUSED` | Suspended, state preserved in VFS |
| `EVICTED` | Forcibly removed by OOM killer or user |

---

## Shard Manifest (Full List)

| Shard ID | File | Domain | Description |
| ---------- | ------ | -------- | ----------- |
| `ai_lab` | `SovereignAIKernelZenith.c` | AI | LLM orchestration and tensor kernel |
| `forensics` | `SovereignForensicMatrix.c` | Security | Memory imaging  + PCAP + audit |
| `pqc` | `SovereignLatticePQC.c` | Security | Post-quantum LWE cryptography |
| `hypervisor` | `SovereignHypervisorZenith.c` | Virtualization | Qubes-style VM domain isolation |
| `ml_core` | `SovereignML.c` | ML | Native tensor ops, gradient descent |
| `voice` | `SovereignVoiceShard.c` | I/O | Voice command pipeline |
| `search` | `SovereignSearch.c` | System | AI-powered VFS file search |
| `distro_forge` | `SovereignDistroForge.h` | Virtualization | Linux distro runner (v86) |
| `personalizer` | `SovereignPersonalizerZenith.c` | UI | Persona-aware UI morphing |
| `calculator` | `SovereignSuperCalculator.c` | Math | High-precision math engine |
| `netmesh` | `SovereignNetMesh.c` | Network | Mesh networking + deep routing |
| `amnesic` | `SovereignAmnesicShard.c` | Security | Zero-trust forensic wipe |
| `omni_shard` | `SovereignOmniShard.c` | System | Master shard loader/coordinator |
| `bpf` | `sovereign_bpf.c` | System | Ring-0 eBPF sandbox |
| `cgroup` | `cgroup_shard.c` | Resource | CPU/Memory isolation domains |
| `oom_killer` | `oom_killer.c` | Resource | Heuristic sacrifice engine |
| `camera_shard`| `camera_shard.c` | Media | Camera/screen capture pipeline |
| `ncert` | `ncert_shard.c` | Education | NCERT curriculum labs |
| `bnss` | `bnss_shard.c` | Legal | BNSS Section 105 forensic compliance |
| `checklist` | `checklist_shard.c` | Productivity | Dynamic checklist manager |
| `automation` | `automation_shard.c` | Automation | Task scheduler + macro engine |

---

## Activating a Shard via UI

```js
window.SIGMA.activateShard('forensics');
window.SIGMA.activateShard('ml_core');
window.SIGMA.activateShard('pqc');
```

---

## Activating via Omni Shell

```bash
sigma shard load forensics
sigma shard load ai_lab --priority 95
sigma shard status
sigma shard evict camera_shard
```

---

## Persona-Aware Shard Profiles

| Persona | Auto-loaded Shards |
| --------- | -------------------- |
| **Developer** | `ai_lab`, `cs_research`, `automation`, `checklist` |
| **Forensic Analyst** | `forensics`, `amnesic`, `bnss`, `pqc`, `netmesh` |
| **Student** | `ncert`, `calculator`, `dsa_shard`, `ml_core` |
| **Researcher** | `ml_core`, `ai_lab`, `data_science`, `sovereign_search` |
| **Gamer** | `gaming`, `xclicker`, `oom_killer`, `automation` |
**Σ SIGMAOS: YOUR HARDWARE. YOUR CODE. YOUR CHOICE. 🧩⚙️🛡️**
