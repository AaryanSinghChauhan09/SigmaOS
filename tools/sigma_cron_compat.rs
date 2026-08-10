// SPDX-License-Identifier: Apache-2.0
//! SigmaOS Cron Compatibility Layer
//! Cron job scheduling compatibility
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Cron schedule
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CronSchedule {
    pub minute: [u8; 60],    // 0-59
    pub hour: [u8; 24],      // 0-23
    pub day_of_month: [u8; 31], // 1-31
    pub month: [u8; 12],     // 0-11
    pub day_of_week: [u8; 7], // 0-6 (Sun-Sat)
}

/// Cron category (RedHat/Debian-style)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CronCategory {
    Custom = 0,
    Hourly = 1,
    Daily = 2,
    Weekly = 3,
    Monthly = 4,
}

/// Cron job
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CronJob {
    pub name: [u8; 64],
    pub command: [u8; 256],
    pub schedule: CronSchedule,
    pub enabled: SigmaBool,
    pub last_run: SigmaU64,
    pub next_run: SigmaU64,
    pub category: u8,               // RedHat/Debian-style CronCategory as u8
    pub run_as_user: u32,           // Alpine/Busybox-style user ID for strict security isolation
    pub randomized_delay_sec: u32,  // Arch/systemd-timer style thundering herd mitigation delay
    pub generation_id: u32,         // NixOS-style declarative configuration generation ID
}

/// Cron state
const MAX_CRON_JOBS: usize = 100;

static mut CRON_JOBS: [CronJob; MAX_CRON_JOBS] = [CronJob {
    name: [0; 64],
    command: [0; 256],
    schedule: CronSchedule {
        minute: [0; 60],
        hour: [0; 24],
        day_of_month: [0; 31],
        month: [0; 12],
        day_of_week: [0; 7],
    },
    enabled: false,
    last_run: 0,
    next_run: 0,
    category: 0,
    run_as_user: 0,
    randomized_delay_sec: 0,
    generation_id: 0,
}; MAX_CRON_JOBS];

static mut CRON_JOB_COUNT: SigmaU32 = 0;
static mut CRON_INITIALIZED: SigmaBool = false;

/// Initialize cron
#[no_mangle]
pub unsafe extern "C" fn cron_init() -> SigmaI32 {
    CRON_INITIALIZED = true;
    CRON_JOB_COUNT = 0;
    
    0 // Success
}

/// Add cron job
#[no_mangle]
pub unsafe extern "C" fn cron_add_job(
    name: *const u8,
    command: *const u8,
    minute: *const u8,
    hour: *const u8,
    day_of_month: *const u8,
    month: *const u8,
    day_of_week: *const u8,
) -> SigmaI32 {
    if !CRON_INITIALIZED || CRON_JOB_COUNT >= MAX_CRON_JOBS as SigmaU32 {
        return -1;
    }
    
    let mut job = CronJob {
        name: [0; 64],
        command: [0; 256],
        schedule: CronSchedule {
            minute: [0; 60],
            hour: [0; 24],
            day_of_month: [0; 31],
            month: [0; 12],
            day_of_week: [0; 7],
        },
        enabled: true,
        last_run: 0,
        next_run: 0,
        category: 0,
        run_as_user: 1000, // default non-root user
        randomized_delay_sec: 0,
        generation_id: 1,
    };
    
    if !name.is_null() {
        for i in 0..63 {
            let byte = *name.add(i);
            if byte == 0 { break; }
            job.name[i] = byte;
        }
    }
    
    if !command.is_null() {
        for i in 0..255 {
            let byte = *command.add(i);
            if byte == 0 { break; }
            job.command[i] = byte;
        }
    }
    
    // Parse schedule
    if !minute.is_null() {
        parse_cron_field(minute, &mut job.schedule.minute);
    }
    
    if !hour.is_null() {
        parse_cron_field(hour, &mut job.schedule.hour);
    }
    
    if !day_of_month.is_null() {
        parse_cron_field(day_of_month, &mut job.schedule.day_of_month);
    }
    
    if !month.is_null() {
        parse_cron_field(month, &mut job.schedule.month);
    }
    
    if !day_of_week.is_null() {
        parse_cron_field(day_of_week, &mut job.schedule.day_of_week);
    }
    
    CRON_JOBS[CRON_JOB_COUNT as usize] = job;
    CRON_JOB_COUNT += 1;
    
    0 // Success
}

