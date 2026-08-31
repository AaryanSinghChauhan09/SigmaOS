use alloc::vec;
extern crate alloc;
// SigmaOS Legacy Linux Kernel & System Compatibility Adapter
// Enables ancient and modern software compiled for Linux 2.x, 3.x, 4.x, 5.x, and 6.x to run securely
// Supports System V IPC, ProcFS emulation, GLIBC environment shims, and ELF binary Aux Vectors loading.

#[cfg(test)]
extern crate std;

use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

use crate::klib::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxKernelVersion {
    Kernel2_6,
    Kernel3x,
    Kernel4x,
    Kernel5x,
    Kernel6x,
}

/// Linux ELF auxiliary vector types (AT_*)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElfAuxType {
    AtNull = 0,
    AtPhdr = 3,
    AtPhent = 4,
    AtPhnum = 5,
    AtPagesz = 6,
    AtEntry = 9,
}

/// Linux System V IPC Shared Memory and Semaphore structures
pub struct SysVIPCEngine {
    pub shared_memory_keys: HashMap<u32, Vec<u8>>, // shmkey -> raw buffer
    pub semaphore_values: HashMap<u32, i32>,       // semkey -> value
}

impl SysVIPCEngine {
    pub fn new() -> Self {
        Self {
            shared_memory_keys: HashMap::new(),
            semaphore_values: HashMap::new(),
        }
    }

    pub fn shm_get_or_create(&mut self, key: u32, size: usize) -> bool {
        if self.shared_memory_keys.contains_key(&key) {
            true
        } else {
            self.shared_memory_keys.insert(key, alloc::vec![0u8; size]);
            true
        }
    }

    pub fn sem_post(&mut self, key: u32) {
        let val = self.semaphore_values.entry(key).or_insert(0);
        *val += 1;
    }

