/**
 * =========================================================================
 * Σ SIGMAOS: WINDOWS REGISTRY API (sigma-reg)
 * =========================================================================
 * Public API for the sigma-reg compatibility registry.
 * Backend: SQLite at /sigma/data/registry.db
 * Accessed via NT key calls (NtCreateKey/NtQueryValueKey) in sigma-ntdll,
 * or directly via this C API from sigma-wine internals.
 * =========================================================================
 */
#pragma once
#include "sigma_nt_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* -----------------------------------------------------------------------
 * Predefined root key handles (match Windows HKEY values)
 * ----------------------------------------------------------------------- */
#define HKEY_CLASSES_ROOT   ((HKEY)0x80000000)
#define HKEY_CURRENT_USER   ((HKEY)0x80000001)
#define HKEY_LOCAL_MACHINE  ((HKEY)0x80000002)
#define HKEY_USERS          ((HKEY)0x80000003)
#define HKEY_CURRENT_CONFIG ((HKEY)0x80000005)

/* Access rights for registry keys */
#define KEY_QUERY_VALUE  0x0001
#define KEY_SET_VALUE    0x0002
#define KEY_CREATE_SUB_KEY 0x0004
#define KEY_ENUMERATE_SUB_KEYS 0x0008
#define KEY_NOTIFY       0x0010
#define KEY_READ  (KEY_QUERY_VALUE|KEY_ENUMERATE_SUB_KEYS|KEY_NOTIFY)
#define KEY_WRITE (KEY_SET_VALUE|KEY_CREATE_SUB_KEY)
#define KEY_ALL_ACCESS 0xF003F

/* CreateDisposition output */
#define REG_CREATED_NEW_KEY     0x00000001
#define REG_OPENED_EXISTING_KEY 0x00000002

/* -----------------------------------------------------------------------
 * sigma-reg initialization
 * Opens (or creates) /sigma/data/registry.db and seeds standard hives.
 * ----------------------------------------------------------------------- */
NTSTATUS sigma_reg_init(const char* db_path);

/* -----------------------------------------------------------------------
 * Key operations
 * ----------------------------------------------------------------------- */
NTSTATUS sigma_reg_create_key(
    HKEY        hParentKey,
    const char* lpSubKey,
    ULONG       ulOptions,       /* 0 = volatile, REG_OPTION_NON_VOLATILE=0 */
    ACCESS_MASK samDesired,
    HKEY*       phkResult,
    ULONG*      lpdwDisposition  /* REG_CREATED_NEW_KEY or REG_OPENED_EXISTING_KEY */
);

NTSTATUS sigma_reg_open_key(
    HKEY        hKey,
    const char* lpSubKey,
    ACCESS_MASK samDesired,
    HKEY*       phkResult
);

NTSTATUS sigma_reg_close_key(HKEY hKey);
NTSTATUS sigma_reg_delete_key(HKEY hKey, const char* lpSubKey);

/* -----------------------------------------------------------------------
 * Value operations
 * ----------------------------------------------------------------------- */
NTSTATUS sigma_reg_query_value(
    HKEY        hKey,
    const char* lpValueName,   /* NULL = default value */
    ULONG*      lpType,        /* REG_SZ, REG_DWORD, etc. */
    void*       lpData,
    ULONG*      lpcbData       /* in: buffer size; out: bytes written */
);

NTSTATUS sigma_reg_set_value(
    HKEY        hKey,
    const char* lpValueName,
    ULONG       dwType,
    const void* lpData,
    ULONG       cbData
);

NTSTATUS sigma_reg_delete_value(HKEY hKey, const char* lpValueName);

/* -----------------------------------------------------------------------
 * Enumeration
 * ----------------------------------------------------------------------- */
NTSTATUS sigma_reg_enum_key(
    HKEY   hKey,
    ULONG  dwIndex,
    char*  lpName,
    ULONG* lpcchName
);

NTSTATUS sigma_reg_enum_value(
    HKEY   hKey,
    ULONG  dwIndex,
    char*  lpValueName,
    ULONG* lpcchValueName,
    ULONG* lpType,
    void*  lpData,
    ULONG* lpcbData
);

/* -----------------------------------------------------------------------
 * Convenience: seed standard hives with Windows-compatible defaults
 * Called by sigma_reg_init().
 * ----------------------------------------------------------------------- */
NTSTATUS sigma_reg_seed_defaults(void);

/* -----------------------------------------------------------------------
 * sigma-reg → sigma-trustd audit hook
 * Every write is ML-DSA attested when audit mode is enabled.
 * ----------------------------------------------------------------------- */
void sigma_reg_enable_audit(int enabled); /* 1 = on, 0 = off */

#ifdef __cplusplus
} /* extern "C" */
#endif
