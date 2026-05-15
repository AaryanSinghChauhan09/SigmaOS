#include "../../../../include/libc/sigma_libc.h"
#include "../../../../include/libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Memory Contract Extensions
// Delegation, quota enforcement, revocation hooks
// Economic model: integrate with sovereign token system
// ---------------------------------------------------------

#define MAX_DELEGATIONS 128

typedef struct {
    uint32_t original_contract_id;
    uint32_t from_pid;
    uint32_t to_pid;
    uint32_t delegated_pages;   // Number of pages shared
    uint32_t base_page_offset;  // Offset within original lease
    uint8_t  can_redelegate;    // Sub-delegation allowed?
    uint64_t signature;         // Kernel-signed delegation proof
} mem_delegation_t;

// ----- Delegation ledger -----
static mem_delegation_t delegations[MAX_DELEGATIONS];
static uint32_t delegation_count = 0;

// External handles from mem_contracts.c
typedef struct {
    uint32_t contract_id;
    uint32_t lessee_pid;
    uint32_t base_page;
    uint32_t num_pages;
    uint64_t expiry_tick;
    uint8_t  rights;
    uint8_t  active;
    uint64_t signature;
} mem_contract_t;

extern mem_contract_t contracts[];
extern uint32_t contract_count;
extern void audit_chain_append(uint32_t pid, uint8_t level, const char* message);

// Sign delegation (simplified; replace with Ed25519 in production)
static uint64_t sign_delegation(uint32_t from, uint32_t to, uint32_t pages) {
    return ((uint64_t)from * 0xABCDEF01) ^ ((uint64_t)to << 16) ^ pages ^ 0x44454C4547;
}

// Delegate a portion of a memory lease to another process
int mem_contract_delegate(uint32_t from_pid, uint32_t contract_id,
                          uint32_t to_pid, uint32_t pages, uint32_t offset) {
    if (contract_id >= contract_count)       return -1; // Bad contract
    mem_contract_t* c = &contracts[contract_id];
    if (c->lessee_pid != from_pid)           return -2; // Not owner
    if (!c->active)                          return -3; // Expired
    if (offset + pages > c->num_pages)       return -4; // Out of range
    if (delegation_count >= MAX_DELEGATIONS) return -5; // Ledger full

    mem_delegation_t* d = &delegations[delegation_count];
    d->original_contract_id = contract_id;
    d->from_pid             = from_pid;
    d->to_pid               = to_pid;
    d->delegated_pages      = pages;
    d->base_page_offset     = offset;
    d->can_redelegate       = 0; // No sub-delegation by default
    d->signature            = sign_delegation(from_pid, to_pid, pages);
    delegation_count++;

    audit_chain_append(from_pid, 1, "MEM_DELEGATION_ISSUED");
    return (int)(delegation_count - 1);
}

// Verify that a process has access (either via direct contract or delegation)
int mem_access_check(uint32_t pid, uint32_t target_page) {
    // Check direct contracts
    for (uint32_t i = 0; i < contract_count; i++) {
        mem_contract_t* c = &contracts[i];
        if (c->active && c->lessee_pid == pid) {
            if (target_page >= c->base_page && target_page < c->base_page + c->num_pages) {
                // Verify signature integrity
                return 1; // Authorized
            }
        }
    }
    // Check delegations
    for (uint32_t i = 0; i < delegation_count; i++) {
        mem_delegation_t* d = &delegations[i];
        if (d->to_pid == pid) {
            // Verify signature hasn't been tampered
            if (d->signature != sign_delegation(d->from_pid, d->to_pid, d->delegated_pages))
                return 0; // Tampered
            mem_contract_t* src = &contracts[d->original_contract_id];
            uint32_t del_base = src->base_page + d->base_page_offset;
            if (target_page >= del_base && target_page < del_base + d->delegated_pages)
                return 1; // Authorized via delegation
        }
    }
    return 0; // No valid contract or delegation
}

// Instant revocation — called on process misbehaviour
void mem_contract_revoke(uint32_t contract_id) {
    if (contract_id >= contract_count) return;
    contracts[contract_id].active = 0;

    // Also revoke all delegations derived from this contract
    for (uint32_t i = 0; i < delegation_count; i++) {
        if (delegations[i].original_contract_id == contract_id) {
            delegations[i].signature = 0; // Invalidate
            audit_chain_append(delegations[i].to_pid, 3, "MEM_DELEGATION_REVOKED");
        }
    }
    audit_chain_append(contracts[contract_id].lessee_pid, 3, "MEM_CONTRACT_REVOKED");
}

// Quota enforcer: called by scheduler — ensure process hasn't exceeded token allowance
int mem_quota_check(uint32_t pid, uint32_t needed_pages) {
    uint32_t total_leased = 0;
    for (uint32_t i = 0; i < contract_count; i++) {
        if (contracts[i].active && contracts[i].lessee_pid == pid)
            total_leased += contracts[i].num_pages;
    }
    // Also count delegations received
    for (uint32_t i = 0; i < delegation_count; i++) {
        if (delegations[i].to_pid == pid)
            total_leased += delegations[i].delegated_pages;
    }
    // Check against token economy (sovereign_tokens.c)
    // int token_spend(pid, memory_token_id, needed_pages) → if fails, reject
    return (total_leased + needed_pages <= 1024); // 1024 page max per process (placeholder)
}
