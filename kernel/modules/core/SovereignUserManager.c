/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN USER & GROUP MANAGER — IMPLEMENTATION (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignUserManager.h"

/* Global system user database */
SigmaUserDB_t g_sigma_userdb;

/* -------------------------------------------------------------------------
 * Minimal sovereign PBKDF2-SHA256 stub
 * (In a full kernel: link against SovereignLatticePQC for real hashing)
 * ---------------------------------------------------------------------- */
static void sovereign_hash_password(const char *password, char *out_hex,
                                    sigma_size_t out_max) {
    /* XOR-fold + rotate — placeholder; replace with real SHA-256 kernel shard */
    sigma_u32 h = 0xDEADBEEF;
    while (*password) {
        h ^= (sigma_u8)*password++;
        h  = (h << 13) | (h >> 19);
        h *= 0x45D9F3B;
    }
    /* Emit 8-char hex result (expand to full 64-char in production) */
    sigma_snprintf(out_hex, out_max, "%08x%08x%08x%08x"
                                     "%08x%08x%08x%08x",
                   h, h ^ 0xCAFEBABE, h * 31, h + 1,
                   ~h, h >> 16, h ^ 0xDEAD, h * 0xBEEF);
}

/* -------------------------------------------------------------------------
 * sigma_userdb_init
 * ---------------------------------------------------------------------- */
void sigma_userdb_init(SigmaUserDB_t *db) {
    sigma_memset(db, 0, sizeof(*db));
    db->next_uid = 1000;   /* System accounts < 1000, human ≥ 1000 */
    db->next_gid = 1000;
}

/* -------------------------------------------------------------------------
 * sigma_user_add — useradd equivalent
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_user_add(SigmaUserDB_t *db, const char *name,
                            const char *password,
                            const char *home, const char *shell) {
    if (db->user_count >= SIGMA_MAX_USERS) return SIGMA_ENOSPC;
    if (sigma_user_lookup(db, name))        return SIGMA_EBUSY;

    SigmaUser_t *u = &db->users[db->user_count++];
    sigma_strcpy(u->name,  name,  SIGMA_USER_NAME_MAX);
    sigma_strcpy(u->home,  home,  SIGMA_USER_HOME_MAX);
    sigma_strcpy(u->shell, shell, SIGMA_USER_SHELL_MAX);
    u->uid    = db->next_uid++;
    u->gid    = db->next_gid++;   /* Create a matching primary group */
    u->locked = SIGMA_FALSE;

    sovereign_hash_password(password, u->pw_hash, SIGMA_USER_HASH_MAX);

    /* Also register the primary group */
    sigma_group_add(db, name);   /* group named after user (useradd default) */

    sigma_printf("Σ [USER]: User added: %s (uid=%u gid=%u home=%s shell=%s)\n",
                 u->name, u->uid, u->gid, u->home, u->shell);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_user_del — userdel equivalent
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_user_del(SigmaUserDB_t *db, const char *name) {
    for (sigma_u32 i = 0; i < db->user_count; i++) {
        if (sigma_streq(db->users[i].name, name)) {
            /* Shift array left */
            sigma_u32 tail = db->user_count - 1 - i;
            sigma_memmove(&db->users[i], &db->users[i + 1],
                          tail * sizeof(SigmaUser_t));
            db->user_count--;
            sigma_printf("Σ [USER]: User deleted: %s\n", name);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

/* -------------------------------------------------------------------------
 * sigma_user_passwd — passwd equivalent
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_user_passwd(SigmaUserDB_t *db, const char *name,
                               const char *new_password) {
    SigmaUser_t *u = sigma_user_lookup(db, name);
    if (!u) return SIGMA_ENOENT;
    sovereign_hash_password(new_password, u->pw_hash, SIGMA_USER_HASH_MAX);
    sigma_printf("Σ [USER]: Password updated for: %s\n", name);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_user_lock / sigma_user_unlock
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_user_lock(SigmaUserDB_t *db, const char *name) {
    SigmaUser_t *u = sigma_user_lookup(db, name);
    if (!u) return SIGMA_ENOENT;
    u->locked = SIGMA_TRUE;
    sigma_printf("Σ [USER]: Account locked: %s\n", name);
    return SIGMA_OK;
}

sigma_err_t sigma_user_unlock(SigmaUserDB_t *db, const char *name) {
    SigmaUser_t *u = sigma_user_lookup(db, name);
    if (!u) return SIGMA_ENOENT;
    u->locked = SIGMA_FALSE;
    sigma_printf("Σ [USER]: Account unlocked: %s\n", name);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_user_lookup — getpwnam equivalent
 * ---------------------------------------------------------------------- */
SigmaUser_t *sigma_user_lookup(SigmaUserDB_t *db, const char *name) {
    for (sigma_u32 i = 0; i < db->user_count; i++) {
        if (sigma_streq(db->users[i].name, name))
            return &db->users[i];
    }
    return SIGMA_NULL;
}

SigmaUser_t *sigma_user_lookup_uid(SigmaUserDB_t *db, sigma_u32 uid) {
    for (sigma_u32 i = 0; i < db->user_count; i++) {
        if (db->users[i].uid == uid)
            return &db->users[i];
    }
    return SIGMA_NULL;
}

/* -------------------------------------------------------------------------
 * sigma_group_add — groupadd equivalent
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_group_add(SigmaUserDB_t *db, const char *name) {
    if (db->group_count >= SIGMA_MAX_GROUPS) return SIGMA_ENOSPC;
    SigmaGroup_t *g = &db->groups[db->group_count++];
    sigma_strcpy(g->name, name, SIGMA_USER_NAME_MAX);
    g->gid = db->next_gid++;
    g->member_count = 0;
    sigma_printf("Σ [USER]: Group added: %s (gid=%u)\n", name, g->gid);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_group_add_user — usermod -aG equivalent
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_group_add_user(SigmaUserDB_t *db,
                                  const char *group, const char *user) {
    SigmaUser_t *u = sigma_user_lookup(db, user);
    if (!u) return SIGMA_ENOENT;

    for (sigma_u32 i = 0; i < db->group_count; i++) {
        SigmaGroup_t *g = &db->groups[i];
        if (sigma_streq(g->name, group)) {
            if (g->member_count >= SIGMA_MAX_GID_PER_USER) return SIGMA_ENOSPC;
            g->members[g->member_count++] = u->uid;
            sigma_printf("Σ [USER]: User %s added to group %s.\n", user, group);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

/* -------------------------------------------------------------------------
 * sigma_userdb_dump — cat /etc/passwd style
 * ---------------------------------------------------------------------- */
void sigma_userdb_dump(const SigmaUserDB_t *db) {
    sigma_printf("Σ [USER]: /etc/sigma-passwd (%u users):\n", db->user_count);
    for (sigma_u32 i = 0; i < db->user_count; i++) {
        const SigmaUser_t *u = &db->users[i];
        sigma_printf("  %-16s  uid=%-5u  gid=%-5u  home=%-20s  shell=%s  %s\n",
                     u->name, u->uid, u->gid, u->home, u->shell,
                     u->locked ? "[LOCKED]" : "");
    }
}

/* -------------------------------------------------------------------------
 * sigma_auth_verify — PAM equivalent password check
 * ---------------------------------------------------------------------- */
sigma_bool sigma_auth_verify(const SigmaUserDB_t *db,
                              const char *name, const char *password) {
    const SigmaUser_t *u = SIGMA_NULL;
    for (sigma_u32 i = 0; i < db->user_count; i++) {
        if (sigma_streq(db->users[i].name, name)) {
            u = &db->users[i];
            break;
        }
    }
    if (!u || u->locked) return SIGMA_FALSE;

    char hash[SIGMA_USER_HASH_MAX];
    sovereign_hash_password(password, hash, SIGMA_USER_HASH_MAX);
    return (sigma_bool)(sigma_memcmp(u->pw_hash, hash, SIGMA_USER_HASH_MAX) == 0);
}

/* -------------------------------------------------------------------------
 * SovereignUserManager_Init — Bootstrap system users
 * ---------------------------------------------------------------------- */
void SovereignUserManager_Init(void) {
    sigma_printf("Σ [USER]: Initialising Sovereign User Manager...\n");
    sigma_userdb_init(&g_sigma_userdb);

    /* Seed system accounts (UID < 1000 — override auto-increment for root) */
    g_sigma_userdb.next_uid = 0;
    sigma_user_add(&g_sigma_userdb, "root",
                   "sovereign_root_pass", "/root", "/bin/sigma-sh");
    g_sigma_userdb.next_uid = 1;
    sigma_user_add(&g_sigma_userdb, "nobody",
                   "!!", "/", "/sbin/nologin");

    /* Reset to human-user range */
    g_sigma_userdb.next_uid = 1000;
    sigma_user_add(&g_sigma_userdb, "aaryan",
                   "zenith_pass", "/home/aaryan", "/bin/sigma-sh");

    /* Group membership */
    sigma_group_add(&g_sigma_userdb, "wheel");
    sigma_group_add(&g_sigma_userdb, "sudo");
    sigma_group_add_user(&g_sigma_userdb, "wheel", "aaryan");
    sigma_group_add_user(&g_sigma_userdb, "sudo",  "aaryan");

    sigma_userdb_dump(&g_sigma_userdb);

    /* Auth test */
    sigma_bool ok = sigma_auth_verify(&g_sigma_userdb, "aaryan", "zenith_pass");
    sigma_printf("Σ [USER]: Auth test (aaryan): %s\n", ok ? "PASS" : "FAIL");

    sigma_printf("Σ [USER]: User Manager online. Multi-user sovereignty achieved.\n");
}
