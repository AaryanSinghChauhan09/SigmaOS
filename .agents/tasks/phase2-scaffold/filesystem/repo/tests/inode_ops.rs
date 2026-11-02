//! Integration tests: inode create / delete / stat operations.

use sigmaos_filesystem::{
    FileMode, FileType,
    InodeError, InodeTable, StatInfo,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn table(max: u64) -> InodeTable { InodeTable::new(max) }
fn mode(v: u16) -> FileMode { FileMode::new(v) }

// ---------------------------------------------------------------------------
// create
// ---------------------------------------------------------------------------

#[test]
fn create_regular_file() {
    let mut t = table(256);
    let ino = t.create("/etc/hostname", FileType::Regular, mode(0o644), 0, 0)
        .expect("create should succeed");
    assert!(ino >= 1, "inode number must be positive");
}

#[test]
fn create_directory() {
    let mut t = table(256);
    let ino = t.create("/var/log", FileType::Directory, mode(0o755), 0, 0).unwrap();
    let st = t.stat(ino).unwrap();
    assert_eq!(st.file_type, FileType::Directory);
}

#[test]
fn create_symlink() {
    let mut t = table(256);
    let ino = t.create("/usr/local/bin", FileType::Symlink, mode(0o777), 0, 0).unwrap();
    let st = t.stat(ino).unwrap();
    assert_eq!(st.file_type, FileType::Symlink);
}

#[test]
fn create_duplicate_path_returns_error() {
    let mut t = table(256);
    t.create("/dup", FileType::Regular, mode(0o644), 0, 0).unwrap();
    assert_eq!(
        t.create("/dup", FileType::Regular, mode(0o644), 0, 0),
        Err(InodeError::AlreadyExists)
    );
}

#[test]
fn create_beyond_capacity_returns_table_full() {
    let mut t = table(2);
    t.create("/a", FileType::Regular, mode(0o644), 0, 0).unwrap();
    t.create("/b", FileType::Regular, mode(0o644), 0, 0).unwrap();
    assert_eq!(
        t.create("/c", FileType::Regular, mode(0o644), 0, 0),
        Err(InodeError::TableFull)
    );
}

// ---------------------------------------------------------------------------
// stat
// ---------------------------------------------------------------------------

#[test]
fn stat_by_inode_number() {
    let mut t = table(256);
    let ino = t.create("/srv/data", FileType::Regular, mode(0o640), 1000, 100).unwrap();
    let st: StatInfo = t.stat(ino).unwrap();
    assert_eq!(st.uid, 1000);
    assert_eq!(st.gid, 100);
    assert_eq!(st.mode.0 & 0o777, 0o640);
}

#[test]
fn stat_by_path() {
    let mut t = table(256);
    t.create("/home/user/.bashrc", FileType::Regular, mode(0o600), 1001, 1001).unwrap();
    let st = t.stat_path("/home/user/.bashrc").unwrap();
    assert_eq!(st.uid, 1001);
}

#[test]
fn stat_unknown_ino_returns_not_found() {
    let t = table(256);
    assert_eq!(t.stat(9999), Err(InodeError::NotFound));
}

#[test]
fn stat_unknown_path_returns_not_found() {
    let t = table(256);
    assert_eq!(t.stat_path("/nonexistent"), Err(InodeError::NotFound));
}

#[test]
fn stat_reflects_size_after_update() {
    let mut t = table(256);
    let ino = t.create("/large.bin", FileType::Regular, mode(0o644), 0, 0).unwrap();
    t.update_size(ino, 8192, 2_000_000).unwrap();
    let st = t.stat(ino).unwrap();
    assert_eq!(st.size, 8192);
    assert_eq!(st.mtime, 2_000_000);
}

#[test]
fn blocks_field_rounds_up_to_512() {
    let mut t = table(256);
    let ino = t.create("/f.bin", FileType::Regular, mode(0o644), 0, 0).unwrap();
    // 1 byte → 1 × 512-byte block
    t.update_size(ino, 1, 0).unwrap();
    assert_eq!(t.stat(ino).unwrap().blocks, 1);
    // 512 bytes → exactly 1 block
    t.update_size(ino, 512, 0).unwrap();
    assert_eq!(t.stat(ino).unwrap().blocks, 1);
    // 513 bytes → 2 blocks
    t.update_size(ino, 513, 0).unwrap();
    assert_eq!(t.stat(ino).unwrap().blocks, 2);
}

// ---------------------------------------------------------------------------
// delete
// ---------------------------------------------------------------------------

#[test]
fn delete_by_ino_removes_inode() {
    let mut t = table(256);
    let ino = t.create("/tmp/delete-me", FileType::Regular, mode(0o600), 0, 0).unwrap();
    t.delete(ino).unwrap();
    assert_eq!(t.stat(ino), Err(InodeError::NotFound));
    // Path lookup must also return NotFound.
    assert_eq!(t.stat_path("/tmp/delete-me"), Err(InodeError::NotFound));
}

#[test]
fn delete_by_path() {
    let mut t = table(256);
    t.create("/var/run/pid", FileType::Regular, mode(0o644), 0, 0).unwrap();
    t.delete_by_path("/var/run/pid").unwrap();
    assert_eq!(t.stat_path("/var/run/pid"), Err(InodeError::NotFound));
}

#[test]
fn delete_unknown_ino_returns_not_found() {
    let mut t = table(256);
    assert_eq!(t.delete(4242), Err(InodeError::NotFound));
}

#[test]
fn delete_allows_re_create_of_same_path() {
    let mut t = table(256);
    let ino1 = t.create("/tmp/reuse", FileType::Regular, mode(0o644), 0, 0).unwrap();
    t.delete(ino1).unwrap();
    // After deletion the path slot is free; create should succeed.
    let ino2 = t.create("/tmp/reuse", FileType::Regular, mode(0o644), 0, 0).unwrap();
    assert_ne!(ino1, ino2, "new inode should have a different number");
}

// ---------------------------------------------------------------------------
// count
// ---------------------------------------------------------------------------

#[test]
fn count_tracks_allocations() {
    let mut t = table(256);
    assert_eq!(t.count(), 0);
    t.create("/a", FileType::Regular, mode(0o644), 0, 0).unwrap();
    assert_eq!(t.count(), 1);
    let ino = t.create("/b", FileType::Regular, mode(0o644), 0, 0).unwrap();
    assert_eq!(t.count(), 2);
    t.delete(ino).unwrap();
    assert_eq!(t.count(), 1);
}
