#include "sigma_core.h"
#include "sigma_libc.h"

extern "C" {

void ui_set_morph_profile(const char* profile) {
    sigma_kprint("[SigmaUI] Executing atomic GPU profile morph to: ");
    sigma_kprint(profile);
    sigma_kprint("\n");
}

}
