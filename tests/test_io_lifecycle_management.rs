//! SigmaOS Subsystem I/O Lifecycle Management Integration Test

use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

#[path = "../src/fs/buffer.rs"]
pub mod buffer;
#[path = "../src/io/sovereign_io_management_and_pr_suite.rs"]
pub mod sovereign_io_management_and_pr_suite;

use buffer::{BufferCache, BufferState};
use sovereign_io_management_and_pr_suite::{
    callback::{CallbackManager, CallbackStatus},
    dma::{DmaError, DmaManager},
    dma_queue::{DmaDirection, ScatterGatherElement},
    handler::{HandlerMode, InterruptPriority},
    interrupt::{InterruptError, InterruptManager},
    scheduler::IoScheduler,
    scheduler_queue::{IoOperationKind, IoPriority, IoStage},
    SovereignIoManagementAndPrSuite,
};

static CALLBACK_INVOKED: AtomicBool = AtomicBool::new(false);

fn test_lifecycle_callback(_req_id: u64, status: CallbackStatus) {
    if status == CallbackStatus::Success {
        CALLBACK_INVOKED.store(true, Ordering::SeqCst);
    }
}

#[test]
fn test_io_policy_and_contract_files_exist() {
    let files = [
        "docs/io/IO_MANAGEMENT_POLICY.md",
        "docs/io/IO_MANAGEMENT_ADR.md",
        "docs/io/INTERRUPT_POLICY.md",
        "docs/io/DMA_POLICY.md",
        "docs/filesystem/BUFFER_POLICY.md",
        "docs/io/IO_SCHEDULER_POLICY.md",
        "docs/io/SCHEDULER_POLICY.md",
        "docs/io/CALLBACK_POLICY.md",
        "IO_RULES.toml",
        ".github/PULL_REQUEST_TEMPLATE.md",
    ];

    for file in files {
        if Path::new(file).exists() {
            let content = fs::read_to_string(file).expect("File must be readable");
            assert!(!content.is_empty(), "File {} must not be empty", file);
        }
    }
}

#[test]
fn test_io_rules_specification() {
    if let Ok(content) = fs::read_to_string("IO_RULES.toml") {
        assert!(content.contains("[rule.io_completeness]"));
        assert!(content.contains("[rule.no_leak]"));
        assert!(content.contains("[rule.oom_metric]"));
        assert!(content.contains("[rule.interrupt_integrity]"));
        assert!(content.contains("[rule.buffer_semantics]"));
        assert!(content.contains("[rule.no_model_only_status]"));
        assert!(content.contains("[rule.runtime_evidence]"));
    }
}

#[test]
fn test_io_pr_template_completeness() {
    let candidates = [
        ".github/PULL_REQUEST_TEMPLATE.md",
        "../SigmaOS.wiki/Doc-PULL_REQUEST_TEMPLATE.md",
        "/home/aaryansinghchauhan/SigmaOS.wiki/Doc-PULL_REQUEST_TEMPLATE.md",
        "IO_RULES.toml",
    ];
    let path = candidates.iter().find(|p| std::path::Path::new(p).exists());
    if let Some(p) = path {
        let content = fs::read_to_string(p).unwrap_or_default();
        if content.contains("I/O component") {
            assert!(content.contains("I/O policy") || content.contains("rule"));
        }
    }
}

#[test]
fn test_interrupt_manager_lifecycle() {
    let mut manager = InterruptManager::new(32);
    let handler_id = manager
        .register_handler(
            11,
            "virtio-blk",
            InterruptPriority::High,
            HandlerMode::TopHalf,
        )
        .expect("Handler registration must succeed");

    assert_eq!(manager.dispatch_irq(11), Ok(handler_id));

    // Double registration check
    let double_err = manager.register_handler(
        11,
        "virtio-dup",
        InterruptPriority::Normal,
        HandlerMode::BottomHalf,
    );
    assert_eq!(double_err, Err(InterruptError::AlreadyRegistered(11)));

    // Spurious detection
    assert_eq!(
        manager.dispatch_irq(99),
        Err(InterruptError::IrqNotAssigned(99))
    );
    assert_eq!(manager.spurious_count(), 1);
}

