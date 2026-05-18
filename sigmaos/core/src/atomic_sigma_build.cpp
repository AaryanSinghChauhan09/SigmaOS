#include "libc/sigma_libc.h"

extern "C" {

void sigma_build_link() {
    sigma_kprint("[SigmaBuild] Executing Custom Sovereign Linker...\n");
    // Stripping away runtime dependencies (no garbage collectors, no VM layers)
    sigma_kprint("[SigmaBuild] Linking atomic objects directly to hardware entrypoint (-nostdlib -nostartfiles).\n");
}

}

} // extern "C"
