/**
 * ===========================================================================
 * Σ SIGMAOS: SOVEREIGN EVENT BUS & AUTOMATION ENGINE (S-AUTO) v1.0
 * ===========================================================================
 * Mission: Ubuntu Snap hooks / NixOS-style reactive event bus with
 *          YAML-style declarative automation workflows, event-driven
 *          system orchestration, and autonomous policy enforcement.
 *
 * Inspired by: systemd / NixOS / Fedora Anaconda / Ubuntu Snap hooks
 * ZERO-DEPENDENCY: No external orchestration runtime.
 * ===========================================================================
 */

#include "../../../include/sigma_log.h"
#include "../../../include/sigma_kernel_types.h"

/* ---- Internal Constants ---- */
#define EVENT_MAX_HANDLERS    128
#define EVENT_MAX_RULES        64
#define EVENT_MAX_QUEUE       256

namespace SigmaOS {
namespace Kernel {
namespace Automation {

/* =========================================================================
 * EVENT TYPES — System-wide event identifiers
 * ========================================================================= */
enum EventType {
    EVT_BATTERY_LOW       = 0,
    EVT_BATTERY_CRITICAL  = 1,
    EVT_NETWORK_CONNECTED = 2,
    EVT_NETWORK_LOST      = 3,
    EVT_GPU_OVERHEATED    = 4,
    EVT_USER_FOCUS_MODE   = 5,
    EVT_SECURITY_THREAT   = 6,
    EVT_DISK_FULL         = 7,
    EVT_UPDATE_AVAILABLE  = 8,
    EVT_SHARD_CRASHED     = 9,
    EVT_BOOT_COMPLETE     = 10,
    EVT_PROFILE_CHANGED   = 11,
    EVT_CUSTOM            = 99
};

/* =========================================================================
 * EVENT ENTRY — Queued event record
 * ========================================================================= */
struct Event {
    sigma_u32  id;
    EventType  type;
    sigma_u32  timestamp;
    char       source[32];
    char       payload[128];
    bool       handled;
};

static Event     s_event_queue[EVENT_MAX_QUEUE];
static sigma_u32 s_event_head = 0;
static sigma_u32 s_event_tail = 0;
static sigma_u32 s_event_total = 0;

/* =========================================================================
 * AUTOMATION RULE — Declarative when/then policy
 * ========================================================================= */
enum ActionType {
    ACTION_ENABLE_POWERSAVE    = 0,
    ACTION_REDUCE_REFRESH      = 1,
    ACTION_SUSPEND_BG_AGENTS   = 2,
    ACTION_ENABLE_FIREWALL     = 3,
    ACTION_CREATE_SNAPSHOT     = 4,
    ACTION_NOTIFY_USER         = 5,
    ACTION_RESTART_SHARD       = 6,
    ACTION_RUN_COMPLIANCE      = 7,
    ACTION_LOG_EVENT           = 8
};

struct AutomationAction {
    ActionType type;
    char       description[64];
};

struct AutomationRule {
    sigma_u32        id;
    char             name[64];
    EventType        trigger;
    AutomationAction actions[4];
    sigma_u32        action_count;
    bool             enabled;
    sigma_u32        times_fired;
};

static AutomationRule s_rules[EVENT_MAX_RULES];
static sigma_u32      s_rule_count = 0;

/* ---- Helper: register a rule ---- */
static void add_rule(const char* name, EventType trigger,
                      const char* a1_desc, ActionType a1,
                      const char* a2_desc, ActionType a2,
                      const char* a3_desc, ActionType a3) {
    if (s_rule_count >= EVENT_MAX_RULES) return;
    AutomationRule* r = &s_rules[s_rule_count];
    r->id = s_rule_count + 1;
    sigma_strncpy(r->name, name, 64);
    r->trigger = trigger;
    r->enabled = true;
    r->times_fired = 0;
    r->action_count = 0;

    if (a1_desc[0]) {
        sigma_strncpy(r->actions[r->action_count].description, a1_desc, 64);
        r->actions[r->action_count].type = a1;
        r->action_count++;
    }
    if (a2_desc[0]) {
        sigma_strncpy(r->actions[r->action_count].description, a2_desc, 64);
        r->actions[r->action_count].type = a2;
        r->action_count++;
    }
    if (a3_desc[0]) {
        sigma_strncpy(r->actions[r->action_count].description, a3_desc, 64);
        r->actions[r->action_count].type = a3;
        r->action_count++;
    }

    s_rule_count++;
}

/* =========================================================================
 * SovereignEventBus — Core Implementation
 * ========================================================================= */
class SovereignEventBus {
public:
    static SovereignEventBus& getInstance() {
        static SovereignEventBus instance;
        return instance;
    }

