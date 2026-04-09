/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TIMER SUBSYSTEM (v1.0 — PURE C11)
 * =========================================================================
 * Competitor Gap Closed: Linux hrtimer + timer_wheel + POSIX timers,
 * macOS kqueue EVFILT_TIMER, FreeBSD callout, Windows TimerQueue.
 * SigmaOS had zero timer infrastructure.
 *
 * This shard implements:
 *   § 1  TSC / monotonic clock (CLOCK_MONOTONIC)
 *   § 2  Real-time clock (CLOCK_REALTIME / gettimeofday)
 *   § 3  High-resolution timer wheel (like Linux hrtimer rbtree)
 *   § 4  POSIX timers — timer_create / timer_settime / timer_gettime / timer_delete
 *   § 5  timerfd — file-descriptor based timers (Linux-specific, very popular)
 *   § 6  setitimer / alarm — legacy POSIX interval timers
 *   § 7  Clock nanosleep  — sleep with sub-millisecond precision
 *   § 8  Timer tick — simulated scheduler tick (CONFIG_HZ=1000 equivalent)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"

/* -----------------------------------------------------------------------
 * ░░ § 1. MONOTONIC CLOCK — TSC-based
 * In real hardware: RDTSC; here we simulate a nanosecond counter.
 * ----------------------------------------------------------------------- */
#define SIGMA_HZ        1000            /* timer ticks per second */
#define NS_PER_TICK     (1000000000ULL / SIGMA_HZ)
#define NS_PER_SEC      1000000000ULL
#define NS_PER_MS       1000000ULL
#define NS_PER_US       1000ULL

static sigma_u64 s_mono_ns   = 0;    /* monotonic nanoseconds since boot */
static sigma_u64 s_realtime_ns = 0;  /* wallclock ns since Unix epoch (simulated) */
static sigma_u64 s_tick_count  = 0;

typedef struct {
    sigma_u64 tv_sec;
    sigma_u64 tv_nsec;
} SigmaTimeSpec_t;

static void timespec_from_ns(SigmaTimeSpec_t *ts, sigma_u64 ns) {
    ts->tv_sec  = ns / NS_PER_SEC;
    ts->tv_nsec = ns % NS_PER_SEC;
}

/* Simulated RDTSC — in real asm: "rdtsc; shl rdx,32; or rax,rdx" */
sigma_u64 sigma_clock_monotonic_ns(void) { return s_mono_ns; }
sigma_u64 sigma_clock_realtime_ns(void)  { return s_realtime_ns; }

void sigma_clock_gettime(int clk_id, SigmaTimeSpec_t *ts) {
    switch (clk_id) {
        case 0:  timespec_from_ns(ts, s_mono_ns);    break;  /* CLOCK_REALTIME    */
        case 1:  timespec_from_ns(ts, s_realtime_ns);break;  /* CLOCK_MONOTONIC   */
        default: timespec_from_ns(ts, s_mono_ns);    break;
    }
}

/* -----------------------------------------------------------------------
 * ░░ § 2. TIMER WHEEL — O(1) expiry bucket (like Linux timer_wheel)
 * 8 levels of 64-bucket wheels → covers 2^48 ticks without drift.
 * Simplified: 1 level of 512 buckets for demonstration.
 * ----------------------------------------------------------------------- */
#define WHEEL_SIZE    512
#define WHEEL_MASK    (WHEEL_SIZE - 1)
#define MAX_TIMERS    256

typedef void (*SigmaTimerCb_t)(void *arg);

typedef struct SigmaTimer {
    sigma_u64      expires_ns;   /* absolute monotonic expiry */
    sigma_u64      interval_ns;  /* 0 = one-shot */
    SigmaTimerCb_t callback;
    void          *arg;
    sigma_u32      timer_id;
    sigma_u32      signo;        /* signal to deliver (POSIX timers) */
    sigma_u32      pid;          /* target process */
    sigma_bool     active;
    sigma_bool     overrun;      /* expired while masked */
    sigma_u32      overrun_count;
    char           name[32];
} SigmaTimer_t;

