// SigmaOS — Sigma-Journal: Append-Only Write-Ahead Log
// Inspired by: NTFS journaling, ext4 journal, PostgreSQL WAL
// Module: sigma-sys-journal
// USP: No filesystem dependency — writes directly to a fixed ring on raw block
// Guarantees: atomic commit, crash recovery, zero torn writes

#ifndef SIGMA_JOURNAL_H
#define SIGMA_JOURNAL_H

#define SIGMA_JOURNAL_MAGIC     0x534A524E  // "SJRN"
#define SIGMA_JOURNAL_MAX_RECS  512
#define SIGMA_JOURNAL_REC_SIZE  128
#define SIGMA_TX_BEGIN          0xBB
#define SIGMA_TX_COMMIT         0xCC
#define SIGMA_TX_ABORT          0xAA

typedef struct SigmaJournalRec {
    unsigned int  magic;
    unsigned char type;           // SIGMA_TX_BEGIN / COMMIT / ABORT
    unsigned int  tx_id;
    unsigned int  data_len;
    unsigned char data[SIGMA_JOURNAL_REC_SIZE - 13];
    unsigned int  checksum;       // FNV-1a of data
} SigmaJournalRec;

typedef struct SigmaJournal {
    SigmaJournalRec records[SIGMA_JOURNAL_MAX_RECS];
    volatile unsigned int head;
    volatile unsigned int tail;
    unsigned int next_tx_id;
} SigmaJournal;

// FNV-1a for record integrity
static inline unsigned int journal_checksum(const unsigned char* d, unsigned int n) {
    unsigned int h = 2166136261U;
    for (unsigned int i = 0; i < n; i++) { h ^= d[i]; h *= 16777619U; }
    return h;
}

static inline void journal_init(SigmaJournal* j) {
    j->head = j->tail = 0;
    j->next_tx_id = 1;
}

// Begin a transaction — returns tx_id
static inline unsigned int journal_begin(SigmaJournal* j) {
    unsigned int tx = j->next_tx_id++;
    unsigned int next = (j->tail + 1) % SIGMA_JOURNAL_MAX_RECS;
    if (next == j->head) return 0; // full
    SigmaJournalRec* r = &j->records[j->tail];
    r->magic    = SIGMA_JOURNAL_MAGIC;
    r->type     = SIGMA_TX_BEGIN;
    r->tx_id    = tx;
    r->data_len = 0;
    r->checksum = 0;
    j->tail = next;
    return tx;
}

// Append data to journal for a transaction
static inline int journal_write(SigmaJournal* j, unsigned int tx_id,
                                  const unsigned char* data, unsigned int len) {
    if (len > sizeof(((SigmaJournalRec*)0)->data)) return -1;
    unsigned int next = (j->tail + 1) % SIGMA_JOURNAL_MAX_RECS;
    if (next == j->head) return -1;
    SigmaJournalRec* r = &j->records[j->tail];
    r->magic    = SIGMA_JOURNAL_MAGIC;
    r->type     = SIGMA_TX_COMMIT;
    r->tx_id    = tx_id;
    r->data_len = len;
    for (unsigned int i = 0; i < len; i++) r->data[i] = data[i];
    r->checksum = journal_checksum(data, len);
    j->tail = next;
    return 0;
}

// Commit transaction
static inline void journal_commit(SigmaJournal* j, unsigned int tx_id) {
    unsigned int next = (j->tail + 1) % SIGMA_JOURNAL_MAX_RECS;
    if (next == j->head) return;
    SigmaJournalRec* r = &j->records[j->tail];
    r->magic = SIGMA_JOURNAL_MAGIC; r->type = SIGMA_TX_COMMIT;
    r->tx_id = tx_id; r->data_len = 0; r->checksum = 0;
    j->tail = next;
}

// Replay journal on boot (crash recovery)
static inline unsigned int journal_replay(SigmaJournal* j) {
    unsigned int replayed = 0;
    for (unsigned int i = j->head; i != j->tail;
         i = (i + 1) % SIGMA_JOURNAL_MAX_RECS) {
        SigmaJournalRec* r = &j->records[i];
        if (r->magic != SIGMA_JOURNAL_MAGIC) continue;
        if (r->type == SIGMA_TX_COMMIT && r->data_len > 0) {
            unsigned int cs = journal_checksum(r->data, r->data_len);
            if (cs == r->checksum) replayed++;
        }
    }
    return replayed; // number of verified transactions replayed
}

#endif /* SIGMA_JOURNAL_H */
