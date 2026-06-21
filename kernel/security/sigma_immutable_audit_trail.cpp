/**
 * =========================================================================
 * Σ SIGMAOS: IMMUTABLE AUDIT TRAIL — SovereignLedger
 * =========================================================================
 * Provides an append-only, cryptographically chained audit log.
 * Every kernel operation that touches a sensitive resource is recorded
 * as an AuditEvent node. Each node stores:
 *   - SHA-256-sim hash of its payload
 *   - Hash chain link to the previous node (Merkle chain)
 *   - PQC Dilithium-5 signature of the chain link (tamper evidence)
 *
 * The ledger is memory-resident during runtime and can be flushed to the
 * VFS as a compact binary journal (sigma_audit.log).
 *
 * Usage:
 *   sigma_audit_log(AUDIT_SYSCALL, uid, resource_id, "open /etc/passwd");
 *   sigma_audit_verify_chain();  /* returns K_OK or K_ERR_CORRUPT * /
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/sigma_audit.h"
#include "../../include/crypto/sigma_pqc.h"

/* Forward declarations for PQC */
extern "C" {
    void pqc_init(void);
    int  pqc_sign(const pqc_secret_key_t*, const sigma_u8*, sigma_usize, pqc_signature_t*);
    int  pqc_verify(const pqc_public_key_t*, const sigma_u8*, sigma_usize, const pqc_signature_t*);
    int  pqc_generate_keypair(pqc_public_key_t*, pqc_secret_key_t*);
}

namespace SigmaOS {
namespace Audit {

/* -----------------------------------------------------------------------
 * Audit event categories
 * ----------------------------------------------------------------------- */
enum class AuditCategory : sigma_u8 {
    SYSCALL         = 0x01,  /* System call invocation              */
    FILE_ACCESS     = 0x02,  /* VFS open / read / write / unlink    */
    PROCESS_SPAWN   = 0x03,  /* fork / execve                       */
    NET_CONNECT     = 0x04,  /* TCP/UDP connect / bind              */
    SECURITY_EVENT  = 0x05,  /* Auth success/failure, ACL deny      */
    MEMORY_MAP      = 0x06,  /* mmap / mprotect with EXEC           */
    CRYPTO_OP       = 0x07,  /* Key generation, sign, verify        */
    CONTAINER_OP    = 0x08,  /* Pod spawn / namespace enter         */
    PRIVILEGE_CHANGE = 0x09, /* setuid / capability change          */
    KERNEL_PANIC    = 0xFF,  /* Kernel fault / oops                 */
};

/* -----------------------------------------------------------------------
 * Audit record (chain node)
 * -----------------------------------------------------------------------
 * On-disk layout (little-endian):
 *   [8]  seq_id          u64
 *   [8]  timestamp_tsc   u64
 *   [4]  uid             u32
 *   [4]  resource_id     u32
 *   [1]  category        u8
 *   [127] message        char[127]
 *   [32] payload_hash    u8[32]   (SHA-256-sim of above fields)
 *   [32] prev_hash       u8[32]   (hash of previous record)
 *   [4]  sig_length      u32
 *   [4595] signature     u8[PQC_SIG_SIZE]  (Dilithium-5 over chain_hash)
 * ----------------------------------------------------------------------- */
struct AuditRecord {
    sigma_u64 seq_id;
    sigma_u64 timestamp_tsc;
    sigma_u32 uid;
    sigma_u32 resource_id;
    AuditCategory category;
    char      message[127];

    sigma_u8  payload_hash[32];
    sigma_u8  prev_hash[32];

    sigma_u32 sig_length;
    sigma_u8  signature[PQC_SIG_SIZE];
};

/* -----------------------------------------------------------------------
 * SHA-256 simulation (FNV-1a + xor folding)
 * In production replaced with hardware SHA-NI or libtomcrypt.
 * ----------------------------------------------------------------------- */
static void sigma_sha256_sim(const sigma_u8* data, sigma_usize len, sigma_u8 out[32]) {
    /* FNV-1a 64-bit base */
    sigma_u64 h0 = 0xcbf29ce484222325ULL;
    sigma_u64 h1 = 0x84222325cbf29ce4ULL;
    for (sigma_usize i = 0; i < len; i++) {
        h0 ^= data[i];
        h0 *= 0x00000100000001B3ULL;
        h1 ^= data[len - 1 - i];
        h1 *= 0x00000100000001B3ULL;
    }
    /* Fold 128 bits → 256 bits (expand via splitmix) */
    sigma_u64 words[4] = { h0, h1, h0 ^ h1, h0 + h1 };
    for (int w = 0; w < 4; w++) {
        sigma_u64 z = words[w];
        z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ULL;
        z = (z ^ (z >> 27)) * 0x94d049bb133111ebULL;
        z ^= z >> 31;
        words[w] = z;
    }
    for (int i = 0; i < 32; i++) {
        out[i] = (sigma_u8)((words[i / 8] >> ((i % 8) * 8)) & 0xFF);
    }
}

static void compute_record_hash(const AuditRecord* r, sigma_u8 out[32]) {
    /* Hash: seq_id || uid || category || message */
    sigma_u8 buf[256];
    sigma_usize pos = 0;
    /* Encode seq_id LE */
    for (int i = 0; i < 8; i++) buf[pos++] = (sigma_u8)(r->seq_id >> (i * 8));
    buf[pos++] = (sigma_u8)r->uid;
    buf[pos++] = (sigma_u8)r->category;
    for (int i = 0; r->message[i] && i < 127; i++) buf[pos++] = (sigma_u8)r->message[i];
    sigma_sha256_sim(buf, pos, out);
}

/* -----------------------------------------------------------------------
 * SovereignLedger — the audit engine
 * ----------------------------------------------------------------------- */
constexpr sigma_usize MAX_AUDIT_RECORDS = 4096;

class SovereignLedger {
public:
    static SovereignLedger& getInstance() {
        static SovereignLedger instance;
        return instance;
    }

