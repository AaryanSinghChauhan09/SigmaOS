// sigma_microvm_isolation.rs — QubesOS-inspired MicroVM Compartmentalization
// Implements the domain isolation model using Firecracker/gVisor style interfaces.
// Each domain (Work, Personal, Banking, Untrusted) runs in its own isolated VM
// with strictly controlled inter-domain communication channels.

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

// ── Domain Definitions ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum DomainTrust {
    Vault,      // Highest trust — encrypted secrets, offline only
    Trusted,    // High trust — personal, banking
    Standard,   // Default — work, browsing
    Untrusted,  // Lowest — unknown apps, downloads
}

#[derive(Debug, Clone)]
pub struct VmDomain {
    pub id: u32,
    pub name: &'static str,
    pub trust_level: DomainTrust,
    pub net_allowed: bool,
    pub usb_passthrough: bool,
    pub clipboard_share: bool,
    pub memory_mb: u32,
    pub vcpu_count: u8,
}

/// The four default SigmaOS security domains
pub const SIGMA_DOMAINS: &[VmDomain] = &[
    VmDomain {
        id: 0,
        name: "sigma-vault",
        trust_level: DomainTrust::Vault,
        net_allowed: false,
        usb_passthrough: false,
        clipboard_share: false,
        memory_mb: 512,
        vcpu_count: 1,
    },
    VmDomain {
        id: 1,
        name: "sigma-personal",
        trust_level: DomainTrust::Trusted,
        net_allowed: true,
        usb_passthrough: false,
        clipboard_share: true,
        memory_mb: 2048,
        vcpu_count: 2,
    },
    VmDomain {
        id: 2,
        name: "sigma-work",
        trust_level: DomainTrust::Standard,
        net_allowed: true,
        usb_passthrough: false,
        clipboard_share: true,
        memory_mb: 4096,
        vcpu_count: 4,
    },
    VmDomain {
        id: 3,
        name: "sigma-untrusted",
        trust_level: DomainTrust::Untrusted,
        net_allowed: false,
        usb_passthrough: false,
        clipboard_share: false,
        memory_mb: 1024,
        vcpu_count: 1,
    },
];

// ── Inter-Domain Policy ─────────────────────────────────────────────────────

/// Determines if data transfer between two domains is permitted.
/// More trusted domains can receive from less trusted; reverse is blocked.
pub fn is_transfer_allowed(from: &VmDomain, to: &VmDomain) -> bool {
    match (&from.trust_level, &to.trust_level) {
        // Vault never shares outward
        (DomainTrust::Vault, _) => false,
        // Untrusted can only send to Untrusted
        (DomainTrust::Untrusted, DomainTrust::Untrusted) => true,
        (DomainTrust::Untrusted, _) => false,
        // Standard can send to Standard or Trusted
        (DomainTrust::Standard, DomainTrust::Trusted) => true,
        (DomainTrust::Standard, DomainTrust::Standard) => true,
        // Trusted can only send to Trusted
        (DomainTrust::Trusted, DomainTrust::Trusted) => true,
        _ => false,
    }
}

// ── Firecracker VMM Interface ───────────────────────────────────────────────

#[repr(C)]
pub struct FirecrackerVmConfig {
    pub domain_id: u32,
    pub mem_size_mib: u32,
    pub vcpu_count: u8,
    pub kernel_image_path: [u8; 128],
    pub rootfs_path: [u8; 128],
    pub net_enabled: bool,
}

impl FirecrackerVmConfig {
    pub fn from_domain(domain: &VmDomain) -> Self {
        let mut config = FirecrackerVmConfig {
            domain_id: domain.id,
            mem_size_mib: domain.memory_mb,
            vcpu_count: domain.vcpu_count,
            kernel_image_path: [0u8; 128],
            rootfs_path: [0u8; 128],
            net_enabled: domain.net_allowed,
        };
        // Write kernel path bytes
        let kernel = b"/sigma/images/vmlinux-sigma";
        config.kernel_image_path[..kernel.len()].copy_from_slice(kernel);
        config
    }
}

/// Boot a domain's microVM using the Firecracker API socket.
/// Returns the VM's process ID on success.
pub fn boot_domain_vm(domain: &VmDomain) -> Result<u32, &'static str> {
    let _config = FirecrackerVmConfig::from_domain(domain);
    // In production: POST config to /run/firecracker-{id}.socket
    // For now, return a simulated PID
    Ok(1000 + domain.id)
}

/// List all running domains
pub fn list_active_domains() -> Vec<&'static str> {
    SIGMA_DOMAINS.iter().map(|d| d.name).collect()
}
