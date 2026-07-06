//! SigmaOS System Monitor (htop/Glances Alternative)
//! Native system monitor reducing dependency on htop, Glances, top
//! Provides CPU, memory, disk, network, and process monitoring

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaF32 = f32;
type SigmaF64 = f64;
type SigmaBool = bool;
type SigmaUsize = usize;

/// Process state
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProcessState {
    Running = 0,
    Sleeping = 1,
    Stopped = 2,
    Zombie = 3,
}

/// Sort field
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SortField {
    PID = 0,
    Name = 1,
    CPU = 2,
    Memory = 3,
    Time = 4,
}

/// CPU info
#[repr(C)]
pub struct CPUInfo {
    pub cpu_id: SigmaU32,
    pub usage_percent: SigmaF32,
    pub user_percent: SigmaF32,
    pub system_percent: SigmaF32,
    pub idle_percent: SigmaF32,
    pub frequency_mhz: SigmaU32,
}

/// Memory info
#[repr(C)]
pub struct MemoryInfo {
    pub total_mb: SigmaU32,
    pub used_mb: SigmaU32,
    pub free_mb: SigmaU32,
    pub available_mb: SigmaU32,
    pub cached_mb: SigmaU32,
    pub swap_total_mb: SigmaU32,
    pub swap_used_mb: SigmaU32,
}

/// Disk info
#[repr(C)]
pub struct DiskInfo {
    pub device: [SigmaU8; 64],
    pub mount_point: [SigmaU8; 128],
    pub total_gb: SigmaU32,
    pub used_gb: SigmaU32,
    pub free_gb: SigmaU32,
    pub usage_percent: SigmaF32,
}

/// Network info
#[repr(C)]
pub struct NetworkInfo {
    pub interface: [SigmaU8; 32],
    pub bytes_sent: SigmaU64,
    pub bytes_recv: SigmaU64,
    pub packets_sent: SigmaU64,
    pub packets_recv: SigmaU64,
    pub upload_speed: SigmaU32,
    pub download_speed: SigmaU32,
}

/// Process info
#[repr(C)]
pub struct ProcessInfo {
    pub pid: SigmaU32,
    pub name: [SigmaU8; 128],
    pub user: [SigmaU8; 64],
    pub cpu_percent: SigmaF32,
    pub memory_mb: SigmaU32,
    pub state: ProcessState,
    pub threads: SigmaU32,
    pub time: SigmaU64,
}

/// System monitor
#[repr(C)]
pub struct SystemMonitor {
    pub cpus: *mut CPUInfo,
    pub cpu_count: SigmaU32,
    pub memory: MemoryInfo,
    pub disks: *mut DiskInfo,
    pub disk_count: SigmaU32,
    pub networks: *mut NetworkInfo,
    pub network_count: SigmaU32,
    pub processes: *mut ProcessInfo,
    pub process_count: SigmaU32,
    pub sort_field: SortField,
    pub sort_descending: SigmaBool,
    pub initialized: SigmaBool,
}

static mut SYSTEM_MONITOR: Option<SystemMonitor> = None;

/// Initialize system monitor
#[no_mangle]
pub unsafe extern "C" fn monitor_init() -> SigmaI32 {
    SYSTEM_MONITOR = Some(SystemMonitor {
        cpus: 0 as *mut CPUInfo,
        cpu_count: 0,
        memory: MemoryInfo {
            total_mb: 0,
            used_mb: 0,
            free_mb: 0,
            available_mb: 0,
            cached_mb: 0,
            swap_total_mb: 0,
            swap_used_mb: 0,
        },
        disks: 0 as *mut DiskInfo,
        disk_count: 0,
        networks: 0 as *mut NetworkInfo,
        network_count: 0,
        processes: 0 as *mut ProcessInfo,
        process_count: 0,
        sort_field: SortField::CPU,
        sort_descending: true,
        initialized: false,
    });

    if let Some(monitor) -> &mut SYSTEM_MONITOR {
        monitor.initialized = true;
        return 0;
    }

    -1
}

/// Update CPU info
#[no_mangle]
pub unsafe extern "C" fn monitor_update_cpu() -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() {
        return -1;
    }

    if let Some(monitor) -> &mut SYSTEM_MONITOR {
        // In real implementation, update CPU info
        return 0;
    }

    -1
}

/// Get CPU count
#[no_mangle]
pub unsafe extern "C" fn monitor_get_cpu_count() -> SigmaU32 {
    if let Some(monitor) = &SYSTEM_MONITOR {
        monitor.cpu_count
    } else {
        0
    }
}

/// List CPUs
#[no_mangle]
pub unsafe extern "C" fn monitor_list_cpus(
    cpus: *mut CPUInfo,
    max_cpus: SigmaU32,
    cpu_count: *mut SigmaU32,
) -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() || cpus.is_null() || cpu_count.is_null() {
        return -1;
    }

    if let Some(monitor) -> &SYSTEM_MONITOR {
        *cpu_count = monitor.cpu_count;
        return 0;
    }

    -1
}

/// Update memory info
#[no_mangle]
pub unsafe extern "C" fn monitor_update_memory() -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() {
        return -1;
    }

    if let Some(monitor) -> &mut SYSTEM_MONITOR {
        // In real implementation, update memory info
        return 0;
    }

    -1
}

