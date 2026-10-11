// Standalone unit tests for Virtual Memory Manager

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MmError {
    NotSupported,
    InvalidParam,
    NotFound,
    PermissionDenied,
    OutOfMemory,
    IoError,
    Unknown,
}

#[path = "../../src/mm/virtual.rs"]
pub mod virtual_mgr;

use virtual_mgr::{VirtualMemoryManager, VmaProtection, USER_SPACE_START, KERNEL_SPACE_START};

#[test]
fn test_vma_user_kernel_isolation() {
    let vmm = VirtualMemoryManager::new(1024 * 1024, 10);

    let user_prot = VmaProtection { read: true, write: true, execute: false, user: true };
    let kernel_prot = VmaProtection { read: true, write: true, execute: false, user: false };

    // Valid user space VMA creation
    let vma_user = vmm.create_vma(USER_SPACE_START, 0x1000, user_prot, "user_heap").expect("User VMA creation should succeed");
    assert_eq!(vma_user.start_addr, USER_SPACE_START);

    // Creating user-accessible VMA in kernel region must fail with PermissionDenied
    let invalid_kernel_vma = vmm.create_vma(KERNEL_SPACE_START, 0x1000, user_prot, "bad_kernel");
    assert_eq!(invalid_kernel_vma, Err(MmError::PermissionDenied));

    // Valid kernel space VMA creation
    let vma_kernel = vmm.create_vma(KERNEL_SPACE_START, 0x1000, kernel_prot, "kernel_stack").expect("Kernel VMA creation should succeed");
    assert_eq!(vma_kernel.start_addr, KERNEL_SPACE_START);
}

#[test]
fn test_vma_wx_protection_policy() {
    let vmm = VirtualMemoryManager::new(1024 * 1024, 10);

    // Attempting W+X (Write AND Execute) protection must fail under W^X policy
    let wx_prot = VmaProtection { read: true, write: true, execute: true, user: true };
    let res = vmm.create_vma(USER_SPACE_START, 0x1000, wx_prot, "wx_text");
    assert_eq!(res, Err(MmError::PermissionDenied));
}

#[test]
fn test_vma_permission_checks() {
    let vmm = VirtualMemoryManager::new(1024 * 1024, 10);
    let rx_prot = VmaProtection { read: true, write: false, execute: true, user: true };

    vmm.create_vma(USER_SPACE_START, 0x1000, rx_prot, "user_code").unwrap();

    // Read/Exec in user mode should succeed
    assert!(vmm.check_permission(USER_SPACE_START + 0x10, false, true, true).is_ok());

    // Write attempt to read-only code VMA should fail with PermissionDenied
    assert_eq!(vmm.check_permission(USER_SPACE_START + 0x10, true, false, true), Err(MmError::PermissionDenied));

    // Access to unmapped address returns NotFound
    assert_eq!(vmm.check_permission(USER_SPACE_START + 0x2000, false, false, true), Err(MmError::NotFound));
}

#[test]
fn test_vma_oom_limit_enforcement() {
    let vmm = VirtualMemoryManager::new(0x2000, 2); // Max 8192 bytes, max 2 VMAs
    let prot = VmaProtection { read: true, write: true, execute: false, user: true };

    vmm.create_vma(USER_SPACE_START, 0x1000, prot, "vma1").unwrap();

    // Exceeding byte limit returns OutOfMemory
    let oom_res = vmm.create_vma(USER_SPACE_START + 0x1000, 0x2000, prot, "vma2");
    assert_eq!(oom_res, Err(MmError::OutOfMemory));
}
