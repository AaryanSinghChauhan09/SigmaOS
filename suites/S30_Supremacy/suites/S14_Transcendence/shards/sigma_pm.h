/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S14_Transcendence/shards/sigma_pm.h
 * =========================================================================
 * Sovereign Power Management — gap-closes:
 *   Linux  : ACPI, cpufreq (governors: performance/powersave/schedutil)
 *            suspend-to-RAM (S3), hibernate (S4), runtime PM, wakelock
 *   Windows: Power Plans (Balanced/High/Power-Saver), Modern Standby (S0ix)
 *   macOS  : IOPMrootDomain, pmset, App Nap, Energy Impact score
 *   Android: PowerManager, WakeLocks, Doze mode, App Standby buckets
 *   RTOS   : tickless idle (CONFIG_NO_HZ), dynamic voltage scaling
 * =========================================================================
 */

#ifndef SIGMA_PM_H
#define SIGMA_PM_H

typedef unsigned long long pm_u64;
typedef unsigned int       pm_u32;
typedef signed   int       pm_i32;
typedef unsigned char      pm_u8;
typedef unsigned char      pm_bool;
#define PM_TRUE  ((pm_bool)1)
#define PM_FALSE ((pm_bool)0)
#define PM_OK    ((pm_i32) 0)
#define PM_ERR   ((pm_i32)-1)

/* -- Power states (ACPI Sx + CPU C-states) --------------------------------- */
typedef enum {
    PM_S0_WORKING    = 0,  /* fully active                              */
    PM_S0IX_STANDBY  = 1,  /* Modern Standby / Connected Standby        */
    PM_S3_SLEEP      = 2,  /* suspend-to-RAM                            */
    PM_S4_HIBERNATE  = 3,  /* suspend-to-disk                           */
    PM_S5_SHUTDOWN   = 4   /* soft-off                                  */
} sigma_system_state_t;

typedef enum {
    CPU_C0 = 0,  /* executing                                          */
    CPU_C1 = 1,  /* halt / HLT                                         */
    CPU_C2 = 2,  /* stop-clock                                         */
    CPU_C3 = 3,  /* sleep — cache flush required                       */
    CPU_C6 = 4,  /* deep power-down (Intel)                            */
    CPU_C8 = 5   /* deeper state (Intel Skylake+)                      */
} sigma_cpu_cstate_t;

/* -- CPU frequency governors (Linux cpufreq parity) ----------------------- */
typedef enum {
    GOV_PERFORMANCE  = 0,  /* always max frequency                     */
    GOV_POWERSAVE    = 1,  /* always min frequency                     */
    GOV_ONDEMAND     = 2,  /* scale on load (legacy)                   */
    GOV_SCHEDUTIL    = 3,  /* CFS utilization signal (modern Linux)    */
    GOV_CONSERVATIVE = 4,  /* gradual steps up/down                    */
    GOV_USERSPACE    = 5   /* manual via sigma-power CLI               */
} sigma_cpu_governor_t;

/* -- Wakelock (Android PowerManager parity) ------------------------------- */
#define PM_WAKELOCK_NAME_LEN 48
#define PM_MAX_WAKELOCKS     64
typedef struct {
    char     name[PM_WAKELOCK_NAME_LEN];
    pm_u32   owner_pid;
    pm_u64   acquired_ns;
    pm_bool  held;
    pm_bool  partial;  /* partial = CPU on, screen off (Android model) */
} sigma_wakelock_t;

/* -- Per-CPU power info ----------------------------------------------------- */
#define SIGMA_PM_MAX_CPUS 256
typedef struct {
    pm_u32              cpu_id;
    pm_u32              cur_freq_mhz;
    pm_u32              min_freq_mhz;
    pm_u32              max_freq_mhz;
    sigma_cpu_cstate_t  cstate;
    sigma_cpu_governor_t governor;
    pm_u64              idle_time_ns;
    pm_u64              active_time_ns;
    pm_u32              temperature_mc;  /* milli-celsius               */
    pm_u32              power_mw;        /* milliwatts                  */
    pm_bool             online;
} sigma_cpu_power_t;

/* -- System-wide power profile --------------------------------------------- */
typedef struct {
    sigma_system_state_t sys_state;
    sigma_cpu_governor_t global_governor;
    pm_u32               total_power_mw;
    pm_u64               uptime_ns;
    pm_u32               wakelock_count;
    pm_bool              doze_active;    /* Android Doze mode           */
    pm_bool              app_nap_active; /* macOS App Nap               */
    pm_bool              s0ix_capable;   /* Modern Standby supported    */
} sigma_pm_profile_t;

/* -- Public API ----------------------------------------------------------- */
void sigma_pm_init(pm_u32 num_cpus);

/* System state transitions */
pm_i32 sigma_pm_suspend(sigma_system_state_t target);
pm_i32 sigma_pm_resume(void);
void   sigma_pm_shutdown(void);
void   sigma_pm_reboot(void);

/* CPU frequency scaling */
pm_i32 sigma_pm_set_governor(pm_u32 cpu_id, sigma_cpu_governor_t gov);
pm_i32 sigma_pm_set_freq(pm_u32 cpu_id, pm_u32 freq_mhz);
pm_u32 sigma_pm_get_freq(pm_u32 cpu_id);
pm_i32 sigma_pm_cpu_hotplug_off(pm_u32 cpu_id);
pm_i32 sigma_pm_cpu_hotplug_on(pm_u32 cpu_id);

/* C-state management */
void   sigma_pm_enter_cstate(pm_u32 cpu_id, sigma_cpu_cstate_t state);
void   sigma_pm_tick(pm_u32 cpu_id, pm_u64 elapsed_ns);

/* Wakelocks (Android parity) */
pm_i32 sigma_pm_wakelock_acquire(const char *name, pm_u32 pid, pm_bool partial);
void   sigma_pm_wakelock_release(const char *name);
pm_bool sigma_pm_may_sleep(void);  /* false if any wakelock held       */

/* Doze mode (Android) / App Nap (macOS) */
void   sigma_pm_doze_enter(void);
void   sigma_pm_doze_exit(void);

/* Thermal throttling */
void   sigma_pm_thermal_update(pm_u32 cpu_id, pm_u32 temp_mc);

void   sigma_pm_report(void);

#endif /* SIGMA_PM_H */
