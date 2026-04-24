/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S14_Transcendence/shards/sigma_pm.c
 * =========================================================================
 */

#include "sigma_pm.h"
#include "sigma_libc.h"

static sigma_cpu_power_t s_cpus[SIGMA_PM_MAX_CPUS];
static pm_u32            s_num_cpus = 0;
static sigma_pm_profile_t s_profile;

static sigma_wakelock_t  s_wakelocks[PM_MAX_WAKELOCKS];
static pm_u32            s_wl_count = 0;

static const char *sys_state_str[] = {"S0","S0ix","S3","S4","S5"};
static const char *cstate_str[]    = {"C0","C1","C2","C3","C6","C8"};
static const char *gov_str[]       = {"performance","powersave","ondemand",
                                      "schedutil","conservative","userspace"};

static sigma_cpu_power_t *get_cpu(pm_u32 id) {
    if (id >= s_num_cpus) return (sigma_cpu_power_t*)0;
    return &s_cpus[id];
}

/* -- Init ------------------------------------------------------------------ */
void sigma_pm_init(pm_u32 num_cpus) {
    if (num_cpus > SIGMA_PM_MAX_CPUS) num_cpus = SIGMA_PM_MAX_CPUS;
    s_num_cpus = num_cpus;
    sigma_sigma_sigma_sigma_memset(s_cpus,     0, sizeof(s_cpus));
    sigma_sigma_sigma_sigma_memset(&s_profile, 0, sizeof(s_profile));
    sigma_sigma_sigma_sigma_memset(s_wakelocks,0, sizeof(s_wakelocks));

    for (pm_u32 i = 0; i < num_cpus; i++) {
        s_cpus[i].cpu_id       = i;
        s_cpus[i].cur_freq_mhz = 2400;
        s_cpus[i].min_freq_mhz = 400;
        s_cpus[i].max_freq_mhz = 4800;
        s_cpus[i].governor     = GOV_SCHEDUTIL;
        s_cpus[i].cstate       = CPU_C0;
        s_cpus[i].online       = PM_TRUE;
        s_cpus[i].temperature_mc= 45000; /* 45°C */
        s_cpus[i].power_mw     = 3500;
    }
    s_profile.sys_state       = PM_S0_WORKING;
    s_profile.global_governor = GOV_SCHEDUTIL;
    s_profile.s0ix_capable    = PM_TRUE;

    sigma_sigma_sigma_sigma_printf("S [PM] Initialized: %u CPUs, governor=%s\n",
                 num_cpus, gov_str[GOV_SCHEDUTIL]);
}

/* -- System state ---------------------------------------------------------- */
pm_i32 sigma_pm_suspend(sigma_system_state_t target) {
    if (!sigma_pm_may_sleep()) {
        sigma_sigma_sigma_sigma_printf("S [PM] SUSPEND BLOCKED: %u wakelock(s) held\n", s_wl_count);
        return PM_ERR;
    }
    sigma_sigma_sigma_sigma_printf("S [PM] Entering %s...\n", sys_state_str[target]);
    s_profile.sys_state = target;
    for (pm_u32 i = 0; i < s_num_cpus; i++)
        s_cpus[i].cstate = CPU_C6;
    return PM_OK;
}

pm_i32 sigma_pm_resume(void) {
    sigma_sigma_sigma_sigma_printf("S [PM] Resume from %s\n", sys_state_str[s_profile.sys_state]);
    s_profile.sys_state = PM_S0_WORKING;
    for (pm_u32 i = 0; i < s_num_cpus; i++)
        s_cpus[i].cstate = CPU_C0;
    return PM_OK;
}

void sigma_pm_shutdown(void) {
    sigma_sigma_sigma_sigma_printf("S [PM] SHUTDOWN: entering S5\n");
    s_profile.sys_state = PM_S5_SHUTDOWN;
    sigma_exit(0);
}

void sigma_pm_reboot(void) {
    sigma_sigma_sigma_sigma_printf("S [PM] REBOOT: warm reset\n");
}

