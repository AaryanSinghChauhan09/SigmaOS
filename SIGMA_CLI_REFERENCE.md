# SigmaOS CLI (SigmaShell) Reference

To match the raw power of Bash/Zsh and the structured data capabilities of PowerShell, SigmaOS natively integrates the **SigmaShell**. It overrides the classic GNU Coreutils with hardware-accelerated, Zero-Trust C11 binaries.

## Native Built-in Commands

| Command | Competitor Equivalent | Description |
| :--- | :--- | :--- |
| `shardctl` | `systemctl` / `services.msc` | Controls Sovereign Shards. E.g., `shardctl start S07_Network`, `shardctl status S02_ZenithUI`. |
| `sigmatop` | `htop` / `Task Manager` | Live telemetry of kernel/userland orchestrator scheduling, mem usage, and NPU inference load. |
| `netmesh` | `ip` / `ifconfig` / `nmap` | Advanced networking tracker. Maps the local TCP/IP stack AND remote nodes inside the distributed OS topology. |
| `siglist` | `ls -la` / `tree` / `Get-ChildItem` | Lists file metadata but supports returning structured JSON objects natively for pipeline chaining. |
| `audittrail` | `journalctl` / `Event Viewer` | Dumps the cryptographically signed logs from the `S08_Security` Zero-Trust audit layer to trace any privilege escalation attempts. |
| `handoff` | *AirDrop API / KDE Connect* | Instantly streams clipboard buffers, files, or active application states to another SigmaOS node on the network. |
| `sigmacrypt` | `openssl` / `ssh-keygen` | Interfaces directly with the biometric enclave to generate mathematically verifiable server keys or encrypt local archives. |
| `sigpkg` | `apt` / `pacman` / `brew` | The sovereign-signed package manager for resolving zero-dependency applications and libraries. |
| `vbox` | `docker` / `virsh` | Natively controls the `ContainerRuntime` APIs to rapidly spin up isolated application sandboxes or whole VMs. |

## Pipeline Architecture
Unlike traditional UNIX text streams, SigmaShell pipes (`|`) can pass raw JSON-like C structs between programs, meaning `siglist | sigmacrypt --encrypt` passes binary file pointers directly in kernel memory without parsing strings, fundamentally outpacing Linux pipeline speeds.
