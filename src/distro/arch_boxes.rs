
use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// =========================================================================
// 1. ARCH BOXES TARGET FORMATS (arch-boxes / QEMU / Vagrant parity)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchBoxFormat {
    Qcow2,      // QEMU / KVM Copy-On-Write sparse disk image
    Raw,        // Bare-metal raw block device image
    Vmdk,       // VMware Workstation / ESXi virtual disk
    VagrantBox, // Vagrant box archive (.box) with metadata.json & Vagrantfile
    CloudInit,  // OpenStack / AWS / Proxmox cloud-init cloud-config pre-seeded image
}

/// Image record descriptor in Arch Boxes catalog
#[derive(Debug, Clone)]
pub struct ArchBoxImageRecord {
    pub image_id: String,
    pub name: String,
    pub version: String,
    pub format: ArchBoxFormat,
    pub target_arch: String,
    pub virtual_size_mb: u64,
    pub actual_size_bytes: u64,
    pub sha256_checksum: [u8; 32],
    pub download_url: String,
    pub is_cloud_init_ready: bool,
}

// =========================================================================
// 2. CLOUD-INIT PRE-SEEDING PROVISIONER
// =========================================================================

#[derive(Debug, Clone)]
pub struct CloudInitUserAccount {
    pub username: String,
    pub ssh_authorized_keys: Vec<String>,
    pub sudo_access: bool,
}

pub struct ArchCloudInitProvisioner {
    pub hostname: String,
    pub user_accounts: Vec<CloudInitUserAccount>,
    pub network_dhcp_enabled: bool,
    pub custom_systemd_units: Vec<String>,
}

impl ArchCloudInitProvisioner {
    pub fn new(hostname: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
            user_accounts: Vec::new(),
            network_dhcp_enabled: true,
            custom_systemd_units: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: &str, ssh_key: &str, sudo: bool) {
        self.user_accounts.push(CloudInitUserAccount {
            username: username.to_string(),
            ssh_authorized_keys: vec![ssh_key.to_string()],
            sudo_access: sudo,
        });
    }

    /// Render user-data cloud-config YAML manifest for cloud image deployment
    pub fn render_user_data_yaml(&self) -> String {
        let mut yaml = format!("#cloud-config\nhostname: {}\nusers:\n", self.hostname);

        for user in &self.user_accounts {
            yaml.push_str(&format!("  - name: {}\n", user.username));
            if user.sudo_access {
                yaml.push_str("    sudo: ['ALL=(ALL) NOPASSWD:ALL']\n");
            }
            if !user.ssh_authorized_keys.is_empty() {
                yaml.push_str("    ssh_authorized_keys:\n");
                for key in &user.ssh_authorized_keys {
                    yaml.push_str(&format!("      - {}\n", key));
                }
            }
        }

        yaml
    }
}

impl Default for ArchCloudInitProvisioner {
    fn default() -> Self {
        Self::new("archlinux-box")
    }
}

// =========================================================================
// 3. VAGRANT & GNOME BOXES CATALOG MANAGER
// =========================================================================

pub struct ArchBoxCatalogManager {
    pub catalog: BTreeMap<String, ArchBoxImageRecord>,
}

impl ArchBoxCatalogManager {
    pub fn new() -> Self {
        Self {
            catalog: BTreeMap::new(),
        }
    }

    pub fn register_image(&mut self, record: ArchBoxImageRecord) {
        self.catalog.insert(record.image_id.clone(), record);
    }

    pub fn find_by_format(&self, format: ArchBoxFormat) -> Vec<&ArchBoxImageRecord> {
        self.catalog
            .values()
            .filter(|rec| rec.format == format)
            .collect()
    }

    /// Render Vagrant metadata.json for Vagrant box catalog publishing
    pub fn render_vagrant_metadata_json(&self, box_name: &str) -> String {
        let matches = self
            .catalog
            .values()
            .filter(|r| r.format == ArchBoxFormat::VagrantBox && r.name == box_name);

        let mut json = format!("{{\n  \"name\": \"{}\",\n  \"versions\": [\n", box_name);
        for rec in matches {
            json.push_str(&format!(
                "    {{\n      \"version\": \"{}\",\n      \"providers\": [\n        {{\n          \"name\": \"qemu\",\n          \"url\": \"{}\"\n        }}\n      ]\n    }}\n",
                rec.version, rec.download_url
            ));
        }
        json.push_str("  ]\n}");
        json
    }
}

