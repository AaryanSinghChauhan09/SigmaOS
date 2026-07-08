/// SigmaOS: Cgroup Implementation
/// CPU/Memory/IO enforcement for containers
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CgroupShard {
    pub id: SigmaU32,
    pub name: [u8; 40],
    pub active: SigmaBool,
    pub cpu_quota_us: SigmaU32,
    pub cpu_period_us: SigmaU32,
    pub cpu_runtime_us: SigmaU64,
    pub cpu_throttle_count: SigmaU64,
    pub mem_limit_bytes: SigmaU64,
    pub mem_current_bytes: SigmaU64,
    pub mem_peak_bytes: SigmaU64,
    pub oom_kill_count: SigmaU64,
    pub io_weight: SigmaU32,
    pub io_bytes_written: SigmaU64,
    pub io_bytes_read: SigmaU64,
    pub io_throttle_count: SigmaU64,
    pub pid_count: SigmaU32,
}

pub struct CgroupManager {
    pub cgroups: [CgroupShard; 128],
    pub cgroup_count: SigmaU32,
}

impl CgroupManager {
    pub const fn new() -> Self {
        Self {
            cgroups: [CgroupShard {
                id: 0,
                name: [0; 40],
                active: false,
                cpu_quota_us: 0,
                cpu_period_us: 0,
                cpu_runtime_us: 0,
                cpu_throttle_count: 0,
                mem_limit_bytes: 0,
                mem_current_bytes: 0,
                mem_peak_bytes: 0,
                oom_kill_count: 0,
                io_weight: 0,
                io_bytes_written: 0,
                io_bytes_read: 0,
                io_throttle_count: 0,
                pid_count: 0,
            }; 128],
            cgroup_count: 0,
        }
    }

    pub unsafe fn create_cgroup(&mut self, name: *const u8, cpu_quota: SigmaU32, mem_limit: SigmaU64) -> i32 {
        if self.cgroup_count >= 128 {
            return -1;
        }

        let idx = self.cgroup_count as usize;
        self.cgroups[idx].id = idx as SigmaU32 + 1;
        
        for i in 0..40 {
            self.cgroups[idx].name[i] = *name.add(i);
        }
        
        self.cgroups[idx].active = true;
        self.cgroups[idx].cpu_quota_us = cpu_quota;
        self.cgroups[idx].cpu_period_us = 100000;
        self.cgroups[idx].mem_limit_bytes = mem_limit;
        self.cgroups[idx].io_weight = 100;
        
        self.cgroup_count += 1;
        idx as i32
    }

    pub unsafe fn enforce_cpu(&mut self, cgroup_id: SigmaU32, runtime_us: SigmaU64) -> bool {
        for i in 0..self.cgroup_count as usize {
            if self.cgroups[i].id == cgroup_id && self.cgroups[i].active {
                self.cgroups[i].cpu_runtime_us += runtime_us;
                
                if self.cgroups[i].cpu_runtime_us >= self.cgroups[i].cpu_quota_us as SigmaU64 {
                    self.cgroups[i].cpu_throttle_count += 1;
                    self.cgroups[i].cpu_runtime_us = 0;
                    return true;
                }
                return false;
            }
        }
        false
    }

    pub unsafe fn enforce_memory(&mut self, cgroup_id: SigmaU32, alloc_bytes: SigmaU64) -> bool {
        for i in 0..self.cgroup_count as usize {
            if self.cgroups[i].id == cgroup_id && self.cgroups[i].active {
                self.cgroups[i].mem_current_bytes += alloc_bytes;
                
                if self.cgroups[i].mem_current_bytes > self.cgroups[i].mem_peak_bytes {
                    self.cgroups[i].mem_peak_bytes = self.cgroups[i].mem_current_bytes;
                }
                
                if self.cgroups[i].mem_current_bytes >= self.cgroups[i].mem_limit_bytes {
                    self.cgroups[i].oom_kill_count += 1;
                    return true;
                }
                return false;
            }
        }
        false
    }

    pub unsafe fn enforce_io(&mut self, cgroup_id: SigmaU32, bytes: SigmaU64, is_write: bool) -> bool {
        for i in 0..self.cgroup_count as usize {
            if self.cgroups[i].id == cgroup_id && self.cgroups[i].active {
                if is_write {
                    self.cgroups[i].io_bytes_written += bytes;
                } else {
                    self.cgroups[i].io_bytes_read += bytes;
                }
                
                let weight = self.cgroups[i].io_weight;
                if weight < 10 {
                    self.cgroups[i].io_throttle_count += 1;
                    return true;
                }
                return false;
            }
        }
        false
    }

    pub unsafe fn reset_cpu_runtime(&mut self, cgroup_id: SigmaU32) {
        for i in 0..self.cgroup_count as usize {
            if self.cgroups[i].id == cgroup_id {
                self.cgroups[i].cpu_runtime_us = 0;
            }
        }
    }

    pub unsafe fn release_memory(&mut self, cgroup_id: SigmaU32, bytes: SigmaU64) {
        for i in 0..self.cgroup_count as usize {
            if self.cgroups[i].id == cgroup_id {
                if self.cgroups[i].mem_current_bytes >= bytes {
                    self.cgroups[i].mem_current_bytes -= bytes;
                } else {
                    self.cgroups[i].mem_current_bytes = 0;
                }
            }
        }
    }
}

static mut CGROUP_MANAGER: CgroupManager = CgroupManager::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_init() {
    CGROUP_MANAGER.cgroup_count = 0;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_create(name: *const u8, cpu_quota: SigmaU32, mem_limit: SigmaU64) -> i32 {
    CGROUP_MANAGER.create_cgroup(name, cpu_quota, mem_limit)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_enforce_cpu(cgroup_id: SigmaU32, runtime_us: SigmaU64) -> bool {
    CGROUP_MANAGER.enforce_cpu(cgroup_id, runtime_us)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_enforce_memory(cgroup_id: SigmaU32, alloc_bytes: SigmaU64) -> bool {
    CGROUP_MANAGER.enforce_memory(cgroup_id, alloc_bytes)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_enforce_io(cgroup_id: SigmaU32, bytes: SigmaU64, is_write: bool) -> bool {
    CGROUP_MANAGER.enforce_io(cgroup_id, bytes, is_write)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_release_memory(cgroup_id: SigmaU32, bytes: SigmaU64) {
    CGROUP_MANAGER.release_memory(cgroup_id, bytes)
}
