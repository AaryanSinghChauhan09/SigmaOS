// =============================================================================
// SigmaOS — S06_Storage — SovereignJournalingFS.c
// Ordered Journaling & CoW Transaction Log
// =============================================================================
// Competitor USPs Absorbed:
//   • ext4 (Linux)  — ordered journaling: metadata + optional data journal
//   • NTFS (Windows)— transaction log ($LogFile) for crash consistency
//   • ZFS (Sun)     — Copy-on-Write: never overwrites live data, always atomic
//   • APFS (Apple)  — atomic safe-save: power-fail consistent transactions
// Architecture:
//   • Journal ring written to a dedicated log partition area
//   • Ordered mode: data blocks flushed before metadata committed (ext4 default)
//   • CoW mode: new writes go to free blocks; pointer swapped atomically (ZFS)
//   • Recovery: replay uncommitted journal entries on mount after crash
// =============================================================================

#include "core/sigma_types.h"


#define SIGMA_JOURNAL_MAGIC       0x5349474D4A524E4CULL // "SIGMAJRNL"
#define SIGMA_JOURNAL_MAX_ENTRIES  8192
#define SIGMA_JOURNAL_BLOCK_SIZE   4096

// ── Journal Entry States ─────────────────────────────────────────────────────
typedef enum {
    JRNL_STATE_FREE      = 0,
    JRNL_STATE_DIRTY     = 1,   // Written to journal, not yet checkpointed
    JRNL_STATE_COMMITTED = 2,   // Metadata flushed to final location
    JRNL_STATE_ABORTED   = 3,   // Rolled back due to error
} JournalEntryState;

// ── Journal Entry ────────────────────────────────────────────────────────────
typedef struct {
    uint64_t          seq_number;
    uint64_t          block_lba;      // Logical block address on disk
    uint8_t           block_data[SIGMA_JOURNAL_BLOCK_SIZE];
    JournalEntryState state;
    uint32_t          checksum;       // CRC32 of block_data
} SigmaJournalEntry;

// ── Transaction Handle ────────────────────────────────────────────────────────
typedef struct {
    uint64_t  tx_id;
    uint32_t  entry_start;
    uint32_t  entry_count;
    bool      use_cow;   // ZFS/APFS CoW mode vs ext4 ordered mode
} SigmaTransaction;

// ── Public API ───────────────────────────────────────────────────────────────

// Start a new atomic file system transaction
SigmaTransaction fs_journal_begin(bool use_cow_mode);

// Write a dirty block into the journal (does NOT touch the live FS yet)
bool fs_journal_log_block(SigmaTransaction* tx, uint64_t lba, void* data);

// Commit the transaction: flush data, then swap metadata pointers atomically
bool fs_journal_commit(SigmaTransaction* tx);

// Abort and roll back a transaction (NTFS rollback equivalent)
void fs_journal_abort(SigmaTransaction* tx);

// On-mount crash recovery: replay all DIRTY entries, discard ABORTED
void fs_journal_recover(void);



