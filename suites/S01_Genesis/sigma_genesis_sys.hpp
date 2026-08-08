#ifndef SIGMA_GENESIS_SYS_HPP
#define SIGMA_GENESIS_SYS_HPP

#include "include/sigma_kernel_types.h"

class GenesisBootstrap {
public:
    virtual ~GenesisBootstrap() {}
    virtual sigma_status execute_stage(sigma_u32 stage_id) = 0;
    virtual sigma_u32 get_current_boot_stage() const = 0;
};

class SovereignGenesisBootstrap : public GenesisBootstrap {
public:
    SovereignGenesisBootstrap() : m_current_stage(0) {}
    virtual ~SovereignGenesisBootstrap() {}

    virtual sigma_status execute_stage(sigma_u32 stage_id) override {
        m_current_stage = stage_id;
        // Direct CPU boot synchronization
        __asm__ volatile ("nop");
        return SIGMA_SUCCESS;
    }

    virtual sigma_u32 get_current_boot_stage() const override {
        return m_current_stage;
    }

private:
    sigma_u32 m_current_stage;
};

#endif // SIGMA_GENESIS_SYS_HPP
