/// SigmaOS: Capability-Based Security System
/// Phase G Blocker Resolution: Capability-Token Delegation System
/// Inspired by seL4, Genode, and OpenBSD pledge/unveil
/// 
/// This implements a capability-based security model where processes receive
/// immutable capability token sets at spawn time, with delegation via IPC.

#[allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Capability Rights & Permissions ────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CapabilityRight {
    // Process capabilities
    CapProcessSpawn = 1 << 0,
    CapProcessSignal = 1 << 1,
    CapProcessDebug = 1 << 2,
    CapProcessKill = 1 << 3,
    
    // Memory capabilities
    CapMemRead = 1 << 4,
    CapMemWrite = 1 << 5,
    CapMemExecute = 1 << 6,
    CapMemMap = 1 << 7,
    
    // I/O capabilities
    CapIoRead = 1 << 8,
    CapIoWrite = 1 << 9,
    CapIoControl = 1 << 10,
    
    // Network capabilities
    CapNetSocket = 1 << 11,
    CapNetBind = 1 << 12,
    CapNetConnect = 1 << 13,
    
    // Filesystem capabilities
    CapFsRead = 1 << 14,
    CapFsWrite = 1 << 15,
    CapFsCreate = 1 << 16,
    CapFsDelete = 1 << 17,
    
    // Device capabilities
    CapDeviceAccess = 1 << 18,
    CapDeviceConfigure = 1 << 19,
    
    // System capabilities
    CapSysTime = 1 << 20,
    CapSysReboot = 1 << 21,
    CapSysShutdown = 1 << 22,
    
    // IPC capabilities
    CapIpcSend = 1 << 23,
    CapIpcReceive = 1 << 24,
    CapIpcDelegate = 1 << 25,
}

// ─── Capability Token Structure ───────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CapabilityToken {
    pub cap_id: SigmaU64,           // Unique capability identifier
    pub rights: SigmaU32,           // Bitmask of CapabilityRight
    pub owner_pid: SigmaU64,        // Process that owns this capability
    pub ref_count: SigmaU32,        // Reference count for delegation
    pub delegatable: SigmaBool,     // Whether this cap can be delegated
    pub revoked: SigmaBool,         // Whether this cap has been revoked
    pub expiry_time: SigmaU64,      // Optional expiry timestamp (0 = never)
}

// ─── Process Capability Set ─────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProcessCapabilitySet {
    pub pid: SigmaU64,               // Process ID
    pub cap_count: SigmaUsize,      // Number of capabilities
    pub caps: [SigmaU64; 64],       // Array of capability IDs (max 64)
    pub parent_pid: SigmaU64,       // Parent process for inheritance
}

// ─── Capability Delegation Request ───────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CapabilityDelegationRequest {
    pub source_pid: SigmaU64,       // Process delegating the capability
    pub target_pid: SigmaU64,       // Process receiving the capability
    pub cap_id: SigmaU64,           // Capability being delegated
    pub rights_mask: SigmaU32,      // Rights to delegate (subset of original)
    pub request_id: SigmaU64,       // Unique request identifier
}

// ─── Capability Manager ────────────────────────────────────────────────────

pub const MAX_CAPABILITIES: usize = 4096;
pub const MAX_PROCESSES: usize = 256;

pub struct CapabilityManager {
    initialized: SigmaBool,
    next_cap_id: SigmaU64,
    capabilities: [Option<CapabilityToken>; MAX_CAPABILITIES],
    process_caps: [Option<ProcessCapabilitySet>; MAX_PROCESSES],
    delegation_queue: [Option<CapabilityDelegationRequest>; 128],
    delegation_queue_head: SigmaUsize,
    delegation_queue_tail: SigmaUsize,
}

impl CapabilityManager {
    pub const fn new() -> Self {
        Self {
            initialized: false,
            next_cap_id: 1,
            capabilities: [None; MAX_CAPABILITIES],
            process_caps: [None; MAX_PROCESSES],
            delegation_queue: [None; 128],
            delegation_queue_head: 0,
            delegation_queue_tail: 0,
        }
    }

    /// Initialize the capability manager
    pub unsafe fn init(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Err("Capability manager already initialized");
        }

        // Clear all data structures
        for i in 0..MAX_CAPABILITIES {
            self.capabilities[i] = None;
        }

        for i in 0..MAX_PROCESSES {
            self.process_caps[i] = None;
        }

        for i in 0..128 {
            self.delegation_queue[i] = None;
        }

