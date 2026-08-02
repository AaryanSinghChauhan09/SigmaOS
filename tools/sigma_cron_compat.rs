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

/// Cron job with extended multi-distro parameters
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
    // Brand new Linux Distro inspired fields:
    pub max_load_average: u32,      // Gentoo/dcron style CPU load mitigation (0 = no limit)
    pub run_if_missed: SigmaBool,   // Debian/Anacron style catch-up execution for offline systems
    pub selinux_context: [u8; 64],  // Fedora/RHEL security context
    pub mailto: [u8; 64],           // RedHat/Vixie style email output configuration
    pub allow_overlap: SigmaBool,   // Cronie flock-style parallel execution prevention
    pub is_running: SigmaBool,      // Active run tracker for flock-style locking
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
    max_load_average: 0,
    run_if_missed: false,
    selinux_context: [0; 64],
    mailto: [0; 64],
    allow_overlap: true,
    is_running: false,
}; MAX_CRON_JOBS];

static mut CRON_JOB_COUNT: SigmaU32 = 0;
static mut CRON_INITIALIZED: SigmaBool = false;

/// Gentoo-style CPU load average mitigation state (e.g. 100 represents 1.00 load)
static mut SYSTEM_LOAD_AVERAGE: u32 = 100;

/// Simple log entry system to capture and verify execution logs in testing & inspection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CronLogEntry {
    pub job_name: [u8; 64],
    pub action: [u8; 32], // "executed", "skipped_load", "skipped_overlap", "skipped_missed", "selinux_enforced", "mail_dispatched"
    pub user_id: u32,
}

static mut CRON_LOGS: [CronLogEntry; 32] = [CronLogEntry {
    job_name: [0; 64],
    action: [0; 32],
    user_id: 0,
}; 32];
static mut CRON_LOG_COUNT: usize = 0;

unsafe fn log_cron_action(job_name: &[u8; 64], action_str: &str, user_id: u32) {
    if CRON_LOG_COUNT < 32 {
        let mut entry = CronLogEntry {
            job_name: *job_name,
            action: [0; 32],
            user_id,
        };
        let bytes = action_str.as_bytes();
        let len = bytes.len().min(31);
        for i in 0..len {
            entry.action[i] = bytes[i];
        }
        CRON_LOGS[CRON_LOG_COUNT] = entry;
        CRON_LOG_COUNT += 1;
    }
}

/// Reset log buffer for testing
#[no_mangle]
pub unsafe extern "C" fn cron_reset_logs() {
    CRON_LOG_COUNT = 0;
    for i in 0..32 {
        CRON_LOGS[i] = CronLogEntry {
            job_name: [0; 64],
            action: [0; 32],
            user_id: 0,
        };
    }
}

/// Retrieve log count
#[no_mangle]
pub unsafe extern "C" fn cron_get_log_count() -> u32 {
    CRON_LOG_COUNT as u32
}

/// Get a log entry details
#[no_mangle]
pub unsafe extern "C" fn cron_get_log_entry(
    index: u32,
    job_name: *mut u8,
    action: *mut u8,
    user_id: *mut u32,
) -> SigmaI32 {
    let idx = index as usize;
    if idx >= CRON_LOG_COUNT {
        return -1;
    }
    let entry = &CRON_LOGS[idx];
    if !job_name.is_null() {
        core::ptr::copy_nonoverlapping(entry.job_name.as_ptr(), job_name, 64);
    }
    if !action.is_null() {
        core::ptr::copy_nonoverlapping(entry.action.as_ptr(), action, 32);
    }
    if !user_id.is_null() {
        *user_id = entry.user_id;
    }
    0
}

/// Set the system load average value (represented as load * 100, e.g., 250 for 2.50 load)
#[no_mangle]
pub unsafe extern "C" fn cron_set_system_load(load: u32) {
    SYSTEM_LOAD_AVERAGE = load;
}

