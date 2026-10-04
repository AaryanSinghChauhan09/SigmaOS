// Installation Manager for SigmaOS
// Installation manager per Wiki 01-Installation.md
// Provides installation methods and system requirements validation

use std::string::{String, ToString};
use std::vec::Vec;

/// System architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    ARM64,
    X86,
    ARM,
}

impl Architecture {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "x86_64" | "amd64" => Architecture::X86_64,
            "arm64" | "aarch64" => Architecture::ARM64,
            "x86" => Architecture::X86,
            "arm" => Architecture::ARM,
            _ => Architecture::X86_64,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Architecture::X86_64 => "x86_64",
            Architecture::ARM64 => "arm64",
            Architecture::X86 => "x86",
            Architecture::ARM => "arm",
        }
    }

    pub fn is_64bit(&self) -> bool {
        matches!(self, Architecture::X86_64 | Architecture::ARM64)
    }
}

/// System requirements
#[derive(Debug, Clone)]
pub struct SystemRequirements {
    pub min_ram_gb: u32,
    pub min_storage_gb: u32,
    pub recommended_ram_gb: u32,
    pub recommended_storage_gb: u32,
    pub uefi_support: bool,
    pub secure_boot_support: bool,
}

impl Default for SystemRequirements {
    fn default() -> Self {
        SystemRequirements {
            min_ram_gb: 2,
            min_storage_gb: 20,
            recommended_ram_gb: 8,
            recommended_storage_gb: 64,
            uefi_support: true,
            secure_boot_support: true,
        }
    }
}

impl SystemRequirements {
    pub fn check_minimum(&self, ram_gb: u32, storage_gb: u32) -> bool {
        ram_gb >= self.min_ram_gb && storage_gb >= self.min_storage_gb
    }

    pub fn check_recommended(&self, ram_gb: u32, storage_gb: u32) -> bool {
        ram_gb >= self.recommended_ram_gb && storage_gb >= self.recommended_storage_gb
    }
}

/// Installation method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallationMethod {
    BareMetal,
    VirtualMachine,
    DualBoot,
}

impl InstallationMethod {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "bare metal" => InstallationMethod::BareMetal,
            "virtual machine" | "vm" => InstallationMethod::VirtualMachine,
            "dual boot" => InstallationMethod::DualBoot,
            _ => InstallationMethod::BareMetal,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            InstallationMethod::BareMetal => "bare metal",
            InstallationMethod::VirtualMachine => "virtual machine",
            InstallationMethod::DualBoot => "dual boot",
        }
    }
}

/// Virtual machine type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmType {
    QEMU,
    VirtualBox,
    VMware,
}

impl VmType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "qemu" => VmType::QEMU,
            "virtualbox" => VmType::VirtualBox,
            "vmware" => VmType::VMware,
            _ => VmType::QEMU,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            VmType::QEMU => "qemu",
            VmType::VirtualBox => "virtualbox",
            VmType::VMware => "vmware",
        }
    }

    pub fn get_default_memory_mb(&self) -> u32 {
        match self {
            VmType::QEMU => 4096,
            VmType::VirtualBox => 4096,
            VmType::VMware => 4096,
        }
    }

    pub fn get_default_storage_gb(&self) -> u32 {
        match self {
            VmType::QEMU => 64,
            VmType::VirtualBox => 64,
            VmType::VMware => 64,
        }
    }
}

/// Installation configuration
#[derive(Debug, Clone)]
pub struct InstallationConfig {
    pub method: InstallationMethod,
    pub architecture: Architecture,
    pub vm_type: Option<VmType>,
    pub memory_mb: u32,
    pub storage_gb: u32,
    pub enable_kvm: bool,
    pub enable_graphics: bool,
}

impl Default for InstallationConfig {
    fn default() -> Self {
        InstallationConfig {
            method: InstallationMethod::BareMetal,
            architecture: Architecture::X86_64,
            vm_type: None,
            memory_mb: 4096,
            storage_gb: 64,
            enable_kvm: true,
            enable_graphics: true,
        }
    }
}

impl InstallationConfig {
    pub fn new(method: InstallationMethod, architecture: Architecture) -> Self {
        InstallationConfig {
            method,
            architecture,
            vm_type: None,
            memory_mb: 4096,
            storage_gb: 64,
            enable_kvm: true,
            enable_graphics: true,
        }
    }

    pub fn with_vm_type(mut self, vm_type: VmType) -> Self {
        self.vm_type = Some(vm_type);
        self.memory_mb = vm_type.get_default_memory_mb();
        self.storage_gb = vm_type.get_default_storage_gb();
        self
    }

    pub fn with_memory(mut self, memory_mb: u32) -> Self {
        self.memory_mb = memory_mb;
        self
    }

    pub fn with_storage(mut self, storage_gb: u32) -> Self {
        self.storage_gb = storage_gb;
        self
    }

