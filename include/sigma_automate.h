/**
 * @file sigma_automate.h
 * @brief SigmaOS Automation Engine API — macOS Shortcuts + Cron + Ansible inspired
 *
 * Defines trigger types, action descriptors, and the runtime pipeline
 * for the automation taskmaster daemon.
 */

#pragma once
#include "sigma_kernel_types.h"

namespace sigma {
namespace automate {

#define SIGMA_MAX_TRIGGERS    64
#define SIGMA_MAX_ACTIONS     32
#define SIGMA_MAX_PLAYBOOKS   128
#define SIGMA_CRON_EXPR_LEN   64
#define SIGMA_CMD_LEN         256

// ─── Trigger Types ────────────────────────────────────────────────────────────
typedef enum : sigma_u32 {
    TRIGGER_ON_BOOT         = 0,   // Fires once after init daemon completes
    TRIGGER_ON_SHUTDOWN     = 1,   // Fires before system power-off
    TRIGGER_ON_LOGIN        = 2,   // Fires after user session opens
    TRIGGER_ON_LOGOUT       = 3,   
    TRIGGER_ON_WIFI_CONNECT = 4,   // Fires when any NIC gets an IP
    TRIGGER_ON_USB_INSERT   = 5,   // Fires when xHCI detects device attach
    TRIGGER_CRON_SCHEDULE   = 6,   // Time-based — uses cron expression
    TRIGGER_ON_LOW_BATTERY  = 7,   // Fires when ACPI reports < 15%
    TRIGGER_ON_FILE_CHANGE  = 8,   // inotify-style path watcher
    TRIGGER_MANUAL          = 9,   // User-invoked via sigma-run CLI
    TRIGGER_SYSTEMD_TIMER   = 10,  // Monotonic systemd-style timer (interval-based)
} TriggerType;

// ─── Action Types ─────────────────────────────────────────────────────────────
typedef enum : sigma_u32 {
    ACTION_RUN_COMMAND      = 0,   // Execute a shell command
    ACTION_SET_THEME        = 1,   // Switch theme via Theme Engine
    ACTION_NOTIFY           = 2,   // Push desktop notification
    ACTION_INSTALL_PKG      = 3,   // Invoke OmniPkg install
    ACTION_MOUNT_FS         = 4,   // Mount a filesystem path
    ACTION_SEND_WEBHOOK     = 5,   // HTTP POST to a URL
    ACTION_APPLY_SYSCTL     = 6,   // Tune kernel parameters
} ActionType;

// ─── Trigger Descriptor ───────────────────────────────────────────────────────
struct TriggerDescriptor {
    TriggerType type;
    char cron_expr[SIGMA_CRON_EXPR_LEN];  // Used when type == TRIGGER_CRON_SCHEDULE
    char watch_path[SIGMA_CMD_LEN];       // Used when type == TRIGGER_ON_FILE_CHANGE
    sigma_u32 interval_sec;               // Used when type == TRIGGER_SYSTEMD_TIMER (OnCalendar/OnUnitActiveSec equivalent)
};

// ─── Action Descriptor ────────────────────────────────────────────────────────
struct ActionDescriptor {
    ActionType  type;
    char        payload[SIGMA_CMD_LEN];   // Command, theme name, pkg name, etc.
    sigma_bool  run_in_sandbox;           // Runs in Sovereign Sandbox if true
};

// ─── Playbook — one trigger → many actions ───────────────────────────────────
struct Playbook {
    sigma_u32         id;
    char              name[64];
    TriggerDescriptor trigger;
    ActionDescriptor  actions[SIGMA_MAX_ACTIONS];
    sigma_u32         num_actions;
    sigma_bool        enabled;
};

// ─── Runtime Registry ─────────────────────────────────────────────────────────
struct AutomateRegistry {
    Playbook  playbooks[SIGMA_MAX_PLAYBOOKS];
    sigma_u32 count;
};

} // namespace automate
} // namespace sigma