static SigmaTimer_t s_timers[MAX_TIMERS];
static sigma_u32    s_timer_count = 0;

/* Wheel buckets — each slot holds a list of timer IDs */
#define BUCKET_DEPTH 16
typedef struct {
    sigma_u32 ids[BUCKET_DEPTH];
    sigma_u32 count;
} SigmaWheelBucket_t;

static SigmaWheelBucket_t s_wheel[WHEEL_SIZE];
static sigma_u32          s_wheel_cursor = 0; /* current tick bucket */

static sigma_u32 timer_bucket(sigma_u64 expires_ns) {
    sigma_u64 ticks = expires_ns / NS_PER_TICK;
    return (sigma_u32)(ticks & WHEEL_MASK);
}

static sigma_u32 timer_alloc_slot(void) {
    for (sigma_u32 i = 0; i < MAX_TIMERS; i++) {
        if (!s_timers[i].active) return i;
    }
    return (sigma_u32)-1;
}

/* -----------------------------------------------------------------------
 * ░░ § 3. POSIX TIMERS — timer_create / timer_settime / timer_delete
 * (like Linux kernel/time/posix-timers.c)
 * ----------------------------------------------------------------------- */
#define MAX_POSIX_TIMERS 64

typedef struct {
    sigma_u32   timer_id;
    sigma_u32   pid;
    sigma_u32   signo;
    sigma_u64   it_value_ns;    /* initial expiry (from now) */
    sigma_u64   it_interval_ns; /* reload interval */
    sigma_bool  armed;
    sigma_bool  in_use;
} SigmaPOSIXTimer_t;

static SigmaPOSIXTimer_t s_posix_timers[MAX_POSIX_TIMERS];
static sigma_u32         s_posix_timer_count = 0;

/** timer_create(CLOCK_REALTIME, &evp, &timerid) */
sigma_err_t sigma_timer_create(sigma_u32 pid, sigma_u32 signo,
                                sigma_u32 *out_id) {
    if (s_posix_timer_count >= MAX_POSIX_TIMERS) return SIGMA_ENOSPC;
    SigmaPOSIXTimer_t *t = &s_posix_timers[s_posix_timer_count];
    t->timer_id   = s_posix_timer_count++;
    t->pid        = pid;
    t->signo      = signo;
    t->armed      = SIGMA_FALSE;
    t->in_use     = SIGMA_TRUE;
    *out_id = t->timer_id;
    sigma_printf("Σ [TIMER]: timer_create: pid=%u signo=%u → id=%u\n",
                 pid, signo, t->timer_id);
    return SIGMA_OK;
}

/** timer_settime(timerid, flags, &new_value, &old_value) */
sigma_err_t sigma_timer_settime(sigma_u32 timer_id,
                                 sigma_u64 value_ns,    /* initial */
                                 sigma_u64 interval_ns) /* reload  */ {
    if (timer_id >= s_posix_timer_count) return SIGMA_EINVAL;
    SigmaPOSIXTimer_t *t = &s_posix_timers[timer_id];
    t->it_value_ns    = value_ns;
    t->it_interval_ns = interval_ns;
    t->armed          = (value_ns != 0);

    /* Insert into wheel */
    sigma_u32 slot_idx = timer_alloc_slot();
    if (slot_idx == (sigma_u32)-1) return SIGMA_ENOSPC;
    SigmaTimer_t *hw = &s_timers[slot_idx];
    hw->expires_ns   = s_mono_ns + value_ns;
    hw->interval_ns  = interval_ns;
    hw->signo        = t->signo;
    hw->pid          = t->pid;
    hw->timer_id     = timer_id;
    hw->active       = SIGMA_TRUE;
    hw->overrun      = SIGMA_FALSE;
    hw->overrun_count = 0;
    sigma_snprintf(hw->name, 31, "posix_%u", timer_id);

    sigma_u32 bucket = timer_bucket(hw->expires_ns);
    if (s_wheel[bucket].count < BUCKET_DEPTH)
        s_wheel[bucket].ids[s_wheel[bucket].count++] = slot_idx;

    sigma_printf("Σ [TIMER]: timer_settime id=%u value=%lluns interval=%lluns "
                 "→ bucket=%u expires_at=%lluns\n",
                 timer_id, (unsigned long long)value_ns,
                 (unsigned long long)interval_ns, bucket,
                 (unsigned long long)hw->expires_ns);
    return SIGMA_OK;
}

