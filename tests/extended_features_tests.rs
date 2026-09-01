// Integration and Unit Test Harness for Extended Features in SigmaOS

extern crate alloc;

use sigmaos::access::append_rights::*;
use sigmaos::app::murano_catalogue::*;
use sigmaos::auth::authentication_pipeline::*;
use sigmaos::compatibility::abi_extended::*;
use sigmaos::kernel::atomic_extended::*;
use sigmaos::process::blocked_state::*;
use sigmaos::storage::block_device_extended::*;

#[test]
fn test_cinder_volume_extended() {
    let mut manager = CinderVolumeManager::new();
    let vol_id = manager.create_volume("test-disk", 5, false);
    let vol = manager.get_volume_mut(vol_id).unwrap();

    assert_eq!(vol.size_gb, 5);
    assert_eq!(vol.attach_state, VolumeAttachState::Detached);

    vol.attach(505).unwrap();
    assert_eq!(vol.attach_state, VolumeAttachState::Attached);

    let snap = vol.create_snapshot(1001, 12345678);
    assert_eq!(snap.snapshot_id, 1001);
    assert!(snap.is_ready);

    vol.detach().unwrap();
    assert_eq!(vol.attach_state, VolumeAttachState::Detached);
}

#[test]
fn test_blocked_process_management() {
    let mut bpm = BlockedProcessManager::new();
    let mut pcb = ProcessControlBlock::new(101, 1, "disk_syncer");

    pcb.transition_to_blocked(
        BlockedProcessState::WaitingIo,
        BlockReason::BlockDeviceIo {
            device_id: 2,
            block_num: 64,
        },
        1000,
    );

    bpm.block_process(pcb);
    assert_eq!(bpm.get_blocked_count(), 1);

    let woken = bpm.wake_process_by_io(2, 64);
    assert_eq!(woken.len(), 1);
    assert_eq!(woken[0].pid, 101);
    assert_eq!(bpm.get_blocked_count(), 0);
}

#[test]
fn test_user_authentication_pipeline() {
    let pipeline = UserAuthenticationPipeline::new(0x12345678);

    let res = pipeline.authenticate_user("user1", 0xABC, 0xABC, Some(123456));
    assert_eq!(res, AuthResultStatus::Success);

    let res_bad = pipeline.authenticate_user("user1", 0xABC, 0xDEF, None);
    assert_eq!(res_bad, AuthResultStatus::InvalidCredential);

    let attr = pipeline
        .attributes_table
        .lookup_attribute("NETWORK_ACCESS")
        .unwrap();
    assert!(attr.automatic_allocation);
}

#[test]
fn test_atomic_bitmap_and_apc() {
    let bitmap = AtomicBitmap::new(64);
    assert!(bitmap.set_bit(10));
    assert!(bitmap.test_bit(10));
    assert!(!bitmap.set_bit(10)); // Second set returns false
    assert!(bitmap.clear_bit(10));
    assert!(!bitmap.test_bit(10));

    let counter = AtomicCounter::new(100);
    assert_eq!(counter.fetch_add_cas(50), 100);
    assert_eq!(counter.get(), 150);

    let mut apc_q = AsyncProcedureCallQueue::new();
    let id = apc_q.queue_apc(200, ApcEnvironment::KernelMode, 0xDEAD);
    assert_eq!(id, 1);
    let count = apc_q.dispatch_apcs_for_thread(200, ApcEnvironment::KernelMode);
    assert_eq!(count, 1);
}

#[test]
fn test_murano_application_catalogue() {
    let mut manager = MuranoApplicationCatalogueManager::new();
    let pkgs = manager.search_by_category("Desktop");
    assert_eq!(pkgs.len(), 1);
    let app_id = pkgs[0].app_id;

    assert!(manager.install_package(app_id).is_ok());
}

#[test]
fn test_abi_extended_frames() {
    let sysv = SystemVAbiFrame::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(sysv.arg_registers[0], 1);
    assert!(sysv.rsp_aligned_16);

    let win64 = WindowsX64AbiFrame::new(&[10, 20, 30, 40]);
    assert_eq!(win64.arg_registers[3], 40);

    let mut rel_table = DynamicAbiRelocationTable::new();
    rel_table.add_relocation("malloc", 0x8000, 1, 0x20);
    assert_eq!(rel_table.resolve_symbol("malloc"), Some(0x8020));
}

#[test]
fn test_append_only_stream() {
    let mut stream = AppendOnlyStream::new(10, "secure_journal.log");
    assert_eq!(stream.append_data(b"Entry 1\n").unwrap(), 8);
    assert!(stream.overwrite_data(b"Malicious").is_err());
    assert!(stream.truncate_stream().is_err());
    assert_eq!(stream.data.len(), 8);
}
