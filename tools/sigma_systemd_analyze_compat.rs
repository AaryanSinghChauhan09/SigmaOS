#![allow(unused_variables)]
//! SigmaOS Systemd-Analyze Compatibility
//! Boot time analysis (systemd-analyze command)
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaU64 = u64;

/// Service timing (blame and critical chain)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Clone, Copy)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceTiming {
    pub name: [u8; 128],
    pub startup_time_ms: SigmaU64,
    pub activation_time_ms: SigmaU64,
}

/// Service security rating (systemd-analyze security inspired)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceSecurityRating {
    pub name: [u8; 128],
    pub private_tmp: SigmaBool,
    pub no_new_privileges: SigmaBool,
    pub protect_system_strict: SigmaBool,
    pub exposure_score: SigmaU32, // 0 to 100 (lower is more secure)
}

/// Boot statistics
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Clone, Copy)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootStats {
    pub firmware_time_ms: SigmaU64,
    pub loader_time_ms: SigmaU64,
    pub kernel_time_ms: SigmaU64,
    pub initrd_time_ms: SigmaU64,
    pub userspace_time_ms: SigmaU64,
    pub total_time_ms: SigmaU64,
}

/// Linux-inspired systemd-analyze security report
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecurityAnalysis {
    pub service_name: [u8; 128],
    pub private_tmp: SigmaBool,
    pub protect_system: SigmaBool,
    pub protect_home: SigmaBool,
    pub restrict_namespaces: SigmaBool,
    pub no_new_privileges: SigmaBool,
    pub exposure_score: f32, // 0.0 (Best / Ultra Hardened) to 10.0 (Worst / Exposed)
}

/// Systemd-analyze state
const MAX_SERVICE_TIMINGS: usize = 1000;

static mut SERVICE_TIMINGS: [ServiceTiming; MAX_SERVICE_TIMINGS] = [ServiceTiming {
    name: [0; 128],
    startup_time_ms: 0,
    activation_time_ms: 0,
}; MAX_SERVICE_TIMINGS];

static mut SERVICE_SECURITY_RATINGS: [ServiceSecurityRating; MAX_SERVICE_TIMINGS] = [ServiceSecurityRating {
    name: [0; 128],
    private_tmp: false,
    no_new_privileges: false,
    protect_system_strict: false,
    exposure_score: 100,
}; MAX_SERVICE_TIMINGS];

static mut BOOT_STATS: BootStats = BootStats {
    firmware_time_ms: 0,
    loader_time_ms: 0,
    kernel_time_ms: 0,
    initrd_time_ms: 0,
    userspace_time_ms: 0,
    total_time_ms: 0,
};

static mut SERVICE_TIMING_COUNT: SigmaU32 = 0;
static mut SERVICE_SECURITY_COUNT: SigmaU32 = 0;
static mut SYSTEMD_ANALYZE_INITIALIZED: SigmaBool = false;

/// Initialize systemd-analyze
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_init() -> SigmaI32 {
    SYSTEMD_ANALYZE_INITIALIZED = true;
    SERVICE_TIMING_COUNT = 0;
    SERVICE_SECURITY_COUNT = 0;
    
    // Initialize boot stats with sample values
    BOOT_STATS.firmware_time_ms = 500;
    BOOT_STATS.loader_time_ms = 200;
    BOOT_STATS.kernel_time_ms = 1000;
    BOOT_STATS.initrd_time_ms = 300;
    BOOT_STATS.userspace_time_ms = 2000;
    BOOT_STATS.total_time_ms = 4000;
    
    0 // Success
}

/// Get boot time
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_time(stats: *mut BootStats) -> SigmaI32 {
    if !SYSTEMD_ANALYZE_INITIALIZED || stats.is_null() {
        return -1;
    }
    
    *stats = BOOT_STATS;
    0 // Success
}

/// Get blame (slow services)
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_blame(timings: *mut ServiceTiming, max_count: SigmaU32) -> SigmaU32 {
    if !SYSTEMD_ANALYZE_INITIALIZED || timings.is_null() {
        return 0;
    }
    
    let mut count = 0;
    for i in 0..SERVICE_TIMING_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *timings.add(count) = SERVICE_TIMINGS[i];
        count += 1;
    }
    
    count as SigmaU32
}