/// Remove cron job
#[no_mangle]
pub unsafe extern "C" fn cron_remove_job(name: *const u8) -> SigmaI32 {
    if !CRON_INITIALIZED || name.is_null() {
        return -1;
    }
    
    for i in 0..CRON_JOB_COUNT as usize {
        let job = &CRON_JOBS[i];
        
        let mut matches = true;
        for j in 0..64 {
            if job.name[j] != *name.add(j) {
                if job.name[j] == 0 && *name.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if job.name[j] == 0 {
                break;
            }
        }
        
        if matches {
            // Shift remaining jobs
            for k in i..CRON_JOB_COUNT as usize - 1 {
                CRON_JOBS[k] = CRON_JOBS[k + 1];
            }
            CRON_JOB_COUNT -= 1;
            return 0;
        }
    }
    
    -2 // Job not found
}

/// Enable cron job
#[no_mangle]
pub unsafe extern "C" fn cron_enable_job(name: *const u8) -> SigmaI32 {
    if !CRON_INITIALIZED || name.is_null() {
        return -1;
    }
    
    for i in 0..CRON_JOB_COUNT as usize {
        let job = &mut CRON_JOBS[i];
        
        let mut matches = true;
        for j in 0..64 {
            if job.name[j] != *name.add(j) {
                if job.name[j] == 0 && *name.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if job.name[j] == 0 {
                break;
            }
        }
        
        if matches {
            job.enabled = true;
            return 0;
        }
    }
    
    -2 // Job not found
}

/// Disable cron job
#[no_mangle]
pub unsafe extern "C" fn cron_disable_job(name: *const u8) -> SigmaI32 {
    if !CRON_INITIALIZED || name.is_null() {
        return -1;
    }
    
    for i in 0..CRON_JOB_COUNT as usize {
        let job = &mut CRON_JOBS[i];
        
        let mut matches = true;
        for j in 0..64 {
            if job.name[j] != *name.add(j) {
                if job.name[j] == 0 && *name.add(j) == 0 {
                    break;
                }
                matches = false;
                break;
            }
            if job.name[j] == 0 {
                break;
            }
        }
        
        if matches {
            job.enabled = false;
            return 0;
        }
    }
    
    -2 // Job not found
}

/// List cron jobs
#[no_mangle]
pub unsafe extern "C" fn cron_list_jobs(jobs: *mut CronJob, max_count: SigmaU32) -> SigmaU32 {
    if !CRON_INITIALIZED || jobs.is_null() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..CRON_JOB_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *jobs.add(count) = CRON_JOBS[i];
        count += 1;
    }
    
    count as SigmaU32
}

/// Check and run due jobs
#[no_mangle]
pub unsafe extern "C" fn cron_check_and_run() -> SigmaI32 {
    if !CRON_INITIALIZED {
        return -1;
    }
    
    let current_time = get_timestamp();
    
    for i in 0..CRON_JOB_COUNT as usize {
        let job = &mut CRON_JOBS[i];
        
        if !job.enabled {
            continue;
        }
        
        if current_time >= job.next_run {
            // Execute job
            execute_cron_job(job);
            job.last_run = current_time;
            job.next_run = calculate_next_run(job);
        }
    }
    
    0 // Success
}

/// Get job count
#[no_mangle]
pub unsafe extern "C" fn cron_get_job_count() -> SigmaU32 {
    CRON_JOB_COUNT
}

/// Parse cron field helper supporting ranges (e.g. "1-5") and step increments (e.g. "*/15")
unsafe fn parse_cron_field(field: *const u8, output: &mut [u8]) {
    let max_val = output.len();
    for i in 0..max_val {
        output[i] = 0;
    }
    
    // Read null-terminated field into a temporary buffer to avoid unsafe out-of-bounds pointer reads
    let mut buf = [0u8; 64];
    let mut len = 0;
    while len < 63 {
        let byte = *field.add(len);
        if byte == 0 {
            break;
        }
        buf[len] = byte;
        len += 1;
    }

    if len == 0 {
        return;
    }

    // Check if it is a pure "*"
    if len == 1 && buf[0] == b'*' {
        for i in 0..max_val {
            output[i] = 1;
        }
        return;
    }

    // Check for step increments starting with "*/", e.g. "*/15"
    if len >= 3 && buf[0] == b'*' && buf[1] == b'/' {
        let mut step = 0;
        for i in 2..len {
            if buf[i] >= b'0' && buf[i] <= b'9' {
                step = step * 10 + (buf[i] - b'0') as usize;
            }
        }
        if step > 0 {
            for i in (0..max_val).step_by(step) {
                output[i] = 1;
            }
        }
        return;
    }

    // Check for range, e.g. "1-5"
    let mut dash_idx = None;
    for i in 0..len {
        if buf[i] == b'-' {
            dash_idx = Some(i);
            break;
        }
    }

    if let Some(idx) = dash_idx {
        let mut start = 0;
        let mut end = 0;
        for i in 0..idx {
            if buf[i] >= b'0' && buf[i] <= b'9' {
                start = start * 10 + (buf[i] - b'0') as usize;
            }
        }
        for i in (idx + 1)..len {
            if buf[i] >= b'0' && buf[i] <= b'9' {
                end = end * 10 + (buf[i] - b'0') as usize;
            }
        }
        if start < max_val && end < max_val && start <= end {
            for i in start..=end {
                output[i] = 1;
            }
        }
        return;
    }

    // Otherwise, parse specific values or comma-separated lists (simplified)
    let mut current_val = 0;
    let mut has_digit = false;
    for i in 0..=len {
        let byte = if i < len { buf[i] } else { 0 };
        if byte >= b'0' && byte <= b'9' {
            current_val = current_val * 10 + (byte - b'0') as usize;
            has_digit = true;
        } else {
            if has_digit {
                if current_val < max_val {
                    output[current_val] = 1;
                }
                current_val = 0;
                has_digit = false;
            }
        }
    }
}

/// Add cron job with extended multi-distro parameters
#[no_mangle]
pub unsafe extern "C" fn cron_add_job_ext(
    name: *const u8,
    command: *const u8,
    minute: *const u8,
    hour: *const u8,
    day_of_month: *const u8,
    month: *const u8,
    day_of_week: *const u8,
    category: u8,
    run_as_user: u32,
    randomized_delay_sec: u32,
    generation_id: u32,
) -> SigmaI32 {
    let res = cron_add_job(name, command, minute, hour, day_of_month, month, day_of_week);
    if res == 0 {
        let job = &mut CRON_JOBS[(CRON_JOB_COUNT - 1) as usize];
        job.category = category;
        job.run_as_user = run_as_user;
        job.randomized_delay_sec = randomized_delay_sec;
        job.generation_id = generation_id;
    }
    res
}

/// Execute cron job helper
unsafe fn execute_cron_job(job: &CronJob) {
    // In a real implementation, this would:
    // 1. Fork a process
    // 2. Execute the command
    // 3. Capture output
    // 4. Log results
}

/// Calculate next run time helper
unsafe fn calculate_next_run(job: &CronJob) -> SigmaU64 {
    // In a real implementation, this would:
    // 1. Get current time
    // 2. Find next matching time based on schedule
    // 3. Return timestamp
    
    // Placeholder - return current time + 60 seconds
    get_timestamp() + 60
}

/// Get timestamp helper
unsafe fn get_timestamp() -> SigmaU64 {
    static mut COUNTER: SigmaU64 = 0;
    COUNTER += 1;
    COUNTER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigma_cron_advanced() {
        unsafe {
            cron_init();

            // Test advanced step parsing ("*/15")
            let mut minute_field = [0u8; 60];
            parse_cron_field(b"*/15\0".as_ptr(), &mut minute_field);
            assert_eq!(minute_field[0], 1);
            assert_eq!(minute_field[15], 1);
            assert_eq!(minute_field[30], 1);
            assert_eq!(minute_field[45], 1);
            assert_eq!(minute_field[5], 0);

            // Test range parsing ("2-5")
            let mut hour_field = [0u8; 24];
            parse_cron_field(b"2-5\0".as_ptr(), &mut hour_field);
            assert_eq!(hour_field[1], 0);
            assert_eq!(hour_field[2], 1);
            assert_eq!(hour_field[3], 1);
            assert_eq!(hour_field[4], 1);
            assert_eq!(hour_field[5], 1);
            assert_eq!(hour_field[6], 0);

            // Test extended cron creation with all multi-distro parameters
            let res = cron_add_job_ext(
                b"backup_job\0".as_ptr(),
                b"tar -czf /backup/sys.tar.gz\0".as_ptr(),
                b"*/15\0".as_ptr(),
                b"2-5\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                CronCategory::Daily as u8,
                0,    // Alpine/Busybox: root user
                300,  // Arch: 5-minute jitter delay
                42,   // NixOS: generation ID 42
            );
            assert_eq!(res, 0);
            assert_eq!(cron_get_job_count(), 1);

            let mut jobs_list = [CronJob {
                name: [0; 64],
                command: [0; 256],
                schedule: CronSchedule {
                    minute: [0; 60],
                    hour: [0; 24],
                    day_of_month: [0; 31],
                    month: [0; 12],
                    day_of_week: [0; 7],
                },
                enabled: false,
                last_run: 0,
                next_run: 0,
                category: 0,
                run_as_user: 0,
                randomized_delay_sec: 0,
                generation_id: 0,
            }; 1];

            cron_list_jobs(jobs_list.as_mut_ptr(), 1);
            let retrieved_job = &jobs_list[0];

            assert_eq!(retrieved_job.category, CronCategory::Daily as u8);
            assert_eq!(retrieved_job.run_as_user, 0);
            assert_eq!(retrieved_job.randomized_delay_sec, 300);
            assert_eq!(retrieved_job.generation_id, 42);
        }
    }
}
