// Integration test for SigmaOS System-Call Development and Maintenance Plan
// (`tests/test_syscall_development_plan.rs`)

#[path = "../src/syscall/sovereign_syscall_development_and_pr_suite.rs"]
mod sovereign_syscall_development_and_pr_suite;

use sovereign_syscall_development_and_pr_suite::*;

#[test]
fn test_syscall_development_plan_17_prs() {
    let tracker = SyscallPrTracker::new();
    assert_eq!(tracker.total_prs(), 17);

    for pr_num in 1..=17 {
        let pr = tracker
            .get_pr(pr_num)
            .unwrap_or_else(|| panic!("PR {} missing from tracker", pr_num));
        assert_eq!(pr.pr_number, pr_num);
        assert!(!pr.title.is_empty());
        assert!(!pr.scope.is_empty());
        assert!(!pr.acceptance_criteria.is_empty());
    }

    assert_eq!(
        tracker.get_pr(1).unwrap().status,
        SyscallSupportStatus::Supported
    );
    assert_eq!(
        tracker.get_pr(4).unwrap().status,
        SyscallSupportStatus::Prototype
    );
}

#[test]
fn test_syscall_abi_policy_governor_uniqueness_and_validation() {
    let gov = SyscallAbiPolicyGovernor::new();

    // Verify canonical published syscall numbers cannot be reassigned
    assert!(gov.validate_number_uniqueness(0, "read").is_ok());
    assert!(gov.validate_number_uniqueness(1, "write").is_ok());
    assert!(gov.validate_number_uniqueness(60, "exit").is_ok());
    assert!(gov.validate_number_uniqueness(60, "invalid_dup").is_err());

    // Verify flag extension validation
    assert!(SyscallAbiPolicyGovernor::validate_extension_flags(0x00, 0x07).is_ok());
    assert!(SyscallAbiPolicyGovernor::validate_extension_flags(0x03, 0x07).is_ok());
    assert!(SyscallAbiPolicyGovernor::validate_extension_flags(0x08, 0x07).is_err());

    // Verify versioned structure size validation
    assert!(SyscallAbiPolicyGovernor::validate_versioned_struct_size(32, 16, 64).is_ok());
    assert!(SyscallAbiPolicyGovernor::validate_versioned_struct_size(8, 16, 64).is_err());
    assert!(SyscallAbiPolicyGovernor::validate_versioned_struct_size(128, 16, 64).is_err());
}

#[test]
fn test_user_memory_access_validation_primitives() {
    let mem = UserMemoryAccessValidationPrimitive::new();

    // Valid canonical user pointer
    let valid_user_ptr = 0x0000_7FFF_0000_1000;
    assert!(mem.validate_user_pointer(valid_user_ptr, 1024).is_ok());

    // Invalid null pointer
    assert!(mem.validate_user_pointer(0, 64).is_err());

    // Invalid higher-half kernel pointer
    let kernel_ptr = 0xFFFF_8000_0000_0000;
    assert!(mem.validate_user_pointer(kernel_ptr, 64).is_err());

    // Cross-boundary overflow
    let boundary_ptr = 0x0000_7FFF_FFFF_FF80;
    assert!(mem.validate_user_pointer(boundary_ptr, 512).is_err());

    // Copy operations
    let mut buf = [0u8; 32];
    assert!(mem.copy_from_user(&mut buf, valid_user_ptr).is_ok());
    assert!(mem.copy_to_user(&buf, valid_user_ptr).is_ok());
}

#[test]
fn test_syscall_dispatch_and_unknown_call_handling() {
    let mut disp = SyscallDispatchAndUnknownCallHandler::new(512);

    // Known call dispatch
    assert_eq!(disp.dispatch(0, &[]), Ok(0)); // read
    assert_eq!(disp.dispatch(1, &[]), Ok(10)); // write
    assert_eq!(disp.dispatch(39, &[]), Ok(1001)); // getpid

    // Unknown call
    assert!(disp.dispatch(500, &[]).is_err());

    // Out of bounds call
    assert!(disp.dispatch(999, &[]).is_err());

    // Explicitly disabled call
    disp.disable_syscall(39);
    assert!(disp.dispatch(39, &[]).is_err());

    // Re-enabled call
    disp.enable_syscall(39);
    assert_eq!(disp.dispatch(39, &[]), Ok(1001));
}

#[test]
fn test_x86_64_entry_frame_and_sysret() {
    let frame = X86_64SyscallRegisterFrame {
        rax: 39,
        rdi: 0,
        rsi: 0,
        rdx: 0,
        r10: 0,
        r8: 0,
        r9: 0,
        rip: 0x0000_7FFF_0001_0000,
        rsp: 0x0000_7FFF_000F_0000,
        rflags: 0x202,
    };

    assert!(X86_64SyscallEntryStateAndReturnPathValidator::validate_entry_frame(&frame).is_ok());

    let ret_frame =
        X86_64SyscallEntryStateAndReturnPathValidator::prepare_sysret_frame(frame, 1001);
    assert_eq!(ret_frame.rax, 1001);

    // Invalid kernel-space RSP in user frame
    let bad_frame = X86_64SyscallRegisterFrame {
        rsp: 0xFFFF_8000_0000_0000,
        ..frame
    };
    assert!(
        X86_64SyscallEntryStateAndReturnPathValidator::validate_entry_frame(&bad_frame).is_err()
    );
}

#[test]
fn test_uapi_errno_mapping() {
    assert_eq!(
        SyscallUapiTypeAndErrorConventionGovernor::map_kernel_error_to_errno("EPERM"),
        -1
    );
    assert_eq!(
        SyscallUapiTypeAndErrorConventionGovernor::map_kernel_error_to_errno("EFAULT"),
        -14
    );
    assert_eq!(
        SyscallUapiTypeAndErrorConventionGovernor::map_kernel_error_to_errno("EINVAL"),
        -22
    );
    assert_eq!(
        SyscallUapiTypeAndErrorConventionGovernor::map_kernel_error_to_errno("ENOSYS"),
        -38
    );
}

#[test]
fn test_master_syscall_development_suite() {
    let mut master = SovereignSyscallDevelopmentAndPrMasterSuite::new();
    assert!(master.run_complete_verification_suite());
}
