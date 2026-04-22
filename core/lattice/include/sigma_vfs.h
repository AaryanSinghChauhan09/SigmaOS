#ifndef SIGMA_VFS_H
#define SIGMA_VFS_H

/* =========================================================================
 * SIGMA OS: VIRTUAL FILE SYSTEM LAYER (SYSTEM-LEVEL HEADER)
 * ========================================================================= */

char* sigma_vfs_read_file(const char* path, long* out_size);
const char* sigma_vfs_resolve_path(const char* request_path, char* resolved_buffer, int buffer_size);

#endif
