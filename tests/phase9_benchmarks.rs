// Phase 9 Performance Benchmarking
// Measures performance of all Phase 9 features

#[cfg(test)]
mod phase9_benchmarks {
    use sigmaos::kernel::ebpf_vm::BpfInstruction;
    use sigmaos::kernel::cgroup_controllers::{
        HugetlbController, HugepageSize, PidsController, RdmaController,
    };
    use sigmaos::syscall::bpf_syscalls::{BpfProgramRegistry, BpfProgType};
    use std::time::Instant;

    fn measure_time<F>(f: F) -> u128
    where
        F: FnOnce(),
    {
        let start = Instant::now();
        f();
        start.elapsed().as_micros()
    }

    #[test]
    fn benchmark_ebpf_program_loading() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];

        let mut registry = BpfProgramRegistry::new();

        let duration = measure_time(|| {
            for _ in 0..100 {
                let program = program.clone();
                let _ = registry.load_program(BpfProgType::Tracing, program, "bench".to_string());
            }
        });

        let avg_time = duration / 100;
        println!("eBPF Program Loading: {} µs per program", avg_time);
        assert!(avg_time < 10000, "Program loading should be fast"); // Less than 10ms
    }

    #[test]
    fn benchmark_ebpf_program_execution() {
        let mut registry = BpfProgramRegistry::new();

        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 10 },
            BpfInstruction::AddImm { dst_reg: 0, imm: 32 },
            BpfInstruction::Return,
        ];

        let fd = registry
            .load_program(BpfProgType::Tracing, program, "exec_test".to_string())
            .expect("Loading failed");

        let duration = measure_time(|| {
            for _ in 0..1000 {
                let _ = registry.execute_program(fd);
            }
        });

        let avg_time = duration / 1000;
        println!("eBPF Program Execution: {} µs per execution", avg_time);
        assert!(avg_time < 100, "Execution should be fast"); // Less than 100µs
    }

    #[test]
    fn benchmark_complex_ebpf_program() {
        let mut registry = BpfProgramRegistry::new();

        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 100 },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 50 },
            BpfInstruction::LoadImm64 { dst_reg: 2, imm64: 25 },
            BpfInstruction::Add {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Sub {
                dst_reg: 0,
                src_reg: 2,
            },
            BpfInstruction::Return,
        ];

        let fd = registry
            .load_program(BpfProgType::Tracing, program, "complex".to_string())
            .expect("Loading failed");

        let duration = measure_time(|| {
            for _ in 0..1000 {
                let _ = registry.execute_program(fd);
            }
        });

        let avg_time = duration / 1000;
        println!("Complex eBPF Execution: {} µs per execution", avg_time);
        assert!(avg_time < 200, "Complex execution should be reasonably fast");
    }

    #[test]
    fn benchmark_cgroup_pids_operations() {
        let mut pids = PidsController::new();
        pids.set_max_pids(10000);

        let duration = measure_time(|| {
            for _ in 0..1000 {
                let _ = pids.fork_process();
            }
        });

        let avg_time = duration / 1000;
        println!("PID Allocation: {} µs per process", avg_time);
        assert!(avg_time < 50, "PID allocation should be fast");

        // Cleanup
        for _ in 0..1000 {
            let _ = pids.exit_process();
        }
    }

    #[test]
    fn benchmark_hugetlb_allocation() {
        let mut hugetlb = HugetlbController::new();
        hugetlb.set_limit(HugepageSize::Two, 10 * 1024 * 1024 * 1024);

        let duration = measure_time(|| {
            for _ in 0..1000 {
                let _ = hugetlb.allocate(HugepageSize::Two, 1);
            }
        });

        let avg_time = duration / 1000;
        println!("Hugetlb Allocation: {} µs per allocation", avg_time);
        assert!(avg_time < 100, "Hugetlb allocation should be fast");
    }

    #[test]
    fn benchmark_rdma_qp_allocation() {
        let mut rdma = RdmaController::new();
        rdma.set_qp_limit(10000);

        let duration = measure_time(|| {
            for _ in 0..1000 {
                let _ = rdma.allocate_qp();
            }
        });

        let avg_time = duration / 1000;
        println!("RDMA QP Allocation: {} µs per allocation", avg_time);
        assert!(avg_time < 50, "RDMA QP allocation should be fast");
    }

    #[test]
    fn benchmark_sequential_program_loading() {
        let mut registry = BpfProgramRegistry::new();

        let base_time = Instant::now();

        for i in 0..50 {
            let program = vec![
                BpfInstruction::LoadImm64 {
                    dst_reg: 0,
                    imm64: i as u64,
                },
                BpfInstruction::Return,
            ];

            let _ = registry.load_program(
                BpfProgType::Tracing,
                program,
                format!("prog{}", i),
            );
        }

        let total_time = base_time.elapsed().as_micros();
        let avg_time = total_time / 50;

        println!("Sequential Program Loading: {} µs per program", avg_time);
        println!("Total for 50 programs: {} µs", total_time);

        let programs = registry.list_programs();
        assert_eq!(programs.len(), 50, "All 50 programs should be loaded");
    }

    #[test]
    fn benchmark_cgroup_enforcement() {
        let mut pids = PidsController::new();
        pids.set_max_pids(1000);

        for _ in 0..100 {
            let _ = pids.fork_process();
        }

        let duration = measure_time(|| {
            for _ in 0..1000 {
                let _ = pids.enforce();
            }
        });

        let avg_time = duration / 1000;
        println!("Cgroup Enforcement: {} µs per check", avg_time);
        assert!(avg_time < 100, "Enforcement should be fast");
    }

    #[test]
    fn benchmark_mixed_operations() {
        let mut registry = BpfProgramRegistry::new();
        let mut pids = PidsController::new();
        let mut rdma = RdmaController::new();

        pids.set_max_pids(1000);
        rdma.set_qp_limit(500);

        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];

        let start = Instant::now();

        for i in 0..100 {
            // Load eBPF program
            let _ = registry.load_program(
                BpfProgType::Tracing,
                program.clone(),
                format!("prog{}", i),
            );

            // Allocate PID
            let _ = pids.fork_process();

            // Allocate RDMA QP (every other iteration)
            if i % 2 == 0 {
                let _ = rdma.allocate_qp();
            }
        }

        let total_time = start.elapsed().as_micros();
        println!(
            "Mixed Operations (100 iterations): {} µs total",
            total_time
        );
        println!("Average per iteration: {} µs", total_time / 100);

        // Verify all operations succeeded
        assert_eq!(registry.list_programs().len(), 100);
        assert_eq!(pids.get_current_pids(), 100);
    }

    #[test]
    fn benchmark_throughput_comparison() {
        let mut registry = BpfProgramRegistry::new();
        let mut pids = PidsController::new();

        pids.set_max_pids(10000);

        // Benchmark eBPF execution throughput
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 100 },
            BpfInstruction::SubImm { dst_reg: 0, imm: 1 },
            BpfInstruction::Return,
        ];

        let fd = registry
            .load_program(BpfProgType::Tracing, program, "throughput".to_string())
            .expect("Loading failed");

        let start = Instant::now();
        let iterations = 10000;

        for _ in 0..iterations {
            let _ = registry.execute_program(fd);
        }

        let ebpf_time = start.elapsed().as_micros();
        let ebpf_throughput = (iterations as u128 * 1_000_000) / ebpf_time;

        println!("eBPF Throughput: {} programs/sec", ebpf_throughput);

        // Benchmark PID allocation throughput
        let start = Instant::now();
        let pid_iterations = 1000;

        for _ in 0..pid_iterations {
            let _ = pids.fork_process();
        }

        let pid_time = start.elapsed().as_micros();
        let pid_throughput = (pid_iterations as u128 * 1_000_000) / pid_time;

        println!("PID Allocation Throughput: {} pids/sec", pid_throughput);

        // Both should have reasonable throughput
        assert!(ebpf_throughput > 100_000, "eBPF should execute >100k programs/sec");
        assert!(pid_throughput > 50_000, "PID allocation should handle >50k/sec");
    }
}