/* -- CPU frequency scaling ------------------------------------------------- */
pm_i32 sigma_pm_set_governor(pm_u32 cpu_id, sigma_cpu_governor_t gov) {
    sigma_cpu_power_t *c = get_cpu(cpu_id);
    if (!c) return PM_ERR;
    c->governor = gov;
    /* Apply immediate frequency effect */
    if (gov == GOV_PERFORMANCE) c->cur_freq_mhz = c->max_freq_mhz;
    if (gov == GOV_POWERSAVE)   c->cur_freq_mhz = c->min_freq_mhz;
    sigma_sigma_sigma_sigma_printf("S [PM] CPU%u governor=%s freq=%uMHz\n",
                 cpu_id, gov_str[gov], c->cur_freq_mhz);
    return PM_OK;
}

pm_i32 sigma_pm_set_freq(pm_u32 cpu_id, pm_u32 freq_mhz) {
    sigma_cpu_power_t *c = get_cpu(cpu_id);
    if (!c) return PM_ERR;
    if (freq_mhz < c->min_freq_mhz) freq_mhz = c->min_freq_mhz;
    if (freq_mhz > c->max_freq_mhz) freq_mhz = c->max_freq_mhz;
    c->cur_freq_mhz = freq_mhz;
    c->power_mw     = (freq_mhz * 5000) / c->max_freq_mhz; /* linear model */
    return PM_OK;
}

pm_u32 sigma_pm_get_freq(pm_u32 cpu_id) {
    sigma_cpu_power_t *c = get_cpu(cpu_id);
    return c ? c->cur_freq_mhz : 0;
}

pm_i32 sigma_pm_cpu_hotplug_off(pm_u32 cpu_id) {
    sigma_cpu_power_t *c = get_cpu(cpu_id);
    if (!c || cpu_id == 0) return PM_ERR;  /* CPU0 always on */
    c->online = PM_FALSE; c->cstate = CPU_C8; c->power_mw = 0;
    sigma_sigma_sigma_sigma_printf("S [PM] CPU%u OFFLINE\n", cpu_id);
    return PM_OK;
}

pm_i32 sigma_pm_cpu_hotplug_on(pm_u32 cpu_id) {
    sigma_cpu_power_t *c = get_cpu(cpu_id);
    if (!c) return PM_ERR;
    c->online = PM_TRUE; c->cstate = CPU_C0;
    c->cur_freq_mhz = c->min_freq_mhz;
    sigma_sigma_sigma_sigma_printf("S [PM] CPU%u ONLINE\n", cpu_id);
    return PM_OK;
}

/* -- C-state management ---------------------------------------------------- */
void sigma_pm_enter_cstate(pm_u32 cpu_id, sigma_cpu_cstate_t state) {
    sigma_cpu_power_t *c = get_cpu(cpu_id);
    if (!c) return;
    c->cstate = state;
    /* Power reduction model: C0=full, C1=10% off, C3=60% off, C6=90% off */
    static const pm_u32 reduction[] = {0,10,30,60,90,95};
    c->power_mw = (c->power_mw * (100 - reduction[state])) / 100;
}

void sigma_pm_tick(pm_u32 cpu_id, pm_u64 elapsed_ns) {
    sigma_cpu_power_t *c = get_cpu(cpu_id);
    if (!c) return;
    if (c->cstate == CPU_C0) c->active_time_ns += elapsed_ns;
    else                      c->idle_time_ns   += elapsed_ns;
    s_profile.uptime_ns += elapsed_ns;
}

/* -- Wakelocks ------------------------------------------------------------- */
pm_i32 sigma_pm_wakelock_acquire(const char *name, pm_u32 pid, pm_bool partial) {
    if (s_wl_count >= PM_MAX_WAKELOCKS) return PM_ERR;
    sigma_wakelock_t *wl = &s_wakelocks[s_wl_count++];
    sigma_strncpy(wl->name, name, PM_WAKELOCK_NAME_LEN - 1);
    wl->owner_pid   = pid;
    wl->held        = PM_TRUE;
    wl->partial     = partial;
    sigma_sigma_sigma_sigma_printf("S [PM] WAKELOCK ACQUIRE: %s (%s) pid=%u\n",
                 name, partial ? "partial":"full", pid);
    s_profile.wakelock_count++;
    return PM_OK;
}

