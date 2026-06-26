// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_trace.h — Static kernel tracing infrastructure (illumos DTrace SDT-inspired)
 *
 * Zero overhead when disabled: every SIGMA_PROBE() compiles to do{}while(0).
 * When SIGMA_TRACING_ENABLED is defined, probes fire through a runtime
 * enable/disable gate — only active probes pay any cost.
 *
 * Usage in kernel code:
 *   SIGMA_PROBE(tcp, connect__start, dst_ip, dst_port);
 *   SIGMA_PROBE(zerotrust, flow__decision, src_pid, dst_pid, decision);
 *   SIGMA_PROBE(hypervisor, vm__create, name, mem_mb);
 *
 * CLI (sigma-traced):
 *   sigma-traced 'tcp:connect__start { printf("%s:%d\n", ip(arg0), arg1); }'
 *   sigma-traced 'zerotrust:flow__decision { @[arg2] = count(); }'
 */

#include <sigma_kernel_types.h>

/* ── Probe macro ──────────────────────────────────────────────────────────── */

#ifdef SIGMA_TRACING_ENABLED
  #define SIGMA_PROBE(provider, name, ...)                              \
      do {                                                              \
          if (sigma_trace_probe_enabled(#provider ":" #name)) {        \
              sigma_trace_fire(#provider ":" #name, ##__VA_ARGS__);    \
          }                                                             \
      } while (0)
#else
  #define SIGMA_PROBE(provider, name, ...)  do {} while (0)  /* zero overhead */
#endif

/* ── Probe registry ───────────────────────────────────────────────────────── */

#define SIGMA_TRACE_MAX_PROBES  256
#define SIGMA_TRACE_NAME_LEN    64

typedef struct sigma_trace_probe {
    char   name[SIGMA_TRACE_NAME_LEN]; /* "provider:probename"          */
    bool   enabled;
    sigma_u64 fire_count;
} sigma_trace_probe_t;

/* Register a probe at module init time */
void sigma_trace_register(const char* probe_name);

/* Runtime gate — called by SIGMA_PROBE macro */
bool sigma_trace_probe_enabled(const char* probe_name);

/* Fire a probe — writes event to the per-CPU trace ring buffer */
void sigma_trace_fire(const char* probe_name, ...);

/* Enable / disable by name or glob (sigma-traced calls these) */
int  sigma_trace_probe_enable(const char* name_or_glob);
int  sigma_trace_probe_disable(const char* name_or_glob);

/* Iterate registered probes (for sigma-traced list) */
const sigma_trace_probe_t* sigma_trace_probe_head(void);

void sigma_trace_init(void);
