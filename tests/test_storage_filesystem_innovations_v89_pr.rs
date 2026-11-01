// SPDX-License-Identifier: MIT
//! Integration tests for Sovereign Storage & Filesystem Advancements (v89 PR Suite)

extern crate alloc;

#[path = "../src/storage/sovereign_storage_filesystem_innovations_v89_pr.rs"]
mod sovereign_storage_filesystem_innovations_v89_pr;

use sovereign_storage_filesystem_innovations_v89_pr::*;

#[test]
fn test_integration_ext4_and_xfs_v89() {
    let mut ext4 = Ext4AdvancementsV89Engine::new(0xABCD_EF01, 4096);
    ext4.record_fast_commit(101, 0, 8192, 16, false);
    assert_eq!(ext4.fast_commit_log.len(), 1);

    assert!(ext4.store_inline_inode(101, b"embedded_small_file_data", 64));
    ext4.insert_htree_entry("kernel.img", 512);
    assert_eq!(ext4.lookup_htree_block("kernel.img"), Some(512));

    let mut xfs = XfsAdvancementsV89Engine::new(8, 50000);
    let (ag, bno) = xfs.allocate_lockless_ag_extent(1024).expect("Allocation should succeed");
    assert_eq!(ag, 0);
    assert_eq!(bno, 0);

    xfs.register_reflink(ag, bno, 1024);
    assert_eq!(xfs.reflink_extents.len(), 1);
    assert_eq!(xfs.reflink_extents[0].ref_count, 2);

    xfs.insert_rmap_record(ag, bno, 1024, 101, 0);
    assert!(xfs.run_online_scrub());
}

#[test]
fn test_integration_btrfs_and_zfs_v89() {
    let mut btrfs = BtrfsAdvancementsV89Engine::new();
    assert!(btrfs.create_subvolume(10, "@var", Some(5)));
    assert!(btrfs.create_snapshot(11, 10, "@var_snapshot"));
    btrfs.allocate_raid_chunk(100, "RAID10", alloc::vec![0, 1, 2, 3], 1024 * 1024 * 1024);
    assert_eq!(btrfs.chunks.len(), 1);
    assert!(btrfs.compress_chunk_zstd(b"ZZZZZZZZZZZZZZ") > 0);
    assert!(btrfs.run_scrub_verify(42, 42));

    let mut zfs = OpenZfsAdvancementsV89Engine::new(4096);
    zfs.access_block(99, 512);
    zfs.access_block(99, 512);
    zfs.access_block(99, 512);
    assert!(!zfs.arc_cache[0].in_mru);
    zfs.add_vdev(0, "draid2", 8_000_000_000);
    assert_eq!(zfs.vdev_tree.len(), 1);

    let stream = zfs.serialize_zfs_send_stream("tank/dataset@v89");
    assert!(stream.starts_with(b"ZFS_SEND_STREAM_V89:"));
}

#[test]
fn test_integration_ufs_f2fs_fat_v89() {
    let mut ufs = UfsFfsAdvancementsV89Engine::new(4, 4096);
    ufs.log_soft_update(12, "DIR_ADD", 100);
    assert_eq!(ufs.commit_journal(), 1);
    assert_eq!(ufs.issue_bio_delete_trim(64), 64);

    let mut f2fs = F2FSAdvancementsV89Engine::new(8);
    for _ in 0..400 {
        f2fs.invalidate_block(1);
    }
    f2fs.map_node_address(100, 5000);
    let gc = f2fs.collect_garbage(200);
    assert_eq!(gc, alloc::vec![1]);

    let mut fat = FatExFatAdvancementsV89Engine::new(256);
    assert!(fat.allocate_cluster_chain(10, 4));
    let chain = fat.walk_cluster_chain(10);
    assert_eq!(chain, alloc::vec![10, 11, 12, 13]);
    assert_eq!(fat.calculate_flash_aligned_lba(1024), 8192);
}

#[test]
fn test_integration_crypto_tools_and_proposals_v89() {
    let mut net = NtfsNfsCifsPseudoFsV89Engine::new(2048);
    net.parse_ntfs_mft_attr(0x80, "$DATA", true, 120);
    net.register_nfs_pnfs_layout(55);
    assert!(net.match_devfs_rule("nvme1n1", "storage"));
    assert!(net.allocate_tmpfs_space(1000));

    let lvm = LvmCryptoMaintenanceUtilsV89Engine::new();
    assert_eq!(lvm.calculate_thin_overcommit_pct(300, 100), 300);
    let key = lvm.derive_argon2id_key(b"argon2id_seed");
    assert_ne!(key, [0u8; 32]);

    let tools = StorageMaintenanceAndToolsV89Engine::new();
    assert!(tools.check_gpt_4k_alignment(4096, 512));
    assert!(tools.audit_smart_health(0, 0, 38));
    let evt = tools.generate_udisks_block_device_event("/dev/nvme0n1", false);
    assert!(evt.starts_with("UDISKS2_EVENT:"));

    let suite = SovereignStorageAndFilesystemAdvancementsV89Suite::new();
    let manifests = suite.gateway.collect_all_proposals();
    assert_eq!(manifests.len(), 10);
    assert_eq!(manifests[0].pr_id, 1121);
    assert_eq!(manifests[9].pr_id, 1130);

    let summary = suite.gateway.generate_pr_markdown_summary();
    assert!(summary.contains("# Sovereign Storage & Filesystem Parity Advancements (v89 PR Suite)"));
    assert!(summary.contains("PR #1121"));
    assert!(summary.contains("PR #1130"));
}
