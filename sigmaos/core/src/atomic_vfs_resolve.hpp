#ifndef ATOMIC_VFS_RESOLVE_HPP
#define ATOMIC_VFS_RESOLVE_HPP

#include "include/sigma_kernel_types.h"

class VfsResolver {
public:
    virtual ~VfsResolver() {}
    virtual sigma_status resolve_path(const char* path, char* resolved_out, sigma_size_t max_len) = 0;
};

class SovereignVfsResolver : public VfsResolver {
public:
    SovereignVfsResolver();
    virtual ~SovereignVfsResolver() {}
    virtual sigma_status resolve_path(const char* path, char* resolved_out, sigma_size_t max_len) override;
};

#endif // ATOMIC_VFS_RESOLVE_HPP
