#ifndef PROCESS_MANAGER_HPP
#define PROCESS_MANAGER_HPP

#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

struct SovereignPCB {
    sigma_u64 pid;
    sigma_u64 cr3;
    sigma_u64 rsp;
    sigma_u32 state; 
};

class IProcess {
public:
    virtual sigma_status spawn(const char* image) = 0;
    virtual void kill() = 0;
    virtual void shard_resources() = 0;
};

class IContainer : public IProcess {
public:
    virtual void isolate_vfs(const char* namespace_root) = 0;
};

class SovereignProcessManager : public SigmaOS::SigmaObject, public IContainer {
private:
    SovereignPCB m_process_table[1024];
    sigma_u32 m_active_count;

public:
    SovereignProcessManager();
    const char* type_name() const noexcept override { return "SovereignProcessManager"; }

    sigma_status spawn(const char* image) override;
    void kill() override;
    void shard_resources() override;
    void isolate_vfs(const char* namespace_root) override;
    void audit();
};

} // namespace Kernel
} // namespace SigmaOS

#endif
