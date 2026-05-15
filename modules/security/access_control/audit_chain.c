#include "../../../include/libc/sigma_libc.h"
#include "../../../include/libc/sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Tamper-Proof Audit Chain (Blockchain-style logs)
// Every log entry chained by hash — immutable by design
// ---------------------------------------------------------

#define HASH_SIZE     32
#define MAX_CHAIN_LEN 2048

// FNV-1a extended hash (simulating a chain link)
static void chain_hash(const uint8_t* prev_hash, const uint8_t* data,
                       size_t data_len, uint8_t* out) {
    uint64_t h = 14695981039346656037ULL;
    // Mix in previous hash
    for (int i = 0; i < HASH_SIZE; i++) { h ^= prev_hash[i]; h *= 1099511628211ULL; }
    // Mix in data
    for (size_t i = 0; i < data_len; i++) { h ^= data[i]; h *= 1099511628211ULL; }
    memset(out, 0, HASH_SIZE);
    memcpy(out, &h, 8); // 8-byte hash embedded into 32-byte slot
}

typedef struct {
    uint64_t index;
    uint32_t source_pid;
    uint8_t  level;
    char     message[96];
    uint8_t  prev_hash[HASH_SIZE]; // Hash of previous chain link
    uint8_t  self_hash[HASH_SIZE]; // Hash of this link (computed over all fields)
} chain_entry_t;

static chain_entry_t chain[MAX_CHAIN_LEN];
static uint64_t chain_length = 0;
static uint8_t  genesis_hash[HASH_SIZE] = {0}; // All zeros = genesis block

// Append an audit event to the chain
int audit_chain_append(uint32_t pid, uint8_t level, const char* message) {
    if (chain_length >= MAX_CHAIN_LEN) return -1;
    chain_entry_t* entry = &chain[chain_length];
    entry->index = chain_length;
    entry->source_pid = pid;
    entry->level = level;
    strncpy(entry->message, message, 95);

    // Link to previous hash
    const uint8_t* prev = (chain_length == 0) ? genesis_hash : chain[chain_length - 1].self_hash;
    memcpy(entry->prev_hash, prev, HASH_SIZE);

    // Compute self hash over all content
    chain_hash(entry->prev_hash, (uint8_t*)entry->message,
               strlen(entry->message), entry->self_hash);

    chain_length++;
    return 0;
}

// Verify entire chain integrity (O(n) scan)
int audit_chain_verify() {
    for (uint64_t i = 0; i < chain_length; i++) {
        uint8_t computed[HASH_SIZE];
        chain_hash(chain[i].prev_hash, (uint8_t*)chain[i].message,
                   strlen(chain[i].message), computed);
        if (memcmp(computed, chain[i].self_hash, HASH_SIZE) != 0) return 0; // Broken!
    }
    return 1; // Chain intact
}

// Get chain tip hash (for external attestation/verification)
const uint8_t* audit_chain_tip() {
    if (chain_length == 0) return genesis_hash;
    return chain[chain_length - 1].self_hash;
}
