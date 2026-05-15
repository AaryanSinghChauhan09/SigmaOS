/**
 * SigmaOS: Sovereign Shard Streaming Protocol (S-9P)
 * Inspired by Plan 9 from Bell Labs.
 * USP: Everything in the 33-suite lattice is a stream, accessible via a unified protocol.
 */

#ifndef SIGMA_SHARD_STREAM_H
#define SIGMA_SHARD_STREAM_H

#include "../../../../../include/libc/sigma_libc.h"

typedef struct {
    uint32_t msg_type;
    uint32_t tag;
    char* shard_path; // e.g., "/lattice/S04_HAL/keyboard"
} sigma_9p_msg_t;

// Unified Shard Access
void sigma_stream_open(const char* path);
void sigma_stream_read(int fd, void* buf, uint32_t len);
void sigma_stream_write(int fd, const void* buf, uint32_t len);

#endif // SIGMA_SHARD_STREAM_H
