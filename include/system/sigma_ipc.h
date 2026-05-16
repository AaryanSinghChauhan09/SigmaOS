#include "../sigma_types.h"
#ifndef SIGMA_IPC_H
#define SIGMA_IPC_H

#include "../sigma_kernel_types.h"
#include "../SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace IPC {

class SovereignSharedMemory : public SigmaObject {
public:
    static SovereignSharedMemory& getInstance() {
        static SovereignSharedMemory instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignSharedMemory"; }

    void init();
    void* createSegment(const char* segment_id, sigma_usize size);
    void audit();

private:
    SovereignSharedMemory() {}
};

class SovereignMessageBus : public SigmaObject {
public:
    static SovereignMessageBus& getInstance() {
        static SovereignMessageBus instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignMessageBus"; }

    void init();
    void sendMessage(sigma_u32 target_id, const void* data, sigma_usize size);
    void receiveMessage(void* buffer, sigma_usize max_size);

private:
    SovereignMessageBus() {}
};

} // namespace IPC
} // namespace Kernel
} // namespace SigmaOS

#ifdef __cplusplus
extern "C" {
#endif

void  shmem_init(void);
void* shmem_create(const char* id, sigma_usize sz);
void  shmem_audit(void);

void  ipc_bus_init(void);
void  ipc_bus_send(sigma_u32 target, const void* data, sigma_usize sz);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_IPC_H
