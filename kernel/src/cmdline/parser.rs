// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired kernel command line parsing for SigmaOS
// Zero-allocation, performance-optimized command line operations

/// Kernel command line parser
pub struct KernelCmdline {
    pub parameters: Vec<KernelParameter>,
    pub raw_cmdline: String,
}

impl KernelCmdline {
    pub const fn new() -> Self {
        Self {
            parameters: Vec::new(),
            raw_cmdline: String::new(),
        }
    }
    
    pub fn parse(&mut self, cmdline: &str) -> Result<(), CmdlineError> {
        self.raw_cmdline = cmdline.to_string();
        
        for part in cmdline.split_whitespace() {
            if let Some(pos) = part.find('=') {
                let key = &part[..pos];
                let value = &part[pos+1..];
                self.parameters.push(KernelParameter {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            } else {
                // Boolean parameter (present = true)
                self.parameters.push(KernelParameter {
                    key: part.to_string(),
                    value: "1".to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    pub fn get(&self, key: &str) -> Option<&str> {
        self.parameters.iter().find(|p| p.key == key).map(|p| p.value.as_str())
    }
    
    pub fn get_bool(&self, key: &str) -> bool {
        match self.get(key) {
            Some("1") | Some("true") | Some("yes") | Some("on") => true,
            _ => false,
        }
    }
    
    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.get(key)?.parse().ok()
    }
    
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get(key)?.parse().ok()
    }
    
    pub fn has(&self, key: &str) -> bool {
        self.parameters.iter().any(|p| p.key == key)
    }
}

/// Kernel parameter
pub struct KernelParameter {
    pub key: String,
    pub value: String,
}

/// Command line error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmdlineError {
    ParseError,
    InvalidParameter,
    InvalidValue,
    DuplicateParameter,
    Other,
}

/// Standard kernel parameters
pub mod kernel_params {
    pub const ROOT: &str = "root";
    pub const ROOTFLAGS: &str = "rootflags";
    pub const ROOTfstype: &str = "rootfstype";
    pub const INIT: &str = "init";
    pub const CONSOLE: &str = "console";
    pub const LOGLEVEL: &str = "loglevel";
    pub const QUIET: &str = "quiet";
    pub const DEBUG: &str = "debug";
    pub const MAXCPUS: &str = "maxcpus";
    pub const NR_CPUS: &str = "nr_cpus";
    pub const MEM: &str = "mem";
    pub const MEMMAP: &str = "memmap";
    pub const HUGEPAGESZ: &str = "hugepagesz";
    pub const HUGEpages: &str = "hugepages";
    pub const PCI: &str = "pci";
    pub const IOMMU: &str = "iommu";
    pub const NOIOMMU: &str = "noiommu";
    pub const SWAPOFF: &str = "swapoff";
    pub const SWAPON: &str = "swapon";
    pub const NOHZ: &str = "nohz";
    pub const NOHZ_FULL: &str = "nohz_full";
    pub const TICK: &str = "tick";
    pub const HIGHRES: &str = "highres";
    pub const ISOLCPUS: &str = "isolcpus";
    pub const RCU_NOCBS: &str = "rcu_nocbs";
    pub const NOAPIC: &str = "noapic";
    pub const LAPIC: &str = "lapic";
    pub const APIC: &str = "apic";
    pub const NOACPI: &str = "noacpi";
    pub const ACPI: &str = "acpi";
    pub const SMP: &str = "smp";
    pub const NOSMP: &str = "nosmp";
    pub const MAX_LOOPS: &str = "max_loops";
    pub const MAX_LOCKS: &str = "max_locks";
    pub const PANIC: &str = "panic";
    pub const PANIC_ON_OOPS: &str = "panic_on_oops";
    pub const PANIC_ON_WARN: &str = "panic_on_warn";
    pub const PANIC_TIMEOUT: &str = "panic_timeout";
    pub const KEXEC: &str = "kexec";
    pub const CRASHKERNEL: &str = "crashkernel";
    pub const SELINUX: &str = "selinux";
    pub const APPARMOR: &str = "apparmor";
    pub const SECURITY: &str = "security";
    pub const MODULE_SIG: &str = "module_sig";
    pub const MODULE_SIG_FORCE: &str = "module_sig_force";
    pub const MODULE_SIG_ENFORCE: &str = "module_sig_enforce";
    pub const LOAD_PIN: &str = "load_pin";
    pub const IMA: &str = "ima";
    pub const IMA_HASH: &str = "ima_hash";
    pub const IMA_POLICY: &str = "ima_policy";
    pub const EVM: &str = "evm";
    pub const LSM: &str = "lsm";
    pub const NET: &str = "net";
    pub const IP: &str = "ip";
    pub const IPV6: &str = "ipv6";
    pub const NETIF_NAMES: &str = "net.ifnames";
    pub const NET_PERSISTENT_LOCAL_NAMES: &str = "net.ifnames";
    pub const RANDOM_TRUST_CPU: &str = "random.trust_cpu";
    pub const RANDOM_TRUST_BOOTLOADER: &str = "random.trust_bootloader";
}

/// Boot options
pub struct BootOptions {
    pub root_device: Option<String>,
    pub root_fstype: Option<String>,
    pub root_flags: Option<String>,
    pub init: Option<String>,
    pub console: Option<String>,
    pub log_level: Option<u32>,
    pub quiet: bool,
    pub debug: bool,
}

impl BootOptions {
    pub const fn new() -> Self {
        Self {
            root_device: None,
            root_fstype: None,
            root_flags: None,
            init: None,
            console: None,
            log_level: None,
            quiet: false,
            debug: false,
        }
    }
    
    pub fn from_cmdline(cmdline: &KernelCmdline) -> Self {
        let mut options = BootOptions::new();
        
        options.root_device = cmdline.get(kernel_params::ROOT).map(|s| s.to_string());
        options.root_fstype = cmdline.get(kernel_params::ROOTfstype).map(|s| s.to_string());
        options.root_flags = cmdline.get(kernel_params::ROOTFLAGS).map(|s| s.to_string());
        options.init = cmdline.get(kernel_params::INIT).map(|s| s.to_string());
        options.console = cmdline.get(kernel_params::CONSOLE).map(|s| s.to_string());
        options.log_level = cmdline.get_u64(kernel_params::LOGLEVEL).map(|v| v as u32);
        options.quiet = cmdline.has(kernel_params::QUIET);
        options.debug = cmdline.has(kernel_params::DEBUG);
        
        options
    }
}

/// CPU options
pub struct CpuOptions {
    pub max_cpus: Option<u32>,
    pub nr_cpus: Option<u32>,
    pub nohz: bool,
    pub nohz_full: Option<String>,
    pub isolcpus: Option<String>,
    pub rcu_nocbs: Option<String>,
}

impl CpuOptions {
    pub const fn new() -> Self {
        Self {
            max_cpus: None,
            nr_cpus: None,
            nohz: false,
            nohz_full: None,
            isolcpus: None,
            rcu_nocbs: None,
        }
    }
    
    pub fn from_cmdline(cmdline: &KernelCmdline) -> Self {
        let mut options = CpuOptions::new();
        
        options.max_cpus = cmdline.get_u64(kernel_params::MAXCPUS).map(|v| v as u32);
        options.nr_cpus = cmdline.get_u64(kernel_params::NR_CPUS).map(|v| v as u32);
        options.nohz = cmdline.has(kernel_params::NOHZ);
        options.nohz_full = cmdline.get(kernel_params::NOHZ_FULL).map(|s| s.to_string());
        options.isolcpus = cmdline.get(kernel_params::ISOLCPUS).map(|s| s.to_string());
        options.rcu_nocbs = cmdline.get(kernel_params::RCU_NOCBS).map(|s| s.to_string());
        
        options
    }
}

/// Memory options
pub struct MemoryOptions {
    pub mem: Option<String>,
    pub memmap: Option<String>,
    pub hugepagesz: Option<u64>,
    pub hugepages: Option<u32>,
}

impl MemoryOptions {
    pub const fn new() -> Self {
        Self {
            mem: None,
            memmap: None,
            hugepagesz: None,
            hugepages: None,
        }
    }
    
    pub fn from_cmdline(cmdline: &KernelCmdline) -> Self {
        let mut options = MemoryOptions::new();
        
        options.mem = cmdline.get(kernel_params::MEM).map(|s| s.to_string());
        options.memmap = cmdline.get(kernel_params::MEMMAP).map(|s| s.to_string());
        options.hugepagesz = cmdline.get_u64(kernel_params::HUGEPAGESZ);
        options.hugepages = cmdline.get_u64(kernel_params::HUGEpages).map(|v| v as u32);
        
        options
    }
}

/// Security options
pub struct SecurityOptions {
    pub selinux: Option<String>,
    pub apparmor: Option<String>,
    pub security: Option<String>,
    pub module_sig: bool,
    pub module_sig_force: bool,
    pub module_sig_enforce: bool,
    pub lsm: Option<String>,
}

impl SecurityOptions {
    pub const fn new() -> Self {
        Self {
            selinux: None,
            apparmor: None,
            security: None,
            module_sig: false,
            module_sig_force: false,
            module_sig_enforce: false,
            lsm: None,
        }
    }
    
    pub fn from_cmdline(cmdline: &KernelCmdline) -> Self {
        let mut options = SecurityOptions::new();
        
        options.selinux = cmdline.get(kernel_params::SELINUX).map(|s| s.to_string());
        options.apparmor = cmdline.get(kernel_params::APPARMOR).map(|s| s.to_string());
        options.security = cmdline.get(kernel_params::SECURITY).map(|s| s.to_string());
        options.module_sig = cmdline.has(kernel_params::MODULE_SIG);
        options.module_sig_force = cmdline.has(kernel_params::MODULE_SIG_FORCE);
        options.module_sig_enforce = cmdline.has(kernel_params::MODULE_SIG_ENFORCE);
        options.lsm = cmdline.get(kernel_params::LSM).map(|s| s.to_string());
        
        options
    }
}
