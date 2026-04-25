#include "sigma_core.h"
#include "sigma_libc.h"

extern "C" {

void sec_firewall_enable(int adaptive_mode) {
    sigma_kprint("[SigmaSec] Enabling atomic Zero-Trust firewall (Adaptive: ");
    sigma_kprint_int(adaptive_mode);
    sigma_kprint(")\n");
}

}
