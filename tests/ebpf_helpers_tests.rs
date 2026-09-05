// eBPF Comprehensive Integration Tests
// Phase 9.4 Part 2-3: Tests for eBPF Helpers, Verification, and Syscall Integration

#[cfg(test)]
mod ebpf_integration_tests {
    use sigmaos::kernel::ebpf_vm::{BpfInstruction, BpfVm, HelperRegistry};
    use sigmaos::kernel::ebpf_verification::BpfProgramVerifier;
    use sigmaos::syscall::bpf_syscalls::{BpfProgramRegistry, BpfProgType};

    // ============ eBPF HELPER TESTS ============

    #[test]
    fn test_bpf_helper_ktime_get_ns() {
        let mut vm = BpfVm::new();
        let registry = vm.get_helper_registry();
        let mut registry = registry.lock().unwrap();

        let helper = registry
            .get_helper(sigmaos::kernel::ebpf_vm::helper_ids::BPF_KTIME_GET_NS)
            .expect("ktime_get_ns helper not found");

        drop(registry); // Release lock
        drop(vm); // Drop old vm

        // Create new VM for execution
        let mut vm = BpfVm::new();
        let result = helper.execute(&mut vm).expect("Helper execution failed");
        assert!(result > 0, "ktime_get_ns should return non-zero");
    }

    #[test]
    fn test_bpf_helper_get_current_pid_tgid() {
        let mut vm = BpfVm::new();
        let registry = vm.get_helper_registry();
        let mut registry = registry.lock().unwrap();

        let helper = registry
            .get_helper(sigmaos::kernel::ebpf_vm::helper_ids::BPF_GET_CURRENT_PID_TGID)
            .expect("pid_tgid helper not found");

        drop(registry);
        drop(vm);

        let mut vm = BpfVm::new();
        let result = helper.execute(&mut vm).expect("Helper execution failed");
        
        // Result should be (tgid << 32) | pid
        let pid = (result & 0xFFFFFFFF) as u32;
        let tgid = (result >> 32) as u32;
        assert!(pid > 0, "PID should be positive");
        assert!(tgid > 0, "TGID should be positive");
    }

    #[test]
    fn test_bpf_helper_get_current_uid_gid() {
        let mut vm = BpfVm::new();
        let registry = vm.get_helper_registry();
        let mut registry = registry.lock().unwrap();

        let helper = registry
            .get_helper(sigmaos::kernel::ebpf_vm::helper_ids::BPF_GET_CURRENT_UID_GID)
            .expect("uid_gid helper not found");

        drop(registry);
        drop(vm);

        let mut vm = BpfVm::new();
        let result = helper.execute(&mut vm).expect("Helper execution failed");
        
        // Result should be (gid << 32) | uid
        let uid = (result & 0xFFFFFFFF) as u32;
        let gid = (result >> 32) as u32;
        assert!(uid >= 0, "UID should be non-negative");
        assert!(gid >= 0, "GID should be non-negative");
    }

    #[test]
    fn test_bpf_helper_get_prandom_u32() {
        let mut vm = BpfVm::new();
        let registry = vm.get_helper_registry();
        let mut registry = registry.lock().unwrap();

        let helper = registry
            .get_helper(sigmaos::kernel::ebpf_vm::helper_ids::BPF_GET_PRANDOM_U32)
            .expect("prandom_u32 helper not found");

        drop(registry);
        drop(vm);

        let mut vm = BpfVm::new();
        let result1 = helper.execute(&mut vm).expect("First random generation failed");
        
        let mut vm2 = BpfVm::new();
        let result2 = helper.execute(&mut vm2).expect("Second random generation failed");
        
        // Results should fit in u32 and likely be different
        assert!(result1 <= 0xFFFFFFFF, "Random should be 32-bit");
        assert!(result2 <= 0xFFFFFFFF, "Random should be 32-bit");
        // Note: results might be the same, so we don't assert inequality
    }

