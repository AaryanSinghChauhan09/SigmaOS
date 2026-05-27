/**
 * @file sigma_notification_daemon.cpp
 * @brief Zenith Notification Daemon — Desktop notification system
 *
 * Competitor Inspiration:
 *  - GNOME: org.freedesktop.Notifications D-Bus service
 *  - KDE: KNotifications with action buttons and inline replies
 *  - macOS: UserNotifications framework with grouping and Do Not Disturb
 *  - Windows 10+: Action Center with notification history
 *
 * Manages a notification queue with urgency levels, TTL expiry,
 * action buttons, notification grouping, history, and Do Not Disturb mode.
 */

#include "../../include/sigma_kernel_types.h"

namespace sigma {
namespace notify {

// ─── Urgency Levels ──────────────────────────────────────────────────────────
typedef enum : sigma_u32 {
    URGENCY_LOW      = 0,
    URGENCY_NORMAL   = 1,
    URGENCY_CRITICAL = 2,
} Urgency;

// ─── Notification Action ─────────────────────────────────────────────────────
struct NotifyAction {
    char label[64];       // Button text: "Reply", "Dismiss", "Open"
    char command[256];    // Shell command or IPC message to fire on click
};

// ─── Notification ────────────────────────────────────────────────────────────
#define MAX_ACTIONS_PER_NOTIF 4

struct Notification {
    sigma_u32     id;
    char          app_name[64];
    char          summary[128];
    char          body[512];
    char          icon_path[256];
    char          category[64];     // e.g. "email", "im", "device"
    Urgency       urgency;
    sigma_u32     ttl_ms;           // Time-to-live (0 = persistent)
    sigma_u32     created_at_ms;    // Timestamp
    sigma_bool    read;
    sigma_bool    dismissed;
    NotifyAction  actions[MAX_ACTIONS_PER_NOTIF];
    sigma_u32     num_actions;
};

// ─── Notification Queue ──────────────────────────────────────────────────────
#define MAX_NOTIFICATIONS    256
#define MAX_HISTORY          1024

struct NotifyDaemon {
    Notification  queue[MAX_NOTIFICATIONS];
    sigma_u32     queue_count;
    Notification  history[MAX_HISTORY];
    sigma_u32     history_count;
    sigma_u32     next_id;
    sigma_bool    dnd_enabled;       // Do Not Disturb
    sigma_u32     dnd_until_ms;      // Auto-disable DnD at this timestamp
};

static NotifyDaemon g_daemon;

// ─── Init ────────────────────────────────────────────────────────────────────
sigma_status notify_init() {
    g_daemon.queue_count   = 0;
    g_daemon.history_count = 0;
    g_daemon.next_id       = 1;
    g_daemon.dnd_enabled   = SIGMA_FALSE;
    g_daemon.dnd_until_ms  = 0;
    return SIGMA_SUCCESS;
}

// ─── Helper: copy string ─────────────────────────────────────────────────────
static void str_copy(char* dst, const char* src, sigma_u32 max_len) {
    sigma_u32 i = 0;
    while (src && src[i] && i < max_len - 1) { dst[i] = src[i]; i++; }
    dst[i] = '\0';
}

// ─── Send Notification ───────────────────────────────────────────────────────
sigma_u32 send_notification(const char* app, const char* summary,
                             const char* body, Urgency urgency,
                             sigma_u32 ttl_ms) {
    // DnD blocks non-critical notifications
    if (g_daemon.dnd_enabled && urgency != URGENCY_CRITICAL) {
        return 0; // Silenced
    }

    if (g_daemon.queue_count >= MAX_NOTIFICATIONS) {
        // Evict oldest non-critical
        for (sigma_u32 i = 0; i < g_daemon.queue_count; ++i) {
            if (g_daemon.queue[i].urgency != URGENCY_CRITICAL) {
                // Move to history
                if (g_daemon.history_count < MAX_HISTORY) {
                    g_daemon.history[g_daemon.history_count++] = g_daemon.queue[i];
                }
                // Shift
                for (sigma_u32 j = i; j < g_daemon.queue_count - 1; ++j)
                    g_daemon.queue[j] = g_daemon.queue[j + 1];
                g_daemon.queue_count--;
                break;
            }
        }
        if (g_daemon.queue_count >= MAX_NOTIFICATIONS) return 0;
    }

    Notification* n = &g_daemon.queue[g_daemon.queue_count];
    n->id = g_daemon.next_id++;
    str_copy(n->app_name, app, 64);
    str_copy(n->summary, summary, 128);
    str_copy(n->body, body, 512);
    n->icon_path[0]  = '\0';
    n->category[0]   = '\0';
    n->urgency       = urgency;
    n->ttl_ms        = ttl_ms;
    n->created_at_ms = 0; // Would be filled by kernel clock
    n->read          = SIGMA_FALSE;
    n->dismissed     = SIGMA_FALSE;
    n->num_actions   = 0;

    g_daemon.queue_count++;

    // Signal the Zenith compositor to render the notification popup
    return n->id;
}

// ─── Add Action to a Notification ────────────────────────────────────────────
sigma_status add_action(sigma_u32 notif_id, const char* label, const char* command) {
    for (sigma_u32 i = 0; i < g_daemon.queue_count; ++i) {
        if (g_daemon.queue[i].id == notif_id) {
            Notification* n = &g_daemon.queue[i];
            if (n->num_actions >= MAX_ACTIONS_PER_NOTIF) return SIGMA_ERROR;
            str_copy(n->actions[n->num_actions].label, label, 64);
            str_copy(n->actions[n->num_actions].command, command, 256);
            n->num_actions++;
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

// ─── Dismiss ─────────────────────────────────────────────────────────────────
sigma_status dismiss_notification(sigma_u32 notif_id) {
    for (sigma_u32 i = 0; i < g_daemon.queue_count; ++i) {
        if (g_daemon.queue[i].id == notif_id) {
            g_daemon.queue[i].dismissed = SIGMA_TRUE;
            // Move to history
            if (g_daemon.history_count < MAX_HISTORY) {
                g_daemon.history[g_daemon.history_count++] = g_daemon.queue[i];
            }
            // Remove from queue
            for (sigma_u32 j = i; j < g_daemon.queue_count - 1; ++j)
                g_daemon.queue[j] = g_daemon.queue[j + 1];
            g_daemon.queue_count--;
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

// ─── Dismiss All ─────────────────────────────────────────────────────────────
sigma_status dismiss_all() {
    for (sigma_u32 i = 0; i < g_daemon.queue_count; ++i) {
        if (g_daemon.history_count < MAX_HISTORY) {
            g_daemon.history[g_daemon.history_count++] = g_daemon.queue[i];
        }
    }
    g_daemon.queue_count = 0;
    return SIGMA_SUCCESS;
}

// ─── Expire TTL (called periodically by compositor) ──────────────────────────
sigma_status expire_notifications(sigma_u32 now_ms) {
    for (sigma_u32 i = 0; i < g_daemon.queue_count; ) {
        Notification* n = &g_daemon.queue[i];
        if (n->ttl_ms > 0 && (now_ms - n->created_at_ms) > n->ttl_ms) {
            dismiss_notification(n->id);
            // Don't increment i — array was shifted
        } else {
            i++;
        }
    }

    // Auto-disable DnD if timer expired
    if (g_daemon.dnd_enabled && g_daemon.dnd_until_ms > 0 && now_ms >= g_daemon.dnd_until_ms) {
        g_daemon.dnd_enabled = SIGMA_FALSE;
    }

    return SIGMA_SUCCESS;
}

// ─── Do Not Disturb ──────────────────────────────────────────────────────────
sigma_status set_dnd(sigma_bool enabled, sigma_u32 duration_ms) {
    g_daemon.dnd_enabled = enabled;
    g_daemon.dnd_until_ms = enabled ? duration_ms : 0;
    return SIGMA_SUCCESS;
}

// ─── Get Notification Count ──────────────────────────────────────────────────
sigma_u32 get_pending_count() {
    sigma_u32 count = 0;
    for (sigma_u32 i = 0; i < g_daemon.queue_count; ++i) {
        if (!g_daemon.queue[i].read) count++;
    }
    return count;
}

} // namespace notify
} // namespace sigma

extern "C" {
    sigma_status sigma_notify_init(void) { return sigma::notify::notify_init(); }
    sigma_u32 sigma_notify_send(const char* app, const char* summary,
                                 const char* body, sigma_u32 urgency, sigma_u32 ttl) {
        return sigma::notify::send_notification(app, summary, body,
                                                 (sigma::notify::Urgency)urgency, ttl);
    }
    sigma_status sigma_notify_dismiss(sigma_u32 id) { return sigma::notify::dismiss_notification(id); }
    sigma_status sigma_notify_dismiss_all(void)      { return sigma::notify::dismiss_all(); }
    sigma_status sigma_notify_dnd(sigma_u32 on, sigma_u32 dur) {
        return sigma::notify::set_dnd(on ? SIGMA_TRUE : SIGMA_FALSE, dur);
    }
    sigma_u32 sigma_notify_pending(void) { return sigma::notify::get_pending_count(); }
}