    void init() {
        sigma_log("[EVENTBUS]: ═══════════════════════════════════════════════\n");
        sigma_log("[EVENTBUS]: Σ SOVEREIGN EVENT BUS v1.0 — Initializing...\n");
        sigma_log("[EVENTBUS]: ═══════════════════════════════════════════════\n");

        s_rule_count = 0;

        /* Register default automation rules */
        add_rule("Battery Saver", EVT_BATTERY_LOW,
                 "Enable power saver", ACTION_ENABLE_POWERSAVE,
                 "Reduce refresh rate", ACTION_REDUCE_REFRESH,
                 "Suspend background agents", ACTION_SUSPEND_BG_AGENTS);

        add_rule("Security Response", EVT_SECURITY_THREAT,
                 "Enable firewall lockdown", ACTION_ENABLE_FIREWALL,
                 "Create safety snapshot", ACTION_CREATE_SNAPSHOT,
                 "Run compliance audit", ACTION_RUN_COMPLIANCE);

        add_rule("Shard Recovery", EVT_SHARD_CRASHED,
                 "Restart crashed shard", ACTION_RESTART_SHARD,
                 "Log crash event", ACTION_LOG_EVENT,
                 "Notify user", ACTION_NOTIFY_USER);

        add_rule("Disk Alert", EVT_DISK_FULL,
                 "Notify user", ACTION_NOTIFY_USER,
                 "Log event", ACTION_LOG_EVENT,
                 "", ACTION_LOG_EVENT);

        add_rule("Thermal Throttle", EVT_GPU_OVERHEATED,
                 "Enable power saver", ACTION_ENABLE_POWERSAVE,
                 "Notify user", ACTION_NOTIFY_USER,
                 "Log event", ACTION_LOG_EVENT);

        sigma_log("[EVENTBUS]: %d automation rules registered.\n", s_rule_count);
        sigma_log("[EVENTBUS]: Event queue capacity: %d events.\n", EVENT_MAX_QUEUE);
        sigma_log("[EVENTBUS]: Event Bus READY.\n");
    }

    void emit(EventType type, const char* source, const char* payload) {
        sigma_u32 idx = s_event_tail % EVENT_MAX_QUEUE;
        Event* e = &s_event_queue[idx];
        e->id = ++s_event_total;
        e->type = type;
        e->timestamp = (sigma_u32)(cpu_rdtsc() & 0xFFFFFFFF);
        sigma_strncpy(e->source, source, 32);
        sigma_strncpy(e->payload, payload, 128);
        e->handled = false;
        s_event_tail++;

        sigma_log("[EVENTBUS]: Event #%d emitted — Type: %d | Source: %s\n",
                  e->id, (int)type, source);

        /* Process matching rules immediately */
        for (sigma_u32 i = 0; i < s_rule_count; i++) {
            if (s_rules[i].enabled && s_rules[i].trigger == type) {
                fireRule(&s_rules[i]);
            }
        }

        e->handled = true;
    }

    void processQueue() {
        while (s_event_head < s_event_tail) {
            sigma_u32 idx = s_event_head % EVENT_MAX_QUEUE;
            if (!s_event_queue[idx].handled) {
                /* Late processing for unhandled events */
                sigma_log("[EVENTBUS]: Late-processing event #%d.\n", s_event_queue[idx].id);
                s_event_queue[idx].handled = true;
            }
            s_event_head++;
        }
    }

    void reportStatus() {
        sigma_log("\n--- Σ SOVEREIGN EVENT BUS STATUS ---\n");
        sigma_log("| Total Events Emitted : %d\n", s_event_total);
        sigma_log("| Queue Head/Tail      : %d/%d\n", s_event_head, s_event_tail);
        sigma_log("| Automation Rules     : %d\n", s_rule_count);
        sigma_log("|\n");
        sigma_log("| Rule Summary:\n");
        for (sigma_u32 i = 0; i < s_rule_count; i++) {
            sigma_log("|   [%d] %-24s Fired: %d | Actions: %d | %s\n",
                      s_rules[i].id, s_rules[i].name,
                      s_rules[i].times_fired, s_rules[i].action_count,
                      s_rules[i].enabled ? "ENABLED" : "DISABLED");
        }
        sigma_log("-------------------------------------\n");
    }

private:
    SovereignEventBus() = default;

    void fireRule(AutomationRule* rule) {
        rule->times_fired++;
        sigma_log("[EVENTBUS/AUTO]: ⚡ Rule '%s' triggered (fire #%d):\n",
                  rule->name, rule->times_fired);
        for (sigma_u32 i = 0; i < rule->action_count; i++) {
            sigma_log("[EVENTBUS/AUTO]:   → %s\n", rule->actions[i].description);
        }
    }
};

} // namespace Automation
} // namespace Kernel
} // namespace SigmaOS

/* ---- C Wrappers ---- */
extern "C" void eventbus_init() {
    SigmaOS::Kernel::Automation::SovereignEventBus::getInstance().init();
}
extern "C" void eventbus_emit(int type, const char* source, const char* payload) {
    SigmaOS::Kernel::Automation::SovereignEventBus::getInstance().emit(
        (SigmaOS::Kernel::Automation::EventType)type, source, payload);
}
extern "C" void eventbus_process() {
    SigmaOS::Kernel::Automation::SovereignEventBus::getInstance().processQueue();
}
extern "C" void eventbus_status() {
    SigmaOS::Kernel::Automation::SovereignEventBus::getInstance().reportStatus();
}