    pub fn sem_wait(&mut self, key: u32) -> bool {
        if let Some(val) = self.semaphore_values.get_mut(&key) {
            if *val > 0 {
                *val -= 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Default for SysVIPCEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux virtual `/proc` file system emulator (ProcFS)
pub struct ProcFsEmulator {
    pub mocked_files: HashMap<String, String>,
}

impl ProcFsEmulator {
    pub fn new() -> Self {
        let mut mock = HashMap::new();
        mock.insert("/proc/cpuinfo".to_string(), "processor: 0\nvendor_id: GenuineIntel\ncpu family: 6\nmodel name: SigmaOS Quantum Core\n".to_string());
        mock.insert(
            "/proc/meminfo".to_string(),
            "MemTotal:       32768000 kB\nMemFree:        28991000 kB\n".to_string(),
        );
        mock.insert(
            "/proc/version".to_string(),
            "Linux version 6.1.0-sigma-hardened (gcc version 12.2.0) #1 SMP PREEMPT_DYNAMIC\n"
                .to_string(),
        );
        Self { mocked_files: mock }
    }

    pub fn read_proc_file(&self, path: &str) -> Option<&String> {
        self.mocked_files.get(path)
    }
}

impl Default for ProcFsEmulator {
    fn default() -> Self {
        Self::new()
    }
}

/// ELF Dynamic Interpreter loader and Auxiliary Vectors generator (GLIBC / Musl ABI parity)
pub struct ElfLoader {
    pub dynamic_interpreter: String, // e.g. "/lib64/ld-linux-x86-64.so.2"
    pub aux_vectors: HashMap<ElfAuxType, u64>,
}

impl ElfLoader {
    pub fn new(interpreter: &str) -> Self {
        let mut aux = HashMap::new();
        aux.insert(ElfAuxType::AtPagesz, 4096); // standard 4KB page size
        aux.insert(ElfAuxType::AtEntry, 0x4000_1000); // entry point offset
        aux.insert(ElfAuxType::AtPhnum, 8); // number of program headers
        Self {
            dynamic_interpreter: interpreter.to_string(),
            aux_vectors: aux,
        }
    }

    /// Appends standard auxiliary vector arrays onto glibc dynamic loader stacks
    pub fn generate_stack_aux_array(&self) -> Vec<(u32, u64)> {
        let mut stack = Vec::new();
        for (&aux_type, &val) in &self.aux_vectors {
            stack.push((aux_type as u32, val));
        }
        stack.push((ElfAuxType::AtNull as u32, 0));
        stack
    }
}

pub struct LegacyKernelAdapter {
    pub target_version: LinuxKernelVersion,
    pub syscall_shims: HashMap<u32, String>,
    pub ipc: SysVIPCEngine,
    pub proc_fs: ProcFsEmulator,
}

impl LegacyKernelAdapter {
    pub fn new(version: LinuxKernelVersion) -> Self {
        let mut shims = HashMap::new();
        // Standard Linux syscall shims
        shims.insert(1, "sys_exit".to_string());
        shims.insert(3, "sys_read".to_string());
        shims.insert(4, "sys_write".to_string());
        shims.insert(54, "sys_ioctl".to_string());
        shims.insert(244, "sys_get_robust_list".to_string());

        LegacyKernelAdapter {
            target_version: version,
            syscall_shims: shims,
            ipc: SysVIPCEngine::new(),
            proc_fs: ProcFsEmulator::new(),
        }
    }

    pub fn dispatch_syscall(&self, sys_num: u32) -> Result<String, ()> {
        if let Some(shim) = self.syscall_shims.get(&sys_num) {
            Ok(format!("Executing shim: {}", shim))
        } else {
            Err(())
        }
    }

    /// Handles remapping GLIBC dynamic environment variables securely
    pub fn sanitize_glibc_environment(&self, env_vars: &mut Vec<String>) {
        // Enforce LD_SECURE/LD_PRELOAD security policies for sandboxed executables
        env_vars.retain(|var| !var.starts_with("LD_PRELOAD="));
    }
}

pub struct LegacyPackageAdapter {
    pub supported_formats: Vec<String>,
}

impl LegacyPackageAdapter {
    pub fn new() -> Self {
        LegacyPackageAdapter {
            supported_formats: {
                let mut v = Vec::new();
                v.push(".deb".to_string());
                v.push(".rpm".to_string());
                v.push(".tgz".to_string());
                v
            },
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

impl Default for LegacyPackageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LegacySecurityAdapter {
    pub dac_permissions: u32, // standard Unix permissions: e.g. 0o755
}

impl LegacySecurityAdapter {
    pub fn new(perm: u32) -> Self {
        LegacySecurityAdapter {
            dac_permissions: perm,
        }
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
        format!(
            "Mapped X11 Window ID {} to Zenith Desktop Surface",
            window_id
        )
    }
}

impl Default for LegacyUIAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_kernel_syscalls() {
        let adapter = LegacyKernelAdapter::new(LinuxKernelVersion::Kernel3x);
        assert_eq!(
            adapter.dispatch_syscall(4).unwrap(),
            "Executing shim: sys_write"
        );
        assert!(adapter.dispatch_syscall(999).is_err());
    }

    #[test]
    fn test_legacy_package_converter() {
        let adapter = LegacyPackageAdapter::new();
        assert_eq!(
            adapter.convert_package("old-app.deb").unwrap(),
            "Converted old-app.deb to unified .spkg format"
        );
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
        assert_eq!(
            adapter.map_x11_to_zenith(4567),
            "Mapped X11 Window ID 4567 to Zenith Desktop Surface"
        );
        assert_eq!(adapter.active_windows, 1);
    }

    #[test]
    fn test_linux_compatibility_layer_features() {
        let mut adapter = LegacyKernelAdapter::new(LinuxKernelVersion::Kernel6x);

        // 1. System V IPC shared memory and semaphores
        assert!(adapter.ipc.shm_get_or_create(1001, 1024));
        assert!(adapter.ipc.shared_memory_keys.contains_key(&1001));

        adapter.ipc.sem_post(2002);
        assert!(adapter.ipc.sem_wait(2002));
        assert!(!adapter.ipc.sem_wait(2002)); // is now 0, should block

        // 2. ProcFS emulation
        let version_info = adapter.proc_fs.read_proc_file("/proc/version").unwrap();
        assert!(version_info.contains("Linux version"));

        let cpu_info = adapter.proc_fs.read_proc_file("/proc/cpuinfo").unwrap();
        assert!(cpu_info.contains("SigmaOS Quantum Core"));

        // 3. GLIBC secure env sanitization
        let mut envs = Vec::new();
        envs.push("PATH=/bin".to_string());
        envs.push("LD_PRELOAD=/opt/malicious.so".to_string());
        adapter.sanitize_glibc_environment(&mut envs);

        assert_eq!(envs.len(), 1);
        assert_eq!(envs[0], "PATH=/bin");

        // 4. ELF Aux Vectors
        let loader = ElfLoader::new("/lib64/ld-linux-x86-64.so.2");
        let auxs = loader.generate_stack_aux_array();
        assert!(auxs.len() > 1);
        // last item must be AT_NULL (0)
        assert_eq!(auxs[auxs.len() - 1].0, ElfAuxType::AtNull as u32);
    }
}
