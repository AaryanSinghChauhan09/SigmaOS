// SigmaOS Legacy Linux Kernel & System Compatibility Adapter
// Enables ancient software compiled for Linux 2.x, 3.x, 4.x, 5.x, and 6.x to run securely

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxKernelVersion {
    Kernel2_6,
    Kernel3x,
    Kernel4x,
    Kernel5x,
    Kernel6x,
}

pub struct LegacyKernelAdapter {
    pub target_version: LinuxKernelVersion,
    pub syscall_shims: HashMap<u32, String>,
}

impl LegacyKernelAdapter {
    pub fn new(version: LinuxKernelVersion) -> Self {
        let mut shims = HashMap::new();
        // Standard Linux syscall shims
        shims.insert(1, "sys_exit".to_string());
        shims.insert(3, "sys_read".to_string());
        shims.insert(4, "sys_write".to_string());

        LegacyKernelAdapter {
            target_version: version,
            syscall_shims: shims,
        }
    }

    pub fn dispatch_syscall(&self, sys_num: u32) -> Result<String, ()> {
        if let Some(shim) = self.syscall_shims.get(&sys_num) {
            Ok(format!("Executing shim: {}", shim))
        } else {
            Err(())
        }
    }
}

pub struct LegacyPackageAdapter {
    pub supported_formats: Vec<String>,
}

impl LegacyPackageAdapter {
    pub fn new() -> Self {
        LegacyPackageAdapter {
            supported_formats: vec![".deb".to_string(), ".rpm".to_string(), ".tgz".to_string()],
        }
    }

    pub fn convert_package(&self, filename: &str) -> Result<String, ()> {
        let ext = filename.split('.').last().unwrap_or("");
        if self.supported_formats.contains(&format!(".{}", ext)) {
            Ok(format!("Converted {} to unified .spkg format", filename))
        } else {
            Err(())
        }
    }
}

pub struct LegacySecurityAdapter {
    pub dac_permissions: u32, // standard Unix permissions: e.g. 0o755
}

impl LegacySecurityAdapter {
    pub fn new(perm: u32) -> Self {
        LegacySecurityAdapter { dac_permissions: perm }
    }

    pub fn check_permission(&self, mode: u32) -> bool {
        (self.dac_permissions & mode) != 0
    }
}

pub struct LegacyUIAdapter {
    pub x11_display_id: u32,
    pub active_windows: usize,
}

impl LegacyUIAdapter {
    pub fn new() -> Self {
        LegacyUIAdapter {
            x11_display_id: 0,
            active_windows: 0,
        }
    }

    pub fn map_x11_to_zenith(&mut self, window_id: u32) -> String {
        self.active_windows += 1;
        format!("Mapped X11 Window ID {} to Zenith Desktop Surface", window_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_kernel_syscalls() {
        let adapter = LegacyKernelAdapter::new(LinuxKernelVersion::Kernel3x);
        assert_eq!(adapter.dispatch_syscall(4).unwrap(), "Executing shim: sys_write");
        assert!(adapter.dispatch_syscall(999).is_err());
    }

    #[test]
    fn test_legacy_package_converter() {
        let adapter = LegacyPackageAdapter::new();
        assert_eq!(adapter.convert_package("old-app.deb").unwrap(), "Converted old-app.deb to unified .spkg format");
        assert!(adapter.convert_package("unsupported.zip").is_err());
    }

    #[test]
    fn test_legacy_security() {
        let adapter = LegacySecurityAdapter::new(0o755);
        assert!(adapter.check_permission(0o400)); // Read permission check
    }

    #[test]
    fn test_legacy_ui_mapping() {
        let mut adapter = LegacyUIAdapter::new();
        assert_eq!(adapter.map_x11_to_zenith(4567), "Mapped X11 Window ID 4567 to Zenith Desktop Surface");
        assert_eq!(adapter.active_windows, 1);
    }
}