/** timer_gettime(timerid, &curr_value) */
sigma_err_t sigma_timer_gettime(sigma_u32 timer_id,
                                  sigma_u64 *remaining_ns,
                                  sigma_u64 *interval_ns) {
    if (timer_id >= s_posix_timer_count) return SIGMA_EINVAL;
    SigmaPOSIXTimer_t *t = &s_posix_timers[timer_id];
    if (!t->armed) { *remaining_ns = 0; *interval_ns = 0; return SIGMA_OK; }
    /* Find the hw timer for exact expiry */
    for (sigma_u32 i = 0; i < s_timer_count; i++) {
        if (s_timers[i].active && s_timers[i].timer_id == timer_id) {
            sigma_u64 exp = s_timers[i].expires_ns;
            *remaining_ns = (exp > s_mono_ns) ? (exp - s_mono_ns) : 0;
            *interval_ns  = s_timers[i].interval_ns;
            return SIGMA_OK;
        }
    }
    *remaining_ns = 0; *interval_ns = t->it_interval_ns;
    return SIGMA_OK;
}

/** timer_delete(timerid) */
sigma_err_t sigma_timer_delete(sigma_u32 timer_id) {
    if (timer_id >= s_posix_timer_count) return SIGMA_EINVAL;
    s_posix_timers[timer_id].armed  = SIGMA_FALSE;
    s_posix_timers[timer_id].in_use = SIGMA_FALSE;
    /* Deactivate hw timer */
    for (sigma_u32 i = 0; i < MAX_TIMERS; i++) {
        if (s_timers[i].active && s_timers[i].timer_id == timer_id)
            s_timers[i].active = SIGMA_FALSE;
    }
    sigma_printf("Σ [TIMER]: timer_delete id=%u\n", timer_id);
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ § 4. timerfd — Linux-specific, FD-based timers (very widely used)
 * (select/poll/epoll block on this fd until timer fires)
 * ----------------------------------------------------------------------- */
#define MAX_TIMERFD  32
#define TIMERFD_BASE_FD 3000

typedef struct {
    sigma_u64  expires_ns;
    sigma_u64  interval_ns;
    sigma_u64  expirations;  /* how many times it has fired (read clears) */
    sigma_bool armed;
    sigma_bool in_use;
} SigmaTimerFD_t;

static SigmaTimerFD_t s_timerfd[MAX_TIMERFD];

int sigma_timerfd_create(void) {
    for (int i = 0; i < MAX_TIMERFD; i++) {
        if (!s_timerfd[i].in_use) {
            sigma_memset(&s_timerfd[i], 0, sizeof(SigmaTimerFD_t));
            s_timerfd[i].in_use = SIGMA_TRUE;
            sigma_printf("Σ [TIMERFD]: created fd=%d\n", TIMERFD_BASE_FD + i);
            return TIMERFD_BASE_FD + i;
        }
    }
    return -1;
}

sigma_err_t sigma_timerfd_settime(int fd, sigma_u64 value_ns, sigma_u64 interval_ns) {
    int i = fd - TIMERFD_BASE_FD;
    if (i < 0 || i >= MAX_TIMERFD || !s_timerfd[i].in_use) return SIGMA_EINVAL;
    s_timerfd[i].expires_ns  = s_mono_ns + value_ns;
    s_timerfd[i].interval_ns = interval_ns;
    s_timerfd[i].armed       = (value_ns != 0);
    s_timerfd[i].expirations = 0;
    sigma_printf("Σ [TIMERFD]: fd=%d armed: value=%llums interval=%llums\n",
                 fd, (unsigned long long)(value_ns / NS_PER_MS),
                 (unsigned long long)(interval_ns / NS_PER_MS));
    return SIGMA_OK;
}

/** read() on a timerfd — returns expiration count */
sigma_u64 sigma_timerfd_read(int fd) {
    int i = fd - TIMERFD_BASE_FD;
    if (i < 0 || i >= MAX_TIMERFD || !s_timerfd[i].in_use) return 0;
    sigma_u64 exp = s_timerfd[i].expirations;
    s_timerfd[i].expirations = 0;
    return exp;
}

/* -----------------------------------------------------------------------
 * ░░ § 5. setitimer / alarm — legacy POSIX
 * ----------------------------------------------------------------------- */
#define ITIMER_REAL    0
#define ITIMER_VIRTUAL 1
#define ITIMER_PROF    2

typedef struct {
    sigma_u64 value_ns;
    sigma_u64 interval_ns;
    sigma_bool active;
} SigmaITimer_t;

#define MAX_PROCS_ITIMER 64
static SigmaITimer_t s_itimers[MAX_PROCS_ITIMER][3];

sigma_err_t sigma_setitimer(sigma_u32 pid, int which,
                             sigma_u64 value_ns, sigma_u64 interval_ns) {
    if (pid >= MAX_PROCS_ITIMER || which < 0 || which > 2) return SIGMA_EINVAL;
    s_itimers[pid][which].value_ns    = value_ns;
    s_itimers[pid][which].interval_ns = interval_ns;
    s_itimers[pid][which].active      = (value_ns != 0);
    const char *names[] = {"ITIMER_REAL","ITIMER_VIRTUAL","ITIMER_PROF"};
    sigma_printf("Σ [TIMER]: setitimer pid=%u %s value=%llums interval=%llums\n",
                 pid, names[which],
                 (unsigned long long)(value_ns / NS_PER_MS),
                 (unsigned long long)(interval_ns / NS_PER_MS));
    return SIGMA_OK;
}

sigma_err_t sigma_alarm(sigma_u32 pid, sigma_u32 seconds) {
    return sigma_setitimer(pid, ITIMER_REAL,
                           (sigma_u64)seconds * NS_PER_SEC, 0);
}

/* -----------------------------------------------------------------------
 * ░░ § 6. CLOCK NANOSLEEP
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_clock_nanosleep(sigma_u64 sleep_ns) {
    sigma_u64 wake_at = s_mono_ns + sleep_ns;
    sigma_printf("Σ [TIMER]: nanosleep %lluns — wakes at %lluns\n",
                 (unsigned long long)sleep_ns,
                 (unsigned long long)wake_at);
    /* In real kernel: put process to sleep, schedule timer, reschedule */
    /* Simulate by advancing monotonic time */
    s_mono_ns      += sleep_ns;
    s_realtime_ns  += sleep_ns;
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ § 7. TIMER TICK — scheduler heartbeat (CONFIG_HZ=1000)
 * Called by local APIC timer ISR every 1 ms.
 * Processes expired wheel buckets and delivers signals.
 * ----------------------------------------------------------------------- */
void sigma_timer_tick(void) {
    s_tick_count++;
    s_mono_ns     += NS_PER_TICK;
    s_realtime_ns += NS_PER_TICK;

    sigma_u32 cursor = (sigma_u32)(s_tick_count & WHEEL_MASK);
    SigmaWheelBucket_t *bucket = &s_wheel[cursor];

    for (sigma_u32 i = 0; i < bucket->count; i++) {
        sigma_u32 slot = bucket->ids[i];
        SigmaTimer_t *t = &s_timers[slot];
        if (!t->active || t->expires_ns > s_mono_ns) continue;

        /* Timer fired */
        if (t->callback) {
            t->callback(t->arg);
        } else if (t->signo) {
            sigma_printf("Σ [TIMER]: FIRE timer_id=%u → SIGNO=%u pid=%u\n",
                         t->timer_id, t->signo, t->pid);
        }

        if (t->interval_ns) {
            /* Reload periodic timer */
            t->expires_ns += t->interval_ns;
            sigma_u32 new_bucket = timer_bucket(t->expires_ns);
            if (s_wheel[new_bucket].count < BUCKET_DEPTH)
                s_wheel[new_bucket].ids[s_wheel[new_bucket].count++] = slot;
        } else {
            t->active = SIGMA_FALSE;
        }
    }
    /* Zero bucket after processing */
    bucket->count = 0;

    /* Check timerfd expirations */
    for (int i = 0; i < MAX_TIMERFD; i++) {
        if (s_timerfd[i].armed && s_timerfd[i].expires_ns <= s_mono_ns) {
            s_timerfd[i].expirations++;
            if (s_timerfd[i].interval_ns)
                s_timerfd[i].expires_ns += s_timerfd[i].interval_ns;
            else
                s_timerfd[i].armed = SIGMA_FALSE;
        }
    }

    /* Process setitimer ITIMER_REAL for all processes */
    for (sigma_u32 p = 0; p < MAX_PROCS_ITIMER; p++) {
        SigmaITimer_t *it = &s_itimers[p][ITIMER_REAL];
        if (!it->active || it->value_ns > NS_PER_TICK) {
            if (it->active) it->value_ns -= NS_PER_TICK;
            continue;
        }
        /* Expired: deliver SIGALRM */
        sigma_printf("Σ [TIMER]: ITIMER_REAL expired → SIGALRM to pid=%u\n", p);
        if (it->interval_ns) it->value_ns = it->interval_ns;
        else                 it->active   = SIGMA_FALSE;
    }
}

/* -----------------------------------------------------------------------
 * ░░ Public init + self-test
 * ----------------------------------------------------------------------- */
void SovereignTimers_Init(void) {
    sigma_printf("Σ [TIMER]: Initialising Sovereign Timer Subsystem...\n");

    /* Seed realtime clock: simulate 2026-04-09 08:00:00 UTC */
    s_realtime_ns = 1744185600ULL * NS_PER_SEC;
    s_mono_ns     = 0;
    s_tick_count  = 0;

    sigma_printf("Σ [TIMER]: CLOCK_REALTIME   = %llu s\n",
                 (unsigned long long)(s_realtime_ns / NS_PER_SEC));
    sigma_printf("Σ [TIMER]: CLOCK_MONOTONIC  = %llu ns\n",
                 (unsigned long long)s_mono_ns);

    /* POSIX timer: SIGALRM in 50ms, repeating every 100ms */
    sigma_u32 tid;
    sigma_timer_create(1, 14 /* SIGALRM */, &tid);
    sigma_timer_settime(tid, 50 * NS_PER_MS, 100 * NS_PER_MS);
    sigma_u64 remain, interval;
    sigma_timer_gettime(tid, &remain, &interval);
    sigma_printf("Σ [TIMER]: timer_gettime: remaining=%llums interval=%llums\n",
                 (unsigned long long)(remain / NS_PER_MS),
                 (unsigned long long)(interval / NS_PER_MS));

    /* timerfd */
    int tfd = sigma_timerfd_create();
    sigma_timerfd_settime(tfd, 200 * NS_PER_MS, 1000 * NS_PER_MS);

    /* setitimer */
    sigma_setitimer(1, ITIMER_REAL, 500 * NS_PER_MS, 0);
    sigma_alarm(2, 5); /* 5 seconds */

    /* nanosleep */
    sigma_clock_nanosleep(10 * NS_PER_MS);

    /* Simulate 100 ticks (100ms) — fire some timers */
    sigma_printf("Σ [TIMER]: Simulating 100 scheduler ticks...\n");
    for (int i = 0; i < 100; i++) sigma_timer_tick();

    /* Read timerfd */
    sigma_u64 exp = sigma_timerfd_read(tfd);
    sigma_printf("Σ [TIMER]: timerfd expirations read: %llu\n",
                 (unsigned long long)exp);

    sigma_timer_delete(tid);
    sigma_printf("Σ [TIMER]: Timer subsystem online. CONFIG_HZ=%d equivalent.\n",
                 SIGMA_HZ);
}