        self.next_cap_id = 1;
        self.delegation_queue_head = 0;
        self.delegation_queue_tail = 0;
        self.initialized = true;

        Ok(())
    }

    /// Create a new capability token
    pub unsafe fn create_capability(
        &mut self,
        owner_pid: SigmaU64,
        rights: SigmaU32,
        delegatable: SigmaBool,
        expiry_time: SigmaU64,
    ) -> Result<SigmaU64, &'static str> {
        if !self.initialized {
            return Err("Capability manager not initialized");
        }

        // Find free slot
        let slot = match self.find_free_cap_slot() {
            Some(slot) => slot,
            None => return Err("No free capability slots"),
        };

        let cap_id = self.next_cap_id;
        self.next_cap_id += 1;

        let token = CapabilityToken {
            cap_id,
            rights,
            owner_pid,
            ref_count: 1,
            delegatable,
            revoked: false,
            expiry_time,
        };

        self.capabilities[slot] = Some(token);

        Ok(cap_id)
    }

    /// Grant capability to a process
    pub unsafe fn grant_capability(
        &mut self,
        pid: SigmaU64,
        cap_id: SigmaU64,
    ) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Capability manager not initialized");
        }

        // Find capability
        let cap_slot = match self.find_capability_slot(cap_id) {
            Some(slot) => slot,
            None => return Err("Capability not found"),
        };

        let cap = match self.capabilities[cap_slot] {
            Some(cap) => cap,
            None => return Err("Capability not found"),
        };

        if cap.revoked {
            return Err("Capability has been revoked");
        }

        // Check expiry
        if cap.expiry_time != 0 {
            let current_time = self.get_timestamp();
            if current_time > cap.expiry_time {
                return Err("Capability has expired");
            }
        }

        // Find or create process capability set
        let pid_usize = (pid % MAX_PROCESSES as SigmaU64) as usize;
        if self.process_caps[pid_usize].is_none() {
            self.process_caps[pid_usize] = Some(ProcessCapabilitySet {
                pid,
                cap_count: 0,
                caps: [0; 64],
                parent_pid: 0,
            });
        }

        let proc_caps = self.process_caps[pid_usize].as_mut().unwrap();

        // Check if already has this capability
        if proc_caps.cap_count < 64 {
            for i in 0..proc_caps.cap_count {
                if proc_caps.caps[i] == cap_id {
                    return Ok(()); // Already has capability
                }
            }

            // Add capability
            proc_caps.caps[proc_caps.cap_count] = cap_id;
            proc_caps.cap_count += 1;

            // Increment reference count
            if let Some(ref mut cap_token) = self.capabilities[cap_slot] {
                cap_token.ref_count += 1;
            }
        } else {
            return Err("Process capability limit reached");
        }

        Ok(())
    }

    /// Revoke capability from a process
    pub unsafe fn revoke_capability(
        &mut self,
        pid: SigmaU64,
        cap_id: SigmaU64,
    ) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Capability manager not initialized");
        }

        let pid_usize = (pid % MAX_PROCESSES as SigmaU64) as usize;
        let proc_caps = match self.process_caps[pid_usize].as_mut() {
            Some(caps) => caps,
            None => return Err("Process capability set not found"),
        };

        // Find and remove capability
        let mut found = false;
        for i in 0..proc_caps.cap_count {
            if proc_caps.caps[i] == cap_id {
                // Shift remaining capabilities
                for j in i..proc_caps.cap_count - 1 {
                    proc_caps.caps[j] = proc_caps.caps[j + 1];
                }
                proc_caps.cap_count -= 1;
                found = true;
                break;
            }
        }

        if !found {
            return Err("Capability not found in process set");
        }

        // Decrement reference count
        let cap_slot = match self.find_capability_slot(cap_id) {
            Some(slot) => slot,
            None => return Err("Capability not found"),
        };

        if let Some(ref mut cap_token) = self.capabilities[cap_slot] {
            if cap_token.ref_count > 0 {
                cap_token.ref_count -= 1;
            }
        }

        Ok(())
    }

    /// Check if process has specific capability right
    pub unsafe fn check_capability(
        &mut self,
        pid: SigmaU64,
        required_right: CapabilityRight,
    ) -> SigmaBool {
        if !self.initialized {
            return false;
        }

        let pid_usize = (pid % MAX_PROCESSES as SigmaU64) as usize;
        let proc_caps = match self.process_caps[pid_usize].as_ref() {
            Some(caps) => caps,
            None => return false,
        };

        let required_bit = required_right as SigmaU32;

        // Check all capabilities for the required right
        for i in 0..proc_caps.cap_count {
            let cap_id = proc_caps.caps[i];
            if let Some(cap_slot) = self.find_capability_slot(cap_id) {
                if let Some(cap) = self.capabilities[cap_slot] {
                    if !cap.revoked && (cap.rights & required_bit) != 0 {
                        // Check expiry
                        if cap.expiry_time == 0 || self.get_timestamp() <= cap.expiry_time {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    /// Request capability delegation
    pub unsafe fn request_delegation(
        &mut self,
        request: CapabilityDelegationRequest,
    ) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Capability manager not initialized");
        }

        // Validate source has the capability
        if !self.check_capability(request.source_pid, CapabilityRight::CapIpcDelegate) {
            return Err("Source process lacks delegation capability");
        }

        // Validate capability exists and is delegatable
        let cap_slot = match self.find_capability_slot(request.cap_id) {
            Some(slot) => slot,
            None => return Err("Capability not found"),
        };

        let cap = match self.capabilities[cap_slot] {
            Some(cap) => cap,
            None => return Err("Capability not found"),
        };

        if !cap.delegatable {
            return Err("Capability is not delegatable");
        }

        if cap.owner_pid != request.source_pid {
            return Err("Source process does not own this capability");
        }

        // Add to delegation queue
        let queue_idx = self.delegation_queue_tail;
        self.delegation_queue[queue_idx] = Some(request);
        self.delegation_queue_tail = (self.delegation_queue_tail + 1) % 128;

        Ok(())
    }

    /// Process pending delegation requests
    pub unsafe fn process_delegations(&mut self) -> Result<SigmaUsize, &'static str> {
        if !self.initialized {
            return Err("Capability manager not initialized");
        }

        let mut processed = 0;

        while self.delegation_queue_head != self.delegation_queue_tail {
            let queue_idx = self.delegation_queue_head;
            let request = match self.delegation_queue[queue_idx] {
                Some(req) => req,
                None => break,
            };

            // Create new capability for target with subset of rights
            let cap_slot = match self.find_capability_slot(request.cap_id) {
                Some(slot) => slot,
                None => {
                    self.delegation_queue_head = (self.delegation_queue_head + 1) % 128;
                    continue;
                }
            };

            let original_cap = match self.capabilities[cap_slot] {
                Some(cap) => cap,
                None => {
                    self.delegation_queue_head = (self.delegation_queue_head + 1) % 128;
                    continue;
                }
            };

            // Create delegated capability with subset of rights
            let delegated_rights = original_cap.rights & request.rights_mask;
            if delegated_rights == 0 {
                self.delegation_queue_head = (self.delegation_queue_head + 1) % 128;
                continue;
            }

            match self.create_capability(
                request.target_pid,
                delegated_rights,
                original_cap.delegatable,
                original_cap.expiry_time,
            ) {
                Ok(new_cap_id) => {
                    // Grant to target process
                    let _ = self.grant_capability(request.target_pid, new_cap_id);
                    processed += 1;
                }
                Err(_) => {}
            }

            self.delegation_queue[queue_idx] = None;
            self.delegation_queue_head = (self.delegation_queue_head + 1) % 128;
        }

        Ok(processed)
    }

    /// Revoke all capabilities for a process (on process exit)
    pub unsafe fn revoke_all_capabilities(&mut self, pid: SigmaU64) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Capability manager not initialized");
        }

        let pid_usize = (pid % MAX_PROCESSES as SigmaU64) as usize;
        
        // Collect capability IDs first to avoid borrowing issues
        let mut caps_to_revoke = [0u64; 64];
        let mut cap_count = 0;
        
        if let Some(proc_caps) = self.process_caps[pid_usize].as_ref() {
            for i in 0..proc_caps.cap_count {
                if i < 64 {
                    caps_to_revoke[i] = proc_caps.caps[i];
                    cap_count += 1;
                }
            }
        }

        // Revoke each capability
        for i in 0..cap_count {
            let cap_id = caps_to_revoke[i];
            let _ = self.revoke_capability(pid, cap_id);
        }

        // Clear process capability set
        self.process_caps[pid_usize] = None;

        Ok(())
    }

    /// Find free capability slot
    fn find_free_cap_slot(&self) -> Option<usize> {
        for i in 0..MAX_CAPABILITIES {
            if self.capabilities[i].is_none() {
                return Some(i);
            }
        }
        None
    }

    /// Find capability slot by ID
    fn find_capability_slot(&self, cap_id: SigmaU64) -> Option<usize> {
        for i in 0..MAX_CAPABILITIES {
            if let Some(cap) = self.capabilities[i] {
                if cap.cap_id == cap_id {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Get current timestamp using RDTSC
    fn get_timestamp(&self) -> SigmaU64 {
        unsafe {
            let mut low: u32;
            let mut high: u32;
            core::arch::asm!(
                "rdtsc",
                out("eax") low,
                out("edx") high,
                options(nomem, nostack)
            );
            ((high as SigmaU64) << 32) | (low as SigmaU64)
        }
    }

    /// Get capability count for process
    pub unsafe fn get_process_cap_count(&mut self, pid: SigmaU64) -> SigmaUsize {
        let pid_usize = (pid % MAX_PROCESSES as SigmaU64) as usize;
        match self.process_caps[pid_usize].as_ref() {
            Some(caps) => caps.cap_count,
            None => 0,
        }
    }

    /// Get total capability count
    pub unsafe fn get_total_cap_count(&mut self) -> SigmaUsize {
        let mut count = 0;
        for i in 0..MAX_CAPABILITIES {
            if self.capabilities[i].is_some() {
                count += 1;
            }
        }
        count
    }
}

// ─── Global Capability Manager Instance ─────────────────────────────────────

static mut CAPABILITY_MANAGER: CapabilityManager = CapabilityManager::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_capability_init() -> SigmaI32 {
    match CAPABILITY_MANAGER.init() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_capability_create(
    owner_pid: SigmaU64,
    rights: SigmaU32,
    delegatable: SigmaBool,
    expiry_time: SigmaU64,
) -> SigmaU64 {
    match CAPABILITY_MANAGER.create_capability(owner_pid, rights, delegatable, expiry_time) {
        Ok(cap_id) => cap_id,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_capability_grant(pid: SigmaU64, cap_id: SigmaU64) -> SigmaI32 {
    match CAPABILITY_MANAGER.grant_capability(pid, cap_id) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_capability_revoke(pid: SigmaU64, cap_id: SigmaU64) -> SigmaI32 {
    match CAPABILITY_MANAGER.revoke_capability(pid, cap_id) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_capability_check(pid: SigmaU64, required_right: SigmaU32) -> SigmaBool {
    CAPABILITY_MANAGER.check_capability(pid, match required_right {
        1 => CapabilityRight::CapProcessSpawn,
        2 => CapabilityRight::CapProcessSignal,
        4 => CapabilityRight::CapProcessDebug,
        8 => CapabilityRight::CapProcessKill,
        16 => CapabilityRight::CapMemRead,
        32 => CapabilityRight::CapMemWrite,
        64 => CapabilityRight::CapMemExecute,
        128 => CapabilityRight::CapMemMap,
        256 => CapabilityRight::CapIoRead,
        512 => CapabilityRight::CapIoWrite,
        1024 => CapabilityRight::CapIoControl,
        2048 => CapabilityRight::CapNetSocket,
        4096 => CapabilityRight::CapNetBind,
        8192 => CapabilityRight::CapNetConnect,
        16384 => CapabilityRight::CapFsRead,
        32768 => CapabilityRight::CapFsWrite,
        65536 => CapabilityRight::CapFsCreate,
        131072 => CapabilityRight::CapFsDelete,
        262144 => CapabilityRight::CapDeviceAccess,
        524288 => CapabilityRight::CapDeviceConfigure,
        1048576 => CapabilityRight::CapSysTime,
        2097152 => CapabilityRight::CapSysReboot,
        4194304 => CapabilityRight::CapSysShutdown,
        8388608 => CapabilityRight::CapIpcSend,
        16777216 => CapabilityRight::CapIpcReceive,
        33554432 => CapabilityRight::CapIpcDelegate,
        _ => return false,
    })
}

#[no_mangle]
pub unsafe extern "C" fn sigma_capability_revoke_all(pid: SigmaU64) -> SigmaI32 {
    match CAPABILITY_MANAGER.revoke_all_capabilities(pid) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_capability_get_process_count(pid: SigmaU64) -> SigmaUsize {
    CAPABILITY_MANAGER.get_process_cap_count(pid)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_capability_get_total_count() -> SigmaUsize {
    CAPABILITY_MANAGER.get_total_cap_count()
}