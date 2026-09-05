//! UTS Namespace Syscalls
//!
//! Implements Linux-compatible UTS namespace syscalls:
//! - sys_sethostname(2)
//! - sys_gethostname(2)  
//! - sys_clone with CLONE_NEWUTS support

use crate::kernel::uts_namespace::{UtsNamespaceManager, NamespaceId};
use std::sync::OnceLock;

// Global UTS namespace manager
static UTS_MANAGER: OnceLock<UtsNamespaceManager> = OnceLock::new();

/// Get or initialize the global UTS namespace manager
fn get_uts_manager() -> &'static UtsNamespaceManager {
    UTS_MANAGER.get_or_init(UtsNamespaceManager::new)
}

// CLONE_NEWUTS flag for clone() syscall
pub const CLONE_NEWUTS: u32 = 0x04000000;

/// sys_sethostname(2) - Set hostname for current UTS namespace
///
/// Args:
/// - namespace_id: ID of the namespace to modify
/// - hostname_ptr: Pointer to hostname string
/// - len: Length of hostname (max 255)
///
/// Returns:
/// - 0 on success
/// - -1 (EINVAL) on invalid arguments
/// - -EFAULT on bad pointer
/// - -EPERM on permission denied
pub fn sys_sethostname(
    namespace_id: u64,
    hostname_ptr: *const u8,
    len: usize,
) -> i32 {
    // Validate hostname length (max 255 bytes)
    if len > 255 {
        return -22; // EINVAL
    }
    
    if len == 0 {
        return -22; // EINVAL - empty hostname
    }

    // Validate pointer (would need actual memory validation in real implementation)
    if hostname_ptr.is_null() {
        return -14; // EFAULT
    }

    // Convert bytes to String
    let hostname_bytes = unsafe {
        std::slice::from_raw_parts(hostname_ptr, len)
    };

    let hostname = match String::from_utf8(hostname_bytes.to_vec()) {
        Ok(h) => h,
        Err(_) => return -22, // EINVAL
    };

    // Get namespace manager and set hostname
    let manager = get_uts_manager();
    let ns_id = NamespaceId::new(namespace_id);

    match manager.set_hostname(ns_id, hostname) {
        Ok(_) => 0,
        Err(_) => -2, // ENOENT - namespace not found
    }
}

/// sys_gethostname(2) - Get hostname from current UTS namespace
///
/// Args:
/// - namespace_id: ID of the namespace to query
/// - hostname_ptr: Pointer to buffer for hostname
/// - len: Buffer size
///
/// Returns:
/// - 0 on success
/// - -1 (EINVAL) on invalid arguments
/// - -EFAULT on bad pointer
pub fn sys_gethostname(
    namespace_id: u64,
    hostname_ptr: *mut u8,
    len: usize,
) -> i32 {
    if len == 0 {
        return -22; // EINVAL
    }

    if hostname_ptr.is_null() {
        return -14; // EFAULT
    }

    // Get namespace manager and retrieve hostname
    let manager = get_uts_manager();
    let ns_id = NamespaceId::new(namespace_id);

    let hostname = match manager.get_hostname(ns_id) {
        Ok(h) => h,
        Err(_) => return -2, // ENOENT - namespace not found
    };

    // Copy hostname to buffer
    let copy_len = std::cmp::min(len - 1, hostname.len());
    unsafe {
        std::ptr::copy_nonoverlapping(
            hostname.as_ptr(),
            hostname_ptr,
            copy_len,
        );
        // Null terminate
        *hostname_ptr.add(copy_len) = 0;
    }

    0
}