/// Get memory info
#[no_mangle]
pub unsafe extern "C" fn monitor_get_memory(memory: *mut MemoryInfo) -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() || memory.is_null() {
        return -1;
    }

    if let Some(monitor) -> &SYSTEM_MONITOR {
        *memory = monitor.memory;
        return 0;
    }

    -1
}

/// Update disk info
#[no_mangle]
pub unsafe extern "C" fn monitor_update_disks() -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() {
        return -1;
    }

    if let Some(monitor) -> &mut SYSTEM_MONITOR {
        // In real implementation, update disk info
        return 0;
    }

    -1
}

/// List disks
#[no_mangle]
pub unsafe extern "C" fn monitor_list_disks(
    disks: *mut DiskInfo,
    max_disks: SigmaU32,
    disk_count: *mut SigmaU32,
) -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() || disks.is_null() || disk_count.is_null() {
        return -1;
    }

    if let Some(monitor) -> &SYSTEM_MONITOR {
        *disk_count = monitor.disk_count;
        return 0;
    }

    -1
}

/// Update network info
#[no_mangle]
pub unsafe extern "C" fn monitor_update_network() -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() {
        return -1;
    }

    if let Some(monitor) -> &mut SYSTEM_MONITOR {
        // In real implementation, update network info
        return 0;
    }

    -1
}

/// List network interfaces
#[no_mangle]
pub unsafe extern "C" fn monitor_list_networks(
    networks: *mut NetworkInfo,
    max_networks: SigmaU32,
    network_count: *mut SigmaU32,
) -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() || networks.is_null() || network_count.is_null() {
        return -1;
    }

    if let Some(monitor) -> &SYSTEM_MONITOR {
        *network_count = monitor.network_count;
        return 0;
    }

    -1
}

/// Update process list
#[no_mangle]
pub unsafe extern "C" fn monitor_update_processes() -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() {
        return -1;
    }

    if let Some(monitor) -> &mut SYSTEM_MONITOR {
        // In real implementation, update process list
        return 0;
    }

    -1
}

/// List processes
#[no_mangle]
pub unsafe extern "C" fn monitor_list_processes(
    processes: *mut ProcessInfo,
    max_processes: SigmaU32,
    process_count: *mut SigmaU32,
) -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() || processes.is_null() || process_count.is_null() {
        return -1;
    }

    if let Some(monitor) -> &SYSTEM_MONITOR {
        *process_count = monitor.process_count;
        return 0;
    }

    -1
}

/// Kill process
#[no_mangle]
pub unsafe extern "C" fn monitor_kill_process(pid: SigmaU32) -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() {
        return -1;
    }

    // In real implementation, kill process
    0
}

/// Set sort field
#[no_mangle]
pub unsafe extern "C" fn monitor_set_sort_field(field: SortField) -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() {
        return -1;
    }

    if let Some(monitor) -> &mut SYSTEM_MONITOR {
        monitor.sort_field = field;
        return 0;
    }

    -1
}

/// Get sort field
#[no_mangle]
pub unsafe extern "C" fn monitor_get_sort_field() -> SortField {
    if let Some(monitor) = &SYSTEM_MONITOR {
        monitor.sort_field
    } else {
        SortField::CPU
    }
}

/// Set sort direction
#[no_mangle]
pub unsafe extern "C" fn monitor_set_sort_descending(descending: SigmaBool) -> SigmaI32 {
    if SYSTEM_MONITOR.is_none() {
        return -1;
    }

    if let Some(monitor) -> &mut SYSTEM_MONITOR {
        monitor.sort_descending = descending;
        return 0;
    }

    -1
}

/// Get sort direction
#[no_mangle]
pub unsafe extern "C" fn monitor_get_sort_descending() -> SigmaBool {
    if let Some(monitor) = &SYSTEM_MONITOR {
        monitor.sort_descending
    } else {
        true
    }
}

/// Get process count
#[no_mangle]
pub unsafe extern "C" fn monitor_get_process_count() -> SigmaU32 {
    if let Some(monitor) = &SYSTEM_MONITOR {
        monitor.process_count
    } else {
        0
    }
}

/// Get disk count
#[no_mangle]
pub unsafe extern "C" fn monitor_get_disk_count() -> SigmaU32 {
    if let Some(monitor) = &SYSTEM_MONITOR {
        monitor.disk_count
    } else {
        0
    }
}

/// Get network count
#[no_mangle]
pub unsafe extern "C" fn monitor_get_network_count() -> SigmaU32 {
    if let Some(monitor) = &SYSTEM_MONITOR {
        monitor.network_count
    } else {
        0
    }
}

/// Check if system monitor is initialized
#[no_mangle]
pub unsafe extern "C" fn monitor_initialized() -> SigmaBool {
    if let Some(monitor) = &SYSTEM_MONITOR {
        monitor.initialized
    } else {
        false
    }
}

/// Helper: Copy string
unsafe fn copy_str(dest: *mut SigmaU8, src: *const SigmaU8, max_len: usize) {
    if dest.is_null() || src.is_null() {
        return;
    }
    let mut i = 0;
    while i < max_len - 1 && *src.add(i) != 0 {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    *dest.add(i) = 0;
}

/// Helper: Get string length
unsafe fn str_len(s: *const SigmaU8) -> usize {
    if s.is_null() {
        return 0;
    }
    let mut len = 0;
    while *s.add(len) != 0 && len < 512 {
        len += 1;
    }
    len
}
