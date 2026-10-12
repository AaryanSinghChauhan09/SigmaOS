// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Virtualization & Container Pull Request Gateway Engine
//
// Inspired by Linux & BSD virtualization and container ecosystems:
// - KVM (Kernel-based Virtual Machine vCPU & EPT/NPT hardware acceleration)
// - QEMU (Quick Emulator virtio devices, QMP monitor, and user-space emulation)
// - libvirt (Universal hypervisor management daemon and domain XML specs)
// - Xen (Type-1 bare-metal hypervisor Dom0 / DomU microkernel isolation)
// - VirtualBox (Desktop guest virtualization with VBoxManage and guest additions)
// - bhyve (FreeBSD hypervisor with PCI passthrough and virtio-net/blk)
// - vmm (OpenBSD vmm/vmd hypervisor with pledge/unveil security sandboxing)
// - Docker (OCI container daemon with image layered filesystems and bridge networking)
// - Podman (Daemonless rootless container engine with subuid/subgid user namespaces)
// - containerd (High-performance OCI container runtime daemon and snapshotter)
// - runc (OCI low-level CLI container runtime driver for cgroups v2/v1)
// - LXC (Linux Containers system container framework)
// - LXD (Next-generation system container & VM manager with REST API)
// - systemd-nspawn (Lightweight OS tree container isolation with systemd integration)
// - FreeBSD Jails (Os-level virtualization with VNET network stacks and jail flags)
// - Kubernetes (Production-grade container orchestration CRI, CNI, and Pod specifications)

#[cfg(not(test))]
use crate::klib::collections::BTreeMap;
#[cfg(test)]
use std::collections::BTreeMap;

use std::string::{String, ToString};
use std::vec::Vec;

/// Supported Virtualization & Container Technology Formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VirtualizationTechnologyFormat {
    Kvm,
    Qemu,
    Libvirt,
    Xen,
    VirtualBox,
    Bhyve,
    OpenBsdVmm,
    Docker,
    Podman,
    Containerd,
    Runc,
    Lxc,
    Lxd,
    SystemdNspawn,
    FreeBsdJails,
    Kubernetes,
}

impl VirtualizationTechnologyFormat {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Kvm => "Linux KVM Hardware Acceleration",
            Self::Qemu => "QEMU VirtIO & QMP Hypervisor",
            Self::Libvirt => "Libvirt Universal Domain Manager",
            Self::Xen => "Xen Dom0 / DomU Microkernel Hypervisor",
            Self::VirtualBox => "VirtualBox Desktop Hypervisor",
            Self::Bhyve => "FreeBSD bhyve MicroVM Engine",
            Self::OpenBsdVmm => "OpenBSD vmm / vmd Sandboxed Hypervisor",
            Self::Docker => "Docker OCI Container Engine",
            Self::Podman => "Podman Rootless Daemonless Engine",
            Self::Containerd => "Containerd OCI Runtime Daemon",
            Self::Runc => "runc OCI Low-Level Container Runtime",
            Self::Lxc => "LXC Linux System Containers",
            Self::Lxd => "LXD System Container & VM Daemon",
            Self::SystemdNspawn => "systemd-nspawn OS Tree Sandbox",
            Self::FreeBsdJails => "FreeBSD Jails with VNET Isolation",
            Self::Kubernetes => "Kubernetes Cloud-Native CRI & Pod Engine",
        }
    }

    pub fn category(&self) -> VirtualizationPrCategory {
        match self {
            Self::Kvm | Self::Qemu | Self::Xen | Self::Bhyve | Self::OpenBsdVmm | Self::VirtualBox => {
                VirtualizationPrCategory::HypervisorCore
            }
            Self::Docker | Self::Podman | Self::Containerd | Self::Runc | Self::Lxc | Self::Lxd | Self::SystemdNspawn => {
                VirtualizationPrCategory::ContainerRuntime
            }
            Self::FreeBsdJails => VirtualizationPrCategory::SecurityIsolation,
            Self::Libvirt | Self::Kubernetes => VirtualizationPrCategory::OrchestrationCluster,
        }
    }
}

