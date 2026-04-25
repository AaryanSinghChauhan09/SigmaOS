// SigmaOS — Sigma-BPF: Programmable Packet/Event Filter
// Inspired by: Linux eBPF — but fully native, zero kernel module overhead
// Module: sigma-net-bpf
// USP over eBPF: No JIT verifier overhead, no BTF dependency, compile once
// Each filter is an atomic C function pointer — no VM, direct silicon execution

#ifndef SIGMA_BPF_H
#define SIGMA_BPF_H

#define SIGMA_BPF_MAX_FILTERS 64
#define SIGMA_BPF_PASS  1
#define SIGMA_BPF_DROP  0

// A Sigma-BPF filter: user-defined function that inspects a raw packet
typedef int (*sigma_bpf_fn)(const unsigned char* pkt, unsigned int len, void* ctx);

typedef struct SigmaBPFFilter {
    sigma_bpf_fn  fn;
    void*         ctx;
    const char*   name;
    unsigned long hit_count;
    unsigned long drop_count;
} SigmaBPFFilter;

typedef struct SigmaBPFChain {
    SigmaBPFFilter filters[SIGMA_BPF_MAX_FILTERS];
    unsigned int   count;
} SigmaBPFChain;

static inline void bpf_chain_init(SigmaBPFChain* c) {
    c->count = 0;
}

// Register a user-defined filter function
static inline int bpf_register(SigmaBPFChain* c, sigma_bpf_fn fn,
                                 void* ctx, const char* name) {
    if (c->count >= SIGMA_BPF_MAX_FILTERS) return -1;
    c->filters[c->count].fn         = fn;
    c->filters[c->count].ctx        = ctx;
    c->filters[c->count].name       = name;
    c->filters[c->count].hit_count  = 0;
    c->filters[c->count].drop_count = 0;
    c->count++;
    return 0;
}

// Run packet through all registered filters (short-circuit on DROP)
static inline int bpf_run(SigmaBPFChain* c, const unsigned char* pkt,
                            unsigned int len) {
    for (unsigned int i = 0; i < c->count; i++) {
        c->filters[i].hit_count++;
        int verdict = c->filters[i].fn(pkt, len, c->filters[i].ctx);
        if (verdict == SIGMA_BPF_DROP) {
            c->filters[i].drop_count++;
            return SIGMA_BPF_DROP;
        }
    }
    return SIGMA_BPF_PASS;
}

// Example built-in filter: block packets shorter than minimum Ethernet frame
static inline int bpf_builtin_min_frame(const unsigned char* pkt,
                                         unsigned int len, void* ctx) {
    (void)pkt; (void)ctx;
    return (len >= 60) ? SIGMA_BPF_PASS : SIGMA_BPF_DROP;
}

#endif /* SIGMA_BPF_H */
