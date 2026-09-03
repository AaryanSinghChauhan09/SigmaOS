// SigmaOS FreeBSD bsdinstall & Arch Netinst Minimal Text Installer Engine
// Text-based, scriptable, no GUI dependency installer for live media and PXE netinst boots.

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallMode {
    InteractiveTui,
    HeadlessAutoInstall,
    NetinstPxe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    ZFS,
    Ext4,
    Btrfs,
}

#[derive(Debug, Clone)]
pub struct TargetDiskConfig {
    pub device_path: String, // e.g. "/dev/nvme0n1"
    pub filesystem: FilesystemType,
    pub is_root_on_zfs: bool,
    pub swap_size_mb: u64,
}

#[derive(Debug, Clone)]
pub struct NetworkProvisioning {
    pub interface_name: String,
    pub ip_address: String,
    pub gateway: String,
    pub dns_servers: Vec<String>,
}

#[derive(Debug)]
pub struct BsdinstallNetinstEngine {
    pub mode: InstallMode,
    pub disk_config: Option<TargetDiskConfig>,
    pub network: Option<NetworkProvisioning>,
    pub zpool_created: bool,
    pub bytes_extracted: u64,
}

impl BsdinstallNetinstEngine {
    pub fn new(mode: InstallMode) -> Self {
        Self {
            mode,
            disk_config: None,
            network: None,
            zpool_created: false,
            bytes_extracted: 0,
        }
    }

    pub fn configure_network_interface(
        &mut self,
        iface: &str,
        ip: &str,
        gateway: &str,
    ) {
        self.network = Some(NetworkProvisioning {
            interface_name: iface.to_string(),
            ip_address: ip.to_string(),
            gateway: gateway.to_string(),
            dns_servers: vec!["1.1.1.1".to_string(), "8.8.8.8".to_string()],
        });
    }

    pub fn partition_disk_zfs(
        &mut self,
        device: &str,
        zpool_name: &str,
    ) -> Result<String, &'static str> {
        if device.is_empty() || zpool_name.is_empty() {
            return Err("bsdinstall: Device and ZPool name cannot be empty");
        }

        let config = TargetDiskConfig {
            device_path: device.to_string(),
            filesystem: FilesystemType::ZFS,
            is_root_on_zfs: true,
            swap_size_mb: 4096,
        };

        self.disk_config = Some(config);
        self.zpool_created = true;

        Ok(format!(
            "Root-on-ZFS successfully created on '{}' (zpool: '{}', datasets: zroot/ROOT/default, zroot/var, zroot/home)",
            device, zpool_name
        ))
    }

    pub fn download_base_tarball_and_extract(
        &mut self,
        mirror_url: &str,
    ) -> Result<u64, &'static str> {
        if mirror_url.is_empty() {
            return Err("bsdinstall/netinst: Mirror URL cannot be empty");
        }

        // Simulate minimal base system tarball extraction (150 MB base OS)
        self.bytes_extracted = 150_000_000;
        Ok(self.bytes_extracted)
    }

    pub fn generate_unattended_script(&self) -> String {
        let hostname = "sigmaos-node";
        let iface = self
            .network
            .as_ref()
            .map(|n| n.interface_name.as_str())
            .unwrap_or("eth0");
        let disk = self
            .disk_config
            .as_ref()
            .map(|d| d.device_path.as_str())
            .unwrap_or("/dev/sda");

        format!(
            "# bsdinstall / install.conf unattended script\n\
             HOSTNAME=\"{}\"\n\
             INTERFACE=\"{}\"\n\
             TARGET_DISK=\"{}\"\n\
             FILESYSTEM=\"ZFS\"\n\
             ROOT_ON_ZFS=1\n\
             AUTO_REBOOT=1\n",
            hostname, iface, disk
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsdinstall_netinst_engine() {
        let mut engine = BsdinstallNetinstEngine::new(InstallMode::InteractiveTui);

        // Network configuration
        engine.configure_network_interface("eth0", "192.168.1.100/24", "192.168.1.1");
        assert!(engine.network.is_some());
        assert_eq!(engine.network.as_ref().unwrap().interface_name, "eth0");

        // ZFS partitioning
        let zfs_res = engine.partition_disk_zfs("/dev/nvme0n1", "zroot");
        assert!(zfs_res.is_ok());
        assert!(engine.zpool_created);

        // Download & extract base system
        let bytes = engine.download_base_tarball_and_extract("https://repo.sigmaos.org/base.txz").unwrap();
        assert_eq!(bytes, 150_000_000);

        // Script generation
        let script = engine.generate_unattended_script();
        assert!(script.contains("FILESYSTEM=\"ZFS\""));
        assert!(script.contains("TARGET_DISK=\"/dev/nvme0n1\""));
    }
}
