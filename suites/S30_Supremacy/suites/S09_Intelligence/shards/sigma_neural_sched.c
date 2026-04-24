/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S09_Intelligence/shards/sigma_neural_sched.c
 * =========================================================================
 * Pure C11 feedforward neural predictor for kernel resource management.
 * Fixed-point Q8 arithmetic — no FPU dependency, freestanding safe.
 * =========================================================================
 */

#include "sigma_neural_sched.h"
#include "sigma_libc.h"

/* ── Fixed-point helpers (Q8: multiplied by 256) ─────────────────────────── */
#define Q8_ONE   256
#define Q8_MUL(a,b) (((a) * (b)) >> 8)
#define Q8_RELU(x)  ((x) > 0 ? (x) : 0)
#define Q8_CLAMP(x,lo,hi) ((x)<(lo)?(lo):((x)>(hi)?(hi):(x)))

/* ── State ───────────────────────────────────────────────────────────────── */
static sigma_balancer_t          s_balancer;
static sigma_resource_snapshot_t s_history[SIGMA_NS_HISTORY_LEN];
static ns_u32                    s_history_head = 0;
static sigma_nn_prediction_t     s_last_pred;

/* ── Default weights (hand-tuned heuristic — no training required) ───────── */
void sigma_neural_sched_load_defaults(void) {
    /* w1: each hidden neuron selects one dominant feature */
    sigma_sigma_memset(&s_balancer, 0, sizeof(s_balancer));
    s_balancer.base.name = "sigma_neural_balancer";

    /* H0: cpu pressure sensor */
    s_balancer.weights.w1[0][0] = 2 * Q8_ONE;
    /* H1: memory pressure sensor */
    s_balancer.weights.w1[1][1] = 2 * Q8_ONE;
    /* H2: network TX load */
    s_balancer.weights.w1[2][3] = Q8_ONE;
    /* H3: IO latency alarm */
    s_balancer.weights.w1[3][4] = Q8_ONE;
    /* H4: thermal alarm */
    s_balancer.weights.w1[4][5] = 3 * Q8_ONE;
    /* H5-H7: compound sensors */
    s_balancer.weights.w1[5][0] = Q8_ONE; s_balancer.weights.w1[5][1] = Q8_ONE;
    s_balancer.weights.w1[6][0] = Q8_ONE; s_balancer.weights.w1[6][4] = Q8_ONE;
    s_balancer.weights.w1[7][1] = Q8_ONE; s_balancer.weights.w1[7][5] = Q8_ONE;

    /* w2: map hidden to outputs */
    s_balancer.weights.w2[2][4] = -Q8_ONE;  /* freq_scale  <- thermal     */

    sigma_sigma_printf("S [NEURAL] Default Q8 weights loaded\n");
}

/* ── Init ────────────────────────────────────────────────────────────────── */
void sigma_neural_sched_init(void) {
    sigma_sigma_memset(&s_history, 0, sizeof(s_history));
    sigma_sigma_memset(&s_last_pred, 0, sizeof(s_last_pred));
    s_last_pred.freq_scale_pct = 100;
    sigma_neural_sched_load_defaults();
    sigma_sigma_printf("S [NEURAL] Resource Balancer initialized (Q8 perceptron)\n");
}

/* ── Feed a new resource snapshot ────────────────────────────────────────── */
void sigma_neural_sched_update(sigma_resource_snapshot_t *snap) {
    if (!snap) return;
    ns_u32 idx = s_history_head % SIGMA_NS_HISTORY_LEN;
    s_history[idx] = *snap;
    s_history_head++;
    if (s_balancer.history_len < SIGMA_NS_HISTORY_LEN) s_balancer.history_len++;
}

