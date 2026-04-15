/*
 * =========================================================================
 * Σ SIGMAOS kernel/suites/S09_Intelligence/shards/sigma_neural_sched.h
 * =========================================================================
 * Neural Resource Balancer — gap-closes:
 *   Linux  : CGroup v2 CPU/MEM pressure signals, PSI (Pressure Stall Info)
 *   Windows: Dynamic Fair Share Scheduling (DFSS), ML-based Governor
 *   macOS  : Quality of Service inference, thermal pressure API
 *   Android: ADPF (Android Dynamic Performance Framework)
 *   ChromeOS: Discardable memory, tab lifecycle manager
 * =========================================================================
 * Implements a lightweight feedforward predictor that adjusts scheduling
 * weights, memory reclaim thresholds, and thermal governors without
 * requiring a full neural runtime (pure C11, zero deps).
 * =========================================================================
 */

#ifndef SIGMA_NEURAL_SCHED_H
#define SIGMA_NEURAL_SCHED_H

typedef unsigned long long ns_u64;
typedef unsigned int       ns_u32;
typedef signed   int       ns_i32;
typedef unsigned char      ns_u8;
typedef unsigned char      ns_bool;
#define NS_TRUE  ((ns_bool)1)
#define NS_FALSE ((ns_bool)0)

/* ── Pressure levels (Linux PSI equivalent) ─────────────────────────────── */
typedef enum {
    PRESSURE_NONE    = 0,
    PRESSURE_LOW     = 1,   /* < 25% resource saturation                */
    PRESSURE_MEDIUM  = 2,   /* 25-75%                                   */
    PRESSURE_HIGH    = 3,   /* > 75% — reclaim/shed load                */
    PRESSURE_CRITICAL= 4    /* > 95% — OOM/thermal emergency            */
} sigma_pressure_t;

/* ── Thermal zone (Android/macOS thermal API parity) ────────────────────── */
typedef enum {
    THERMAL_NORMAL   = 0,
    THERMAL_WARM     = 1,   /* light throttle                           */
    THERMAL_HOT      = 2,   /* moderate throttle                        */
    THERMAL_CRITICAL = 3    /* emergency — suspend non-essential tasks  */
} sigma_thermal_t;

/* ── Resource telemetry snapshot ─────────────────────────────────────────── */
typedef struct {
    ns_u64 cpu_util_pct;        /* 0-100                                */
    ns_u64 mem_used_kb;
    ns_u64 mem_total_kb;
    ns_u64 net_rx_kbps;
    ns_u64 net_tx_kbps;
    ns_u64 io_lat_us;           /* IO latency in microseconds           */
    ns_u64 timestamp_ns;
    sigma_thermal_t thermal;
} sigma_resource_snapshot_t;

/* ── Neural weight vector (tiny 3-layer perceptron) ─────────────────────── */
#define NS_FEATURES   6     /* cpu, mem, net_rx, net_tx, io_lat, thermal */
#define NS_HIDDEN     8
#define NS_OUTPUTS    3     /* sched_boost, mem_reclaim_pct, freq_scale  */

typedef struct {
    /* Layer 1: features -> hidden */
    int w1[NS_HIDDEN][NS_FEATURES];   /* fixed-point Q8 weights         */
    int b1[NS_HIDDEN];
    /* Layer 2: hidden -> outputs */
    int w2[NS_OUTPUTS][NS_HIDDEN];
    int b2[NS_OUTPUTS];
} sigma_nn_weights_t;

/* ── Prediction output ───────────────────────────────────────────────────── */
typedef struct {
    ns_i32 sched_boost;         /* +/- priority adjustment              */
    ns_u32 mem_reclaim_pct;     /* 0-100: % of inactive pages to reclaim*/
    ns_u32 freq_scale_pct;      /* 50-200: CPU frequency scaling %      */
    sigma_pressure_t cpu_pressure;
    sigma_pressure_t mem_pressure;
    sigma_thermal_t  thermal_advice;
} sigma_nn_prediction_t;

#define SIGMA_NS_HISTORY_LEN 64

/* ── Public API ─────────────────────────────────────────────────────────── */
void sigma_neural_sched_init(void);
void sigma_neural_sched_update(sigma_resource_snapshot_t *snap);
sigma_nn_prediction_t sigma_neural_sched_predict(void);
void sigma_neural_sched_apply(void);    /* apply prediction to sched/mm  */
void sigma_neural_sched_stats(void);

/* Pretrain with a default heuristic weight set */
void sigma_neural_sched_load_defaults(void);

#endif /* SIGMA_NEURAL_SCHED_H */