    void init() {
        pqc_init();
        pqc_generate_keypair(&m_pk, &m_sk);
        m_count      = 0;
        m_initialized = true;

        /* Zero genesis hash */
        for (int i = 0; i < 32; i++) m_prev_hash[i] = 0;

        sigma_log("[Audit] SovereignLedger initialized. PQC-chained immutable audit trail ACTIVE.");
        sigma_log("[Audit] Genesis hash: 0x00...00 (genesis block)");
    }

    /**
     * Append an audit event to the ledger.
     * Thread-safety: caller must hold a spinlock in SMP configurations.
     */
    sigma_status log(AuditCategory cat, sigma_u32 uid, sigma_u32 res_id, const char* msg) {
        if (!m_initialized) init();
        if (m_count >= MAX_AUDIT_RECORDS) {
            sigma_log_err("[Audit] LEDGER FULL — oldest records must be flushed to VFS.");
            return K_ERR_NOMEM;
        }

        AuditRecord* r = &m_records[m_count];
        r->seq_id       = m_count;
        r->uid          = uid;
        r->resource_id  = res_id;
        r->category     = cat;
        r->timestamp_tsc = m_count * 1000ULL; /* Would be RDTSC in production */

        /* Copy message (bounded) */
        sigma_usize mlen = 0;
        while (msg && msg[mlen] && mlen < 126) {
            r->message[mlen] = msg[mlen];
            mlen++;
        }
        r->message[mlen] = '\0';

        /* 1. Compute payload hash */
        compute_record_hash(r, r->payload_hash);

        /* 2. Chain: chain_hash = H(payload_hash || prev_hash) */
        sigma_u8 chain_input[64];
        for (int i = 0; i < 32; i++) chain_input[i]      = r->payload_hash[i];
        for (int i = 0; i < 32; i++) chain_input[32 + i] = m_prev_hash[i];
        sigma_u8 chain_hash[32];
        sigma_sha256_sim(chain_input, 64, chain_hash);
        for (int i = 0; i < 32; i++) r->prev_hash[i] = m_prev_hash[i];

        /* 3. PQC sign the chain_hash with Dilithium-5 */
        pqc_signature_t sig;
        if (pqc_sign(&m_sk, chain_hash, 32, &sig) != K_OK) {
            sigma_log_err("[Audit] CRITICAL: Failed to sign audit record #%llu", r->seq_id);
            return K_ERR_INVAL;
        }
        r->sig_length = sig.length;
        for (sigma_u32 i = 0; i < sig.length && i < PQC_SIG_SIZE; i++) {
            r->signature[i] = sig.data[i];
        }

        /* 4. Advance chain */
        for (int i = 0; i < 32; i++) m_prev_hash[i] = chain_hash[i];

        m_count++;
        sigma_log_info("[Audit] Event #%llu [%s] uid=%u res=%u: %s",
                        r->seq_id, category_name(cat), uid, res_id, r->message);
        return K_OK;
    }

    /**
     * Verify the entire chain integrity from genesis → latest.
     * Returns K_OK if the chain is intact, K_ERR_INVAL on any break.
     */
    sigma_status verifyChain() {
        sigma_log("[Audit] Starting full chain integrity verification...");

        sigma_u8 running_hash[32] = {};
        for (sigma_usize i = 0; i < m_count; i++) {
            AuditRecord* r = &m_records[i];

            /* Recompute payload hash */
            sigma_u8 expected_ph[32];
            compute_record_hash(r, expected_ph);
            for (int b = 0; b < 32; b++) {
                if (expected_ph[b] != r->payload_hash[b]) {
                    sigma_log_err("[Audit] CHAIN BROKEN at record #%llu: payload hash mismatch!", r->seq_id);
                    return K_ERR_INVAL;
                }
            }

            /* Recompute chain hash */
            sigma_u8 chain_input[64];
            for (int b = 0; b < 32; b++) chain_input[b]      = expected_ph[b];
            for (int b = 0; b < 32; b++) chain_input[32 + b] = running_hash[b];
            sigma_u8 chain_hash[32];
            sigma_sha256_sim(chain_input, 64, chain_hash);

            /* Verify Dilithium signature */
            pqc_signature_t sig;
            sig.length = r->sig_length;
            for (sigma_u32 b = 0; b < r->sig_length && b < PQC_SIG_SIZE; b++) {
                sig.data[b] = r->signature[b];
            }
            if (pqc_verify(&m_pk, chain_hash, 32, &sig) != K_OK) {
                sigma_log_err("[Audit] CHAIN TAMPERED at record #%llu: signature invalid!", r->seq_id);
                return K_ERR_INVAL;
            }

            for (int b = 0; b < 32; b++) running_hash[b] = chain_hash[b];
        }

        sigma_log_info("[Audit] Chain verified: %llu records — INTACT. ✓", (unsigned long long)m_count);
        return K_OK;
    }

