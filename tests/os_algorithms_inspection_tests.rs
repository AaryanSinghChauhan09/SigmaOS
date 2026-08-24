// SPDX-License-Identifier: MIT
//! SigmaOS Comprehensive Core OS Algorithms Inspection Suite
//!
//! Deeply inspects and verifies fundamental OS algorithms:
//! 1. Real-Time Scheduling Algorithms:
//!    - Earliest Deadline First (EDF) Real-Time Scheduling
//!    - Probabilistic Lottery Scheduling with LCG Randomness
//!    - Multi-Level Feedback Queue (MLFQ) Scheduling with Priority Demotion & Aging Boost
//! 2. Security & Access Control Algorithms:
//!    - POSIX.1e ACL & Bell-LaPadula Mandatory Access Control (MLS) Evaluation Matrix
//!    - Consensual Cryptographic Audit Ledger Block Chain Integrity Verification
//! 3. System Architecture & Memory Alignment Algorithms:
//!    - Sovereign Alignment Verification (System V 16-byte stack, page boundaries, SIMD AVX-512)
//! 4. Filesystem Cache Eviction Algorithms:
//!    - ZFS Adaptive Replacement Cache (ARC) MRU & MFU Two-Tier Eviction
//! 5. Dependency Solver Algorithms:
//!    - Arch Package Dependency SAT Solver Chain Verification

extern crate alloc;

#[path = "../src/kernel/structures.rs"]
mod structures;

#[path = "../src/kernel/sched/sigma_mlfq.rs"]
mod sigma_mlfq;

#[path = "../src/sigpkg/zero_alloc_resolver.rs"]
mod zero_alloc_resolver;

#[path = "../src/kernel/os_innovations.rs"]
mod os_innovations;

#[path = "../src/compatibility/abi_translator.rs"]
mod abi_translator;

#[path = "../src/access/control.rs"]
mod access_control;

#[path = "../src/sigpkg/multi_distro.rs"]
mod multi_distro;

#[path = "../src/ipc/std_streams.rs"]
mod std_streams;

use structures::{
    AdvancedAlgorithmsManager, EdfTask, LotteryTask,
};
use sigma_mlfq::MlfqScheduler;
use zero_alloc_resolver::{PackageDependencyResolver, PackageRecipe, Version, MAX_RECIPE_DEPENDENCIES};
use os_innovations::IllumosZfsArcCache;
use abi_translator::SovereignAlignmentChecker;
use access_control::{
    PosixAcl, AclEntry, AclTag, MacSecurityLabel, SensitivityLevel,
};

#[test]
fn test_edf_realtime_scheduler_algorithm_inspection() {
    let mgr = AdvancedAlgorithmsManager::new(12345);

    let task_far = EdfTask {
        tid: 1,
        absolute_deadline: 1000,
        remaining_execution: 10,
        is_active: true,
    };
    let task_urgent = EdfTask {
        tid: 2,
        absolute_deadline: 200,
        remaining_execution: 5,
        is_active: true,
    };
    let task_medium = EdfTask {
        tid: 3,
        absolute_deadline: 500,
        remaining_execution: 8,
        is_active: true,
    };

    assert!(mgr.add_edf_task(task_far).is_ok());
    assert!(mgr.add_edf_task(task_urgent).is_ok());
    assert!(mgr.add_edf_task(task_medium).is_ok());

    // EDF core algorithm must pick task_urgent (deadline 200) first
    let scheduled = mgr.schedule_edf().unwrap();
    assert_eq!(scheduled.tid, 2);
    assert_eq!(scheduled.absolute_deadline, 200);
}

#[test]
fn test_lottery_scheduler_algorithm_inspection() {
    let mgr = AdvancedAlgorithmsManager::new(99999);

    let t1 = LotteryTask {
        tid: 10,
        tickets: 100,
        is_active: true,
    };
    let t2 = LotteryTask {
        tid: 20,
        tickets: 10,
        is_active: true,
    };

    assert!(mgr.add_lottery_task(t1).is_ok());
    assert!(mgr.add_lottery_task(t2).is_ok());

    let mut t1_wins = 0;
    let mut t2_wins = 0;

    for _ in 0..100 {
        let winner = mgr.schedule_lottery().unwrap();
        if winner.tid == 10 {
            t1_wins += 1;
        } else if winner.tid == 20 {
            t2_wins += 1;
        }
    }

    // Task with 100 tickets should win significantly more than task with 10 tickets
    assert!(t1_wins > t2_wins);
}