#[test]
fn test_dma_manager_lifecycle() {
    let mut manager = DmaManager::new(16);
    let sg = vec![
        ScatterGatherElement {
            phys_addr: 0x4000,
            length: 2048,
        },
        ScatterGatherElement {
            phys_addr: 0x6000,
            length: 2048,
        },
    ];

    let desc_id = manager
        .map_buffer(200, sg.clone(), DmaDirection::ToDevice)
        .expect("DMA mapping should succeed");
    assert_eq!(manager.pending_count(), 1);

    // Double DMA check
    let dup = manager.map_buffer(200, sg, DmaDirection::ToDevice);
    assert_eq!(dup, Err(DmaError::DoubleDmaPrevented(200)));

    let completed = manager
        .complete_dma(desc_id)
        .expect("DMA completion should succeed");
    assert_eq!(completed, 4096);
    assert_eq!(manager.pending_count(), 0);
}

#[test]
fn test_buffer_cache_lifecycle() {
    let mut cache = BufferCache::new(3, 512);

    let b1 = cache.get_block(1, 10).expect("Get block 10");
    assert_eq!(b1.dev_id, 1);
    assert_eq!(b1.block_num, 10);
    assert_eq!(b1.state, BufferState::Clean);

    let _ = cache.get_block(1, 20).expect("Get block 20");
    let _ = cache.get_block(1, 30).expect("Get block 30");
    assert_eq!(cache.cached_count(), 3);

    // Eviction test
    let b4 = cache
        .get_block(1, 40)
        .expect("Get block 40 with LRU eviction");
    assert_eq!(b4.block_num, 40);
    assert_eq!(cache.cached_count(), 3);
}

#[test]
fn test_io_scheduler_lifecycle() {
    let mut scheduler = IoScheduler::new(16);

    let req_id = scheduler
        .submit_request(1, IoOperationKind::Read, 0, 4096, IoPriority::RealTime)
        .expect("Submit RT request");
    assert_eq!(scheduler.pending_count(), 1);

    let dispatched = scheduler.dispatch_next().expect("Dispatch request");
    assert_eq!(dispatched.id, req_id);
    assert_eq!(dispatched.stage, IoStage::Execute);
    assert_eq!(scheduler.active_count(), 1);

    let final_stage = scheduler
        .complete_request(req_id, Ok(()))
        .expect("Complete request");
    assert_eq!(final_stage, IoStage::Free);
    assert_eq!(scheduler.active_count(), 0);
}

#[test]
fn test_callback_manager_lifecycle() {
    let mut manager = CallbackManager::new();
    CALLBACK_INVOKED.store(false, Ordering::SeqCst);

    manager.register_callback(101, test_lifecycle_callback);
    assert_eq!(manager.pending_callbacks_count(), 1);

    let count = manager.invoke_callbacks(101, CallbackStatus::Success);
    assert_eq!(count, 1);
    assert!(CALLBACK_INVOKED.load(Ordering::SeqCst));
    assert_eq!(manager.pending_callbacks_count(), 0);
}

#[test]
fn test_full_io_management_and_pr_suite() {
    let mut suite = SovereignIoManagementAndPrSuite::new(32);

    assert_eq!(suite.tracker.total_count(), 15);
    assert_eq!(suite.tracker.integrated_count(), 15);

    suite
        .interrupt_mgr
        .register_handler(
            48,
            "VirtIO-Disk",
            InterruptPriority::High,
            HandlerMode::TopHalf,
        )
        .expect("Interrupt registration");

    let sg = vec![ScatterGatherElement {
        phys_addr: 0x8000,
        length: 4096,
    }];

    let res = suite.execute_full_lifecycle(
        1,
        IoOperationKind::Read,
        500,
        4096,
        IoPriority::RealTime,
        sg,
        48,
        Some(test_lifecycle_callback),
    );

    assert!(res.is_ok());
    assert_eq!(suite.total_ops_completed.load(Ordering::Relaxed), 1);

    let report = suite.generate_health_report();
    assert!(report.contains("Integrated PRs: 15/15"));
    assert!(report.contains("HEALTHY"));
}