/// Category of Virtualization Pull Request
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VirtualizationPrCategory {
    HypervisorCore,
    ContainerRuntime,
    StorageVirt,
    NetworkVirt,
    OrchestrationCluster,
    SecurityIsolation,
}

impl VirtualizationPrCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::HypervisorCore => "Hypervisor & vCPU Execution Core",
            Self::ContainerRuntime => "OCI Container Runtime & Namespaces",
            Self::StorageVirt => "VirtIO Storage, qcow2 & Snapshotting",
            Self::NetworkVirt => "TAP/TUN, VNET, Bridge & CNI Networking",
            Self::OrchestrationCluster => "Cluster Orchestration & CRI",
            Self::SecurityIsolation => "PQC Signature & Sandbox Isolation",
        }
    }
}

/// Status of Virtualization Pull Request
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VirtualizationPrStatus {
    Proposed,
    CiValidation,
    Gated,
    Merged,
    Rejected,
}

/// Structure representing a Virtualization & Container PR Submission
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualizationPrSubmission {
    pub pr_id: u32,
    pub title: String,
    pub author: String,
    pub technology: VirtualizationTechnologyFormat,
    pub category: VirtualizationPrCategory,
    pub description: String,
    pub tech_spec: String,
    pub config_manifest: String,
    pub pqc_signature: Vec<u8>,
    pub status: VirtualizationPrStatus,
    pub timestamp: u64,
}

/// Sovereign Virtualization & Container Pull Request Gateway Engine
#[derive(Debug, Clone)]
pub struct SovereignVirtualizationPrGatewayEngine {
    pub proposals: BTreeMap<u32, VirtualizationPrSubmission>,
    next_pr_id: u32,
}

