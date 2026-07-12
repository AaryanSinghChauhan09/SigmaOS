# SigmaOS Competitor Comparison

| Distro | Strengths | Where SigmaOS is weak | SigmaOS strategy to surpass |
|--------|-----------|------------------------|-----------------------------|
| Raspberry Pi OS | Huge hardware ecosystem, optimized drivers, easy setup | Limited driver matrix (PS/2, VGA, e1000) | Expand HAL + sovereign drivers; ARM profile |
| SteamOS | Gaming integration, Proton, polished UX | No mature GPU/gaming layer | Sovereign graphics + Zenith WM + native SDK |
| Clear Linux | Intel-tuned performance | Basic scheduler tuning | Silicon-aware scheduler + PGO builds |
| NixOS | Declarative builds, reproducibility | Registry incomplete | Sovereign `.spkg` registry + signed recipes |
| SlackBuilds | Community build scripts | No contribution pipeline | Sovereign build registry workflow |
| Rescuezilla / SystemRescue | Mature recovery GUI/tools | Recovery mostly fallback/shell | Rollback + resilient mode + automation |
| Fedora CoreOS / Flatcar | Immutable base, auto-updates | Immutable loop incomplete | A/B updates + rollback + safe-mode boot |
| RancherOS | Container-first, Docker-native | Namespace/cgroup partial | `sigma-pod run-native` + orchestrator |
| Solus | Cohesive desktop UX | Zenith maturing | Theme + tiling + `~/.sigma_profile` |
| Ubuntu / Canonical | Enterprise support, cloud | Enterprise gaps | Profiles + automation + orchestration |
| CAINE | Forensics specialization | No forensic profile | Secure/forensic profile |
| EndeavourOS | Rolling updates, installer | Installer/rolling early | Profile-based releases |
| Linux From Scratch | DIY sovereignty + education | Docs depth | Wiki playbooks + Phase checklists |

## Key weaknesses

1. Hardware support breadth (GPU/Wi-Fi/Bluetooth/ARM matrix)

2. Package ecosystem reproducibility

3. Recovery UX beyond resilient shell

4. Scheduler/compiler performance tuning

5. Zenith desktop polish

6. Immutable update verification on all release branches

7. Wiki/docs synchronization discipline

See [SigmaOS-Differentiation-Blueprint](SigmaOS-Differentiation-Blueprint) and [Stability-Playbook](Stability-Playbook).
