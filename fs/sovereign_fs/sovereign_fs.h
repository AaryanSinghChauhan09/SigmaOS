#ifndef SOVEREIGN_FS_H
#define SOVEREIGN_FS_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN FS (SFS)
 * =========================================================================
 * Journaling, Copy-on-Write, Cryptographically Verified Filesystem.
 * =========================================================================
 */

#define SFS_BLOCK_SIZE 4096
#define SFS_MAGIC 0x53465321 // "SFS!"

// Error codes
#define SFS_OK 0
#define SFS_ERR_CORRUPT -1
#define SFS_ERR_IO -2
#define SFS_ERR_NO_SPACE -3
#define SFS_ERR_NOT_FOUND -4

typedef int sfs_status_t;

// BLAKE3 Checksum type (stub)
typedef struct {
    uint8_t hash[32];
} sfs_blake3_t;

// Inode Structure
typedef struct {
    uint64_t inode_num;
    uint64_t size_bytes;
    uint64_t permissions; // Lattice-Based Access Control labels
    uint64_t created_at_ns;
    uint64_t modified_at_ns;
    sfs_blake3_t data_checksum; // Merkle root of data extents
} sfs_inode_t;

// Mount handle
typedef struct sfs_mount sfs_mount_t;

/**
 * Mount a SovereignFS volume.
 * Verifies the superblock and replays the journal if dirty.
 */
sfs_status_t sfs_mount(const char* device, sfs_mount_t** out_mount);

/**
 * Unmount and flush all pending transactions to the journal.
 */
sfs_status_t sfs_unmount(sfs_mount_t* mnt);

/**
 * Open or create a file on SFS.
 * Returns inode metadata.
 */
sfs_status_t sfs_open(sfs_mount_t* mnt, const char* path, int flags, sfs_inode_t* out_inode);

/**
 * Read data with automatic BLAKE3 block integrity verification.
 * Fails immediately if the block hash does not match the Merkle root.
 */
sfs_status_t sfs_read(sfs_mount_t* mnt, sfs_inode_t* inode, uint64_t offset, void* buffer, size_t size, size_t* out_read);

/**
 * Write data using Copy-on-Write extents.
 * Commits metadata to the journal atomically.
 */
sfs_status_t sfs_write(sfs_mount_t* mnt, sfs_inode_t* inode, uint64_t offset, const void* buffer, size_t size, size_t* out_written);

/**
 * Create an instant CoW snapshot of the filesystem state.
 */
sfs_status_t sfs_snapshot_create(sfs_mount_t* mnt, const char* snapshot_name);

#ifdef __cplusplus
}
#endif

#endif // SOVEREIGN_FS_H
