#include <stdint.h>
#include <stddef.h>

// ---------------------------------------------------------
// SigmaOS Memory-as-Contracts Prototype
// Cryptographic memory leasing and dynamic quotas
// ---------------------------------------------------------

#define MAX_CONTRACTS 256

typedef struct {
    uint32_t contract_id;
    uint32_t lessee_pid;      // Process leasing the memory
    uint32_t base_page;       // Starting physical page
    uint32_t num_pages;       // Size of the lease
    uint64_t expiry_tick;     // When the lease expires (0 = indefinite)
    uint8_t  rights;          // Read/Write/Execute permissions
    uint8_t  active;
    uint64_t signature;       // Cryptographic proof of lease issuance
} mem_contract_t;

static mem_contract_t contracts[MAX_CONTRACTS];
static uint32_t contract_count = 0;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* message);
extern void page_table_unmap(uint32_t pid, uint32_t base_page, uint32_t num_pages);

// Sign a contract (mock signature, replace with ed25519)
static uint64_t sign_contract(uint32_t pid, uint32_t base, uint32_t pages) {
    return ((uint64_t)pid << 32) ^ base ^ pages ^ 0xCAFEBABE;
}

// Issue a new memory lease
int mem_contract_lease(uint32_t pid, uint32_t base_page, uint32_t num_pages, uint64_t duration_ticks) {
    if (contract_count >= MAX_CONTRACTS) return -1;
    
    mem_contract_t* c = &contracts[contract_count];
    c->contract_id = contract_count++;
    c->lessee_pid = pid;
    c->base_page = base_page;
    c->num_pages = num_pages;
    // c->expiry_tick = get_system_uptime() + duration_ticks;
    c->rights = 0x03; // RW by default
    c->active = 1;
    c->signature = sign_contract(pid, base_page, num_pages);

    audit_chain_append(pid, 2, "MEMORY_LEASE_ISSUED");
    return c->contract_id;
}

// Verify a memory access against active contracts
int mem_contract_verify(uint32_t pid, uint32_t target_page) {
    for (uint32_t i = 0; i < contract_count; i++) {
        if (contracts[i].active && contracts[i].lessee_pid == pid) {
            if (target_page >= contracts[i].base_page && 
                target_page < (contracts[i].base_page + contracts[i].num_pages)) {
                
                // Check signature integrity
                if (contracts[i].signature != sign_contract(pid, contracts[i].base_page, contracts[i].num_pages)) {
                    // Tampering detected
                    return 0;
                }
                return 1; // Access authorized
            }
        }
    }
    return 0; // No valid contract found
}

// Periodic hook called by scheduler to enforce expiries
void mem_contract_enforce_expiries(uint64_t current_tick) {
    for (uint32_t i = 0; i < contract_count; i++) {
        if (contracts[i].active && contracts[i].expiry_tick != 0) {
            if (current_tick >= contracts[i].expiry_tick) {
                // Lease expired! Revoke instantly.
                contracts[i].active = 0;
                // page_table_unmap(contracts[i].lessee_pid, contracts[i].base_page, contracts[i].num_pages);
                audit_chain_append(contracts[i].lessee_pid, 3, "MEMORY_LEASE_EXPIRED");
            }
        }
    }
}
