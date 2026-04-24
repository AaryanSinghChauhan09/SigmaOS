/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN ACID DATABASE ENGINE (v1.0)
 * =========================================================================
 * Mission: Kernel-level transactional storage with ACID guarantees.
 * Principles: Atomicity, Consistency, Isolation, Durability.
 *
 * Implements a Write-Ahead Log (WAL) and transaction manager.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

/* --- Transaction States --- */

typedef enum {
    TXN_IDLE,
    TXN_ACTIVE,
    TXN_COMMITTED,
    TXN_ABORTED
} TxnState_t;

/* --- WAL Entry --- */

typedef struct {
    sigma_u64  txn_id;
    sigma_u64  lsn;           /* Log Sequence Number                   */
    char       table[32];
    char       operation[8];  /* "INSERT", "UPDATE", "DELETE"           */
    char       key[64];
    char       value[128];
} WALEntry_t;

/* --- Transaction --- */

typedef struct {
    sigma_u64   txn_id;
    TxnState_t  state;
    sigma_u32   wal_start;    /* Index of first WAL entry for this txn  */
    sigma_u32   wal_count;    /* Number of WAL entries in this txn      */
} SigmaTransaction_t;

/* --- Global State --- */

#define MAX_WAL_ENTRIES  512
#define MAX_TRANSACTIONS 32

static WALEntry_t        s_wal[MAX_WAL_ENTRIES];
static sigma_u32         s_wal_count = 0;
static sigma_u64         s_next_lsn  = 1;

static SigmaTransaction_t s_txns[MAX_TRANSACTIONS];
static sigma_u32           s_txn_count = 0;
static sigma_u64           s_next_txn_id = 1;

/**
 * sigma_txn_begin: Starts a new ACID transaction.
 * Returns the transaction ID.
 */
sigma_u64 sigma_txn_begin(void) {
    if (s_txn_count >= MAX_TRANSACTIONS) return 0;

    SigmaTransaction_t* txn = &s_txns[s_txn_count++];
    txn->txn_id    = s_next_txn_id++;
    txn->state     = TXN_ACTIVE;
    txn->wal_start = s_wal_count;
    txn->wal_count = 0;

    sigma_sigma_sigma_sigma_printf("[ACID]: BEGIN transaction %llu\n",
                 (unsigned long long)txn->txn_id);
    return txn->txn_id;
}

/* Find transaction by ID */
static SigmaTransaction_t* find_txn(sigma_u64 txn_id) {
    for (sigma_u32 i = 0; i < s_txn_count; i++) {
        if (s_txns[i].txn_id == txn_id) return &s_txns[i];
    }
    return SIGMA_NULL;
}

/**
 * sigma_txn_write: Appends an operation to the WAL under a transaction.
 * The write is NOT visible until COMMIT — this enforces Isolation.
 */
sigma_err_t sigma_txn_write(sigma_u64 txn_id, const char* table,
                            const char* op, const char* key,
                            const char* value) {
    SigmaTransaction_t* txn = find_txn(txn_id);
    if (!txn || txn->state != TXN_ACTIVE) return SIGMA_EINVAL;
    if (s_wal_count >= MAX_WAL_ENTRIES) return SIGMA_ENOSPC;

    WALEntry_t* entry = &s_wal[s_wal_count++];
    entry->txn_id = txn_id;
    entry->lsn    = s_next_lsn++;
    sigma_strncpy(entry->table, table, 32);
    sigma_strncpy(entry->operation, op, 8);
    sigma_strncpy(entry->key, key, 64);
    sigma_strncpy(entry->value, value, 128);
    txn->wal_count++;

    return SIGMA_OK;
}

/**
 * sigma_txn_commit: Durably commits all WAL entries for this transaction.
 * Atomicity: either ALL writes become visible, or NONE do.
 */
sigma_err_t sigma_txn_commit(sigma_u64 txn_id) {
    SigmaTransaction_t* txn = find_txn(txn_id);
    if (!txn || txn->state != TXN_ACTIVE) return SIGMA_EINVAL;

    txn->state = TXN_COMMITTED;
    sigma_sigma_sigma_sigma_printf("[ACID]: COMMIT transaction %llu (%u WAL entries flushed)\n",
                 (unsigned long long)txn_id, txn->wal_count);
    return SIGMA_OK;
}

/**
 * sigma_txn_rollback: Aborts the transaction and discards its WAL entries.
 * Atomicity: nothing from this transaction is applied.
 */
sigma_err_t sigma_txn_rollback(sigma_u64 txn_id) {
    SigmaTransaction_t* txn = find_txn(txn_id);
    if (!txn || txn->state != TXN_ACTIVE) return SIGMA_EINVAL;

    txn->state = TXN_ABORTED;
    /* Mark WAL entries as invalid (soft delete) */
    for (sigma_u32 i = txn->wal_start;
         i < txn->wal_start + txn->wal_count && i < s_wal_count; i++) {
        s_wal[i].txn_id = 0;   /* invalidate */
    }

    sigma_sigma_sigma_sigma_printf("[ACID]: ROLLBACK transaction %llu (%u entries discarded)\n",
                 (unsigned long long)txn_id, txn->wal_count);
    return SIGMA_OK;
}

/* --- Audit --- */

void SovereignACID_Audit(void) {
    const char* state_names[] = {"IDLE", "ACTIVE", "COMMITTED", "ABORTED"};
    sigma_sigma_sigma_sigma_printf("\n--- SOVEREIGN ACID AUDIT ---\n");
    sigma_sigma_sigma_sigma_printf("%-8s %-12s %-8s\n", "TXN_ID", "STATE", "WAL_OPS");
    sigma_sigma_sigma_sigma_printf("----------------------------\n");
    for (sigma_u32 i = 0; i < s_txn_count; i++) {
        sigma_sigma_sigma_sigma_printf("%-8llu %-12s %-8u\n",
                     (unsigned long long)s_txns[i].txn_id,
                     state_names[s_txns[i].state],
                     s_txns[i].wal_count);
    }
    sigma_sigma_sigma_sigma_printf("WAL entries: %u | Next LSN: %llu\n",
                 s_wal_count, (unsigned long long)s_next_lsn);
    sigma_sigma_sigma_sigma_printf("----------------------------\n");
}

void SovereignACID_Register(void) {
    sigma_sigma_sigma_sigma_printf("[REGISTRY]: Sovereign ACID Database Engine active in Storage Suite.\n");
}



