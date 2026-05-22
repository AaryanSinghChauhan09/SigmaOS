/*
 * =========================================================================
 * Î£ SIGMAOS: SIGMA KERNEL TUNER (sigma_tuner) v1.0
 * =========================================================================
 * Mission: Live kernel parameter adjustment.
 * Inspiration: sysctl + Clear Linux autotuning.
 * Principle: Zero-reboot live patching of kernel tunables.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

struct KernelTunable {
    char      name[64];
    sigma_u32 value;
    sigma_u32 min_val;
    sigma_u32 max_val;
    sigma_u8  is_readonly;
};

class SigmaKernelTuner : public SigmaObject, public SigmaSingleton<SigmaKernelTuner> {
    friend class SigmaSingleton<SigmaKernelTuner>;
public:
    const char* type_name() const noexcept override { return "SigmaKernelTuner"; }

    void init() {
        m_tunable_count = 0;
        sigma_log_info("[TUNER] Sigma Kernel Tuner v1.0 initialized.");
        
        /* Register defaults */
        register_tunable("vm.swappiness", 60, 0, 100, 0);
        register_tunable("vm.dirty_ratio", 20, 0, 100, 0);
        register_tunable("net.core.somaxconn", 1024, 128, 65535, 0);
        register_tunable("kernel.pid_max", 32768, 1024, 4194304, 0);
        register_tunable("kernel.hz", 1000, 100, 10000, 1); /* Read-only example */
    }

    void register_tunable(const char* name, sigma_u32 val, sigma_u32 min, sigma_u32 max, sigma_u8 ro) {
        if (m_tunable_count >= MAX_TUNABLES) return;
        KernelTunable& t = m_tunables[m_tunable_count++];
        sigma_u32 i = 0;
        while (name[i] && i < 63) { t.name[i] = name[i]; i++; } t.name[i] = '\0';
        t.value = val;
        t.min_val = min;
        t.max_val = max;
        t.is_readonly = ro;
    }

    void set_value(const char* name, sigma_u32 new_val) {
        for (sigma_u32 i = 0; i < m_tunable_count; i++) {
            sigma_u32 j = 0;
            while (m_tunables[i].name[j] == name[j] && name[j]) j++;
            if (!name[j] && !m_tunables[i].name[j]) {
                if (m_tunables[i].is_readonly) {
                    sigma_log_infoor("[TUNER] Cannot set '%s': parameter is read-only.", name);
                    return;
                }
                if (new_val < m_tunables[i].min_val || new_val > m_tunables[i].max_val) {
                    sigma_log_infoor("[TUNER] Value %u out of range [%u, %u] for '%s'.", 
                        new_val, m_tunables[i].min_val, m_tunables[i].max_val, name);
                    return;
                }
                sigma_u32 old_val = m_tunables[i].value;
                m_tunables[i].value = new_val;
                sigma_log_info("[TUNER] Tunable '%s' updated: %u -> %u", name, old_val, new_val);
                return;
            }
        }
        sigma_log_infoor("[TUNER] Tunable '%s' not found.", name);
    }

    void list_tunables() const {
        sigma_log_info("[TUNER] ===== Kernel Tunables =====");
        for (sigma_u32 i = 0; i < m_tunable_count; i++) {
            sigma_log_info("[TUNER] %-24s = %-8u [%s]", 
                m_tunables[i].name, m_tunables[i].value,
                m_tunables[i].is_readonly ? "RO" : "RW");
        }
    }

private:
    static constexpr sigma_u32 MAX_TUNABLES = 128;
    SigmaKernelTuner() : m_tunable_count(0) {}
    KernelTunable m_tunables[MAX_TUNABLES];
    sigma_u32 m_tunable_count;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void tuner_init()                                    { SigmaOS::Tools::SigmaKernelTuner::getInstance().init(); }
void tuner_set(const char* name, sigma_u32 val)      { SigmaOS::Tools::SigmaKernelTuner::getInstance().set_value(name, val); }
void tuner_list()                                    { SigmaOS::Tools::SigmaKernelTuner::getInstance().list_tunables(); }
}

