// Standalone Unit Tests for File Range & Advisory Locking Engine
// Part of SigmaOS File Management Subsystem

#[path = "../../src/fs/fd.rs"]
mod fd;
#[path = "../../src/fs/flock.rs"]
mod flock;

use fd::FsError;
use flock::{FileLockManager, F_RDLCK, F_UNLCK, F_WRLCK, LOCK_EX, LOCK_NB, LOCK_SH, LOCK_UN};

#[test]
fn test_flock_shared_and_exclusive_locks() {
    let mut mgr = FileLockManager::new();

    mgr.apply_flock(100, 3, LOCK_SH).expect("p100 shared lock");
    assert_eq!(mgr.active_lock_count(), 1);

    mgr.apply_flock(200, 4, LOCK_SH).expect("p200 shared lock");
    assert_eq!(mgr.active_lock_count(), 2);

    let err = mgr.apply_flock(300, 5, LOCK_EX | LOCK_NB);
    assert_eq!(err.err(), Some(FsError::ResourceBusy));

    mgr.apply_flock(100, 3, LOCK_UN).expect("p100 unlock");
    mgr.apply_flock(200, 4, LOCK_UN).expect("p200 unlock");
    assert_eq!(mgr.active_lock_count(), 0);

    mgr.apply_flock(300, 5, LOCK_EX | LOCK_NB).expect("p300 exclusive lock");
    assert_eq!(mgr.active_lock_count(), 1);
}

#[test]
fn test_range_lock_overlaps_and_cleanup() {
    let mut mgr = FileLockManager::new();

    mgr.apply_range_lock(100, 3, F_WRLCK, 0, 100, true)
        .expect("p100 write lock bytes 0..100");

    let err = mgr.apply_range_lock(200, 4, F_RDLCK, 50, 50, true);
    assert_eq!(err.err(), Some(FsError::ResourceBusy));

    mgr.apply_range_lock(200, 4, F_RDLCK, 150, 50, true)
        .expect("p200 read lock bytes 150..200");

    mgr.release_locks_for_process(100);
    assert_eq!(mgr.active_lock_count(), 1);

    mgr.apply_range_lock(200, 4, F_UNLCK, 150, 50, false).expect("unlock range");
    assert_eq!(mgr.active_lock_count(), 0);
}