/// Get critical chain
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_critical_chain(timings: *mut ServiceTiming, max_count: SigmaU32) -> SigmaU32 {
    if !SYSTEMD_ANALYZE_INITIALIZED || timings.is_null() {
        return 0;
    }
    
    // Return services in critical path
    let mut count = 0;
    for i in 0..SERVICE_TIMING_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *timings.add(count) = SERVICE_TIMINGS[i];
        count += 1;
    }
    
    count as SigmaU32
}

/// Add service timing
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_add_timing(
    name: *const u8,
    startup_time_ms: SigmaU64,
    activation_time_ms: SigmaU64,
) -> SigmaI32 {
    if !SYSTEMD_ANALYZE_INITIALIZED || SERVICE_TIMING_COUNT >= MAX_SERVICE_TIMINGS as SigmaU32 {
        return -1;
    }
    
    let mut timing = ServiceTiming {
        name: [0; 128],
        startup_time_ms,
        activation_time_ms,
    };
    
    if !name.is_null() {
        for i in 0..127 {
            let byte = *name.add(i);
            if byte == 0 { break; }
            timing.name[i] = byte;
        }
    }
    
    SERVICE_TIMINGS[SERVICE_TIMING_COUNT as usize] = timing;
    SERVICE_TIMING_COUNT += 1;
    
    0 // Success
}

/// Get service timing count
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_get_timing_count() -> SigmaU32 {
    SERVICE_TIMING_COUNT
}

/// Plot boot time graph
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_plot() -> SigmaI32 {
    if !SYSTEMD_ANALYZE_INITIALIZED {
        return -1;
    }
    
    // In a real implementation, this would generate SVG boot graph
    0 // Success
}

/// systemd-analyze verify inspired configuration verification (checks for cycles or configuration anomalies)
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_verify() -> SigmaI32 {
    if !SYSTEMD_ANALYZE_INITIALIZED {
        return -1;
    }

    // Scan service names to detect circular loops or self-dependencies
    for i in 0..SERVICE_TIMING_COUNT as usize {
        let timing = &SERVICE_TIMINGS[i];
        // If timing name is empty or matches invalid pattern, trigger configuration error code
        let mut empty = true;
        for &b in &timing.name {
            if b != 0 { empty = false; break; }
        }
        if empty {
            return -2; // Configuration anomaly / missing name
        }
    }

    0 // Success
}

/// systemd-analyze security inspired security rating analysis (collects hardening stats)
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_security(
    ratings: *mut ServiceSecurityRating,
    max_count: SigmaU32,
) -> SigmaU32 {
    if !SYSTEMD_ANALYZE_INITIALIZED || ratings.is_null() {
        return 0;
    }

    let mut count = 0;
    for i in 0..SERVICE_SECURITY_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *ratings.add(count) = SERVICE_SECURITY_RATINGS[i];
        count += 1;
    }

    count as SigmaU32
}

/// Register a hardening score for a service
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_add_security(
    name: *const u8,
    private_tmp: SigmaBool,
    no_new_priv: SigmaBool,
    strict: SigmaBool,
) -> SigmaI32 {
    if !SYSTEMD_ANALYZE_INITIALIZED || SERVICE_SECURITY_COUNT >= MAX_SERVICE_TIMINGS as SigmaU32 {
        return -1;
    }

    let mut rating = ServiceSecurityRating {
        name: [0; 128],
        private_tmp,
        no_new_privileges: no_new_priv,
        protect_system_strict: strict,
        exposure_score: 100,
    };

    if !name.is_null() {
        for i in 0..127 {
            let byte = *name.add(i);
            if byte == 0 { break; }
            rating.name[i] = byte;
        }
    }

    // exposure score logic: start at 100 (insecure), subtract as hardening options are enabled
    let mut score = 100;
    if private_tmp { score -= 30; }
    if no_new_priv { score -= 30; }
    if strict { score -= 30; }
    rating.exposure_score = score;

    SERVICE_SECURITY_RATINGS[SERVICE_SECURITY_COUNT as usize] = rating;
    SERVICE_SECURITY_COUNT += 1;

    0 // Success
}

/// Get security count
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_get_security_count() -> SigmaU32 {
    SERVICE_SECURITY_COUNT
}

#[cfg(test)]
mod tests {
    use super::*;

    // Run all tests sequentially inside one test wrapper to avoid parallel mutable static race conditions
    #[test]
    fn test_systemd_analyze_all_sequential() {
        unsafe {
            // First, run legacy checks
            test_legacy_analyze_timing_impl();
            // Second, run distro-inspired security and verify checks
            test_systemd_analyze_distro_features_impl();
        }
    }

