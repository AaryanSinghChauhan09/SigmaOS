#ifndef ATOMIC_IPC_DELIVER_HPP
#define ATOMIC_IPC_DELIVER_HPP

#include "include/sigma_kernel_types.h"

class IpcDispatcher {
public:
    virtual ~IpcDispatcher() {}
    virtual sigma_status deliver_message(sigma_u32 dest_shard, const sigma_u8* payload, sigma_size_t size) = 0;
};

class SovereignIpcDispatcher : public IpcDispatcher {
public:
    SovereignIpcDispatcher();
    virtual ~SovereignIpcDispatcher() {}
    virtual sigma_status deliver_message(sigma_u32 dest_shard, const sigma_u8* payload, sigma_size_t size) override;
};

#endif // ATOMIC_IPC_DELIVER_HPP
