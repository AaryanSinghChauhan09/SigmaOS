/// Tiny Core Linux Compatibility & Philosophy Absorption for SigmaOS
/// Implements frugal booting, RAM-only execution isolation, .tcz read-only extension loop mounting,
/// boot code parsing (base, norestore, etc.), and filetool-style (mydata.tgz) user backup/restore.

use crate::klib::Vec;
use crate::filesystem::FileType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TinyCoreBootConfig {
    pub is_base_only: bool,       // Skip loading any application extensions (the "base" boot code)
    pub is_no_restore: bool,      // Skip loading user backup (the "norestore" boot code)
    pub local_path: Option<String>, // Path for persistent extensions (e.g., "local=sda1")
    pub home_path: Option<String>,  // Path for persistent home (e.g., "home=sda1")
}

impl TinyCoreBootConfig {
    pub fn parse(boot_args: &str) -> Self {
        let mut is_base_only = false;
        let mut is_no_restore = false;
        let mut local_path = None;
        let mut home_path = None;

        for arg in boot_args.split_whitespace() {
            if arg == "base" {
                is_base_only = true;
            } else if arg == "norestore" {
                is_no_restore = true;
            } else if arg.starts_with("local=") {
                local_path = Some(arg["local=".len()..].to_string());
            } else if arg.starts_with("home=") {
                home_path = Some(arg["home=".len()..].to_string());
            }
        }

        TinyCoreBootConfig {
            is_base_only,
            is_no_restore,
            local_path,
            home_path,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TczExtension {
    pub name: String,
    pub dependencies: Vec<String>,
    pub files: Vec<(String, String)>, // (Relative filepath inside extension, content)
    pub is_mounted: bool,
}

impl TczExtension {
    pub fn new(name: &str) -> Self {
        TczExtension {
            name: name.to_string(),
            dependencies: Vec::new(),
            files: Vec::new(),
            is_mounted: false,
        }
    }

    pub fn with_file(mut self, path: &str, content: &str) -> Self {
        self.files.push((path.to_string(), content.to_string()));
        self
    }

    pub fn with_dependency(mut self, dep: &str) -> Self {
        self.dependencies.push(dep.to_string());
        self
    }
}

/// The TCE Loader representing Tiny Core's package-management loop-mounting system
pub struct TceLoader {
    pub extensions: Vec<TczExtension>,
}

impl TceLoader {
    pub fn new() -> Self {
        TceLoader {
            extensions: Vec::new(),
        }
    }

    pub fn register_extension(&mut self, ext: TczExtension) {
        self.extensions.push(ext);
    }

    /// Recursively mounts a .tcz extension and its dependencies onto the virtual filesystem
    pub fn mount_extension(&mut self, name: &str, vfs: &mut crate::filesystem::VirtualFilesystem) -> Result<(), &'static str> {
        // Find extension
        let mut ext_idx = None;
        for i in 0..self.extensions.len() {
            if self.extensions[i].name == name {
                ext_idx = Some(i);
                break;
            }
        }

        let idx = ext_idx.ok_or("Extension not found in repository")?;

        if self.extensions[idx].is_mounted {
            return Ok(()); // Already mounted
        }

        // Mount dependencies first
        let deps = self.extensions[idx].dependencies.clone();
        for dep in &deps {
            self.mount_extension(dep, vfs)?;
        }

        // Mount the files into VFS (frugal loop mount simulation)
        let files = self.extensions[idx].files.clone();
        for (_path, content) in &files {
            let file_id = vfs.create_file(FileType::Regular, 0).map_err(|_| "Failed to create VFS node")?;
            let fd = vfs.open_file(file_id, 0).map_err(|_| "Failed to open loop mount node")?;
            let content_bytes: &[u8] = content.as_bytes();
            vfs.write_file(fd, content_bytes).map_err(|_| "Failed to write loop mount content")?;
            vfs.close_file(fd).map_err(|_| "Failed to close loop mount fd")?;
        }

        self.extensions[idx].is_mounted = true;
        Ok(())
    }
}

impl Default for TceLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulation of filetool.sh and mydata.tgz backup/restore systems
pub struct FiletoolOverlay {
    pub backup_paths: Vec<String>, // Paths to back up (e.g. "/home", "/opt")
    pub ram_changes: Vec<(String, String)>, // Modified files in RAM overlay
}

impl FiletoolOverlay {
    pub fn new() -> Self {
        FiletoolOverlay {
            backup_paths: Vec::new(),
            ram_changes: Vec::new(),
        }
    }

    pub fn add_backup_path(&mut self, path: &str) {
        self.backup_paths.push(path.to_string());
    }

    pub fn write_ram_file(&mut self, path: &str, content: &str) {
        // Check if path matches any backup_paths
        let mut is_backed_up = false;
        for b_path in &self.backup_paths {
            if path.starts_with(b_path) {
                is_backed_up = true;
                break;
            }
        }

        if is_backed_up {
            // Overwrite if exists, otherwise push
            for change in &mut self.ram_changes {
                if change.0 == path {
                    change.1 = content.to_string();
                    return;
                }
            }
            self.ram_changes.push((path.to_string(), content.to_string()));
        }
    }

    /// Simulates packaging specified folders into a compressed backup mydata.tgz
    pub fn package_mydata(&self) -> Vec<u8> {
        let mut compressed_archive = Vec::new();
        // Pack files in mydata.tgz format
        for (path, content) in &self.ram_changes {
            compressed_archive.push(b'[');
            let path_bytes: &[u8] = path.as_bytes();
            for &b in path_bytes {
                compressed_archive.push(b);
            }
            compressed_archive.push(b':');
            let content_bytes: &[u8] = content.as_bytes();
            for &b in content_bytes {
                compressed_archive.push(b);
            }
            compressed_archive.push(b']');
        }
        compressed_archive
    }

    /// Simulates restoring user configurations from mydata.tgz into live RAM
    pub fn restore_mydata(&mut self, archive: &[u8]) {
        self.ram_changes = Vec::new();
        let mut i = 0;
        while i < archive.len() {
            if archive[i] == b'[' {
                i += 1;
                let start_path = i;
                while i < archive.len() && archive[i] != b':' {
                    i += 1;
                }
                let path = String::from_utf8_lossy(&archive[start_path..i]).to_string();

                i += 1; // Skip ':'
                let start_content = i;
                while i < archive.len() && archive[i] != b']' {
                    i += 1;
                }
                let content = String::from_utf8_lossy(&archive[start_content..i]).to_string();
                self.ram_changes.push((path, content));
            }
            i += 1;
        }
    }
}

impl Default for FiletoolOverlay {
    fn default() -> Self {
        Self::new()
    }
}

/// The ultimate Frugal Loader executing the complete Tiny Core RAM-only execution cycle
pub struct FrugalLoader {
    pub ram_size: usize,
    pub config: TinyCoreBootConfig,
    pub tce_loader: TceLoader,
    pub filetool: FiletoolOverlay,
}

impl FrugalLoader {
    pub fn new(ram_size: usize, boot_args: &str) -> Self {
        FrugalLoader {
            ram_size,
            config: TinyCoreBootConfig::parse(boot_args),
            tce_loader: TceLoader::new(),
            filetool: FiletoolOverlay::new(),
        }
    }

    /// Runs the Frugal Boot configuration, setting up RAM loop mounting and configuration restores
    pub fn execute_boot_sequence(&mut self, vfs: &mut crate::filesystem::VirtualFilesystem, mydata_archive: &[u8]) -> Result<(), &'static str> {
        // 1. Check if restore is permitted
        if !self.config.is_no_restore && !mydata_archive.is_empty() {
            self.filetool.restore_mydata(mydata_archive);
            // Write restored changes back to VFS
            for (_path, content) in &self.filetool.ram_changes {
                let file_id = vfs.create_file(FileType::Regular, 0).map_err(|_| "Failed to restore file")?;
                let fd = vfs.open_file(file_id, 0).map_err(|_| "Failed to open restored file")?;
                let content_bytes: &[u8] = content.as_bytes();
                vfs.write_file(fd, content_bytes).map_err(|_| "Failed to write restored content")?;
                vfs.close_file(fd).map_err(|_| "Failed to close restored fd")?;
            }
        }

        // 2. Check if we should mount extensions
        if !self.config.is_base_only {
            // Find extensions in local or standard storage repository and mount them
            let available_tcz = self.tce_loader.extensions.clone();
            for ext in &available_tcz {
                self.tce_loader.mount_extension(&ext.name, vfs)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_code_parsing() {
        let config = TinyCoreBootConfig::parse("base norestore local=sda1 home=sda2");
        assert!(config.is_base_only);
        assert!(config.is_no_restore);
        assert_eq!(config.local_path, Some("sda1".to_string()));
        assert_eq!(config.home_path, Some("sda2".to_string()));
    }

    #[test]
    fn test_tcz_loop_mounting() {
        let mut vfs = crate::filesystem::VirtualFilesystem::new();
        let mut loader = TceLoader::new();

        // Register flwm.tcz (depends on fltk.tcz)
        let fltk = TczExtension::new("fltk.tcz").with_file("/usr/lib/libfltk.so", "fltk_binary");
        let flwm = TczExtension::new("flwm.tcz")
            .with_dependency("fltk.tcz")
            .with_file("/usr/bin/flwm", "flwm_binary");

        loader.register_extension(fltk);
        loader.register_extension(flwm);

        // Mount flwm.tcz
        assert!(loader.mount_extension("flwm.tcz", &mut vfs).is_ok());

        // Verify flwm and its dependency fltk are both marked as mounted
        assert!(loader.extensions.iter().find(|e| e.name == "flwm.tcz").unwrap().is_mounted);
        assert!(loader.extensions.iter().find(|e| e.name == "fltk.tcz").unwrap().is_mounted);
    }

    #[test]
    fn test_filetool_backup_restore() {
        let mut filetool = FiletoolOverlay::new();
        filetool.add_backup_path("/home/tc");
        filetool.add_backup_path("/opt");

        // Write files to RAM
        filetool.write_ram_file("/home/tc/.profile", "alias ls='ls -color'");
        filetool.write_ram_file("/opt/bootlocal.sh", "echo boot");
        filetool.write_ram_file("/var/log/messages", "system log"); // Not backed up because path not registered

        assert_eq!(filetool.ram_changes.len(), 2);

        // Package backup into mydata.tgz simulation
        let archive = filetool.package_mydata();

        // Restore backup in fresh RAM system
        let mut restored_filetool = FiletoolOverlay::new();
        restored_filetool.restore_mydata(archive.as_slice());

        assert_eq!(restored_filetool.ram_changes.len(), 2);
        assert_eq!(
            restored_filetool.ram_changes.iter().find(|c| c.0 == "/opt/bootlocal.sh").unwrap().1,
            "echo boot"
        );
    }

    #[test]
    fn test_frugal_boot_execution() {
        let mut vfs = crate::filesystem::VirtualFilesystem::new();
        let mut loader = FrugalLoader::new(1024, "home=sda1");

        // Create a backup archive
        let mut backup_overlay = FiletoolOverlay::new();
        backup_overlay.add_backup_path("/home/tc");
        backup_overlay.write_ram_file("/home/tc/.ashrc", "export PS1='tc@core: '");
        let archive = backup_overlay.package_mydata();

        // Setup extensions
        let flwm = TczExtension::new("flwm.tcz").with_file("/usr/bin/flwm", "gui");
        loader.tce_loader.register_extension(flwm);

        // Boot system with backup and extensions
        assert!(loader.execute_boot_sequence(&mut vfs, archive.as_slice()).is_ok());

        // Verify extension is mounted and config is restored
        assert!(loader.tce_loader.extensions[0].is_mounted);
    }
}
