#![allow(warnings, unused, dead_code, non_camel_case_types, non_snake_case, unexpected_cfgs, mismatched_lifetime_syntaxes, static_mut_refs)]
// SigmaOS Algorithm & OS Component Inspection Test Suite
// Verifies internal kernel algorithms, scheduling models, memory allocators, and security mechanisms.

#[path = "../src/virtualization/vm_manager.rs"]
mod vm_manager;

#[path = "../src/compatibility/bsd.rs"]
mod bsd;

#[path = "../src/distro/linux_bsd_inspirations.rs"]
mod distro_inspirations;

#[cfg(test)]
mod algorithm_inspection_tests {
    use super::bsd::{FreeBsdJailManager, NetBsdRumpKernelRouter, RumpHypercall};
    use super::distro_inspirations::{
        BoreTaskProfile, CachyBoreScheduler, CoreTypePreference, MemoryPagePerms,
        SovereignKaslrWxAllocator,
    };
    use super::vm_manager::{KvmExitReason, KvmVirtualCpu, QemuMonitorEngine};

    #[test]
    fn test_kvm_vcpu_algorithm_execution() {
        let mut vcpu = KvmVirtualCpu::new(1);
        vcpu.registers.rip = 0x00007FFF00000000;
        vcpu.registers.rsp = 0x00007FFFFFFFE000;
        assert_eq!(vcpu.registers.rip, 0x00007FFF00000000);

        let exit = vcpu.run_vcpu();
        assert_eq!(exit, KvmExitReason::Hlt);

        vcpu.inject_interrupt(48);
        let irq_exit = vcpu.run_vcpu();
        assert_eq!(irq_exit, KvmExitReason::Interrupt);
    }

    #[test]
    fn test_qemu_monitor_protocol_qmp_execution() {
        let mut qmp = QemuMonitorEngine::new();
        let status = qmp.execute_qmp_command("query-status").unwrap();
        assert!(status.contains("running"));
        assert_eq!(qmp.command_history.len(), 1);
    }

    #[test]
    fn test_freebsd_jail_manager_algorithm() {
        let mut mgr = FreeBsdJailManager::new();
        let jail_id = mgr
            .create_jail("web_jail", "192.168.1.10", "/vfs/jails/web")
            .unwrap();
        assert_eq!(jail_id, 1);

        assert!(mgr.jails.contains_key(&jail_id));
        assert!(mgr.stop_jail(jail_id).is_ok());
    }

    #[test]
    fn test_netbsd_rump_hypercall_router() {
        let res = NetBsdRumpKernelRouter::dispatch_hypercall(RumpHypercall::Syscall, 0x10);
        assert_eq!(res, 17); // 16 + 1 = 17
    }

    #[test]
    fn test_cachy_bore_burstiness_scheduler() {
        let mut sched = CachyBoreScheduler::new(2000000);
        sched.register_task(BoreTaskProfile {
            task_id: 101,
            name: "browser".to_string(),
            priority: 10,
            interactive_score: 90,
            burst_time_ns: 10000,
            preferred_core: CoreTypePreference::PerformancePCore,
            ipc_intensity: 50,
        });
        let next_task = sched.schedule_next_task(CoreTypePreference::PerformancePCore);
        assert!(next_task.is_some());
        assert_eq!(next_task.unwrap().task_id, 101);
    }

    #[test]
    fn test_sovereign_kaslr_wx_allocator() {
        let mut alloc = SovereignKaslrWxAllocator::new(0x2000000);
        let phys_virt = alloc
            .allocate_page(0x1000000, 4096, MemoryPagePerms::ReadExecute)
            .unwrap();
        assert!(phys_virt > 0);
    }
}