impl Default for ArchBoxCatalogManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. ARCH BOXES IMAGE ENGINE
// =========================================================================

/// Arch Boxes Image Engine managing QEMU/RAW/Vagrant image builds
pub struct ArchBoxesImageEngine {
    pub catalog: ArchBoxCatalogManager,
    pub provisioner: ArchCloudInitProvisioner,
    pub next_image_id: u64,
}

impl ArchBoxesImageEngine {
    pub fn new() -> Self {
        Self {
            catalog: ArchBoxCatalogManager::new(),
            provisioner: ArchCloudInitProvisioner::new("sigmaos-arch-box"),
            next_image_id: 1,
        }
    }

    /// Build and register a virtual box image
    pub fn build_box_image(
        &mut self,
        name: &str,
        version: &str,
        format: ArchBoxFormat,
        virtual_size_mb: u64,
        raw_bytes: &[u8],
    ) -> String {
        let image_id = format!("box-{}-{}", format as u32, self.next_image_id);
        self.next_image_id += 1;

        // FNV-1a SHA-256 mock calculation
        let mut sha256 = [0u8; 32];
        let mut state: u64 = 0xcbf29ce484222325;
        for (i, &b) in raw_bytes.iter().enumerate() {
            state ^= b as u64;
            state = state.wrapping_mul(0x100000001b3);
            sha256[i % 32] ^= (state >> ((i % 8) * 8)) as u8;
        }

        let record = ArchBoxImageRecord {
            image_id: image_id.clone(),
            name: name.to_string(),
            version: version.to_string(),
            format,
            target_arch: String::from("x86_64"),
            virtual_size_mb,
            actual_size_bytes: raw_bytes.len() as u64,
            sha256_checksum: sha256,
            download_url: format!("https://boxes.archlinux.org/{}/{}.box", name, version),
            is_cloud_init_ready: format == ArchBoxFormat::CloudInit,
        };

        self.catalog.register_image(record);
        image_id
    }
}

impl Default for ArchBoxesImageEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_cloud_init_provisioner_yaml() {
        let mut provisioner = ArchCloudInitProvisioner::new("arch-test-host");
        provisioner.add_user("archuser", "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5...", true);

        let yaml = provisioner.render_user_data_yaml();
        assert!(yaml.contains("hostname: arch-test-host"));
        assert!(yaml.contains("name: archuser"));
        assert!(yaml.contains("ssh_authorized_keys:"));
    }

    #[test]
    fn test_arch_boxes_image_building_and_catalog() {
        let mut engine = ArchBoxesImageEngine::new();
        let mock_raw = vec![0x41u8; 1024];

        let img_id = engine.build_box_image(
            "archlinux/archlinux",
            "2026.08.31",
            ArchBoxFormat::Qcow2,
            20480,
            &mock_raw,
        );

        assert!(img_id.starts_with("box-0-"));
        let qcow2_boxes = engine.catalog.find_by_format(ArchBoxFormat::Qcow2);
        assert_eq!(qcow2_boxes.len(), 1);
        assert_eq!(qcow2_boxes[0].name, "archlinux/archlinux");
        assert_eq!(qcow2_boxes[0].virtual_size_mb, 20480);
    }

    #[test]
    fn test_vagrant_metadata_json_rendering() {
        let mut engine = ArchBoxesImageEngine::new();
        let mock_raw = vec![0x55u8; 512];

        engine.build_box_image(
            "archlinux/archlinux",
            "2026.08.31",
            ArchBoxFormat::VagrantBox,
            20480,
            &mock_raw,
        );

        let json = engine.catalog.render_vagrant_metadata_json("archlinux/archlinux");
        assert!(json.contains("\"name\": \"archlinux/archlinux\""));
        assert!(json.contains("\"version\": \"2026.08.31\""));
    }
}