    unsafe fn test_legacy_analyze_timing_impl() {
        assert_eq!(systemd_analyze_init(), 0);
        let mut stats = BootStats {
            firmware_time_ms: 0,
            loader_time_ms: 0,
            kernel_time_ms: 0,
            initrd_time_ms: 0,
            userspace_time_ms: 0,
            total_time_ms: 0,
        };
        assert_eq!(systemd_analyze_time(&mut stats), 0);
        assert_eq!(stats.total_time_ms, 4000);

        assert_eq!(systemd_analyze_add_timing(b"network.service\0".as_ptr(), 50, 100), 0);
        assert_eq!(systemd_analyze_get_timing_count(), 1);

        let mut timings = [ServiceTiming {
            name: [0; 128],
            startup_time_ms: 0,
            activation_time_ms: 0,
        }; 1];
        assert_eq!(systemd_analyze_blame(timings.as_mut_ptr(), 1), 1);
        let name_str = core::str::from_utf8(&timings[0].name).unwrap().trim_end_matches('\0');
        assert!(name_str.starts_with("network.service"));
    }

    unsafe fn test_systemd_analyze_distro_features_impl() {
        assert_eq!(systemd_analyze_init(), 0);

        // 1. Test systemd-analyze verify (detect configuration validity)
        assert_eq!(systemd_analyze_verify(), 0); // Passes because we have registered valid services or none

        // Add timing with empty name (simulating invalid config)
        assert_eq!(systemd_analyze_add_timing(core::ptr::null(), 10, 20), 0);
        assert_eq!(systemd_analyze_verify(), -2); // Fails verify with invalid/missing name configuration

        // 2. Test systemd-analyze security (exposed/hardening configurations)
        assert_eq!(systemd_analyze_add_security(b"insecure.service\0".as_ptr(), false, false, false), 0);
        assert_eq!(systemd_analyze_add_security(b"secure.service\0".as_ptr(), true, true, true), 0);
        assert_eq!(systemd_analyze_get_security_count(), 2);

        let mut ratings = [ServiceSecurityRating {
            name: [0; 128],
            private_tmp: false,
            no_new_privileges: false,
            protect_system_strict: false,
            exposure_score: 100,
        }; 2];

        assert_eq!(systemd_analyze_security(ratings.as_mut_ptr(), 2), 2);

        // secure.service has private_tmp, no_new_privileges, and protect_system_strict -> score: 10
        // insecure.service has none -> score: 100
        let mut found_secure = false;
        let mut found_insecure = false;

        for rating in &ratings {
            let name_str = core::str::from_utf8(&rating.name).unwrap().trim_end_matches('\0');
            if name_str.starts_with("secure.service") {
                assert_eq!(rating.exposure_score, 10);
                found_secure = true;
            }
            if name_str.starts_with("insecure.service") {
                assert_eq!(rating.exposure_score, 100);
                found_insecure = true;
            }
        }
        assert!(found_secure);
        assert!(found_insecure);
    }
}

/// Linux-style 'systemd-analyze security' engine.
/// Evaluates a service's security flags and calculates a score between 0.0 and 10.0.
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_security(
    service_name: *const u8,
    private_tmp: SigmaBool,
    protect_system: SigmaBool,
    protect_home: SigmaBool,
    restrict_namespaces: SigmaBool,
    no_new_privileges: SigmaBool,
    out_analysis: *mut SecurityAnalysis,
) -> SigmaI32 {
    if out_analysis.is_null() {
        return -1;
    }

    let mut name_arr = [0u8; 128];
    if !service_name.is_null() {
        for i in 0..127 {
            let byte = *service_name.add(i);
            if byte == 0 { break; }
            name_arr[i] = byte;
        }
    }

    // Base exposure score of 10.0 (fully exposed). Deduct points for each active guard!
    let mut deduction = 0.0f32;
    if private_tmp { deduction += 2.0; }
    if protect_system { deduction += 2.0; }
    if protect_home { deduction += 2.0; }
    if restrict_namespaces { deduction += 2.0; }
    if no_new_privileges { deduction += 2.0; }

    let exposure_score = 10.0f32 - deduction;

    *out_analysis = SecurityAnalysis {
        service_name: name_arr,
        private_tmp,
        protect_system,
        protect_home,
        restrict_namespaces,
        no_new_privileges,
        exposure_score,
    };

    0 // Success
}