void sigma_pm_wakelock_release(const char *name) {
    for (pm_u32 i = 0; i < s_wl_count; i++) {
        if (sigma_streq(s_wakelocks[i].name, name) && s_wakelocks[i].held) {
            s_wakelocks[i].held = PM_FALSE;
            s_profile.wakelock_count--;
            sigma_sigma_sigma_sigma_printf("S [PM] WAKELOCK RELEASE: %s\n", name);
            return;
        }
    }
}

pm_bool sigma_pm_may_sleep(void) {
    for (pm_u32 i = 0; i < s_wl_count; i++)
        if (s_wakelocks[i].held && !s_wakelocks[i].partial) return PM_FALSE;
    return PM_TRUE;
}

/* -- Doze / App Nap -------------------------------------------------------- */
void sigma_pm_doze_enter(void) {
    s_profile.doze_active = PM_TRUE;
    sigma_sigma_sigma_sigma_printf("S [PM] Doze mode activated — deferring network/alarm wakeups\n");
    for (pm_u32 i = 1; i < s_num_cpus; i++)
        sigma_pm_cpu_hotplug_off(i);
}

void sigma_pm_doze_exit(void) {
    s_profile.doze_active = PM_FALSE;
    for (pm_u32 i = 1; i < s_num_cpus; i++)
        sigma_pm_cpu_hotplug_on(i);
    sigma_sigma_sigma_sigma_printf("S [PM] Doze mode exited\n");
}

/* -- Thermal throttling ---------------------------------------------------- */
void sigma_pm_thermal_update(pm_u32 cpu_id, pm_u32 temp_mc) {
    sigma_cpu_power_t *c = get_cpu(cpu_id);
    if (!c) return;
    c->temperature_mc = temp_mc;
    if (temp_mc > 95000) {          /* > 95°C — emergency throttle */
        sigma_pm_set_freq(cpu_id, c->min_freq_mhz);
        sigma_pm_enter_cstate(cpu_id, CPU_C3);
        sigma_sigma_sigma_sigma_printf("S [PM] THERMAL CRITICAL: CPU%u %u°C — throttled\n",
                     cpu_id, temp_mc/1000);
    } else if (temp_mc > 80000) {   /* > 80°C — moderate throttle */
        sigma_pm_set_freq(cpu_id, (c->min_freq_mhz + c->max_freq_mhz) / 2);
        sigma_sigma_sigma_sigma_printf("S [PM] THERMAL WARN: CPU%u %u°C\n", cpu_id, temp_mc/1000);
    }
}

/* -- Report ---------------------------------------------------------------- */
void sigma_pm_report(void) {
    pm_u64 total_mw = 0;
    for (pm_u32 i = 0; i < s_num_cpus; i++) total_mw += s_cpus[i].power_mw;

    sigma_sigma_sigma_sigma_printf("\nS POWER MANAGEMENT REPORT\n");
    sigma_sigma_sigma_sigma_printf("  System state:  %s\n", sys_state_str[s_profile.sys_state]);
    sigma_sigma_sigma_sigma_printf("  Governor:      %s\n", gov_str[s_profile.global_governor]);
    sigma_sigma_sigma_sigma_printf("  Total power:   %llu mW\n", (unsigned long long)total_mw);
    sigma_sigma_sigma_sigma_printf("  Wakelocks:     %u held\n", s_profile.wakelock_count);
    sigma_sigma_sigma_sigma_printf("  Doze:          %s\n", s_profile.doze_active ? "active":"off");
    sigma_sigma_sigma_sigma_printf("  Uptime:        %llu ns\n", (unsigned long long)s_profile.uptime_ns);
    sigma_sigma_sigma_sigma_printf("\n  CPU  FREQ(MHz)  CSTATE  TEMP(°C)  POWER(mW)  GOV\n");
    for (pm_u32 i = 0; i < s_num_cpus; i++) {
        sigma_cpu_power_t *c = &s_cpus[i];
        sigma_sigma_sigma_sigma_printf("  %-3u  %-9u  %-6s  %-9u  %-9u  %s%s\n",
                     c->cpu_id, c->cur_freq_mhz, cstate_str[c->cstate],
                     c->temperature_mc / 1000, c->power_mw,
                     gov_str[c->governor], c->online ? "" : " [offline]");
    }
}