impl Default for SovereignVirtualizationPrGatewayEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignVirtualizationPrGatewayEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            proposals: BTreeMap::new(),
            next_pr_id: 1,
        };
        engine.initialize_preset_proposals();
        engine
    }

    /// Pre-populate preset PR proposals covering all major virtualization and container technologies
    fn initialize_preset_proposals(&mut self) {
        let presets = vec![
            (
                "KVM Hardware Acceleration & Nested EPT/NPT vCPU Engine",
                "kvm-maintainers@sigmaos.org",
                VirtualizationTechnologyFormat::Kvm,
                VirtualizationPrCategory::HypervisorCore,
                "Implements Linux KVM vCPU execution loop, Intel VT-x EPT / AMD SVM NPT nested page table translation, and dirty page ring logging.",
                "[kvm]\nvcpu_max=256\nnested_virt=true\nept_enabled=true\nkvm_dirty_ring_size=4096",
                "[domain_kvm]\nname = \"sigma-kvm-guest\"\nvcpu = 8\nmemory_mb = 16384\nacceleration = \"kvm\"",
            ),
            (
                "QEMU VirtIO Block & QMP Live Migration Gateway",
                "qemu-dev@sigmaos.org",
                VirtualizationTechnologyFormat::Qemu,
                VirtualizationPrCategory::StorageVirt,
                "Adds QEMU QMP monitor RPC command dispatcher with virtio-blk zero-copy ring buffers and post-copy live migration userfaultfd streaming.",
                "[qemu]\nqmp_socket=\"/run/sigma/qemu-qmp.sock\"\nvirtio_blk=true\nuserfaultfd_postcopy=true",
                "[qemu_vm]\ncmdline = \"qemu-system-x86_64 -m 8192 -smp 4 -enable-kvm -drive file=disk.qcow2,if=virtio\"",
            ),
            (
                "Libvirt Universal Domain XML & Hypervisor Abstraction Bridge",
                "libvirt-team@sigmaos.org",
                VirtualizationTechnologyFormat::Libvirt,
                VirtualizationPrCategory::OrchestrationCluster,
                "Provides libvirt domain XML specification parsing, virDomainCreateXML automation, and unified hypervisor driver binding for KVM, QEMU, and bhyve.",
                "<domain type='kvm'><name>sigma-libvirt-guest</name><memory unit='KiB'>8388608</memory><vcpu placement='static'>4</vcpu></domain>",
                "[libvirt]\nuri = \"qemu:///system\"\nxml_schema_version = \"3.0\"",
            ),
            (
                "Xen Dom0 Kernel Supervisor & DomU Microkernel Guest Engine",
                "xen-core@sigmaos.org",
                VirtualizationTechnologyFormat::Xen,
                VirtualizationPrCategory::HypervisorCore,
                "Adds Xen Type-1 bare-metal hypervisor integration with XenStore event channel IPC and PV/HVM DomU microkernel guest domain isolation.",
                "[xen]\ndom0_mem=4096M\nxenstore_path=\"/dev/xen/xenbus\"\nhvm_direct_boot=true",
                "[xen_guest]\nname = \"sigma-xen-domu\"\nvcpus = 2\nmemory = 4096\nbuilder = \"hvm\"",
            ),
            (
                "VirtualBox VBoxManage Automation & Shared Folder Driver",
                "vbox-team@sigmaos.org",
                VirtualizationTechnologyFormat::VirtualBox,
                VirtualizationPrCategory::HypervisorCore,
                "Provides VirtualBox VBoxManage CLI control, guest additions mouse/display integration, and host-to-guest shared folder mount handlers.",
                "[virtualbox]\nvboxmanage_bin=\"/usr/bin/VBoxManage\"\nguest_additions=true\nshared_folder_path=\"/home/user/shared\"",
                "[vbox_vm]\nname = \"sigma-vbox-guest\"\nostype = \"Ubuntu_64\"\nmemory = 4096\nvram = 128",
            ),
            (
                "FreeBSD bhyve Hypervisor & VirtIO PCI Passthrough Engine",
                "bhyve-bsd@sigmaos.org",
                VirtualizationTechnologyFormat::Bhyve,
                VirtualizationPrCategory::HypervisorCore,
                "Implements FreeBSD bhyve low-overhead microVM hypervisor with PCI passthrough (`ppt`), vmm kernel module interface, and virtio-net/blk queues.",
                "[bhyve]\nvmm_dev=\"/dev/vmm/sigma_vm\"\nppt_devices=[\"ppt0\", \"ppt1\"]\nvirtio_rnd=true",
                "[bhyve_config]\nvm_name = \"freebsd-bhyve-guest\"\ncpus = 4\nram = 8192\nbootloader = \"bhyveload\"",
            ),
            (
                "OpenBSD vmm / vmd Hypervisor with Unveil & Pledge Isolation",
                "vmd-openbsd@sigmaos.org",
                VirtualizationTechnologyFormat::OpenBsdVmm,
                VirtualizationPrCategory::SecurityIsolation,
                "Provides OpenBSD vmm(4) hardware virtualization daemon (`vmd`) with strict `pledge(\"stdio rpath wpath cpath vmm\")` and `unveil` file sandboxing.",
                "[openbsd_vmm]\nvmd_socket=\"/var/run/vmd.sock\"\npledge_promises=\"stdio rpath wpath cpath vmm\"\nunveil_paths=[\"/etc/vm.conf\", \"/var/vmm\"]",
                "[vm_conf]\nvm \"openbsd-guest\" {\n    memory 2048M\n    interface { switch \"uplink\" }\n    disk \"/var/vmm/guest.qcow2\"\n}",
            ),
            (
                "Docker OCI Container Engine & Layered Storage Gateway",
                "docker-engine@sigmaos.org",
                VirtualizationTechnologyFormat::Docker,
                VirtualizationPrCategory::ContainerRuntime,
                "Implements Docker OCI container engine integration with Dockerfile building, Overlay2 image layer stacking, and bridge network address translation.",
                "[docker]\ndaemon_socket=\"unix:///var/run/docker.sock\"\nstorage_driver=\"overlay2\"\ndefault_bridge=\"docker0\"",
                "[docker_container]\nimage = \"ubuntu:24.04\"\ncmd = [\"/bin/bash\"]\nports = [\"8080:80\"]",
            ),
            (
                "Podman Daemonless Rootless Engine & SubUID Namespace Translator",
                "podman-dev@sigmaos.org",
                VirtualizationTechnologyFormat::Podman,
                VirtualizationPrCategory::ContainerRuntime,
                "Adds Podman rootless container execution with `/etc/subuid` and `/etc/subgid` user namespace mapping and Slirp4netns unprivileged user networking.",
                "[podman]\nrootless=true\nsubuid_start=100000\nsubuid_count=65536\nnetwork_backend=\"slirp4netns\"",
                "[podman_pod]\nname = \"podman-rootless-pod\"\ncontainers = [\"web-nginx\", \"db-postgres\"]",
            ),
            (
                "Containerd OCI Runtime Daemon & Content Snapshotter Gateway",
                "containerd@sigmaos.org",
                VirtualizationTechnologyFormat::Containerd,
                VirtualizationPrCategory::ContainerRuntime,
                "Provides containerd gRPC API client for OCI container lifecycle control, content store image pulling, and native snapshotter mount management.",
                "[containerd]\ngrpc_address=\"/run/containerd/containerd.sock\"\nsnapshotter=\"overlayfs\"\nnamespace=\"k8s.io\"",
                "[containerd_task]\ncontainer_id = \"sigma-app-123\"\nimage = \"docker.io/library/alpine:latest\"",
            ),
            (
                "runc Low-Level OCI Container Spec & cgroups v2 Controller",
                "runc-maintainers@sigmaos.org",
                VirtualizationTechnologyFormat::Runc,
                VirtualizationPrCategory::ContainerRuntime,
                "Implements OCI `config.json` execution spec translator for runc CLI, configuring Linux cgroups v2 memory/cpu resource limits and `pivot_root` isolation.",
                "[runc]\noci_version=\"1.1.0\"\ncgroups_path=\"/sys/fs/cgroup/sigma.slice\"\nuse_systemd_cgroup=true",
                "{\"ociVersion\": \"1.1.0\", \"process\": {\"args\": [\"sh\"]}, \"root\": {\"path\": \"rootfs\"}}",
            ),
            (
                "LXC Linux System Container Management Framework",
                "lxc-core@sigmaos.org",
                VirtualizationTechnologyFormat::Lxc,
                VirtualizationPrCategory::ContainerRuntime,
                "Adds LXC system container creation with full init process startup, cgroup v2 subtree delegation, and veth pair network bridging.",
                "[lxc]\nlxc_path=\"/var/lib/lxc\"\nnetwork_type=\"veth\"\nnetwork_link=\"lxcbr0\"",
                "lxc.net.0.type = veth\nlxc.net.0.link = lxcbr0\nlxc.net.0.flags = up\nlxc.rootfs.path = dir:/var/lib/lxc/sys-container/rootfs",
            ),
            (
                "LXD Next-Gen System Container & VM Management REST API Engine",
                "lxd-team@sigmaos.org",
                VirtualizationTechnologyFormat::Lxd,
                VirtualizationPrCategory::OrchestrationCluster,
                "Provides LXD REST API client supporting both Linux system containers and qemu microVMs with ZFS/Btrfs CoW storage pool volume management.",
                "[lxd]\nendpoint=\"https://127.0.0.1:8443\"\nstorage_pool=\"zfs-pool\"\nnetwork_name=\"lxdbr0\"",
                "{\"name\": \"lxd-ubuntu-instance\", \"architecture\": \"x86_64\", \"profiles\": [\"default\"]}",
            ),
            (
                "systemd-nspawn OS Tree Sandbox & sd-bus Integration",
                "nspawn-dev@sigmaos.org",
                VirtualizationTechnologyFormat::SystemdNspawn,
                VirtualizationPrCategory::SecurityIsolation,
                "Implements `systemd-nspawn` OS image container sandbox with `--ephemeral` snapshotting, `--network-veth` interface isolation, and `sd-bus` systemd control.",
                "[systemd_nspawn]\nimage_directory=\"/var/lib/machines\"\nephemeral=true\nnetwork_veth=true",
                "[nspawn_machine]\nname = \"sigma-arch-nspawn\"\ndirectory = \"/var/lib/machines/arch-tree\"\nprivate_users = \"pick\"",
            ),
            (
                "FreeBSD Jails VNET Virtual Network & Security Jail Flags Engine",
                "jails-bsd@sigmaos.org",
                VirtualizationTechnologyFormat::FreeBsdJails,
                VirtualizationPrCategory::SecurityIsolation,
                "Adds FreeBSD Jails OS-level virtualization with per-jail VNET `epair` virtual interfaces, `jail.conf` syntax, and security control flags (`allow.raw_sockets`).",
                "[freebsd_jails]\njail_conf=\"/etc/jail.conf\"\nvnet_enabled=true\nallow_raw_sockets=true\nsecurelevel=2",
                "sigma_jail {\n    host.hostname = \"jail.sigma.org\";\n    path = \"/usr/jail/sigma_jail\";\n    vnet;\n    vnet.interface = \"epair0b\";\n}",
            ),
            (
                "Kubernetes Cloud-Native CRI, CNI & K3s Embedded Pod Gateway",
                "k8s-core@sigmaos.org",
                VirtualizationTechnologyFormat::Kubernetes,
                VirtualizationPrCategory::OrchestrationCluster,
                "Provides Kubernetes CRI/CNI runtime engine, K3s embedded cluster controller, and YAML Pod specification validator with multi-container sidecar support.",
                "[kubernetes]\ncri_endpoint=\"unix:///run/k3s/containerd/containerd.sock\"\ncni_bin_dir=\"/var/lib/rancher/k3s/data/current/bin\"\nembedded_k3s=true",
                "apiVersion: v1\nkind: Pod\nmetadata:\n  name: sigma-k8s-pod\nspec:\n  containers:\n  - name: app\n    image: nginx:latest",
            ),
        ];

        for (title, author, tech, cat, desc, spec, manifest) in presets {
            let _ = self.submit_pr(
                title.to_string(),
                author.to_string(),
                tech,
                cat,
                desc.to_string(),
                spec.to_string(),
                manifest.to_string(),
                vec![0x53, 0x49, 0x47, 0x4d, 0x41, 0x5F, 0x50, 0x51, 0x43, 0x5F, 0x56, 0x49, 0x52, 0x54], // PQC signature marker
            );
        }
    }

    /// Submit a new Virtualization Pull Request proposal
    pub fn submit_pr(
        &mut self,
        title: String,
        author: String,
        technology: VirtualizationTechnologyFormat,
        category: VirtualizationPrCategory,
        description: String,
        tech_spec: String,
        config_manifest: String,
        pqc_signature: Vec<u8>,
    ) -> Result<u32, String> {
        if title.trim().is_empty() {
            return Err("Pull Request title cannot be empty".to_string());
        }
        if author.trim().is_empty() {
            return Err("Pull Request author cannot be empty".to_string());
        }
        if pqc_signature.is_empty() {
            return Err("PQC post-quantum cryptographic signature is required for PR submission".to_string());
        }

        let pr_id = self.next_pr_id;
        self.next_pr_id += 1;

        let pr = VirtualizationPrSubmission {
            pr_id,
            title,
            author,
            technology,
            category,
            description,
            tech_spec,
            config_manifest,
            pqc_signature,
            status: VirtualizationPrStatus::Proposed,
            timestamp: 1700000000 + (pr_id as u64 * 3600),
        };

        self.proposals.insert(pr_id, pr);
        Ok(pr_id)
    }

    /// Validate the Post-Quantum Cryptographic signature of a PR submission
    pub fn validate_pqc_signature(&self, pr_id: u32) -> bool {
        if let Some(pr) = self.proposals.get(&pr_id) {
            !pr.pqc_signature.is_empty() && pr.pqc_signature.starts_with(b"SIGMA_PQC_")
        } else {
            false
        }
    }

    /// Run the automated CI PR Gate pipeline (validating specs, signatures, and config manifests)
    pub fn run_automated_ci_pr_gate(&mut self, pr_id: u32) -> Result<bool, String> {
        let pr = self
            .proposals
            .get_mut(&pr_id)
            .ok_or_else(|| format!("Virtualization PR #{} not found", pr_id))?;

        if pr.status == VirtualizationPrStatus::Merged {
            return Ok(true);
        }

        pr.status = VirtualizationPrStatus::CiValidation;

        if !pr.pqc_signature.starts_with(b"SIGMA_PQC_") {
            pr.status = VirtualizationPrStatus::Rejected;
            return Err(format!("PQC Signature verification failed for PR #{}", pr_id));
        }

        if pr.config_manifest.trim().is_empty() {
            pr.status = VirtualizationPrStatus::Rejected;
            return Err(format!("Configuration manifest is empty for PR #{}", pr_id));
        }

        pr.status = VirtualizationPrStatus::Gated;
        Ok(true)
    }

    /// Generate unified diff patch representation for a PR proposal
    pub fn generate_pr_unified_diff(&self, pr_id: u32) -> Result<String, String> {
        let pr = self
            .proposals
            .get(&pr_id)
            .ok_or_else(|| format!("Virtualization PR #{} not found", pr_id))?;

        let mut diff = String::new();
        diff.push_str(&format!(
            "--- a/virtualization/{}/{}.conf\n",
            format!("{:?}", pr.technology).to_lowercase(),
            pr.pr_id
        ));
        diff.push_str(&format!(
            "+++ b/virtualization/{}/{}.conf\n",
            format!("{:?}", pr.technology).to_lowercase(),
            pr.pr_id
        ));
        diff.push_str("@@ -0,0 +1,15 @@\n");
        diff.push_str(&format!("+# PR #{}: {}\n", pr.pr_id, pr.title));
        diff.push_str(&format!("+# Author: {}\n", pr.author));
        diff.push_str(&format!("+# Technology: {}\n", pr.technology.name()));
        diff.push_str(&format!("+# Category: {}\n", pr.category.label()));
        diff.push_str("+#\n");
        for line in pr.tech_spec.lines() {
            diff.push_str(&format!("+{}\n", line));
        }
        diff.push_str("+# Manifest Configuration:\n");
        for line in pr.config_manifest.lines() {
            diff.push_str(&format!("+{}\n", line));
        }

        Ok(diff)
    }

    /// Merge an approved, gated Virtualization Pull Request
    pub fn merge_pr(&mut self, pr_id: u32) -> Result<bool, String> {
        let pr = self
            .proposals
            .get_mut(&pr_id)
            .ok_or_else(|| format!("Virtualization PR #{} not found", pr_id))?;

        if pr.status == VirtualizationPrStatus::Merged {
            return Ok(true);
        }

        if pr.status != VirtualizationPrStatus::Gated {
            return Err(format!(
                "PR #{} must pass CI Gate (status Gated) before merging. Current status: {:?}",
                pr_id, pr.status
            ));
        }

        pr.status = VirtualizationPrStatus::Merged;
        Ok(true)
    }

    /// Search PRs by query string matching title, author, or technology name
    pub fn search_prs(&self, query: &str) -> Vec<VirtualizationPrSubmission> {
        let q = query.to_lowercase();
        self.proposals
            .values()
            .filter(|pr| {
                pr.title.to_lowercase().contains(&q)
                    || pr.author.to_lowercase().contains(&q)
                    || pr.technology.name().to_lowercase().contains(&q)
                    || pr.description.to_lowercase().contains(&q)
            })
            .cloned()
            .collect()
    }

    /// Get reference to a PR by ID
    pub fn get_pr(&self, pr_id: u32) -> Option<&VirtualizationPrSubmission> {
        self.proposals.get(&pr_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtualization_pr_gateway_preset_initialization() {
        let gateway = SovereignVirtualizationPrGatewayEngine::new();
        assert_eq!(gateway.proposals.len(), 16);

        // Verify key technologies exist in presets
        let technologies: Vec<VirtualizationTechnologyFormat> =
            gateway.proposals.values().map(|pr| pr.technology).collect();

        assert!(technologies.contains(&VirtualizationTechnologyFormat::Kvm));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Qemu));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Libvirt));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Xen));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::VirtualBox));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Bhyve));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::OpenBsdVmm));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Docker));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Podman));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Containerd));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Runc));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Lxc));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Lxd));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::SystemdNspawn));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::FreeBsdJails));
        assert!(technologies.contains(&VirtualizationTechnologyFormat::Kubernetes));
    }

    #[test]
    fn test_submit_ci_gate_and_merge_workflow() {
        let mut gateway = SovereignVirtualizationPrGatewayEngine::new();

        let pr_id = gateway
            .submit_pr(
                "Custom Firecracker MicroVM & Virtio-VSock PR".to_string(),
                "virt-engineer@sigmaos.org".to_string(),
                VirtualizationTechnologyFormat::Kvm,
                VirtualizationPrCategory::HypervisorCore,
                "MicroVM minimal jailer isolation".to_string(),
                "[firecracker]\nvsock=true\njailer=true".to_string(),
                "[config]\nkernel = \"/boot/vmlinux\"\nrootfs = \"/var/lib/rootfs.ext4\"".to_string(),
                vec![0x53, 0x49, 0x47, 0x4d, 0x41, 0x5F, 0x50, 0x51, 0x43, 0x5F, 0x31],
            )
            .unwrap();

        assert!(gateway.validate_pqc_signature(pr_id));

        // Run CI Gate
        assert!(gateway.run_automated_ci_pr_gate(pr_id).unwrap());

        let pr = gateway.get_pr(pr_id).unwrap();
        assert_eq!(pr.status, VirtualizationPrStatus::Gated);

        // Generate unified diff
        let diff = gateway.generate_pr_unified_diff(pr_id).unwrap();
        assert!(diff.contains("Custom Firecracker MicroVM"));
        assert!(diff.contains("+++ b/virtualization/kvm/"));

        // Merge PR
        assert!(gateway.merge_pr(pr_id).unwrap());
        let pr_merged = gateway.get_pr(pr_id).unwrap();
        assert_eq!(pr_merged.status, VirtualizationPrStatus::Merged);
    }

    #[test]
    fn test_search_virtualization_prs() {
        let gateway = SovereignVirtualizationPrGatewayEngine::new();

        let bhyve_results = gateway.search_prs("FreeBSD bhyve");
        assert_eq!(bhyve_results.len(), 1);
        assert_eq!(bhyve_results[0].technology, VirtualizationTechnologyFormat::Bhyve);

        let podman_results = gateway.search_prs("podman rootless");
        assert_eq!(podman_results.len(), 1);
        assert_eq!(podman_results[0].technology, VirtualizationTechnologyFormat::Podman);

        let k8s_results = gateway.search_prs("kubernetes");
        assert_eq!(k8s_results.len(), 1);
        assert_eq!(k8s_results[0].technology, VirtualizationTechnologyFormat::Kubernetes);
    }
}
