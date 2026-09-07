# AI Agent Guidelines: Cloud Computing Operations Management in SigmaOS

## Overview
This document defines operational guidelines and architectural directives for AI agents working on **Cloud Computing Operations Management** in SigmaOS. It specifies `cloud-init` first-boot provisioning, OpenStack Cinder block volume storage, Rancher k3s/Harvester hyper-converged micro-VM governors, SigmaKube container orchestration, peer-to-peer cloud state synchronization, and `SystemTarget::Cloud` server initialization profiles across `#![no_std]` runtime environments in SigmaOS.

---

## 1. Cloud Management Subsystems & Modules

AI agents interacting with cloud-native deployments, container orchestration, or cloud-config provisioning in SigmaOS must interface with the following core subsystems:

| Subsystem / Module | Location | Description |
| :--- | :--- | :--- |
| **Cloud-Init Provisioning Engine** | `src/distro/linux_bsd_parity_extended.rs` | First-boot cloud instance bootstrap engine (`CloudInitBootstrapEngine`) executing SSH key staging, package installs, and runcmd commands. |
| **ArchBox Cloud-Config Generator** | `src/distro/arch_boxes.rs` | OpenStack/AWS/Proxmox cloud image format (`ArchBoxFormat::CloudInit`) generating `#cloud-config` YAML manifests. |
| **OpenStack Cinder Volume Storage** | `src/storage/cinder.rs` | Cloud block volume manager (`CinderVolumeManager`) providing thin/thick provisioning, volume encryption, and snapshotting. |
| **Rancher k3s & Harvester Governor** | `src/virtualization/rancher.rs` | Hyper-converged micro-VM governor (`RancherHarvesterVirtualMachineGovernor`) and embedded k3s cluster controller (`RancherK3sEmbeddedClusterController`). |
| **SigmaKube Container Orchestrator** | `src/orchestration/sigmakube.rs` | Native container orchestrator (`SigmaKubeOrchestrator`) handling service mesh routing, CNI networking, and pod scheduling. |
| **P2P Cloud Synchronization** | `src/cloud/sync.rs`, `src/network/sync.rs` | Peer-to-peer cloud sync engine (`SimpleCloudSync`, `CloudSyncEngine`) for state replication and Nextcloud/S3 integration. |
| **Headless Cloud Init Target** | `src/init/sigmainit.rs` | Headless cloud server target profile (`SystemTarget::Cloud`) executing `cloud.target` and `cloud-init` service activation. |

---

## 2. Architectural Rules & Cloud Invariants

AI agents must enforce the following 4 core invariants when implementing or auditing cloud computing operations:

```
+-------------------------------------------------------------------------+
|                SIGMAOS CLOUD COMPUTING OPERATIONS ARCHITECTURE          |
+-------------------------------------------------------------------------+
                                     |
         +---------------------------+---------------------------+
         |                           |                           |
         v                           v                           v
  [cloud-init Provisioning]  [SigmaKube Container Grid]   [Cinder Block Storage]
  • User-Data YAML Manifest   • CNI Fast-Packet Routing    • Thin/Thick Allocation
  • SSH Key Staging           • Pod Service Mesh           • PQC Volume Encryption
  • Capability-Gated Runcmd   • k3s/Harvester Micro-VMs    • CoW Merkle Snapshots
```

### 1. Headless Cloud Target Optimization (`SystemTarget::Cloud`)
- **Invariant:** When booting under `SystemTarget::Cloud` (`cloud.target`), the OS MUST bypass GUI/framebuffer compositor loops and prioritize zero-copy E1000/xHCI network queues.
- **Rule:** Background daemons must be minimal; memory footprint for headless cloud instances must remain under 16MB.

### 2. Zero-Trust Cloud-Init Execution
- **Invariant:** User-data `#cloud-config` manifests and `runcmd` scripts executed by `CloudInitBootstrapEngine` MUST run inside capability-sandboxed Ring 3 environments governed by `PledgeManager`.
- **Rule:** Cloud provisioning scripts cannot perform arbitrary un-pledged register or raw disk writes.

### 3. Encrypted Cloud Volume Storage
- **Invariant:** OpenStack Cinder volumes managed by `CinderVolumeManager` in `src/storage/cinder.rs` MUST enforce AES-256 or Kyber-1024 volume encryption.
- **Rule:** Volume snapshots must generate immutable Merkle-tree state hashes allowing sub-millisecond rollback on failure.

### 4. Zero Ring 0 Panic Rule
- Cloud orchestration, volume attachment, and P2P sync operations must return explicit `Result<T, &'static str>` status values instead of triggering unhandled kernel panics.

---

## 3. Verification & Testing Protocols

Every cloud computing operations change must be verified via standalone unit tests and integrated test execution:

```bash
# Run standalone unit test for cloud-init provisioning
rustc --test --edition 2021 src/distro/linux_bsd_parity_extended.rs -o build/test_parity_extended && ./build/test_parity_extended

# Run full test suite
./run_sigma_tests.sh
```

---

## 4. AI Agent Self-Assessment Checklist

Before finalizing changes touching cloud provisioning, virtualization, or container orchestration:

- [ ] Does `SystemTarget::Cloud` correctly activate `cloud.target` and `cloud-init` services?
- [ ] Are `cloud-init` user-data commands executed inside capability-gated sandboxes?
- [ ] Are Cinder block volumes encrypted and backed by CoW Merkle tree snapshots?
- [ ] Have all unit tests passed with 0 failures in `./run_sigma_tests.sh`?