/// Initialize cron
#[no_mangle]
pub unsafe extern "C" fn cron_init() -> SigmaI32 {
    CRON_INITIALIZED = true;
    CRON_JOB_COUNT = 0;
    SYSTEM_LOAD_AVERAGE = 100;
    cron_reset_logs();
    
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
        max_load_average: 0,
        run_if_missed: false,
        selinux_context: [0; 64],
        mailto: [0; 64],
        allow_overlap: true,
        is_running: false,
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

/// Check and run due jobs (Improved with modern Linux distro-inspired controls)
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
            // Check overlapping runs (Cronie flock style prevention)
            if job.is_running && !job.allow_overlap {
                log_cron_action(&job.name, "skipped_overlap", job.run_as_user);
                continue;
            }

            // Check Gentoo-style CPU load average limit
            if job.max_load_average > 0 && SYSTEM_LOAD_AVERAGE > job.max_load_average {
                log_cron_action(&job.name, "skipped_load", job.run_as_user);
                continue;
            }

            // Check Debian/Anacron style missed execution windows
            let missed_by = current_time - job.next_run;
            if missed_by > 50 && !job.run_if_missed {
                // Skip execution because we missed the window and run_if_missed is false
                log_cron_action(&job.name, "skipped_missed", job.run_as_user);
                job.next_run = calculate_next_run(job);
                continue;
            }

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

/// Add cron job with advanced Linux-distro inspired parameters
#[no_mangle]
pub unsafe extern "C" fn cron_add_job_linux(
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
    max_load_average: u32,
    run_if_missed: SigmaBool,
    selinux_context: *const u8,
    mailto: *const u8,
    allow_overlap: SigmaBool,
) -> SigmaI32 {
    let res = cron_add_job(name, command, minute, hour, day_of_month, month, day_of_week);
    if res == 0 {
        let job = &mut CRON_JOBS[(CRON_JOB_COUNT - 1) as usize];
        job.category = category;
        job.run_as_user = run_as_user;
        job.randomized_delay_sec = randomized_delay_sec;
        job.generation_id = generation_id;
        job.max_load_average = max_load_average;
        job.run_if_missed = run_if_missed;
        job.allow_overlap = allow_overlap;

        if !selinux_context.is_null() {
            for i in 0..63 {
                let byte = *selinux_context.add(i);
                if byte == 0 { break; }
                job.selinux_context[i] = byte;
            }
        }

        if !mailto.is_null() {
            for i in 0..63 {
                let byte = *mailto.add(i);
                if byte == 0 { break; }
                job.mailto[i] = byte;
            }
        }
    }
    res
}

/// Execute cron job helper
unsafe fn execute_cron_job(job: &mut CronJob) {
    job.is_running = true;
    log_cron_action(&job.name, "executed", job.run_as_user);

    // Simulate SELinux transition and MAILTO configuration logs
    let mut mailto_has_val = false;
    for &b in &job.mailto {
        if b != 0 { mailto_has_val = true; break; }
    }
    if mailto_has_val {
        log_cron_action(&job.name, "mail_dispatched", job.run_as_user);
    }

    let mut selinux_has_val = false;
    for &b in &job.selinux_context {
        if b != 0 { selinux_has_val = true; break; }
    }
    if selinux_has_val {
        log_cron_action(&job.name, "selinux_enforced", job.run_as_user);
    }

    job.is_running = false;
}

/// Calculate next run time helper
unsafe fn calculate_next_run(_job: &CronJob) -> SigmaU64 {
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
                max_load_average: 0,
                run_if_missed: false,
                selinux_context: [0; 64],
                mailto: [0; 64],
                allow_overlap: true,
                is_running: false,
            }; 1];

            cron_list_jobs(jobs_list.as_mut_ptr(), 1);
            let retrieved_job = &jobs_list[0];

            assert_eq!(retrieved_job.category, CronCategory::Daily as u8);
            assert_eq!(retrieved_job.run_as_user, 0);
            assert_eq!(retrieved_job.randomized_delay_sec, 300);
            assert_eq!(retrieved_job.generation_id, 42);
        }
    }

    #[test]
    fn test_linux_distro_features() {
        unsafe {
            cron_init();
            cron_reset_logs();

            // Set system load high (e.g. 3.00, represented as 300)
            cron_set_system_load(300);

            // 1. Gentoo load average mitigation test
            let res_load = cron_add_job_linux(
                b"gentoo_job\0".as_ptr(),
                b"echo high_load_task\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                CronCategory::Custom as u8,
                1000,
                0,
                1,
                250, // max_load_average: 2.50 (current is 3.00, so it should be skipped)
                true,
                core::ptr::null(),
                core::ptr::null(),
                true,
            );
            assert_eq!(res_load, 0);

            // This job has high/unlimited load average allowed (0 = disabled)
            let res_normal = cron_add_job_linux(
                b"normal_job\0".as_ptr(),
                b"echo normal_task\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                CronCategory::Custom as u8,
                1000,
                0,
                1,
                0, // no limit
                true,
                core::ptr::null(),
                core::ptr::null(),
                true,
            );
            assert_eq!(res_normal, 0);

            // Execute cron_check_and_run
            cron_check_and_run();

            // Check that gentoo_job was skipped due to load, and normal_job executed
            let mut found_skipped_load = false;
            let mut found_executed_normal = false;
            for idx in 0..cron_get_log_count() {
                let mut name_buf = [0u8; 64];
                let mut act_buf = [0u8; 32];
                let mut uid = 0;
                cron_get_log_entry(idx, name_buf.as_mut_ptr(), act_buf.as_mut_ptr(), &mut uid);

                let name_str = core::str::from_utf8(&name_buf).unwrap().trim_end_matches('\0');
                let act_str = core::str::from_utf8(&act_buf).unwrap().trim_end_matches('\0');

                if name_str.starts_with("gentoo_job") && act_str == "skipped_load" {
                    found_skipped_load = true;
                }
                if name_str.starts_with("normal_job") && act_str == "executed" {
                    found_executed_normal = true;
                }
            }
            assert!(found_skipped_load);
            assert!(found_executed_normal);

            // 2. Cronie flock-style overlapping prevention test
            cron_init();
            cron_reset_logs();
            cron_set_system_load(100); // normal load

            let res_overlap = cron_add_job_linux(
                b"overlap_job\0".as_ptr(),
                b"echo parallel_task\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                CronCategory::Custom as u8,
                1000,
                0,
                1,
                0,
                true,
                core::ptr::null(),
                core::ptr::null(),
                false, // allow_overlap = false
            );
            assert_eq!(res_overlap, 0);

            // Manually mark job as running
            CRON_JOBS[0].is_running = true;

            cron_check_and_run();

            // Verify it was skipped due to overlap
            let mut found_skipped_overlap = false;
            for idx in 0..cron_get_log_count() {
                let mut name_buf = [0u8; 64];
                let mut act_buf = [0u8; 32];
                let mut uid = 0;
                cron_get_log_entry(idx, name_buf.as_mut_ptr(), act_buf.as_mut_ptr(), &mut uid);

                let name_str = core::str::from_utf8(&name_buf).unwrap().trim_end_matches('\0');
                let act_str = core::str::from_utf8(&act_buf).unwrap().trim_end_matches('\0');

                if name_str.starts_with("overlap_job") && act_str == "skipped_overlap" {
                    found_skipped_overlap = true;
                }
            }
            assert!(found_skipped_overlap);

            // 3. Debian/Anacron offline catch-up test
            cron_init();
            cron_reset_logs();

            // Add anacron-like job (run_if_missed = true)
            let res_anacron = cron_add_job_linux(
                b"anacron_job\0".as_ptr(),
                b"echo offline_task\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                CronCategory::Custom as u8,
                1000,
                0,
                1,
                0,
                true, // run_if_missed = true
                core::ptr::null(),
                core::ptr::null(),
                true,
            );
            assert_eq!(res_anacron, 0);

            // Add standard job (run_if_missed = false)
            let res_standard_missed = cron_add_job_linux(
                b"standard_job\0".as_ptr(),
                b"echo standard_offline_task\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                CronCategory::Custom as u8,
                1000,
                0,
                1,
                0,
                false, // run_if_missed = false
                core::ptr::null(),
                core::ptr::null(),
                true,
            );
            assert_eq!(res_standard_missed, 0);

            // Simulate that next_run is 10, but current_time is 100 (missed by 90 ticks)
            CRON_JOBS[0].next_run = 10;
            CRON_JOBS[1].next_run = 10;

            // Set counter so get_timestamp returns 100
            for _ in 0..100 {
                get_timestamp();
            }

            cron_check_and_run();

            // Verify anacron_job ran, but standard_job was skipped_missed
            let mut found_anacron_executed = false;
            let mut found_standard_skipped_missed = false;
            for idx in 0..cron_get_log_count() {
                let mut name_buf = [0u8; 64];
                let mut act_buf = [0u8; 32];
                let mut uid = 0;
                cron_get_log_entry(idx, name_buf.as_mut_ptr(), act_buf.as_mut_ptr(), &mut uid);

                let name_str = core::str::from_utf8(&name_buf).unwrap().trim_end_matches('\0');
                let act_str = core::str::from_utf8(&act_buf).unwrap().trim_end_matches('\0');

                if name_str.starts_with("anacron_job") && act_str == "executed" {
                    found_anacron_executed = true;
                }
                if name_str.starts_with("standard_job") && act_str == "skipped_missed" {
                    found_standard_skipped_missed = true;
                }
            }
            assert!(found_anacron_executed);
            assert!(found_standard_skipped_missed);

            // 4. SELinux & Mailto logging integration test
            cron_init();
            cron_reset_logs();

            let res_sec = cron_add_job_linux(
                b"secure_job\0".as_ptr(),
                b"echo sec\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                b"*\0".as_ptr(),
                CronCategory::Custom as u8,
                0, // Root
                0,
                1,
                0,
                true,
                b"system_u:system_r:cronjob_t:s0\0".as_ptr(),
                b"admin@sigmaos.org\0".as_ptr(),
                true,
            );
            assert_eq!(res_sec, 0);

            cron_check_and_run();

            let mut found_selinux = false;
            let mut found_mail = false;
            for idx in 0..cron_get_log_count() {
                let mut name_buf = [0u8; 64];
                let mut act_buf = [0u8; 32];
                let mut uid = 0;
                cron_get_log_entry(idx, name_buf.as_mut_ptr(), act_buf.as_mut_ptr(), &mut uid);

                let name_str = core::str::from_utf8(&name_buf).unwrap().trim_end_matches('\0');
                let act_str = core::str::from_utf8(&act_buf).unwrap().trim_end_matches('\0');

                if name_str.starts_with("secure_job") && act_str == "selinux_enforced" {
                    found_selinux = true;
                }
                if name_str.starts_with("secure_job") && act_str == "mail_dispatched" {
                    found_mail = true;
                }
            }
            assert!(found_selinux);
            assert!(found_mail);
        }
    }
}
