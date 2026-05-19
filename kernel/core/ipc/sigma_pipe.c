/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: UNIX-STYLE PIPE MECHANISM
 * =============================================================================
 * Inspired by: Linux kernel fs/pipe.c
 *              FreeBSD sys/kern/sys_pipe.c
 *              Plan 9 pipes and channels
 * =============================================================================
 * Unidirectional byte-stream IPC between producer/consumer processes.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define SIGMA_PIPE_BUF_SIZE  4096
#define SIGMA_PIPE_MAX       32

typedef struct {
    sigma_u8  buffer[SIGMA_PIPE_BUF_SIZE];
    sigma_u32 read_pos;
    sigma_u32 write_pos;
    sigma_u32 count;
    sigma_u32 reader_pid;
    sigma_u32 writer_pid;
    sigma_bool active;
    sigma_bool reader_closed;
    sigma_bool writer_closed;
} sigma_pipe_t;

static sigma_pipe_t pipe_table[SIGMA_PIPE_MAX];
static sigma_u32 pipe_next_fd = 3; /* 0=stdin, 1=stdout, 2=stderr */

int sigma_pipe_create(sigma_u32 creator_pid, int fds[2]) {
    for (sigma_u32 i = 0; i < SIGMA_PIPE_MAX; i++) {
        if (!pipe_table[i].active) {
            sigma_memset(&pipe_table[i], 0, sizeof(sigma_pipe_t));
            pipe_table[i].active     = SIGMA_TRUE;
            pipe_table[i].reader_pid = creator_pid;
            pipe_table[i].writer_pid = creator_pid;
            fds[0] = (int)pipe_next_fd++;  /* read end  */
            fds[1] = (int)pipe_next_fd++;  /* write end */
            sigma_printf("[pipe] Created pipe (read_fd=%d, write_fd=%d) for PID %u\n",
                         fds[0], fds[1], creator_pid);
            return 0;
        }
    }
    sigma_printf("[pipe] ERR: No free pipe slots\n");
    return -1;
}

int sigma_pipe_write(sigma_u32 pipe_idx, const void* data, sigma_u32 len) {
    if (pipe_idx >= SIGMA_PIPE_MAX || !pipe_table[pipe_idx].active) return -1;
    sigma_pipe_t* p = &pipe_table[pipe_idx];

    if (p->reader_closed) {
        sigma_printf("[pipe] ERR: Broken pipe (SIGPIPE) — reader closed\n");
        return -1;
    }

    sigma_u32 space = SIGMA_PIPE_BUF_SIZE - p->count;
    if (len > space) len = space;
    if (len == 0) return 0;

    const sigma_u8* src = (const sigma_u8*)data;
    for (sigma_u32 i = 0; i < len; i++) {
        p->buffer[p->write_pos] = src[i];
        p->write_pos = (p->write_pos + 1) % SIGMA_PIPE_BUF_SIZE;
    }
    p->count += len;
    sigma_printf("[pipe] Wrote %u bytes (buffered: %u/%u)\n",
                 len, p->count, SIGMA_PIPE_BUF_SIZE);
    return (int)len;
}

int sigma_pipe_read(sigma_u32 pipe_idx, void* buf, sigma_u32 len) {
    if (pipe_idx >= SIGMA_PIPE_MAX || !pipe_table[pipe_idx].active) return -1;
    sigma_pipe_t* p = &pipe_table[pipe_idx];

    if (p->count == 0) {
        if (p->writer_closed) return 0; /* EOF */
        return -1; /* Would block */
    }

    if (len > p->count) len = p->count;
    sigma_u8* dst = (sigma_u8*)buf;
    for (sigma_u32 i = 0; i < len; i++) {
        dst[i] = p->buffer[p->read_pos];
        p->read_pos = (p->read_pos + 1) % SIGMA_PIPE_BUF_SIZE;
    }
    p->count -= len;
    sigma_printf("[pipe] Read %u bytes (remaining: %u)\n", len, p->count);
    return (int)len;
}

void sigma_pipe_close_read(sigma_u32 pipe_idx) {
    if (pipe_idx < SIGMA_PIPE_MAX && pipe_table[pipe_idx].active) {
        pipe_table[pipe_idx].reader_closed = SIGMA_TRUE;
        sigma_printf("[pipe] Read end closed for pipe %u\n", pipe_idx);
        if (pipe_table[pipe_idx].writer_closed) {
            pipe_table[pipe_idx].active = SIGMA_FALSE;
            sigma_printf("[pipe] Pipe %u fully closed and released\n", pipe_idx);
        }
    }
}

void sigma_pipe_close_write(sigma_u32 pipe_idx) {
    if (pipe_idx < SIGMA_PIPE_MAX && pipe_table[pipe_idx].active) {
        pipe_table[pipe_idx].writer_closed = SIGMA_TRUE;
        sigma_printf("[pipe] Write end closed for pipe %u\n", pipe_idx);
        if (pipe_table[pipe_idx].reader_closed) {
            pipe_table[pipe_idx].active = SIGMA_FALSE;
            sigma_printf("[pipe] Pipe %u fully closed and released\n", pipe_idx);
        }
    }
}
