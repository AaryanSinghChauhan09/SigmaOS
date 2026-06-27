// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_notify.h — Notification system
 *
 * Apps send notifications via sigma-bus (sigma.Notifications interface).
 * sigma-notify daemon collects them and forwards to the Zenith panel.
 * All routing happens on sigma-bus — no direct socket needed by apps.
 *
 * Usage (from any app):
 *   sigma_bus_emit(bus, SIGMA_IFACE_NOTIFY, "Notify",
 *       "{\"title\":\"Download complete\","
 *       "\"body\":\"ffmpeg-6.0 installed\","
 *       "\"icon\":\"package\","
 *       "\"actions\":[{\"id\":\"open\",\"label\":\"Open\"}]}");
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Notification urgency ────────────────────────────────────────────────── */
typedef enum {
    SIGMA_NOTIFY_LOW      = 0,  /* ambient info, auto-dismiss after 4s      */
    SIGMA_NOTIFY_NORMAL   = 1,  /* default — dismiss after 8s or on click   */
    SIGMA_NOTIFY_CRITICAL = 2,  /* stays until explicitly dismissed         */
} sigma_notify_urgency_t;

/* ── Action button ───────────────────────────────────────────────────────── */
typedef struct {
    char id[32];     /* action identifier returned to app on click        */
    char label[64];  /* button text shown to user                         */
} sigma_notify_action_t;

/* ── Notification descriptor ─────────────────────────────────────────────── */
typedef struct {
    sigma_u32              id;           /* assigned by daemon, 0=new         */
    char                   app_name[64];
    char                   title[128];
    char                   body[512];
    char                   icon[64];     /* icon name or /sigma/share/icons/  */
    sigma_notify_urgency_t urgency;
    sigma_u32              expire_ms;    /* 0 = use urgency default           */
    sigma_notify_action_t  actions[4];
    int                    action_count;
    bool                   resident;     /* don't auto-dismiss                */
    sigma_u64              timestamp_ns;
} sigma_notification_t;

/* ── Client API (thin wrapper over sigma-bus) ────────────────────────────── */

/* Send a notification. Returns the notification ID. */
sigma_u32 sigma_notify_send(const sigma_notification_t* notif);

/* Convenience: simple text notification */
sigma_u32 sigma_notify_info(const char* app, const char* title, const char* body);
sigma_u32 sigma_notify_warn(const char* app, const char* title, const char* body);
sigma_u32 sigma_notify_error(const char* app, const char* title, const char* body);

/* Update or replace a notification by ID */
sigma_u32 sigma_notify_update(sigma_u32 id, const sigma_notification_t* notif);

/* Close a notification programmatically */
void sigma_notify_close(sigma_u32 id);

/* ── Daemon API (used by sigma-notify daemon internally) ─────────────────── */

typedef void (*sigma_notify_action_cb_t)(sigma_u32 notif_id,
                                          const char* action_id,
                                          void* userdata);

void sigma_notify_daemon_init(sigma_notify_action_cb_t on_action, void* ctx);
void sigma_notify_daemon_dispatch(void);
