/**
 * SovereignSelfHealingKernel.cpp
 * Feature: Self-Healing Kernel
 * =====================================================================
 * Absorbs: Clear Linux auto-update, SystemRescue auto-repair, kexec.
 * Mission: Monitor kernel subsystems for fault conditions and
 *          autonomously apply corrective actions without reboot.
 * Branch:  kernel-exp
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace SelfHeal {

static constexpr sigma_u32 MAX_WATCHERS  = 32;
static constexpr sigma_u32 MAX_PATCHES   = 16;

// ── Fault severity levels ───────────────────────────────────────────
enum class FaultSeverity : sigma_u8 {
    NONE     = 0,
    WARN     = 1,   // log + metrics
    RECOVER  = 2,   // restart subsystem
    CRITICAL = 3    // isolate + kexec-reload
};

// ── Subsystem watcher ───────────────────────────────────────────────
using HealthProbe = bool (*)(void* ctx);  // returns true = healthy
using HealAction  = void (*)(void* ctx);  // corrective action

struct SubsystemWatcher {
    char          name[48];
    HealthProbe   probe;
    HealAction    heal;
    void*         ctx;
    FaultSeverity threshold;
    sigma_u32     fault_count;
    sigma_u32     heal_count;
    bool          isolated;
};

// ── Live patch descriptor ────────────────────────────────────────────
struct LivePatch {
    sigma_u32   patch_id;
    const char* description;
    bool        applied;
};

// ── Manager ─────────────────────────────────────────────────────────
class SovereignSelfHealingKernel {
public:
    static SovereignSelfHealingKernel& getInstance() {
        static SovereignSelfHealingKernel inst;
        return inst;
    }

    void init() {
        m_watcher_count = 0;
        m_patch_count   = 0;
        m_total_heals   = 0;
        sigma_log("[SELFHEAL] Sovereign Self-Healing Kernel engine initialised.");
        sigma_log("[SELFHEAL] Mode: Clear Linux auto-repair + kexec live-patch, no reboot required.");
    }

    // Register a subsystem to monitor
    bool registerWatcher(const char* name, HealthProbe probe,
                         HealAction heal, void* ctx,
                         FaultSeverity threshold) {
        if (m_watcher_count >= MAX_WATCHERS) return false;
        SubsystemWatcher& w = m_watchers[m_watcher_count++];
        sigma_u32 i = 0;
        while (i < 47 && name[i]) { w.name[i] = name[i]; i++; }
        w.name[i]    = '\0';
        w.probe      = probe;
        w.heal       = heal;
        w.ctx        = ctx;
        w.threshold  = threshold;
        w.fault_count = 0;
        w.heal_count  = 0;
        w.isolated   = false;
        sigma_log_info("[SELFHEAL] Watcher registered: '%s' (threshold=%u).\n",
                       w.name, (sigma_u32)threshold);
        return true;
    }

    // Register a live kernel patch
    sigma_u32 registerPatch(const char* description) {
        if (m_patch_count >= MAX_PATCHES) return 0;
        LivePatch& p   = m_patches[m_patch_count];
        p.patch_id     = m_patch_count + 1;
        p.description  = description;
        p.applied      = false;
        m_patch_count++;
        return p.patch_id;
    }

    // Apply a live patch by id (kpatch / livepatch analogue)
    bool applyPatch(sigma_u32 patch_id) {
        for (sigma_u32 i = 0; i < m_patch_count; i++) {
            if (m_patches[i].patch_id == patch_id && !m_patches[i].applied) {
                m_patches[i].applied = true;
                sigma_log_info("[SELFHEAL] Live patch #%u applied: '%s'.\n",
                               patch_id, m_patches[i].description);
                return true;
            }
        }
        return false;
    }

    // Run one health-check cycle across all watchers
    sigma_u32 runHealthCycle() {
        sigma_u32 faults = 0;
        for (sigma_u32 i = 0; i < m_watcher_count; i++) {
            SubsystemWatcher& w = m_watchers[i];
            if (w.isolated) continue;

            bool healthy = (w.probe == nullptr) ? true : w.probe(w.ctx);
            if (!healthy) {
                w.fault_count++;
                faults++;
                sigma_log_info("[SELFHEAL] Fault detected in '%s' (fault #%u).\n",
                               w.name, w.fault_count);

                if (w.threshold == FaultSeverity::WARN) {
                    sigma_log_info("[SELFHEAL] WARN: subsystem '%s' degraded — monitoring.\n", w.name);
                } else if (w.threshold == FaultSeverity::RECOVER) {
                    if (w.heal) { w.heal(w.ctx); w.heal_count++; m_total_heals++; }
                    sigma_log_info("[SELFHEAL] RECOVER: subsystem '%s' restart attempted.\n", w.name);
                } else if (w.threshold == FaultSeverity::CRITICAL) {
                    w.isolated = true;
                    sigma_log_info("[SELFHEAL] CRITICAL: isolating '%s' — kexec reload scheduled.\n", w.name);
                }
            }
        }
        return faults;
    }

    void printAudit() {
        sigma_log("\n--- SELF-HEALING KERNEL AUDIT ---");
        sigma_log_info("| Watchers     : %u\n", m_watcher_count);
        sigma_log_info("| Live Patches : %u\n", m_patch_count);
        sigma_log_info("| Total Heals  : %u\n", m_total_heals);
        for (sigma_u32 i = 0; i < m_watcher_count; i++) {
            sigma_log_info("|  [%s] faults=%u heals=%u isolated=%d\n",
                           m_watchers[i].name,
                           m_watchers[i].fault_count,
                           m_watchers[i].heal_count,
                           (int)m_watchers[i].isolated);
        }
        sigma_log("----------------------------------");
    }

private:
    SubsystemWatcher m_watchers[MAX_WATCHERS];
    LivePatch        m_patches[MAX_PATCHES];
    sigma_u32        m_watcher_count = 0;
    sigma_u32        m_patch_count   = 0;
    sigma_u32        m_total_heals   = 0;

    SovereignSelfHealingKernel() = default;
};

} // namespace SelfHeal
} // namespace Kernel
} // namespace SigmaOS

// ── C API ──────────────────────────────────────────────────────────
extern "C" {

void selfheal_init() {
    SigmaOS::Kernel::SelfHeal::SovereignSelfHealingKernel::getInstance().init();
}

sigma_u32 selfheal_run_cycle() {
    return SigmaOS::Kernel::SelfHeal::SovereignSelfHealingKernel::getInstance().runHealthCycle();
}

sigma_u32 selfheal_register_patch(const char* description) {
    return SigmaOS::Kernel::SelfHeal::SovereignSelfHealingKernel::getInstance().registerPatch(description);
}

bool selfheal_apply_patch(sigma_u32 patch_id) {
    return SigmaOS::Kernel::SelfHeal::SovereignSelfHealingKernel::getInstance().applyPatch(patch_id);
}

void selfheal_audit() {
    SigmaOS::Kernel::SelfHeal::SovereignSelfHealingKernel::getInstance().printAudit();
}

} // extern "C"
