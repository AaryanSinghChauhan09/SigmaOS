/**
 * =========================================================================
 * Σ SIGMAOS: PAM & ACCESS CONTROL (ZERO-TRUST)
 * =========================================================================
 * Implements Role-Based Access Control (RBAC) matrices and Pluggable
 * Authentication Modules. Integrates with the Virtual File System (VFS)
 * to enforce Zero-Trust resource boundaries.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/sigma_vfs_crypto.h"

namespace SigmaOS {
namespace Security {

/* Access Flags */
#define ACL_READ    0x01
#define ACL_WRITE   0x02
#define ACL_EXEC    0x04
#define ACL_ADMIN   0x08

/* System Roles */
enum class SystemRole : sigma_u32 {
    GUEST       = 0,
    USER        = 1,
    SERVICE     = 2,
    SUPERADMIN  = 3
};

/* Access Control Entry (ACE) */
struct ACE {
    SystemRole role;
    sigma_u32  allowed_mask;
    sigma_u32  denied_mask;
};

/* Access Control List (ACL) */
struct ACL {
    sigma_u32 resource_id;
    sigma_u32 num_entries;
    ACE       entries[8];
};

/* Simulated Active Directory / Shadow equivalent */
struct UserRecord {
    sigma_u32  uid;
    char       username[32];
    char       pwd_hash[64]; /* Simulated Argon2id hash */
    SystemRole role;
};

class AccessController {
public:
    static AccessController& getInstance() {
        static AccessController instance;
        return instance;
    }

    void init() {
        sigma_log("[Security] Initializing Zero-Trust Access Controller...");
        
        /* Initialize default system users */
        m_users[0] = { 0, "root",  "hash_root", SystemRole::SUPERADMIN };
        m_users[1] = { 1, "guest", "hash_gst",  SystemRole::GUEST };
        m_users[2] = { 2, "sys",   "hash_sys",  SystemRole::SERVICE };
        m_user_count = 3;

        /* Initialize a secure resource (e.g. /etc/shadow equivalent) */
        m_acls[0].resource_id = 100; /* Simulated Inode ID for sensitive file */
        m_acls[0].num_entries = 2;
        m_acls[0].entries[0] = { SystemRole::SUPERADMIN, ACL_READ | ACL_WRITE, 0 };
        m_acls[0].entries[1] = { SystemRole::USER,       0, ACL_READ | ACL_WRITE };
        m_acl_count = 1;

        sigma_log_info("[Security] Defined %u user records and %u resource ACLs.", m_user_count, m_acl_count);
    }

    sigma_status authenticateUser(const char* username, const char* password, sigma_u32* out_uid) {
        for (sigma_u32 i = 0; i < m_user_count; i++) {
            if (sigma_strcmp(m_users[i].username, username) == 0) {
                /* In reality: Argon2id verify(password, m_users[i].pwd_hash) */
                sigma_log_info("[Security] User '%s' authenticated successfully.", username);
                if (out_uid) *out_uid = m_users[i].uid;
                return K_OK;
            }
        }
        sigma_log_err("[Security] Authentication failed for user '%s'.", username);
        return K_ERR_INVAL; /* EACCES */
    }

    /* Zero-Trust VFS Hook */
    sigma_status checkAcl(sigma_u32 uid, sigma_u32 resource_id, sigma_u32 requested_access) {
        UserRecord* user = getUser(uid);
        if (!user) {
            sigma_log_err("[Security] ACL check failed: UID %u not found.", uid);
            return K_ERR_INVAL;
        }

        if (user->role == SystemRole::SUPERADMIN) {
            /* Root bypasses standard ACLs in POSIX, but in Zero-Trust we might still log it */
            sigma_log_info("[Security] SuperAdmin access granted to resource %u.", resource_id);
            return K_OK;
        }

        ACL* acl = getAcl(resource_id);
        if (!acl) {
            /* Default secure stance: Deny if no explicit ACL allows it */
            sigma_log_err("[Security] Access Denied: No ACL exists for resource %u.", resource_id);
            return K_ERR_INVAL;
        }

        /* Evaluate ACEs */
        for (sigma_u32 i = 0; i < acl->num_entries; i++) {
            if (acl->entries[i].role == user->role) {
                if (acl->entries[i].denied_mask & requested_access) {
                    sigma_log_err("[Security] Access Denied: Role %u explicitly denied on resource %u.", 
                                  (sigma_u32)user->role, resource_id);
                    return K_ERR_INVAL;
                }
                if ((acl->entries[i].allowed_mask & requested_access) == requested_access) {
                    return K_OK;
                }
            }
        }

        sigma_log_err("[Security] Access Denied: Implicit denial for UID %u on resource %u.", uid, resource_id);
        return K_ERR_INVAL;
    }

private:
    AccessController() : m_user_count(0), m_acl_count(0) {}

    UserRecord* getUser(sigma_u32 uid) {
        for (sigma_u32 i = 0; i < m_user_count; i++) {
            if (m_users[i].uid == uid) return &m_users[i];
        }
        return SIGMA_NULL;
    }

    ACL* getAcl(sigma_u32 resource_id) {
        for (sigma_u32 i = 0; i < m_acl_count; i++) {
            if (m_acls[i].resource_id == resource_id) return &m_acls[i];
        }
        return SIGMA_NULL;
    }

    UserRecord m_users[64];
    sigma_u32  m_user_count;

    ACL        m_acls[256];
    sigma_u32  m_acl_count;
};

} // namespace Security
} // namespace SigmaOS

/* --- C API Wrappers --- */
extern "C" {
    void sigma_security_init(void) {
        SigmaOS::Security::AccessController::getInstance().init();
    }

    sigma_status sigma_auth_user(const char* username, const char* password, sigma_u32* out_uid) {
        return SigmaOS::Security::AccessController::getInstance().authenticateUser(username, password, out_uid);
    }

    sigma_status sigma_acl_check(sigma_u32 user_id, sigma_u32 resource_id, sigma_u32 req_access) {
        return SigmaOS::Security::AccessController::getInstance().checkAcl(user_id, resource_id, req_access);
    }
}