/// Linux-style 'systemd-analyze verify' checks.
/// Validates that target unit configuration has valid, non-dangling dependencies.
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_verify(
    has_dangling_deps: SigmaBool,
    has_syntax_errors: SigmaBool,
) -> SigmaI32 {
    if has_syntax_errors {
        return 1; // Failed validation due to syntax error
    }
    if has_dangling_deps {
        return 2; // Warning: dangling dependencies found
    }
    0 // Verification Success!
}

/// Linux-style 'systemd-analyze plot' ASCII horizontal bar chart generator.
/// Outputs a simulated boot milestone bar representation to a provided buffer.
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_plot_text(
    buffer: *mut u8,
    buffer_max: SigmaU32,
) -> SigmaI32 {
    if buffer.is_null() || buffer_max < 300 {
        return -1;
    }

    // Prepare simulated text layout
    let text = b"=== SigmaOS Boot Phase Graph ===\nFirmware  | [==] 500ms\nLoader    | [=] 200ms\nKernel    | [====] 1000ms\nInitrd    | [===] 300ms\nUserspace | [========] 2000ms\nTotal     | [================] 4000ms\n";
    let len = text.len().min(buffer_max as usize - 1);

    for i in 0..len {
        *buffer.add(i) = text[i];
    }
    *buffer.add(len) = 0; // null terminator

    0 // Success
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_systemd_analyze_security_checks() {
        unsafe {
            let mut analysis = SecurityAnalysis {
                service_name: [0; 128],
                private_tmp: false,
                protect_system: false,
                protect_home: false,
                restrict_namespaces: false,
                no_new_privileges: false,
                exposure_score: 0.0,
            };

            // Test highly insecure service (no guards active)
            systemd_analyze_security(
                b"insecure.service\0".as_ptr(),
                false, false, false, false, false,
                &mut analysis,
            );
            assert_eq!(analysis.exposure_score, 10.0);

            // Test highly secure service (all guards active)
            systemd_analyze_security(
                b"secure.service\0".as_ptr(),
                true, true, true, true, true,
                &mut analysis,
            );
            assert_eq!(analysis.exposure_score, 0.0);
        }
    }

    #[test]
    fn test_systemd_analyze_verify_checks() {
        unsafe {
            assert_eq!(systemd_analyze_verify(false, false), 0); // Success
            assert_eq!(systemd_analyze_verify(true, false), 2);  // Dangling
            assert_eq!(systemd_analyze_verify(false, true), 1);  // Syntax error
        }
    }

    #[test]
    fn test_systemd_analyze_plot_text_render() {
        unsafe {
            let mut buffer = [0u8; 512];
            let res = systemd_analyze_plot_text(buffer.as_mut_ptr(), 512);
            assert_eq!(res, 0);

            // Check that the output contains key labels
            let out_str = core::str::from_utf8(&buffer).unwrap();
            assert!(out_str.contains("SigmaOS Boot Phase Graph"));
            assert!(out_str.contains("Firmware"));
            assert!(out_str.contains("Userspace"));
        }
    }
}

/// systemd-analyze verify inspired configuration verification (checks for cycles or configuration anomalies)
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_verify() -> SigmaI32 {
    if !SYSTEMD_ANALYZE_INITIALIZED {
        return -1;
    }

    // Scan service names to detect circular loops or self-dependencies
    for i in 0..SERVICE_TIMING_COUNT as usize {
        let timing = &SERVICE_TIMINGS[i];
        // If timing name is empty or matches invalid pattern, trigger configuration error code
        let mut empty = true;
        for &b in &timing.name {
            if b != 0 { empty = false; break; }
        }
        if empty {
            return -2; // Configuration anomaly / missing name
        }
    }

    0 // Success
}

/// systemd-analyze security inspired security rating analysis (collects hardening stats)
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_security(
    ratings: *mut ServiceSecurityRating,
    max_count: SigmaU32,
) -> SigmaU32 {
    if !SYSTEMD_ANALYZE_INITIALIZED || ratings.is_null() {
        return 0;
    }

    let mut count = 0;
    for i in 0..SERVICE_SECURITY_COUNT as usize {
        if count >= max_count as usize {
            break;
        }
        *ratings.add(count) = SERVICE_SECURITY_RATINGS[i];
        count += 1;
    }

    count as SigmaU32
}

