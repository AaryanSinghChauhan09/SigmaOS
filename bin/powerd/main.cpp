// SPDX-License-Identifier: GPL-2.0-or-later
// powerd — ACPI power management daemon for SigmaOS
//
// Handles: lid close/open, power button, battery low, thermal trips,
//          suspend (S3), hibernate (S4), CPU governor switching.
//
// Runs as a privileged shard with ACPI access rights.
// Communicates with:
//   • sigma-cpufreq (for governor changes)
//   • sigma-notifyd (for battery low notifications)
//   • sigma-session (for screen lock on suspend)
//   • sigma-journal (for power event audit log)
//
// CLI interface (via sigma-bus):
//   sigma-power status
//   sigma-power suspend
//   sigma-power hibernate
//   sigma-power profile set performance|balanced|saver
//
// Inspired by: UPower, logind, acpid, tlp

#include "../../include/drivers/driver_interface.h"
#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>

// ── ACPI event types ──────────────────────────────────────────────────────

typedef enum acpi_event {
    ACPI_EVENT_LID_CLOSE      = 0x01,
    ACPI_EVENT_LID_OPEN       = 0x02,
    ACPI_EVENT_POWER_BUTTON   = 0x03,
    ACPI_EVENT_SLEEP_BUTTON   = 0x04,
    ACPI_EVENT_BATTERY_LOW    = 0x05,
    ACPI_EVENT_BATTERY_CRIT   = 0x06,
    ACPI_EVENT_AC_ONLINE      = 0x07,
    ACPI_EVENT_AC_OFFLINE     = 0x08,
    ACPI_EVENT_THERMAL_WARN   = 0x09,
    ACPI_EVENT_THERMAL_CRIT   = 0x0A,
    ACPI_EVENT_FAN_ERROR      = 0x0B,
} acpi_event_t;

// ── Battery status ────────────────────────────────────────────────────────

typedef struct battery_status {
    uint32_t present;        // 1 = battery present
    uint32_t capacity_pct;   // 0–100
    uint32_t voltage_mv;
    int32_t  charge_rate_mw; // positive=charging, negative=discharging
    uint32_t time_remaining_min;
    char     state[16];      // "Charging", "Discharging", "Full"
    uint32_t cycle_count;
    uint32_t health_pct;
} battery_status_t;

// ── Power profile ─────────────────────────────────────────────────────────

typedef enum power_profile {
    PROFILE_PERFORMANCE = 0,
    PROFILE_BALANCED    = 1,
    PROFILE_POWER_SAVER = 2,
} power_profile_t;

static const char *profile_names[] = { "performance", "balanced", "saver" };

// ── Daemon state ──────────────────────────────────────────────────────────

static power_profile_t g_profile     = PROFILE_BALANCED;
static battery_status_t g_battery    = {0};
static bool              g_ac_online = true;
static bool              g_lid_open  = true;
static uint32_t          g_cpu_temp  = 0;
static uint32_t          g_fan_rpm   = 0;

// ── External hooks (implemented in respective subsystems) ─────────────────

extern void sigma_cpufreq_set_governor(uint32_t cpu, uint8_t governor);
extern void sigma_notifyd_send       (uint32_t urgency, const char *app,
                                       const char *summary, const char *body);
extern void sigma_journal_log        (uint16_t severity, const char *subsys,
                                       const char *fmt, ...);
extern void sigma_session_lock       (void);
extern void sigma_acpi_enter_s3      (void);  // suspend to RAM
extern void sigma_acpi_enter_s4      (void);  // hibernate
extern void sigma_acpi_power_off     (void);

// ── Profile enforcement ───────────────────────────────────────────────────

static void apply_profile(power_profile_t p) {
    g_profile = p;
    uint8_t governor;
    switch (p) {
    case PROFILE_PERFORMANCE: governor = 0 /* GOVERNOR_PERFORMANCE */; break;
    case PROFILE_POWER_SAVER: governor = 1 /* GOVERNOR_POWERSAVE   */; break;
    default:                  governor = 2 /* GOVERNOR_SCHEDUTIL   */; break;
    }
    // Apply to all online CPUs
    for (uint32_t cpu = 0; cpu < 64; cpu++)
        sigma_cpufreq_set_governor(cpu, governor);

    printf("[powerd] Profile set: %s\n", profile_names[p]);
    sigma_journal_log(1 /*INFO*/, "power",
                      "profile changed to %s", profile_names[p]);
}

// ── Battery polling ───────────────────────────────────────────────────────