#[test]
fn test_consensus_audit_ledger_chain_algorithm_inspection() {
    let mgr = AdvancedAlgorithmsManager::new(42);

    assert!(mgr.audit_process_event(1000, 1, 101, 0xABC1).is_ok());
    assert!(mgr.audit_process_event(1005, 2, 102, 0xABC2).is_ok());
    assert!(mgr.audit_process_event(1010, 3, 103, 0xABC3).is_ok());

    // Verify un-tampered chain passes verification
    assert!(mgr.verify_ledger_integrity());

    // Tamper with Block 2
    {
        let mut ledger = mgr.audit_ledger.borrow_mut();
        if let Some(ref mut block) = ledger[1] { // Block index 1 = Block ID 2
            block.actor_hash = 0xBAD1;
        }
    }

    // Tampered chain must fail verification
    assert!(!mgr.verify_ledger_integrity());
}

#[test]
fn test_mlfq_scheduling_demotion_and_aging_algorithm_inspection() {
    let mut mlfq = MlfqScheduler::new(3);

    mlfq.enqueue(101, 0); // Priority Queue 0
    mlfq.enqueue(102, 1); // Priority Queue 1

    // Highest priority task (101) dequeued first
    assert_eq!(mlfq.dequeue(), Some(101));

    // Demote task 101 from queue 0 to queue 1
    mlfq.enqueue(101, 0);
    mlfq.demote(101);
    assert!(mlfq.queues[1].contains(&101));

    // Promote task 101 back from queue 1 to queue 0
    mlfq.promote(101);
    assert!(mlfq.queues[0].contains(&101));
}

#[test]
fn test_arch_dependency_sat_resolver_algorithm_inspection() {
    let mut resolver = PackageDependencyResolver::new();

    let base_pkg = PackageRecipe {
        name: "libc",
        version: Version { major: 1, minor: 0 },
        dependencies: [""; MAX_RECIPE_DEPENDENCIES],
        dep_count: 0,
    };

    let app_pkg = PackageRecipe {
        name: "zenith",
        version: Version { major: 2, minor: 1 },
        dependencies: {
            let mut deps = [""; MAX_RECIPE_DEPENDENCIES];
            deps[0] = "libc";
            deps
        },
        dep_count: 1,
    };

    assert!(resolver.register_recipe(base_pkg).is_ok());
    assert!(resolver.register_recipe(app_pkg).is_ok());

    // Normal chain (zenith -> libc -> none) verifies clean reproducibility
    assert!(resolver.verify_reproducible_chain("zenith"));

    // Corrupted circular chain (libc -> zenith) must fail
    let mut corrupted_base_pkg = base_pkg;
    corrupted_base_pkg.dependencies[0] = "zenith";
    corrupted_base_pkg.dep_count = 1;

    let mut cyclic_resolver = PackageDependencyResolver::new();
    assert!(cyclic_resolver.register_recipe(corrupted_base_pkg).is_ok());
    assert!(cyclic_resolver.register_recipe(app_pkg).is_ok());
    assert!(!cyclic_resolver.verify_reproducible_chain("zenith"));
}

#[test]
fn test_illumos_zfs_arc_mru_mfu_cache_eviction_algorithm_inspection() {
    let mut arc = IllumosZfsArcCache::new(2); // Capacity 2 items

    arc.insert_block(101, alloc::vec![1, 2, 3]);
    arc.insert_block(102, alloc::vec![4, 5, 6]);

    // Accessing 101 promotes it from MRU to MFU
    assert_eq!(arc.read_block(101), Some(alloc::vec![1, 2, 3]));

    // Insert third block 103 — MRU evicted unpromoted block 102
    arc.insert_block(103, alloc::vec![7, 8, 9]);

    let (mru_len, mfu_len, mru_ghost, _mfu_ghost) = arc.stats();
    assert_eq!(mfu_len, 1);   // 101 promoted
    assert_eq!(mru_ghost, 1); // 102 evicted to ghost
    assert_eq!(mru_len, 1);   // 103 in MRU
}

#[test]
fn test_system_alignment_checker_algorithm_inspection() {
    // 1. System V x86-64 16-byte stack alignment check
    assert!(SovereignAlignmentChecker::check_stack_alignment(0x7FFFFFFFD000, 16));
    assert!(!SovereignAlignmentChecker::check_stack_alignment(0x7FFFFFFFD008, 16));

    // 2. Memory 4KB page boundary alignment check
    assert!(SovereignAlignmentChecker::check_page_alignment(0x0000_1000, 4096));
    assert!(!SovereignAlignmentChecker::check_page_alignment(0x0000_100F, 4096));

    // 3. SIMD AVX-512 64-byte alignment check
    assert!(SovereignAlignmentChecker::check_simd_alignment(0x0000_2000, 64));
    assert!(!SovereignAlignmentChecker::check_simd_alignment(0x0000_2020, 64));
}

