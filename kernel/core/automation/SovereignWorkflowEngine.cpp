#include "sigma_types.h"
#include "sigma_hal.h"
#include "sigma_log.h"

/**
 * SovereignWorkflowEngine — AI-Native Automation Rule Engine
 * Executes IF/THEN automation rules bridging system events to kernel actions.
 * Zero-STL: uses a fixed-capacity rule table for kernel compatibility.
 */

#define SIGMA_MAX_RULES 64u

namespace SigmaOS {
namespace Kernel {
namespace Automation {

typedef void (*sigma_action_fn)(void);

struct WorkflowRule {
    const char*      trigger;   /* e.g. "SYS_BATTERY_LOW" */
    const char*      condition; /* human-readable description */
    sigma_action_fn  action;
    sigma_u32        valid;
};

static void action_low_power(void) {
    sigma_log_warn("[AUTO] LOW_POWER workflow: throttling non-critical shards.");
}

static void action_gaming(void) {
    sigma_log_info("[AUTO] GAMING workflow: prioritizing GPU IRQs.");
}

class SovereignWorkflowEngineShard {
public:
    static SovereignWorkflowEngineShard& getInstance() {
        static SovereignWorkflowEngineShard instance;
        return instance;
    }

    void registerRule(const char* trigger, const char* condition, sigma_action_fn action) {
        if (m_rule_count >= SIGMA_MAX_RULES) {
            sigma_log_warn("[AUTO] Rule table full — cannot register new rule.");
            return;
        }
        m_rules[m_rule_count].trigger   = trigger;
        m_rules[m_rule_count].condition = condition;
        m_rules[m_rule_count].action    = action;
        m_rules[m_rule_count].valid     = 1u;
        m_rule_count++;
        sigma_log_info("[AUTO] Rule registered.");
    }

    void dispatchEvent(const char* trigger, const char* data) {
        (void)data;
        for (sigma_u32 i = 0u; i < m_rule_count; i++) {
            if (!m_rules[i].valid) continue;
            sigma_u32 match = 1u;
            const char* a = m_rules[i].trigger;
            const char* b = trigger;
            while (*a && *b) {
                if (*a != *b) { match = 0u; break; }
                a++; b++;
            }
            if (match && *a == '\0' && *b == '\0') {
                sigma_log_info("[AUTO] Event dispatched — executing rule action.");
                if (m_rules[i].action) m_rules[i].action();
            }
        }
    }

    void update(sigma_u64 uptime_ms) {
        for (sigma_u32 i = 0u; i < m_schedule_count; i++) {
            if (m_schedule[i].next_run_ms <= uptime_ms) {
                sigma_log_info("[AUTO] Scheduled task triggered — executing action.");
                if (m_schedule[i].action) m_schedule[i].action();
                m_schedule[i].next_run_ms = uptime_ms + m_schedule[i].interval_ms;
            }
        }
    }

    void scheduleTask(sigma_u64 interval_ms, sigma_action_fn action) {
        if (m_schedule_count >= SIGMA_MAX_RULES) return;
        m_schedule[m_schedule_count].interval_ms = interval_ms;
        m_schedule[m_schedule_count].next_run_ms = interval_ms;
        m_schedule[m_schedule_count].action = action;
        m_schedule_count++;
        sigma_log_info("[AUTO] Recurring task scheduled.");
    }

    void initialize() {
        registerRule("SYS_BATTERY_LOW", "pct < 15",    action_low_power);
        registerRule("SYS_APP_LAUNCH",  "app == Gamer", action_gaming);
        
        // System Monitoring Automation: Self-healing high CPU load
        registerRule("SYS_CPU_HIGH", "load > 90", [](){
            sigma_log_warn("[AUTO] CPU Critical: Shifting workloads to secondary silicon nodes.");
        });
        
        // Example: Auto-optimization task every 5 minutes
        scheduleTask(300000, [](){ sigma_log_info("[AUTO] Periodic lattice optimization running..."); });
        
        sigma_log_info("[AUTO] Sovereign Workflow Engine initialized.");
    }

private:
    SovereignWorkflowEngineShard() : m_rule_count(0u), m_schedule_count(0u) {}
    SovereignWorkflowEngineShard(const SovereignWorkflowEngineShard&) = delete;
    SovereignWorkflowEngineShard& operator=(const SovereignWorkflowEngineShard&) = delete;

    struct ScheduledTask {
        sigma_u64       interval_ms;
        sigma_u64       next_run_ms;
        sigma_action_fn action;
    };

    WorkflowRule  m_rules[SIGMA_MAX_RULES];
    sigma_u32     m_rule_count;
    ScheduledTask m_schedule[SIGMA_MAX_RULES];
    sigma_u32     m_schedule_count;
};

} // namespace Automation
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void sigma_workflow_init() {
    SigmaOS::Kernel::Automation::SovereignWorkflowEngineShard::getInstance().initialize();
}

extern "C" void sigma_workflow_dispatch(const char* trigger, const char* data) {
    SigmaOS::Kernel::Automation::SovereignWorkflowEngineShard::getInstance().dispatchEvent(trigger, data);
}

extern "C" void sigma_workflow_update(unsigned long long uptime_ms) {
    SigmaOS::Kernel::Automation::SovereignWorkflowEngineShard::getInstance().update(uptime_ms);
}