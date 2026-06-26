# Flagship Niche Profiles

To compete with highly specialized Linux distributions (like SteamOS, Fedora CoreOS, or Whonix), SigmaOS introduces **Flagship Profiles**. These are compile-time configurations that radically alter the behavior of the kernel for specific use cases.

## Sovereign Cloud Profile (`sigma_cloud_profile.h`)
Targeted at HPC and cloud-native workloads, replacing traditional Docker/K8s nodes.
- **GUI Disabled**: Strips out `sigma-wm` and Zenith Desktop for minimal memory footprint.
- **Throughput Scheduling**: Tunes the hybrid scheduler to favor long execution timeslices over UI latency.
- **Maximized IO buffers**: Massively expands network socket tracking and RX/TX buffers.

## Secure Government Profile (`sigma_gov_profile.h`)
Targeted at high-security environments, intelligence agencies, and defense.
- **Strict Execution**: Enforces Dilithium-5 PQC signatures on all userland executables.
- **Anti-BadUSB**: Completely disables USB hotplugging.
- **Auto-Forensics**: Triggers Forensic Audit Mode automatically on any kernel panic.
