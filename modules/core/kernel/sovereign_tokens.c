#include <stdint.h>
#include <stddef.h>

// ---------------------------------------------------------
// SigmaOS Resource Economy — Sovereign Token System
// CPU cycles, I/O bandwidth, and memory all "leased" via tokens
// ---------------------------------------------------------

#define MAX_TOKEN_LEDGER  512
#define TOKEN_TYPE_MEMORY  0x01
#define TOKEN_TYPE_CPU     0x02
#define TOKEN_TYPE_IO      0x04

typedef struct {
    uint32_t token_id;
    uint32_t owner_pid;
    uint8_t  resource_type;     // TOKEN_TYPE_MEMORY | CPU | IO
    uint64_t amount;            // Pages / CPU nanoseconds / IO bytes
    uint64_t expiry_tick;
    uint8_t  transferable;      // Can this token be delegated?
    uint64_t signature;         // Kernel-signed mint proof
    uint8_t  spent;             // 1 = consumed
} sovereign_token_t;

static sovereign_token_t ledger[MAX_TOKEN_LEDGER];
static uint32_t ledger_size = 0;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* message);

static uint64_t mint_signature(uint32_t pid, uint8_t type, uint64_t amount) {
    return ((uint64_t)pid * 0xDEADBEEF) ^ ((uint64_t)type << 40) ^ amount ^ 0x5359534F53 /* "SIGMAOS" */;
}

// Kernel mints a resource token for a process
int token_mint(uint32_t pid, uint8_t resource_type, uint64_t amount, uint64_t expiry_tick, uint8_t transferable) {
    if (ledger_size >= MAX_TOKEN_LEDGER) return -1;

    sovereign_token_t* t = &ledger[ledger_size];
    t->token_id       = ledger_size++;
    t->owner_pid      = pid;
    t->resource_type  = resource_type;
    t->amount         = amount;
    t->expiry_tick    = expiry_tick;
    t->transferable   = transferable;
    t->signature      = mint_signature(pid, resource_type, amount);
    t->spent          = 0;

    audit_chain_append(pid, 1, "TOKEN_MINTED");
    return t->token_id;
}

// Spend a token to authorize resource access
int token_spend(uint32_t pid, uint32_t token_id, uint64_t amount_needed) {
    if (token_id >= ledger_size) return -1;
    sovereign_token_t* t = &ledger[token_id];

    if (t->owner_pid != pid)   return -2; // Not owner
    if (t->spent)              return -3; // Already spent
    if (t->amount < amount_needed) return -4; // Insufficient tokens

    // Verify kernel signature hasn't been tampered
    if (t->signature != mint_signature(pid, t->resource_type, t->amount)) return -5;

    t->amount -= amount_needed;
    if (t->amount == 0) t->spent = 1;

    audit_chain_append(pid, 1, "TOKEN_SPENT");
    return 0;
}

// Delegate (transfer) a portion of a token to another process
int token_delegate(uint32_t from_pid, uint32_t token_id, uint32_t to_pid, uint64_t delegate_amount) {
    if (token_id >= ledger_size) return -1;
    sovereign_token_t* t = &ledger[token_id];

    if (t->owner_pid != from_pid) return -2;
    if (!t->transferable)         return -3;
    if (t->amount < delegate_amount) return -4;

    // Deduct from source
    t->amount -= delegate_amount;

    // Mint a new token for the recipient
    int new_id = token_mint(to_pid, t->resource_type, delegate_amount, t->expiry_tick, 0 /* not re-delegatable */);
    audit_chain_append(from_pid, 1, "TOKEN_DELEGATED");
    return new_id;
}

// Enforce expiries — called by the scheduler tick
void token_enforce_expiries(uint64_t current_tick) {
    for (uint32_t i = 0; i < ledger_size; i++) {
        if (!ledger[i].spent && ledger[i].expiry_tick != 0 && current_tick >= ledger[i].expiry_tick) {
            ledger[i].spent = 1;
            audit_chain_append(ledger[i].owner_pid, 3, "TOKEN_EXPIRED");
        }
    }
}
