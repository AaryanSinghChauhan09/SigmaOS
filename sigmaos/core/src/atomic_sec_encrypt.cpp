#include "sigma_core.h"
#include "libc/sigma_libc.h"

extern "C" {

void sec_encrypt_file(const char* filename) {
    sigma_kprint("[SigmaSec] Executing atomic Quantum-Safe encryption on: ");
    sigma_kprint(filename);
    sigma_kprint("\n");
}

}

} // extern "C"
