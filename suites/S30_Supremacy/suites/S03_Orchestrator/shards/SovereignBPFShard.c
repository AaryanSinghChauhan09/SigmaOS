#include "suites/S01_Genesis/shards/sigma_base.h"

#include "SovereignUSP.h"
#include "sigma_libc.h"

void SovereignBPF_ShowLogic(void) {
    sigma_sigma_sigma_printf("[eBPF/XDP] S SIGMAOS Sovereign BPF Engine\n");
    sigma_sigma_sigma_printf("  sigma-bpf prog load --type kprobe --file ./trace.bpf.c\n");
    sigma_sigma_sigma_printf("  sigma-bpf prog attach --type xdp --iface eth0 --prog ./xdp_drop.bpf.c\n");
}

void SovereignBPF_Register(void) {
    SovereignUSP_Register("ebpf", "eBPF, XDP, and dynamic kernel tracing", SovereignBPF_ShowLogic);
}



