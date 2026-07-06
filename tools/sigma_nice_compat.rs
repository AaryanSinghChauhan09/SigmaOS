//! SigmaOS Nice Compatibility
//! Process priority management (nice command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Process priority info
#[repr(C)]
pub struct ProcessPriority {
    pub pid: SigmaU32,
    pub nice_value: SigmaI32,
    pub priority: SigmaU32,
}

/// Nice state
const MAX_PRIORITIES: usize = 10000;

static mut PROCESS_PRIORITIES: [ProcessPriority; MAX_PRIORITIES] = [ProcessPriority {
    pid: 0,
    nice_value: 0,
    priority: 0,
}; MAX_PRIORITIES];

static mut PRIORITY_COUNT: SigmaU32 = 0;
static mut NICE_INITIALIZED: SigmaBool = false;

/// Initialize nice
#[no_mangle]
pub unsafe extern "C" fn nice_init() -> SigmaI32 {
    NICE_INITIALIZED = true;
    PRIORITY_COUNT = 0;
    
    0 // Success
}

/// Set nice value for new process
#[no_mangle]
pub unsafe extern "C" fn nice_set(nice_value: SigmaI32) -> SigmaI32 {
    if !NICE_INITIALIZED {
        return -1;
    }
    
    // Clamp nice value to valid range (-20 to 19)
    let clamped = if nice_value < -20 {
        -20
    } else if nice_value > 19 {
        19
    } else {
        nice_value
    };
    
    // In a real implementation, this would set the nice value for the current process
    // and all child processes spawned from it
    
    0 // Success
}

/// Get nice value for process
#[no_mangle]
pub unsafe extern "C" fn nice_get(pid: SigmaU32, nice_value: *mut SigmaI32) -> SigmaI32 {
    if !NICE_INITIALIZED || nice_value.isnull() {
        return -1;
    }
    
    for i in 0..PRIORITY_COUNT as usize {
        if PROCESS_PRIORITIES[i].pid == pid {
            *nice_value = PROCESS_PRIORITIES[i].nice_value;
            return 0;
        }
    }
    
    // Default nice value if not found
    *nice_value = 0;
    
    0 // Success
}

/// Renice existing process
#[no_mangle]
pub unsafe extern "C" fn nice_renice(pid: SigmaU32, nice_value: SigmaI32) -> SigmaI32 {
    if !NICE_INITIALIZED {
        return -1;
    }
    
    // Clamp nice value to valid range (-20 to 19)
    let clamped = if nice_value < -20 {
        -20
    } else if nice_value > 19 {
        19
    } else {
        nice_value
    };
    
    // Check if process already exists
    for i in 0..PRIORITY_COUNT as usize {
        if PROCESS_PRIORITIES[i].pid == pid {
            PROCESS_PRIORITIES[i].nice_value = clamped;
            PROCESS_PRIORITIES[i].priority = (20 + clamped) as SigmaU32;
            return 0;
        }
    }
    
    // Add new process
    if PRIORITY_COUNT >= MAX_PRIORITIES as SigmaU32 {
        return -1;
    }
    
    let mut priority = ProcessPriority {
        pid,
        nice_value: clamped,
        priority: (20 + clamped) as SigmaU32,
    };
    
    PROCESS_PRIORITIES[PRIORITY_COUNT as usize] = priority;
    PRIORITY_COUNT += 1;
    
    0 // Success
}

/// List process priorities
#[no_mangle]
pub unsafe extern "C" fn nice_list(priorities: *mut ProcessPriority, max_count: SigmaU32) -> SigmaU32 {
    if !NICE_INITIALIZED || priorities.isnull() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..PRIORITY_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *priorities.add(count) = PROCESS_PRIORITIES[i];
        count += 1;
    }
    
    count
}

/// Get priority count
#[no_mangle]
pub unsafe extern "C" fn nice_get_count() -> SigmaU32 {
    PRIORITY_COUNT
}
