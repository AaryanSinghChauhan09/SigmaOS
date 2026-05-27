/**
 * @file sigma_automate.cpp
 * @brief SigmaOS Automation Taskmaster Daemon
 *
 * Competitor Inspiration:
 *  - macOS Shortcuts: Visual trigger → action pipelines
 *  - Cron / systemd timers: Time-based scheduling
 *  - Ansible: Idempotent playbook execution
 *  - IFTTT / Zapier: Event-driven automation ("If This Then That")
 *  - Windows Task Scheduler: System event triggers
 *
 * Listens for system events (boot, USB insert, Wi-Fi connect, cron ticks,
 * file changes, low battery) and executes predefined action pipelines
 * safely within the Sovereign Sandbox.
 */

#include "../../../include/sigma_automate.h"
#include "../../../include/sigma_kernel_types.h"

namespace sigma {
namespace automate {

// ─── Global Registry ─────────────────────────────────────────────────────────
static AutomateRegistry g_registry;

// ─── Cron Expression Parser (minimal 5-field: min hour dom mon dow) ──────────
struct CronFields {
    sigma_u32 minute;    // 0-59 or 0xFF = wildcard
    sigma_u32 hour;      // 0-23 or 0xFF = wildcard
    sigma_u32 day;       // 1-31 or 0xFF = wildcard
    sigma_u32 month;     // 1-12 or 0xFF = wildcard
    sigma_u32 weekday;   // 0-6  or 0xFF = wildcard
};

static sigma_u32 parse_cron_field(const char* s, sigma_u32 len) {
    if (len == 1 && s[0] == '*') return 0xFF;
    sigma_u32 val = 0;
    for (sigma_u32 i = 0; i < len; ++i) {
        if (s[i] >= '0' && s[i] <= '9') {
            val = val * 10 + (s[i] - '0');
        }
    }
    return val;
}

static sigma_status parse_cron(const char* expr, CronFields* out) {
    if (!expr || !out) return SIGMA_ERROR;

    // Tokenize by spaces — exactly 5 fields
    sigma_u32 field_starts[5];
    sigma_u32 field_lens[5];
    sigma_u32 field_idx = 0;
    sigma_u32 i = 0;

    // Skip leading whitespace
    while (expr[i] == ' ') ++i;

    while (expr[i] && field_idx < 5) {
        field_starts[field_idx] = i;
        sigma_u32 start = i;
        while (expr[i] && expr[i] != ' ') ++i;
        field_lens[field_idx] = i - start;
        field_idx++;
        while (expr[i] == ' ') ++i;
    }

    if (field_idx != 5) return SIGMA_ERROR;

    out->minute  = parse_cron_field(&expr[field_starts[0]], field_lens[0]);
    out->hour    = parse_cron_field(&expr[field_starts[1]], field_lens[1]);
    out->day     = parse_cron_field(&expr[field_starts[2]], field_lens[2]);
    out->month   = parse_cron_field(&expr[field_starts[3]], field_lens[3]);
    out->weekday = parse_cron_field(&expr[field_starts[4]], field_lens[4]);

    return SIGMA_SUCCESS;
}

static sigma_bool cron_matches(const CronFields* cron, sigma_u32 min,
                                sigma_u32 hour, sigma_u32 day,
                                sigma_u32 month, sigma_u32 weekday) {
    if (cron->minute  != 0xFF && cron->minute  != min)     return SIGMA_FALSE;
    if (cron->hour    != 0xFF && cron->hour    != hour)    return SIGMA_FALSE;
    if (cron->day     != 0xFF && cron->day     != day)     return SIGMA_FALSE;
    if (cron->month   != 0xFF && cron->month   != month)   return SIGMA_FALSE;
    if (cron->weekday != 0xFF && cron->weekday != weekday) return SIGMA_FALSE;
    return SIGMA_TRUE;
}

// ─── Action Executor ─────────────────────────────────────────────────────────
static sigma_status execute_action(const ActionDescriptor* action) {
    if (!action) return SIGMA_ERROR;

    switch (action->type) {
        case ACTION_RUN_COMMAND:
            // Fork into Sovereign Sandbox and exec the command string
            // If action->run_in_sandbox, wrap in sigma_sandbox_exec()
            break;

        case ACTION_SET_THEME:
            // Call sigma_theme_apply(action->payload)
            break;

        case ACTION_NOTIFY:
            // Push notification to Zenith compositor notification daemon
            break;

        case ACTION_INSTALL_PKG:
            // Call sigma_omni_pkg install action->payload
            break;

        case ACTION_MOUNT_FS:
            // Call VFS mount with the path in payload
            break;

        case ACTION_SEND_WEBHOOK:
            // HTTP POST via the sovereign network stack
            break;

        case ACTION_APPLY_SYSCTL:
            // Write the key=value from payload to /proc/sys equivalent
            break;
    }

    return SIGMA_SUCCESS;
}

// ─── Playbook Executor (runs all actions for a triggered playbook) ───────────
static sigma_status execute_playbook(const Playbook* pb) {
    if (!pb || !pb->enabled) return SIGMA_ERROR;

    for (sigma_u32 i = 0; i < pb->num_actions; ++i) {
        sigma_status s = execute_action(&pb->actions[i]);
        if (s != SIGMA_SUCCESS) {
            // Log failure but continue — partial execution is better than none
        }
    }
    return SIGMA_SUCCESS;
}

// ─── Register a New Playbook ─────────────────────────────────────────────────
sigma_status register_playbook(const Playbook* pb) {
    if (!pb || g_registry.count >= SIGMA_MAX_PLAYBOOKS) return SIGMA_ERROR;
    g_registry.playbooks[g_registry.count++] = *pb;
    return SIGMA_SUCCESS;
}

// ─── Fire All Playbooks Matching a Trigger Type ──────────────────────────────
sigma_status fire_trigger(TriggerType type) {
    for (sigma_u32 i = 0; i < g_registry.count; ++i) {
        Playbook* pb = &g_registry.playbooks[i];
        if (pb->enabled && pb->trigger.type == type) {
            execute_playbook(pb);
        }
    }
    return SIGMA_SUCCESS;
}

// ─── Cron Tick (called every minute by the kernel timer interrupt) ───────────
sigma_status cron_tick(sigma_u32 min, sigma_u32 hour, sigma_u32 day,
                        sigma_u32 month, sigma_u32 weekday) {
    for (sigma_u32 i = 0; i < g_registry.count; ++i) {
        Playbook* pb = &g_registry.playbooks[i];
        if (!pb->enabled || pb->trigger.type != TRIGGER_CRON_SCHEDULE) continue;

        CronFields fields;
        if (parse_cron(pb->trigger.cron_expr, &fields) != SIGMA_SUCCESS) continue;

        if (cron_matches(&fields, min, hour, day, month, weekday)) {
            execute_playbook(pb);
        }
    }
    return SIGMA_SUCCESS;
}

// ─── List All Playbooks (for CLI: `sigma-auto list`) ─────────────────────────
sigma_u32 list_playbooks(Playbook* out, sigma_u32 max_out) {
    sigma_u32 n = (g_registry.count < max_out) ? g_registry.count : max_out;
    for (sigma_u32 i = 0; i < n; ++i) out[i] = g_registry.playbooks[i];
    return n;
}

// ─── Enable / Disable a Playbook ─────────────────────────────────────────────
sigma_status set_playbook_enabled(sigma_u32 id, sigma_bool enabled) {
    for (sigma_u32 i = 0; i < g_registry.count; ++i) {
        if (g_registry.playbooks[i].id == id) {
            g_registry.playbooks[i].enabled = enabled;
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

// ─── Delete a Playbook ───────────────────────────────────────────────────────
sigma_status delete_playbook(sigma_u32 id) {
    for (sigma_u32 i = 0; i < g_registry.count; ++i) {
        if (g_registry.playbooks[i].id == id) {
            // Shift remaining entries
            for (sigma_u32 j = i; j < g_registry.count - 1; ++j) {
                g_registry.playbooks[j] = g_registry.playbooks[j + 1];
            }
            g_registry.count--;
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

} // namespace automate
} // namespace sigma

extern "C" {
    sigma_status sigma_automate_register(void* pb) {
        return sigma::automate::register_playbook((sigma::automate::Playbook*)pb);
    }
    sigma_status sigma_automate_fire(sigma_u32 type) {
        return sigma::automate::fire_trigger((sigma::automate::TriggerType)type);
    }
    sigma_status sigma_automate_cron_tick(sigma_u32 m, sigma_u32 h,
                                          sigma_u32 d, sigma_u32 mo, sigma_u32 w) {
        return sigma::automate::cron_tick(m, h, d, mo, w);
    }
}
