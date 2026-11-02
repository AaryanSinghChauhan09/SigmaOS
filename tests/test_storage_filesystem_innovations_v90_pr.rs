// SPDX-License-Identifier: MIT
//! Integration tests for Sovereign Storage & Filesystem Advancements (v90 PR Suite)

extern crate alloc;

#[path = "../src/storage/sovereign_storage_filesystem_innovations_v90_pr.rs"]
mod sovereign_storage_filesystem_innovations_v90_pr;

use sovereign_storage_filesystem_innovations_v90_pr::*;

#[test]
fn test_integration_ext4_and_xfs_v90() {
    let mut ext4 = Ext4AdvancementsV90Engine::new(0xABCD_1234, 4096);
    let csum = ext4.append_fast_commit(10, 101, 0, 8192, 16, false);
    assert_ne!(csum, 0);
    assert_eq!(ext4.fast_commit_records.len(), 1);

    assert!(ext4.insert_extent(0, 16384, 32));
    assert_eq!(ext4.lookup_extent_pblk(10), Some(16394));

    let (stored, xattr) = ext4.store_inline_data(101, b"inline_small_payload", 64);
    assert!(stored && !xattr);

    ext4.insert_htree_node("system.conf", 512);
    ext4.mark_bad_block(4096);
    assert_eq!(ext4.tune2fs_calculate_reserved(20000, 5), 1000);

    let mut xfs = XfsAdvancementsV90Engine::new(8, 50000);
    let (ag, bno) = xfs.allocate_lockless_extent(2048).expect("Allocation should succeed");
    assert_eq!(ag, 0);
    assert_eq!(bno, 0);

    xfs.share_reflink_extent(ag, bno, 2048);
    assert_eq!(xfs.reflink_extents.len(), 1);
    assert_eq!(xfs.reflink_extents[0].ref_count, 2);
    assert!(xfs.cow_diverge_extent(ag, bno));

    xfs.insert_rmap(ag, bno, 2048, 101, 0);
    xfs.configure_rt_extent(0, 4096);
    assert!(xfs.run_online_scrub());
}

#[test]
fn test_integration_btrfs_and_zfs_v90() {
    let mut btrfs = BtrfsAdvancementsV90Engine::new();
    assert!(btrfs.create_subvol(10, "@opt", Some(5), false, false));
    assert!(btrfs.create_subvol(11, "@opt_snapshot", Some(10), true, true));
    assert!(btrfs.allocate_raid_chunk(100, "RAID10", alloc::vec![0, 1, 2, 3], 1024 * 1024 * 1024));
    assert_eq!(btrfs.chunks.len(), 1);

    let (comp_len, algo) = btrfs.compress_chunk_adaptive(b"WWWWWWWWXXXXXXXXYYYYYYYY");
    assert!(comp_len > 0);
    assert_eq!(algo, "ZSTD");
    assert!(btrfs.scrub_verify_block(42, 42));

    let mut zfs = OpenZfsAdvancementsV90Engine::new(8192);
    zfs.access_arc_block(99, 1024);
    zfs.access_arc_block(99, 1024);
    assert!(zfs.arc_cache[0].in_mfu);

    zfs.add_vdev(0, "draid2", 16_000_000_000);
    zfs.configure_dataset("pool/root", 50000, 20000);
    assert!(zfs.can_write_to_dataset("pool/root", 10000));

    let stream = zfs.generate_send_stream("pool/root", Some("base"), "target");
    assert!(stream.starts_with(b"ZFS_STREAM_V90_HEADER:"));
}

#[test]
fn test_integration_ufs_f2fs_fat_v90() {
    let mut ufs = UfsFfsAdvancementsV90Engine::new(4, 4096, 1024);
    let id = ufs.log_suj_operation(12, "DIR_MKDIR", 1024);
    assert_eq!(id, 1);
    assert_eq!(ufs.flush_suj_journal(), 1);
    assert_eq!(ufs.bio_delete_blocks(128), 128);
    assert!(ufs.fsck_verify_cg_health());

    let mut f2fs = F2FSAdvancementsV90Engine::new(8);
    for _ in 0..400 {
        f2fs.invalidate_block(1);
    }
    f2fs.map_node(100, 5000);
    let gc = f2fs.run_cost_benefit_gc(200);
    assert_eq!(gc, alloc::vec![1]);

    let mut fat = FatExFatAdvancementsV90Engine::new(256, 4096);
    assert!(fat.allocate_clusters(10, 4));
    let chain = fat.walk_clusters(10);
    assert_eq!(chain, alloc::vec![10, 11, 12, 13]);
    assert_eq!(fat.align_flash_lba(1024), 8192);

    fat.texfat_transaction_begin();
    assert!(fat.texfat_transaction_commit());
}

#[test]
fn test_integration_crypto_tools_and_proposals_v90() {
    let mut net = NtfsNfsCifsPseudoFsV90Engine::new(4096);
    net.parse_mft_record(50, 0x80, true, 256);
    net.register_pnfs_layout(77);
    assert!(net.match_devfs_rule("nvme1n1", "disk"));
    assert!(net.allocate_tmpfs(2048));

    let lvm = LvmCryptoMaintenanceUtilsV90Engine::new();
    assert_eq!(lvm.calculate_thin_overcommit(600, 200), 300);
    let (p, q) = lvm.compute_raid6_p_q(&[b"test_payload_1", b"test_payload_2"]);
    assert_eq!(p.len(), 14);
    assert_eq!(q.len(), 14);

    let seed: [u8; 8] = [0xA1, 0xB2, 0xC3, 0xD4, 0xE5, 0xF6, 0x07, 0x18];
    let key = lvm.derive_argon2id_keyslot(b"argon2id_pass", &seed);
    assert_ne!(key, [0u8; 32]);
    assert!(lvm.verify_dm_verity_roothash(&[9u8; 32], &[9u8; 32]));

    let tools = StorageMaintenanceAndToolsV90Engine::new();
    assert!(tools.check_gpt_4k_alignment(4096, 512));
    assert!(tools.audit_smart_health(0, 0, 40));
    assert_eq!(
        tools.parse_autofs_map_entry("/data /storage/server"),
        Some(("/data".to_string(), "/storage/server".to_string()))
    );
    let evt = tools.generate_udisks_block_event("/dev/nvme0n1", false, true);
    assert!(evt.starts_with("UDISKS2_EVENT:"));

    let suite = SovereignStorageAndFilesystemAdvancementsV90Suite::new();
    let manifests = suite.gateway.collect_all_proposals();
    assert_eq!(manifests.len(), 10);
    assert_eq!(manifests[0].pr_id, 1131);
    assert_eq!(manifests[9].pr_id, 1140);

    let summary = suite.gateway.generate_pr_markdown_summary();
    assert!(summary.contains("# Sovereign Storage & Filesystem Parity Advancements (v90 PR Suite)"));
    assert!(summary.contains("PR #1131"));
    assert!(summary.contains("PR #1140"));
}