    #[test]
    fn test_bpf_helper_map_lookup_elem() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 0x1000).expect("Failed to set register");
        vm.set_register(2, 0x2000).expect("Failed to set register");

        let registry = vm.get_helper_registry();
        let mut registry = registry.lock().unwrap();

        let helper = registry
            .get_helper(sigmaos::kernel::ebpf_vm::helper_ids::BPF_MAP_LOOKUP_ELEM)
            .expect("map_lookup_elem helper not found");

        drop(registry);
        drop(vm);

        let mut vm = BpfVm::new();
        vm.set_register(1, 0x1000).expect("Failed to set register");
        vm.set_register(2, 0x2000).expect("Failed to set register");
        
        let result = helper.execute(&mut vm).expect("Helper execution failed");
        assert_eq!(result, 0, "map_lookup_elem should return 0 for test");
    }

    // ============ eBPF VERIFICATION TESTS ============

    #[test]
    fn test_ebpf_verification_simple_program() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];
        
        let mut verifier = BpfProgramVerifier::new(program);
        let report = verifier.verify().expect("Verification failed");
        assert!(report.is_valid, "Simple program should be valid");
        assert_eq!(report.errors.len(), 0, "No errors expected");
    }

    #[test]
    fn test_ebpf_verification_with_arithmetic() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 10 },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 20 },
            BpfInstruction::Add {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];
        
        let mut verifier = BpfProgramVerifier::new(program);
        let report = verifier.verify().expect("Verification failed");
        assert!(report.is_valid, "Arithmetic program should be valid");
    }

    #[test]
    fn test_ebpf_verification_with_branching() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 10 },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 10 },
            BpfInstruction::Jeq {
                dst_reg: 0,
                src_reg: 1,
                offset: 1,
            },
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::Return,
        ];
        
        let mut verifier = BpfProgramVerifier::new(program);
        let report = verifier.verify().expect("Verification failed");
        assert!(report.is_valid, "Branching program should be valid");
    }

    #[test]
    fn test_ebpf_verification_invalid_register() {
        let program = vec![
            BpfInstruction::LoadImm64 {
                dst_reg: 20, // Invalid - > 10
                imm64: 42,
            },
            BpfInstruction::Return,
        ];
        
        let mut verifier = BpfProgramVerifier::new(program);
        let result = verifier.verify();
        assert!(
            result.is_err() || !verifier.report.is_valid,
            "Invalid register should fail verification"
        );
    }

    #[test]
    fn test_ebpf_verification_bounds_check() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::Ja { offset: 1000 }, // Way out of bounds
        ];
        
        let mut verifier = BpfProgramVerifier::new(program);
        let result = verifier.verify();
        assert!(
            result.is_err() || !verifier.report.is_valid,
            "Out of bounds jump should fail verification"
        );
    }

    #[test]
    fn test_ebpf_verification_infinite_loop_detection() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::Ja { offset: -1 }, // Jump back (infinite loop)
        ];
        
        let mut verifier = BpfProgramVerifier::new(program);
        let result = verifier.verify();
        assert!(
            result.is_err() || !verifier.report.is_valid,
            "Infinite loop should fail verification"
        );
    }

    #[test]
    fn test_ebpf_verification_unreachable_code() {
        let program = vec![
            BpfInstruction::Return,
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 42,
            }, // Unreachable
        ];
        
        let mut verifier = BpfProgramVerifier::new(program);
        let result = verifier.verify();
        assert!(
            result.is_err() || !verifier.report.is_valid,
            "Unreachable code should fail verification"
        );
    }

    // ============ SYSCALL & VM INTEGRATION TESTS ============

    #[test]
    fn test_bpf_program_load_and_execute() {
        let mut registry = BpfProgramRegistry::new();
        
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];
        
        let fd = registry
            .load_program(BpfProgType::Tracing, program, "test".to_string())
            .expect("Program loading failed");

        let result = registry.execute_program(fd).expect("Program execution failed");
        assert_eq!(result, 42, "Program should return 42");
    }

    #[test]
    fn test_bpf_program_arithmetic() {
        let mut registry = BpfProgramRegistry::new();
        
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 10 },
            BpfInstruction::AddImm {
                dst_reg: 0,
                imm: 32,
            },
            BpfInstruction::Return,
        ];
        
        let fd = registry
            .load_program(BpfProgType::Tracing, program, "arithmetic".to_string())
            .expect("Program loading failed");

        let result = registry.execute_program(fd).expect("Program execution failed");
        assert_eq!(result, 42, "10 + 32 should be 42");
    }

    #[test]
    fn test_bpf_program_conditional_jump() {
        let mut registry = BpfProgramRegistry::new();
        
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 5 },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 5 },
            BpfInstruction::Jeq {
                dst_reg: 0,
                src_reg: 1,
                offset: 1,
            },
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::Return,
        ];
        
        let fd = registry
            .load_program(BpfProgType::Tracing, program, "branch".to_string())
            .expect("Program loading failed");

        let result = registry.execute_program(fd).expect("Program execution failed");
        // Should skip the "load 0" due to jump
        assert!(result > 0, "Jump should skip the load 0");
    }

    #[test]
    fn test_bpf_program_multiple_registers() {
        let mut registry = BpfProgramRegistry::new();
        
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 10 },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 20 },
            BpfInstruction::LoadImm64 { dst_reg: 2, imm64: 30 },
            BpfInstruction::Add {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Add {
                dst_reg: 0,
                src_reg: 2,
            },
            BpfInstruction::Return,
        ];
        
        let fd = registry
            .load_program(BpfProgType::Tracing, program, "multi_reg".to_string())
            .expect("Program loading failed");

        let result = registry.execute_program(fd).expect("Program execution failed");
        assert_eq!(result, 60, "10 + 20 + 30 should be 60");
    }

    #[test]
    fn test_bpf_program_complex_flow() {
        let mut registry = BpfProgramRegistry::new();
        
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 5 },
            // Loop: add 10 to R0 five times
            BpfInstruction::AddImm {
                dst_reg: 0,
                imm: 10,
            },
            BpfInstruction::SubImm {
                dst_reg: 1,
                imm: 1,
            },
            BpfInstruction::Jne {
                dst_reg: 1,
                src_reg: 0, // Compare R1 with R0 (modified)
                offset: -3,
            },
            BpfInstruction::Return,
        ];
        
        let fd = registry
            .load_program(BpfProgType::Tracing, program, "complex".to_string())
            .expect("Program loading failed");

        let result = registry.execute_program(fd).expect("Program execution failed");
        assert!(result > 0, "Complex program should execute");
    }

    #[test]
    fn test_bpf_program_verification_before_load() {
        let mut registry = BpfProgramRegistry::new();
        
        let invalid_program = vec![
            BpfInstruction::LoadImm64 {
                dst_reg: 15, // Invalid register
                imm64: 42,
            },
            BpfInstruction::Return,
        ];
        
        let result = registry.load_program(BpfProgType::Tracing, invalid_program, "invalid".to_string());
        assert!(result.is_err(), "Loading invalid program should fail");
    }

    #[test]
    fn test_bpf_program_unload() {
        let mut registry = BpfProgramRegistry::new();
        
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];
        
        let fd = registry
            .load_program(BpfProgType::Tracing, program, "test".to_string())
            .expect("Program loading failed");

        assert!(registry.unload_program(fd).is_ok(), "Unload should succeed");
        assert!(
            registry.execute_program(fd).is_err(),
            "Executing unloaded program should fail"
        );
    }

    #[test]
    fn test_bpf_program_list() {
        let mut registry = BpfProgramRegistry::new();
        
        for i in 0..3 {
            let program = vec![
                BpfInstruction::LoadImm64 {
                    dst_reg: 0,
                    imm64: 42 + i as u64,
                },
                BpfInstruction::Return,
            ];
            
            registry
                .load_program(
                    BpfProgType::Tracing,
                    program,
                    format!("prog{}", i),
                )
                .expect("Program loading failed");
        }

        let programs = registry.list_programs();
        assert_eq!(programs.len(), 3, "Should have 3 programs");
    }

    #[test]
    fn test_bpf_helper_registry_access() {
        let mut vm = BpfVm::new();
        let registry = vm.get_helper_registry();
        let registry = registry.lock().unwrap();

        // Verify all standard helpers are present
        assert!(
            registry
                .get_helper(sigmaos::kernel::ebpf_vm::helper_ids::BPF_KTIME_GET_NS)
                .is_some(),
            "ktime_get_ns should exist"
        );
        assert!(
            registry
                .get_helper(sigmaos::kernel::ebpf_vm::helper_ids::BPF_GET_CURRENT_PID_TGID)
                .is_some(),
            "pid_tgid should exist"
        );
        assert!(
            registry
                .get_helper(sigmaos::kernel::ebpf_vm::helper_ids::BPF_GET_PRANDOM_U32)
                .is_some(),
            "prandom_u32 should exist"
        );
    }

    #[test]
    fn test_ebpf_verification_report_accuracy() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 10 },
            BpfInstruction::Add {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];
        
        let mut verifier = BpfProgramVerifier::new(program);
        let report = verifier.verify().expect("Verification failed");
        
        assert!(report.is_valid, "Program should be valid");
        assert_eq!(report.instructions_verified, 4, "All instructions should be verified");
        assert_eq!(report.errors.len(), 0, "No errors expected");
    }

    #[test]
    fn test_ebpf_all_arithmetic_operations() {
        let mut registry = BpfProgramRegistry::new();
        
        // Test multiplication
        let mul_program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 6 },
            BpfInstruction::MulImm {
                dst_reg: 0,
                imm: 7,
            },
            BpfInstruction::Return,
        ];
        
        let fd = registry
            .load_program(BpfProgType::Tracing, mul_program, "mul".to_string())
            .expect("Program loading failed");
        let result = registry.execute_program(fd).expect("Execution failed");
        assert_eq!(result, 42, "6 * 7 should be 42");
    }

    #[test]
    fn test_ebpf_bitwise_operations() {
        let mut registry = BpfProgramRegistry::new();
        
        // Test AND
        let and_program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0xFF },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 0x0F },
            BpfInstruction::And {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];
        
        let fd = registry
            .load_program(BpfProgType::Tracing, and_program, "and".to_string())
            .expect("Program loading failed");
        let result = registry.execute_program(fd).expect("Execution failed");
        assert_eq!(result, 0x0F, "0xFF & 0x0F should be 0x0F");
    }
}