#[test]
fn test_posix_acl_and_bell_lapadula_mls_access_control_algorithm_inspection() {
    // POSIX 1003.1e ACL Evaluation Algorithm
    let mut acl = PosixAcl::from_mode(1000, 1000, 0o700);
    acl.add_entry_direct(AclEntry::new(AclTag::User(1005), 5)); // User 1005 granted Read+Exec (5)

    assert!(acl.evaluate_access(1005, 1005, &[], 1000, 1000, 5));
    assert!(!acl.evaluate_access(1005, 1005, &[], 1000, 1000, 2)); // Denied Write (2)

    // Bell-LaPadula MLS (No Read Up, No Write Down) Algorithm
    let sub_top_secret = MacSecurityLabel::new(SensitivityLevel::TopSecret, 0x01);
    let obj_secret = MacSecurityLabel::new(SensitivityLevel::Secret, 0x01);

    assert!(sub_top_secret.can_read(&obj_secret)); // TopSecret can read Secret (No Read Up)
    assert!(!obj_secret.can_read(&sub_top_secret)); // Secret CANNOT read TopSecret
}

#[test]
fn test_multi_distro_package_manager_algorithm_inspection() {
    use multi_distro::{
        AptPinPriority, DnfDeltaEngine, PacmanAlpmHookRegistry, HookPhase, PortageSlotResolver, XbpsCasExtractor, SovereignMultiDistroPackageManager,
    };

    // 1. APT Pinning Priority Ordering
    let mut mgr = SovereignMultiDistroPackageManager::new();
    mgr.stage_package("bash", "5.2", AptPinPriority::Automatic);
    mgr.stage_package("linux-kernel", "6.8", AptPinPriority::HoldExclusive);
    assert_eq!(mgr.staged_txs[0].package_name, "linux-kernel");

    // 2. DNF Delta RPM
    let delta_engine = DnfDeltaEngine::new();
    let drpm = delta_engine.create_drpm(b"Pkg V1", b"Pkg V2");
    let rebuilt = delta_engine.apply_drpm(b"Pkg V1", &drpm).unwrap();
    assert_eq!(rebuilt, b"Pkg V2");

    // 3. Pacman ALPM Hook Triggering
    let mut alpm = PacmanAlpmHookRegistry::new();
    alpm.register_hook("post_install", HookPhase::PostTransaction, "ldconfig", &["glibc"]);
    let cmds = alpm.execute_hooks(HookPhase::PostTransaction, "glibc");
    assert_eq!(cmds.len(), 1);
    assert_eq!(cmds[0], "ldconfig");

    // 4. Portage Slot Resolution
    let mut portage = PortageSlotResolver::new();
    assert!(portage.install_slot("gcc", "13", "13.2.0").is_ok());
    assert!(portage.install_slot("gcc", "14", "14.1.0").is_ok());
    assert_eq!(portage.resolve_slot_versions("gcc").len(), 2);

    // 5. XBPS CAS Hash Extraction
    let xbps = XbpsCasExtractor::new();
    let hash = xbps.compute_cas_hash(b"test archive payload");
    assert!(xbps.verify_rsa_signature(b"test archive payload", &hash));
}

#[test]
fn test_standard_streams_controller_algorithm_inspection() {
    use std_streams::{
        StandardStreamController, StreamBufferMode, STDOUT_FILENO, STDERR_FILENO,
    };

    let mut controller = StandardStreamController::new();

    // 1. Verify standard streams initial setup
    assert!(controller.handles.contains_key(&0)); // stdin
    assert!(controller.handles.contains_key(&1)); // stdout
    assert!(controller.handles.contains_key(&2)); // stderr

    // 2. Unbuffered stderr output
    let stderr_data = controller.write_to_fd(STDERR_FILENO, b"kernel warning").unwrap();
    assert_eq!(stderr_data, b"kernel warning");

    // 3. Line buffered stdout output
    let partial = controller.write_to_fd(STDOUT_FILENO, b"line 1").unwrap();
    assert_eq!(partial.len(), 0);
    let flushed = controller.write_to_fd(STDOUT_FILENO, b"\n").unwrap();
    assert_eq!(flushed, b"line 1\n");

    // 4. POSIX dup2 file descriptor redirection
    let dup_fd = controller.dup2(STDOUT_FILENO, 100).unwrap();
    assert_eq!(dup_fd, 100);

    // 5. OpenBSD pledge stdio enforcement
    assert!(controller.validate_pledge_stdio().is_ok());
    controller.active_pledges.clear();
    assert!(controller.validate_pledge_stdio().is_err());
}