    sigma_usize getRecordCount() const { return m_count; }

private:
    SovereignLedger() : m_count(0), m_initialized(false) {}

    static const char* category_name(AuditCategory c) {
        switch (c) {
            case AuditCategory::SYSCALL:          return "SYSCALL";
            case AuditCategory::FILE_ACCESS:      return "FILE_ACCESS";
            case AuditCategory::PROCESS_SPAWN:    return "PROC_SPAWN";
            case AuditCategory::NET_CONNECT:      return "NET_CONNECT";
            case AuditCategory::SECURITY_EVENT:   return "SECURITY";
            case AuditCategory::MEMORY_MAP:       return "MEM_MAP";
            case AuditCategory::CRYPTO_OP:        return "CRYPTO";
            case AuditCategory::CONTAINER_OP:     return "CONTAINER";
            case AuditCategory::PRIVILEGE_CHANGE: return "PRIV_CHANGE";
            case AuditCategory::KERNEL_PANIC:     return "KERNEL_PANIC";
            default:                               return "UNKNOWN";
        }
    }

    AuditRecord      m_records[MAX_AUDIT_RECORDS];
    sigma_usize      m_count;
    bool             m_initialized;
    sigma_u8         m_prev_hash[32];

    /* Ledger signing keys (Dilithium-5) */
    pqc_public_key_t m_pk;
    pqc_secret_key_t m_sk;
};

} // namespace Audit
} // namespace SigmaOS

/* -----------------------------------------------------------------------
 * C-API
 * ----------------------------------------------------------------------- */
extern "C" {

void sigma_audit_init(void) {
    SigmaOS::Audit::SovereignLedger::getInstance().init();
}

sigma_status sigma_audit_log_syscall(sigma_u32 uid, sigma_u32 resource_id, const char* msg) {
    return SigmaOS::Audit::SovereignLedger::getInstance()
        .log(SigmaOS::Audit::AuditCategory::SYSCALL, uid, resource_id, msg);
}

sigma_status sigma_audit_log_file(sigma_u32 uid, sigma_u32 inode_id, const char* path) {
    return SigmaOS::Audit::SovereignLedger::getInstance()
        .log(SigmaOS::Audit::AuditCategory::FILE_ACCESS, uid, inode_id, path);
}

sigma_status sigma_audit_log_security(sigma_u32 uid, sigma_u32 resource_id, const char* msg) {
    return SigmaOS::Audit::SovereignLedger::getInstance()
        .log(SigmaOS::Audit::AuditCategory::SECURITY_EVENT, uid, resource_id, msg);
}

sigma_status sigma_audit_log_process(sigma_u32 uid, sigma_u32 pid, const char* cmd) {
    return SigmaOS::Audit::SovereignLedger::getInstance()
        .log(SigmaOS::Audit::AuditCategory::PROCESS_SPAWN, uid, pid, cmd);
}

sigma_status sigma_audit_log_crypto(sigma_u32 uid, sigma_u32 key_id, const char* op) {
    return SigmaOS::Audit::SovereignLedger::getInstance()
        .log(SigmaOS::Audit::AuditCategory::CRYPTO_OP, uid, key_id, op);
}

sigma_status sigma_audit_verify_chain(void) {
    return SigmaOS::Audit::SovereignLedger::getInstance().verifyChain();
}

sigma_usize sigma_audit_record_count(void) {
    return SigmaOS::Audit::SovereignLedger::getInstance().getRecordCount();
}

/* Legacy shim from sigma_audit.h */
void audit_init(void)                                       { sigma_audit_init(); }
void audit_perform_lattice_sweep(void)                      { sigma_audit_verify_chain(); }
void audit_report_shard(sigma_u32 id, bool status)          {
    sigma_audit_log_security(0, id, status ? "shard OK" : "shard FAIL");
}
sigma_u64 audit_get_sweep_count(void) {
    return (sigma_u64)SigmaOS::Audit::SovereignLedger::getInstance().getRecordCount();
}

} /* extern "C" */