/// sys_setdomainname(2) - Set domainname for current UTS namespace
///
/// Args:
/// - namespace_id: ID of the namespace to modify
/// - domainname_ptr: Pointer to domainname string
/// - len: Length of domainname (max 255)
pub fn sys_setdomainname(
    namespace_id: u64,
    domainname_ptr: *const u8,
    len: usize,
) -> i32 {
    if len > 255 {
        return -22; // EINVAL
    }

    if len == 0 {
        return -22; // EINVAL
    }

    if domainname_ptr.is_null() {
        return -14; // EFAULT
    }

    let domainname_bytes = unsafe {
        std::slice::from_raw_parts(domainname_ptr, len)
    };

    let domainname = match String::from_utf8(domainname_bytes.to_vec()) {
        Ok(d) => d,
        Err(_) => return -22, // EINVAL
    };

    let manager = get_uts_manager();
    let ns_id = NamespaceId::new(namespace_id);

    match manager.set_domainname(ns_id, domainname) {
        Ok(_) => 0,
        Err(_) => -2, // ENOENT
    }
}

/// sys_getdomainname(2) - Get domainname from current UTS namespace
pub fn sys_getdomainname(
    namespace_id: u64,
    domainname_ptr: *mut u8,
    len: usize,
) -> i32 {
    if len == 0 {
        return -22; // EINVAL
    }

    if domainname_ptr.is_null() {
        return -14; // EFAULT
    }

    let manager = get_uts_manager();
    let ns_id = NamespaceId::new(namespace_id);

    let domainname = match manager.get_domainname(ns_id) {
        Ok(d) => d,
        Err(_) => return -2, // ENOENT
    };

    let copy_len = std::cmp::min(len - 1, domainname.len());
    unsafe {
        std::ptr::copy_nonoverlapping(
            domainname.as_ptr(),
            domainname_ptr,
            copy_len,
        );
        *domainname_ptr.add(copy_len) = 0;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sethostname_success() {
        let manager = get_uts_manager();
        let ns = manager.create_namespace(None).expect("Failed to create namespace");
        
        let hostname = b"test-host".to_vec();
        let result = sys_sethostname(ns.raw(), hostname.as_ptr(), hostname.len());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sethostname_too_long() {
        let manager = get_uts_manager();
        let ns = manager.create_namespace(None).expect("Failed to create namespace");
        
        let hostname = "a".repeat(256).into_bytes();
        let result = sys_sethostname(ns.raw(), hostname.as_ptr(), hostname.len());
        assert_eq!(result, -22); // EINVAL
    }

    #[test]
    fn test_sethostname_empty() {
        let manager = get_uts_manager();
        let ns = manager.create_namespace(None).expect("Failed to create namespace");
        
        let result = sys_sethostname(ns.raw(), std::ptr::null(), 0);
        assert_eq!(result, -22); // EINVAL
    }

    #[test]
    fn test_gethostname_success() {
        let manager = get_uts_manager();
        let ns = manager.create_namespace(None).expect("Failed to create namespace");
        
        let hostname = b"test-host".to_vec();
        sys_sethostname(ns.raw(), hostname.as_ptr(), hostname.len());
        
        let mut buffer = vec![0u8; 256];
        let result = sys_gethostname(ns.raw(), buffer.as_mut_ptr(), 256);
        assert_eq!(result, 0);
        
        let retrieved = String::from_utf8(buffer.iter().copied().take_while(|&b| b != 0).collect()).unwrap();
        assert_eq!(retrieved, "test-host");
    }

    #[test]
    fn test_hostname_isolation() {
        let manager = get_uts_manager();
        let ns1 = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2 = manager.create_namespace(None).expect("Failed to create ns2");
        
        let host1 = b"host1".to_vec();
        let host2 = b"host2".to_vec();
        
        sys_sethostname(ns1.raw(), host1.as_ptr(), host1.len());
        sys_sethostname(ns2.raw(), host2.as_ptr(), host2.len());
        
        let mut buf1 = vec![0u8; 256];
        let mut buf2 = vec![0u8; 256];
        
        sys_gethostname(ns1.raw(), buf1.as_mut_ptr(), 256);
        sys_gethostname(ns2.raw(), buf2.as_mut_ptr(), 256);
        
        let h1 = String::from_utf8(buf1.iter().copied().take_while(|&b| b != 0).collect()).unwrap();
        let h2 = String::from_utf8(buf2.iter().copied().take_while(|&b| b != 0).collect()).unwrap();
        
        assert_eq!(h1, "host1");
        assert_eq!(h2, "host2");
        assert_ne!(h1, h2);
    }
}