static void poll_battery(void) {
    // Read from ACPI battery shard via sigma-bus
    // (simplified — reads /sys-equivalent via sigma-acpid)
    if (g_battery.capacity_pct <= 5 && !g_ac_online) {
        sigma_notifyd_send(2 /* CRITICAL */, "sigma-power",
                           "Battery critically low",
                           "System will hibernate in 60 seconds.");
        sigma_journal_log(2 /*WARN*/, "power",
                          "battery critical: %u%%", g_battery.capacity_pct);
    } else if (g_battery.capacity_pct <= 15 && !g_ac_online) {
        sigma_notifyd_send(1 /* NORMAL */, "sigma-power",
                           "Battery low",
                           "Plug in charger to continue.");
        apply_profile(PROFILE_POWER_SAVER);
    }
}

// ── ACPI event handler ────────────────────────────────────────────────────

static void handle_acpi_event(acpi_event_t ev) {
    switch (ev) {
    case ACPI_EVENT_LID_CLOSE:
        g_lid_open = false;
        sigma_session_lock();
        sigma_acpi_enter_s3();
        break;

    case ACPI_EVENT_LID_OPEN:
        g_lid_open = true;
        // Session manager handles screen unlock
        break;

    case ACPI_EVENT_POWER_BUTTON:
        sigma_journal_log(1, "power", "power button pressed — initiating shutdown");
        // Send shutdown IPC to init shard
        break;

    case ACPI_EVENT_SLEEP_BUTTON:
        sigma_session_lock();
        sigma_acpi_enter_s3();
        break;

    case ACPI_EVENT_BATTERY_LOW:
        poll_battery();
        break;

    case ACPI_EVENT_BATTERY_CRIT:
        sigma_notifyd_send(2, "sigma-power", "Battery critical — hibernating",
                           "Saving system state to disk.");
        sigma_acpi_enter_s4();
        break;

    case ACPI_EVENT_AC_ONLINE:
        g_ac_online = true;
        if (g_profile == PROFILE_POWER_SAVER)
            apply_profile(PROFILE_BALANCED);
        sigma_notifyd_send(0, "sigma-power", "AC power connected", "");
        break;

    case ACPI_EVENT_AC_OFFLINE:
        g_ac_online = false;
        apply_profile(PROFILE_BALANCED);
        sigma_notifyd_send(0, "sigma-power", "Running on battery", "");
        break;

    case ACPI_EVENT_THERMAL_WARN:
        printf("[powerd] Thermal warning — %u°C\n", g_cpu_temp);
        apply_profile(PROFILE_POWER_SAVER);
        break;

    case ACPI_EVENT_THERMAL_CRIT:
        printf("[powerd] THERMAL CRITICAL — %u°C — emergency shutdown\n", g_cpu_temp);
        sigma_acpi_power_off();
        break;

    default: break;
    }
}

// ── IPC request handler (sigma-bus messages from sigma-power CLI) ─────────

#define POWERD_OP_STATUS       0xA000
#define POWERD_OP_SUSPEND      0xA001
#define POWERD_OP_HIBERNATE    0xA002
#define POWERD_OP_PROFILE_SET  0xA003

static void handle_ipc(const sigma_ipc_msg_t *msg) {
    switch (msg->opcode) {
    case POWERD_OP_STATUS:
        printf("[powerd] status: profile=%s ac=%s bat=%u%% lid=%s temp=%u°C\n",
               profile_names[g_profile],
               g_ac_online ? "online" : "offline",
               g_battery.capacity_pct,
               g_lid_open ? "open" : "closed",
               g_cpu_temp);
        break;
    case POWERD_OP_SUSPEND:
        sigma_session_lock();
        sigma_acpi_enter_s3();
        break;
    case POWERD_OP_HIBERNATE:
        sigma_acpi_enter_s4();
        break;
    case POWERD_OP_PROFILE_SET:
        if (msg->arg1 < 3)
            apply_profile((power_profile_t)msg->arg1);
        break;
    }
}

// ── Main ──────────────────────────────────────────────────────────────────

int main(void) {
    printf("[powerd] starting ACPI power management daemon\n");

    // Register with driver bus
    sigma_driver_reg_t reg = {0};
    strncpy(reg.name, "sigma-powerd", sizeof(reg.name));
    reg.caps  = DRIVER_CAP_POWER_MGMT;
    reg.flags = 0;
    sigma_driver_register(&reg);

    // Default profile: balanced on battery, performance on AC
    apply_profile(g_ac_online ? PROFILE_PERFORMANCE : PROFILE_BALANCED);

    sigma_ipc_msg_t msg;
    while (1) {
        // Poll for ACPI events (1s timeout)
        if (sigma_driver_recv(&msg, 1000) == 0) {
            if (msg.opcode == SIGMA_DRV_OP_IRQ_NOTIFY) {
                handle_acpi_event((acpi_event_t)msg.arg1);
            } else {
                handle_ipc(&msg);
            }
        }
        // Periodic battery check every 30s
        static uint32_t ticks = 0;
        if (++ticks % 30 == 0) poll_battery();
    }
}
