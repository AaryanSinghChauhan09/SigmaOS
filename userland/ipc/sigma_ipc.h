/*
 * =========================================================================
 * Σ SIGMAOS userland/ipc/sigma_ipc.h
 * =========================================================================
 * Modular IPC subsystem — fills gaps vs Linux pipes/sockets, macOS Mach
 * ports, Windows Named Pipes, and Android Binder.
 * =========================================================================
 */

#ifndef SIGMA_IPC_H
#define SIGMA_IPC_H

typedef unsigned int       ipc_u32;
typedef unsigned long long ipc_u64;
typedef unsigned char      ipc_bool;
#define IPC_TRUE  ((ipc_bool)1)
#define IPC_FALSE ((ipc_bool)0)
#define IPC_NULL  ((void*)0)
#define IPC_OK    ((ipc_i32) 0)
#define IPC_ERR   ((ipc_i32)-1)
typedef signed int ipc_i32;

/* ── IPC channel types ──────────────────────────────────────────────────── */
typedef enum {
    IPC_PIPE      = 0,  /* POSIX pipe — anonymous, unidirectional      */
    IPC_SOCKET    = 1,  /* Unix domain socket — bidirectional          */
    IPC_MACH_PORT = 2,  /* Mach port semantics (macOS gap)             */
    IPC_BINDER    = 3,  /* Android Binder transaction model            */
    IPC_SHMEM     = 4,  /* Shared memory ring (fastest)                */
    IPC_NAMED_PIPE= 5   /* Windows Named Pipe parity                   */
} sigma_ipc_type_t;

#define SIGMA_IPC_MAX_CHANNELS 256
#define SIGMA_IPC_BUF_SIZE    4096

/* ── IPC channel descriptor ─────────────────────────────────────────────── */
typedef struct {
    ipc_u32          id;
    sigma_ipc_type_t type;
    char             name[64];
    ipc_u32          owner_pid;
    ipc_u32          peer_pid;
    ipc_bool         is_open;
    ipc_u64          bytes_sent;
    ipc_u64          bytes_recv;
    /* Ring buffer for shared-memory channels */
    unsigned char    buf[SIGMA_IPC_BUF_SIZE];
    ipc_u32          head;
    ipc_u32          tail;
} sigma_ipc_chan_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
ipc_i32  sigma_ipc_create(const char *name, sigma_ipc_type_t type, ipc_u32 owner_pid);
ipc_i32  sigma_ipc_connect(const char *name, ipc_u32 peer_pid);
ipc_i32  sigma_ipc_send(ipc_u32 chan_id, const void *data, ipc_u32 len);
ipc_i32  sigma_ipc_recv(ipc_u32 chan_id, void *buf, ipc_u32 max_len);
void     sigma_ipc_close(ipc_u32 chan_id);
void     sigma_ipc_status(void);

#endif /* SIGMA_IPC_H */
