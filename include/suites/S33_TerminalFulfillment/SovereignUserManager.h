/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN USER & GROUP MANAGER (v1.0 — PURE C11)
 * =========================================================================
 * Mission: Multi-user POSIX UID/GID management, shadow-password hashing.
 * Inspired By: Linux /etc/passwd + /etc/shadow, macOS DirectoryService,
 *              FreeBSD pw(8), OpenBSD useradd(8).
 * Principle: Zero-dependency. bcrypt-free (sovereign PBKDF2-SHA256).
 * =========================================================================
 */

#ifndef SOVEREIGN_USER_MANAGER_H
#define SOVEREIGN_USER_MANAGER_H

#include "suites/S01_Genesis/shards/sigma_types.h"

#define SIGMA_USER_NAME_MAX    32
#define SIGMA_USER_HOME_MAX   128
#define SIGMA_USER_SHELL_MAX   64
#define SIGMA_USER_HASH_MAX    64   /* Hex-encoded SHA-256 */
#define SIGMA_MAX_USERS        64
#define SIGMA_MAX_GROUPS       32
#define SIGMA_MAX_GID_PER_USER 16

/* -------------------------------------------------------------------------
 * User record  (mirrors /etc/passwd + /etc/shadow)
 * ---------------------------------------------------------------------- */
typedef struct {
    char       name    [SIGMA_USER_NAME_MAX];
    sigma_u32  uid;
    sigma_u32  gid;       /* Primary group */
    char       home    [SIGMA_USER_HOME_MAX];
    char       shell   [SIGMA_USER_SHELL_MAX];
    char       pw_hash [SIGMA_USER_HASH_MAX]; /* PBKDF2-SHA256 hex */
    sigma_bool locked;                         /* passwd -l equivalent */
} SigmaUser_t;

/* -------------------------------------------------------------------------
 * Group record (mirrors /etc/group)
 * ---------------------------------------------------------------------- */
typedef struct {
    char       name    [SIGMA_USER_NAME_MAX];
    sigma_u32  gid;
    sigma_u32  members [SIGMA_MAX_GID_PER_USER]; /* UIDs */
    sigma_u32  member_count;
} SigmaGroup_t;

/* -------------------------------------------------------------------------
 * User/Group database
 * ---------------------------------------------------------------------- */
typedef struct {
    SigmaUser_t  users  [SIGMA_MAX_USERS];
    sigma_u32    user_count;
    SigmaGroup_t groups [SIGMA_MAX_GROUPS];
    sigma_u32    group_count;
    sigma_u32    next_uid;   /* Auto-increment UID (useradd parity) */
    sigma_u32    next_gid;
} SigmaUserDB_t;

/* -------------------------------------------------------------------------
 * Public API
 * ---------------------------------------------------------------------- */
void         sigma_userdb_init    (SigmaUserDB_t *db);
sigma_err_t  sigma_user_add       (SigmaUserDB_t *db, const char *name,
                                    const char *password,
                                    const char *home,   const char *shell);
sigma_err_t  sigma_user_del       (SigmaUserDB_t *db, const char *name);
sigma_err_t  sigma_user_passwd    (SigmaUserDB_t *db, const char *name,
                                    const char *new_password);
sigma_err_t  sigma_user_lock      (SigmaUserDB_t *db, const char *name);
sigma_err_t  sigma_user_unlock    (SigmaUserDB_t *db, const char *name);
SigmaUser_t *sigma_user_lookup    (SigmaUserDB_t *db, const char *name);
SigmaUser_t *sigma_user_lookup_uid(SigmaUserDB_t *db, sigma_u32 uid);

sigma_err_t  sigma_group_add      (SigmaUserDB_t *db, const char *name);
sigma_err_t  sigma_group_add_user (SigmaUserDB_t *db, const char *group,
                                    const char *user);
void         sigma_userdb_dump    (const SigmaUserDB_t *db); /* cat /etc/passwd */

sigma_bool   sigma_auth_verify    (const SigmaUserDB_t *db, const char *name,
                                    const char *password); /* PAM equivalent */

/* Global system user database */
extern SigmaUserDB_t g_sigma_userdb;

void SovereignUserManager_Init(void);

#endif /* SOVEREIGN_USER_MANAGER_H */