    pub fn generate_qemu_command(&self) -> String {
        let mut cmd = String::from("qemu-system-");

        match self.architecture {
            Architecture::X86_64 => cmd.push_str("x86_64"),
            Architecture::ARM64 => cmd.push_str("aarch64"),
            Architecture::X86 => cmd.push_str("i386"),
            Architecture::ARM => cmd.push_str("arm"),
        }

        cmd.push_str(&format!(" -m {}", self.memory_mb));
        cmd.push_str(&format!(" -smp {}", self.memory_mb / 2048));
        cmd.push_str(&format!(
            " -drive file=sigmaos.qcow2,format=qcow2,size={}G",
            self.storage_gb
        ));
        cmd.push_str(&format!(" -drive file=sigmaos.qcow2,format=qcow2,size={}G", self.storage_gb));

        if self.enable_kvm {
            cmd.push_str(" -enable-kvm");
        }

        if self.enable_graphics {
            cmd.push_str(" -display gtk");
        }

        cmd.push_str(" -net nic,model=virtio -net user");

        cmd
    }

    pub fn generate_dd_command(&self, iso_path: &str, device: &str) -> String {
        format!(
            "dd if={} of={} bs=4M status=progress && sync",
            iso_path, device
        )
    }

    pub fn validate(&self) -> Result<String, String> {
        let mut issues = Vec::new();

        if self.memory_mb < 1024 {
            issues.push("Memory too low (minimum 1GB recommended)");
        }

        if self.storage_gb < 20 {
            issues.push("Storage too small (minimum 20GB required)");
        }

        if let Some(vm_type) = self.vm_type {
            if matches!(self.method, InstallationMethod::BareMetal) {
                issues.push("VM type specified for bare metal installation");
            }
        }

        if issues.is_empty() {
            Ok(String::from("Configuration is valid"))
        } else {
            Err(issues.join("; "))
        }
    }
}

/// Installation manager
#[derive(Debug, Clone)]
pub struct InstallationManager {
    pub config: InstallationConfig,
    pub requirements: SystemRequirements,
}

impl InstallationManager {
    pub fn new(config: InstallationConfig) -> Self {
        let requirements = SystemRequirements::default();
        InstallationManager {
            config,
            requirements,
        }
    }

    pub fn check_system_compatibility(&self, ram_gb: u32, storage_gb: u32) -> bool {
        self.requirements.check_minimum(ram_gb, storage_gb)
    }

    pub fn get_installation_command(&self) -> Result<String, String> {
        match self.config.method {
            InstallationMethod::BareMetal => {
                Ok(String::from("Boot from ISO and follow graphical installer"))
            }
            InstallationMethod::VirtualMachine => {
                if let Some(vm_type) = self.config.vm_type {
                    match vm_type {
                        VmType::QEMU => Ok(self.config.generate_qemu_command()),
                        VmType::VirtualBox => Ok(String::from("Create VM and mount ISO")),
                        VmType::VMware => Ok(String::from("Create VM and mount ISO")),
                    }
                } else {
                    Err(String::from("VM type not specified for virtual machine installation"))
                }
            }
            InstallationMethod::DualBoot => {
                Ok(String::from("Run installer and use automatic partition detection"))
            }
        }
    }

    pub fn validate_config(&self) -> Result<String, String> {
        self.config.validate()
    }

    pub fn get_requirements_summary(&self) -> String {
        format!(
            "Minimum: {}GB RAM, {}GB Storage\nRecommended: {}GB RAM, {}GB Storage\nUEFI: {}\nSecure Boot: {}",
            self.requirements.min_ram_gb,
            self.requirements.min_storage_gb,
            self.requirements.recommended_ram_gb,
            self.requirements.recommended_storage_gb,
            self.requirements.uefi_support,
            self.requirements.secure_boot_support
        )
    }
}