/* ── Forward pass ────────────────────────────────────────────────────────── */
sigma_nn_prediction_t sigma_neural_sched_predict(void) {
    if (s_balancer.history_len == 0) return s_last_pred;

    /* Average over last 4 snapshots for smoothing */
    ns_u64 avg[NS_FEATURES] = {0};
    ns_u32 window = s_balancer.history_len < 4 ? s_balancer.history_len : 4;
    for (ns_u32 i = 0; i < window; i++) {
        ns_u32 idx = (s_history_head - 1 - i) % SIGMA_NS_HISTORY_LEN;
        sigma_resource_snapshot_t *s = &s_history[idx];
        avg[0] += s->cpu_util_pct;
        avg[1] += (s->mem_used_kb * 100) / (s->mem_total_kb ? s->mem_total_kb : 1);
        avg[2] += s->net_rx_kbps / 1024;
        avg[3] += s->net_tx_kbps / 1024;
        avg[4] += s->io_lat_us   / 1000;
        avg[5] += (ns_u64)s->thermal * 33;  /* map 0-3 to 0-99           */
    }
    int feat[NS_FEATURES];
    for (int f = 0; f < NS_FEATURES; f++)
        feat[f] = (int)((avg[f] / window) * Q8_ONE / 100);

    /* Layer 1: features -> hidden */
    int hidden[NS_HIDDEN];
    for (int h = 0; h < NS_HIDDEN; h++) {
        int sum = s_balancer.weights.b1[h];
        for (int f = 0; f < NS_FEATURES; f++)
            sum += Q8_MUL(s_balancer.weights.w1[h][f], feat[f]);
        hidden[h] = Q8_RELU(sum);
    }

    /* Layer 2: hidden -> outputs */
    int out[NS_OUTPUTS];
    for (int o = 0; o < NS_OUTPUTS; o++) {
        int sum = s_balancer.weights.b2[o];
        for (int h = 0; h < NS_HIDDEN; h++)
            sum += Q8_MUL(s_balancer.weights.w2[o][h], hidden[h]);
        out[o] = sum;
    }

    /* Map raw outputs to meaningful ranges */
    sigma_nn_prediction_t pred;
    pred.sched_boost      = Q8_CLAMP(out[0] >> 8, -20, 20);
    pred.mem_reclaim_pct  = (ns_u32)Q8_CLAMP(out[1] >> 8, 0, 100);
    pred.freq_scale_pct   = (ns_u32)Q8_CLAMP(100 + (out[2] >> 8), 50, 200);

    /* Derive pressure levels from feature magnitudes */
    ns_u32 cpu_pct = (ns_u32)(avg[0] / window);
    ns_u32 mem_pct = (ns_u32)(avg[1] / window);
    pred.cpu_pressure = cpu_pct < 25 ? PRESSURE_NONE  :
                        cpu_pct < 50 ? PRESSURE_LOW    :
                        cpu_pct < 75 ? PRESSURE_MEDIUM :
                        cpu_pct < 90 ? PRESSURE_HIGH   : PRESSURE_CRITICAL;
    pred.mem_pressure = mem_pct < 60 ? PRESSURE_NONE  :
                        mem_pct < 80 ? PRESSURE_MEDIUM :
                        mem_pct < 95 ? PRESSURE_HIGH   : PRESSURE_CRITICAL;

    ns_u32 temp = (ns_u32)(avg[5] / window);
    pred.thermal_advice = temp < 33 ? THERMAL_NORMAL :
                          temp < 66 ? THERMAL_WARM   :
                          temp < 90 ? THERMAL_HOT    : THERMAL_CRITICAL;

    s_last_pred = pred;
    return pred;
}

/* ── Apply prediction to kernel subsystems ───────────────────────────────── */
void sigma_neural_sched_apply(void) {
    sigma_nn_prediction_t p = sigma_neural_sched_predict();

    sigma_sigma_printf("S [NEURAL] APPLY: sched_boost=%d reclaim=%u%% freq=%u%%\n",
                 p.sched_boost, p.mem_reclaim_pct, p.freq_scale_pct);

    if (p.cpu_pressure >= PRESSURE_HIGH)
        sigma_sigma_printf("S [NEURAL] ⚠ CPU PRESSURE HIGH — boosting CFS quantum\n");
    if (p.mem_pressure >= PRESSURE_HIGH)
        sigma_sigma_printf("S [NEURAL] ⚠ MEM PRESSURE HIGH — triggering reclaim %u%%\n",
                     p.mem_reclaim_pct);
    if (p.thermal_advice >= THERMAL_HOT)
        sigma_sigma_printf("S [NEURAL] 🌡 THERMAL HOT — scaling freq to %u%%\n",
                     p.freq_scale_pct);
}

/* ── Stats ───────────────────────────────────────────────────────────────── */
void sigma_neural_sched_stats(void) {
    sigma_nn_prediction_t p = s_last_pred;
    static const char *pres[] = {"none","low","medium","high","critical"};
    static const char *thm[]  = {"normal","warm","hot","critical"};
    sigma_sigma_printf("\nS NEURAL RESOURCE BALANCER — Last Prediction\n");
    sigma_sigma_printf("  sched_boost:   %+d\n",    p.sched_boost);
    sigma_sigma_printf("  mem_reclaim:   %u%%\n",   p.mem_reclaim_pct);
    sigma_sigma_printf("  freq_scale:    %u%%\n",   p.freq_scale_pct);
    sigma_sigma_printf("  cpu_pressure:  %s\n",     pres[p.cpu_pressure]);
    sigma_sigma_printf("  mem_pressure:  %s\n",     pres[p.mem_pressure]);
    sigma_sigma_printf("  thermal:       %s\n",     thm[p.thermal_advice]);
    sigma_sigma_printf("  history_sz:    %u\n",     s_balancer.history_len);
}
