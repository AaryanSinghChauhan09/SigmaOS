/*
 * =========================================================================
 * S SIGMAOS userland/ipc/sigma_ipc.c — IPC Implementation
 * =========================================================================
 */

#include "sigma_ipc.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

static sigma_ipc_chan_t s_channels[SIGMA_IPC_MAX_CHANNELS];
static ipc_u32          s_chan_count = 0;
static ipc_u32          s_next_id   = 1;

static sigma_ipc_chan_t *find_chan_by_id(ipc_u32 id) {
    for (ipc_u32 i = 0; i < s_chan_count; i++)
        if (s_channels[i].id == id && s_channels[i].is_open)
            return &s_channels[i];
    return IPC_NULL;
}

static sigma_ipc_chan_t *find_chan_by_name(const char *name) {
    for (ipc_u32 i = 0; i < s_chan_count; i++)
        if (s_channels[i].is_open && sigma_streq(s_channels[i].name, name))
            return &s_channels[i];
    return IPC_NULL;
}

ipc_i32 sigma_ipc_create(const char *name, sigma_ipc_type_t type, ipc_u32 owner_pid) {
    if (s_chan_count >= SIGMA_IPC_MAX_CHANNELS) return IPC_ERR;
    sigma_ipc_chan_t *ch = &s_channels[s_chan_count++];
    sigma_memset(ch, 0, sizeof(*ch));
    sigma_strncpy(ch->name, name, 63);
    ch->id        = s_next_id++;
    ch->type      = type;
    ch->owner_pid = owner_pid;
    ch->is_open   = IPC_TRUE;
    sigma_printf("S [IPC] CREATE: %s (type=%d, pid=%u, id=%u)\n",
                 name, (int)type, owner_pid, ch->id);
    return (ipc_i32)ch->id;
}

ipc_i32 sigma_ipc_connect(const char *name, ipc_u32 peer_pid) {
    sigma_ipc_chan_t *ch = find_chan_by_name(name);
    if (!ch) { sigma_printf("S [IPC] ERROR: channel '%s' not found\n", name); return IPC_ERR; }
    ch->peer_pid = peer_pid;
    sigma_printf("S [IPC] CONNECT: pid=%u -> channel '%s'\n", peer_pid, name);
    return (ipc_i32)ch->id;
}

/* Ring-buffer send */
ipc_i32 sigma_ipc_send(ipc_u32 chan_id, const void *data, ipc_u32 len) {
    sigma_ipc_chan_t *ch = find_chan_by_id(chan_id);
    if (!ch || !data) return IPC_ERR;
    const unsigned char *src = (const unsigned char*)data;
    ipc_u32 written = 0;
    while (written < len) {
        ipc_u32 next = (ch->head + 1) % SIGMA_IPC_BUF_SIZE;
        if (next == ch->tail) break;  /* full */
        ch->buf[ch->head] = src[written++];
        ch->head = next;
    }
    ch->bytes_sent += written;
    return (ipc_i32)written;
}

/* Ring-buffer recv */
ipc_i32 sigma_ipc_recv(ipc_u32 chan_id, void *buf, ipc_u32 max_len) {
    sigma_ipc_chan_t *ch = find_chan_by_id(chan_id);
    if (!ch || !buf) return IPC_ERR;
    unsigned char *dst = (unsigned char*)buf;
    ipc_u32 read = 0;
    while (read < max_len && ch->tail != ch->head) {
        dst[read++] = ch->buf[ch->tail];
        ch->tail = (ch->tail + 1) % SIGMA_IPC_BUF_SIZE;
    }
    ch->bytes_recv += read;
    return (ipc_i32)read;
}

void sigma_ipc_close(ipc_u32 chan_id) {
    sigma_ipc_chan_t *ch = find_chan_by_id(chan_id);
    if (!ch) return;
    sigma_printf("S [IPC] CLOSE: channel '%s' (sent=%llu recv=%llu)\n",
                 ch->name, (unsigned long long)ch->bytes_sent,
                 (unsigned long long)ch->bytes_recv);
    ch->is_open = IPC_FALSE;
}

void sigma_ipc_status(void) {
    static const char *type_str[] = {
        "PIPE","SOCKET","MACH_PORT","BINDER","SHMEM","NAMED_PIPE"
    };
    sigma_printf("\nS IPC CHANNEL TABLE\n");
    sigma_printf("%-6s %-20s %-12s %-8s %-8s\n",
                 "ID", "NAME", "TYPE", "OWNER", "PEER");
    for (ipc_u32 i = 0; i < s_chan_count; i++) {
        if (!s_channels[i].is_open) continue;
        sigma_printf("  %-4u %-20s %-12s %-8u %-8u\n",
                     s_channels[i].id,
                     s_channels[i].name,
                     type_str[s_channels[i].type],
                     s_channels[i].owner_pid,
                     s_channels[i].peer_pid);
    }
}