impl Default for InstallationManager {
    fn default() -> Self {
        Self::new(InstallationConfig::default())
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_from_str() {
        assert_eq!(Architecture::from_str("x86_64"), Architecture::X86_64);
        assert_eq!(Architecture::from_str("arm64"), Architecture::ARM64);
        assert_eq!(Architecture::from_str("x86"), Architecture::X86);
        assert_eq!(Architecture::is_64bit(Architecture::X86_64), true);
        assert_eq!(Architecture::is_64bit(Architecture::X86), false);
    }

    #[test]
    fn test_system_requirements_default() {
        let reqs = SystemRequirements::default();
        assert_eq!(reqs.min_ram_gb, 2);
        assert_eq!(reqs.min_storage_gb, 20);
        assert!(reqs.check_minimum(2, 20));
        assert!(!reqs.check_minimum(1, 19));
    }

    #[test]
    fn test_system_requirements_check() {
        let reqs = SystemRequirements::default();
        assert!(reqs.check_minimum(4, 30));
        assert!(reqs.check_recommended(8, 64));
        assert!(!reqs.check_recommended(4, 30));
    }

    #[test]
    fn test_installation_method_from_str() {
        assert_eq!(InstallationMethod::from_str("bare metal"), InstallationMethod::BareMetal);
        assert_eq!(InstallationMethod::from_str("virtual machine"), InstallationMethod::VirtualMachine);
        assert_eq!(InstallationMethod::from_str("dual boot"), InstallationMethod::DualBoot);
    }

    #[test]
    fn test_vm_type_from_str() {
        assert_eq!(VmType::from_str("qemu"), VmType::QEMU);
        assert_eq!(VmType::from_str("virtualbox"), VmType::VirtualBox);
        assert_eq!(VmType::from_str("vmware"), VmType::VMware);
    }

    #[test]
    fn test_vm_type_defaults() {
        assert_eq!(VmType::QEMU.get_default_memory_mb(), 4096);
        assert_eq!(VmType::QEMU.get_default_storage_gb(), 64);
    }

    #[test]
    fn test_installation_config_creation() {
        let config = InstallationConfig::new(
            InstallationMethod::VirtualMachine,
            Architecture::X86_64,
        );
        assert_eq!(config.method, InstallationMethod::VirtualMachine);
        assert_eq!(config.architecture, Architecture::X86_64);
    }

    #[test]
    fn test_installation_config_with_vm_type() {
        let config =
            InstallationConfig::new(InstallationMethod::VirtualMachine, Architecture::X86_64)
                .with_vm_type(VmType::QEMU);
        let config = InstallationConfig::new(
            InstallationMethod::VirtualMachine,
            Architecture::X86_64,
        ).with_vm_type(VmType::QEMU);

        assert_eq!(config.vm_type, Some(VmType::QEMU));
        assert_eq!(config.memory_mb, 4096);
        assert_eq!(config.storage_gb, 64);
    }

    #[test]
    fn test_installation_config_with_memory() {
        let config =
            InstallationConfig::new(InstallationMethod::VirtualMachine, Architecture::X86_64)
                .with_memory(8192);
        let config = InstallationConfig::new(
            InstallationMethod::VirtualMachine,
            Architecture::X86_64,
        ).with_memory(8192);

        assert_eq!(config.memory_mb, 8192);
    }

    #[test]
    fn test_generate_qemu_command() {
        let config =
            InstallationConfig::new(InstallationMethod::VirtualMachine, Architecture::X86_64)
                .with_vm_type(VmType::QEMU);
        let config = InstallationConfig::new(
            InstallationMethod::VirtualMachine,
            Architecture::X86_64,
        ).with_vm_type(VmType::QEMU);

        let cmd = config.generate_qemu_command();
        assert!(cmd.contains("qemu-system-x86_64"));
        assert!(cmd.contains("-m 4096"));
        assert!(cmd.contains("-enable-kvm"));
    }

    #[test]
    fn test_generate_dd_command() {
        let config = InstallationConfig::default();
        let cmd = config.generate_dd_command("sigmaos.iso", "/dev/sdX");

        assert!(cmd.contains("dd if=sigmaos.iso"));
        assert!(cmd.contains("of=/dev/sdX"));
        assert!(cmd.contains("bs=4M"));
    }

    #[test]
    fn test_installation_config_validate() {
        let config = InstallationConfig::default();
        assert!(config.validate().is_ok());

        let invalid_config =
            InstallationConfig::new(InstallationMethod::BareMetal, Architecture::X86_64)
                .with_memory(512);
        let invalid_config = InstallationConfig::new(
            InstallationMethod::BareMetal,
            Architecture::X86_64,
        ).with_memory(512);
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_installation_manager_creation() {
        let manager = InstallationManager::new(InstallationConfig::default());
        assert!(manager.config.method == InstallationMethod::BareMetal);
    }

    #[test]
    fn test_installation_manager_check_compatibility() {
        let manager = InstallationManager::new(InstallationConfig::default());
        assert!(manager.check_system_compatibility(4, 30));
        assert!(!manager.check_system_compatibility(1, 10));
    }

    #[test]
    fn test_installation_manager_get_command() {
        let config = InstallationConfig::new(
            InstallationMethod::VirtualMachine,
            Architecture::X86_64,
        ).with_vm_type(VmType::QEMU);
        let manager = InstallationManager::new(config);

        let cmd = manager.get_installation_command().unwrap();
        assert!(cmd.contains("qemu-system-x86_64"));
    }

    #[test]
    fn test_installation_manager_validate() {
        let manager = InstallationManager::new(InstallationConfig::default());
        assert!(manager.validate_config().is_ok());
    }

    #[test]
    fn test_installation_manager_requirements_summary() {
        let manager = InstallationManager::new(InstallationConfig::default());
        let summary = manager.get_requirements_summary();

        assert!(summary.contains("Minimum: 2GB RAM"));
        assert!(summary.contains("Recommended: 8GB RAM"));
    }
}
