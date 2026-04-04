/* 
 Σ SIGMAOS ZENITH: VFS STUB SHARD (v2800.0)
 Mission: Filesystem Abstraction Blueprint.
 Status: Zero-Dependency. Pure Silicon.
*/

#include "sigma_kernel_types.h"

// Σ VFS NODE SHARD
typedef struct {
    char   name[64];
    u32    ino;
    u32    size;
    bool_t is_dir;
} VFSNodeStub;

// Σ STUB INTERFACE (TO BE LINKED WITH VFS.C)
void vfs_init(void);
i32  vfs_open(const char* path, u32 flags, u32 mode);
void vfs_audit(void);
void vfs_sync(void);
