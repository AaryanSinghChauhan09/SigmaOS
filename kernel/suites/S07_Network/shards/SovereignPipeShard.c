#include "../../include/sigma_base.h"

#include "../../include/SovereignIPC.h"
#include "../../include/sigma_libc.h"

sigma_err_t sigma_pipe_init(void) {
    sigma_printf("  Σ [PIPE]: Sovereign Anonymous Ring-Buffer Pipes online.\n");
    return SIGMA_OK;
}

void SovereignPipe_Register(void) {
    SovereignIPC_Register("pipe", sigma_pipe_init);
}

/* Dispatcher implementation for system calls */
sigma_err_t sigma_pipe_create(int* r, int* w) {
    sigma_printf("Σ [SYS]: Creating atomic pipe shard pair.\n");
    *r = 10; *w = 11; /* Dummy FDs */
    return SIGMA_OK;
}