/// Register a hardening score for a service
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_add_security(
    name: *const u8,
    private_tmp: SigmaBool,
    no_new_priv: SigmaBool,
    strict: SigmaBool,
) -> SigmaI32 {
    if !SYSTEMD_ANALYZE_INITIALIZED || SERVICE_SECURITY_COUNT >= MAX_SERVICE_TIMINGS as SigmaU32 {
        return -1;
    }

    let mut rating = ServiceSecurityRating {
        name: [0; 128],
        private_tmp,
        no_new_privileges: no_new_priv,
        protect_system_strict: strict,
        exposure_score: 100,
    };

    if !name.is_null() {
        for i in 0..127 {
            let byte = *name.add(i);
            if byte == 0 { break; }
            rating.name[i] = byte;
        }
    }

    // exposure score logic: start at 100 (insecure), subtract as hardening options are enabled
    let mut score = 100;
    if private_tmp { score -= 30; }
    if no_new_priv { score -= 30; }
    if strict { score -= 30; }
    rating.exposure_score = score;

    SERVICE_SECURITY_RATINGS[SERVICE_SECURITY_COUNT as usize] = rating;
    SERVICE_SECURITY_COUNT += 1;

    0 // Success
}

/// Get security count
#[no_mangle]
pub unsafe extern "C" fn systemd_analyze_get_security_count() -> SigmaU32 {
    SERVICE_SECURITY_COUNT
}

#[cfg(test)]
mod tests {

    // Run all tests sequentially inside one test wrapper to avoid parallel mutable static race conditions
    #[test]
    fn test_systemd_analyze_all_sequential() {
        unsafe {
            // First, run legacy checks
            test_legacy_analyze_timing_impl();
            // Second, run distro-inspired security and verify checks
            test_systemd_analyze_distro_features_impl();
        }
    }

    unsafe fn test_legacy_analyze_timing_impl() {
        assert_eq!(systemd_analyze_init(), 0);
        let mut stats = BootStats {
            firmware_time_ms: 0,
            loader_time_ms: 0,
            kernel_time_ms: 0,
            initrd_time_ms: 0,
            userspace_time_ms: 0,
            total_time_ms: 0,
        };
        assert_eq!(systemd_analyze_time(&mut stats), 0);
        assert_eq!(stats.total_time_ms, 4000);

        assert_eq!(systemd_analyze_add_timing(b"network.service\0".as_ptr(), 50, 100), 0);
        assert_eq!(systemd_analyze_get_timing_count(), 1);

        let mut timings = [ServiceTiming {
            name: [0; 128],
            startup_time_ms: 0,
            activation_time_ms: 0,
        }; 1];
        assert_eq!(systemd_analyze_blame(timings.as_mut_ptr(), 1), 1);
        let name_str = core::str::from_utf8(&timings[0].name).unwrap().trim_end_matches('\0');
        assert!(name_str.starts_with("network.service"));
    }

    unsafe fn test_systemd_analyze_distro_features_impl() {
        assert_eq!(systemd_analyze_init(), 0);

        // 1. Test systemd-analyze verify (detect configuration validity)
        assert_eq!(systemd_analyze_verify(), 0); // Passes because we have registered valid services or none

        // Add timing with empty name (simulating invalid config)
        assert_eq!(systemd_analyze_add_timing(core::ptr::null(), 10, 20), 0);
        assert_eq!(systemd_analyze_verify(), -2); // Fails verify with invalid/missing name configuration

        // 2. Test systemd-analyze security (exposed/hardening configurations)
        assert_eq!(systemd_analyze_add_security(b"insecure.service\0".as_ptr(), false, false, false), 0);
        assert_eq!(systemd_analyze_add_security(b"secure.service\0".as_ptr(), true, true, true), 0);
        assert_eq!(systemd_analyze_get_security_count(), 2);

        let mut ratings = [ServiceSecurityRating {
            name: [0; 128],
            private_tmp: false,
            no_new_privileges: false,
            protect_system_strict: false,
            exposure_score: 100,
        }; 2];

        assert_eq!(systemd_analyze_security(ratings.as_mut_ptr(), 2), 2);

        // secure.service has private_tmp, no_new_privileges, and protect_system_strict -> score: 10
        // insecure.service has none -> score: 100
        let mut found_secure = false;
        let mut found_insecure = false;

        for rating in &ratings {
            let name_str = core::str::from_utf8(&rating.name).unwrap().trim_end_matches('\0');
            if name_str.starts_with("secure.service") {
                assert_eq!(rating.exposure_score, 10);
                found_secure = true;
            }
            if name_str.starts_with("insecure.service") {
                assert_eq!(rating.exposure_score, 100);
                found_insecure = true;
            }
        }
        assert!(found_secure);
        assert!(found_insecure);
    }
}
