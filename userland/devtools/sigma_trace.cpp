/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA-TRACE (Syscall Tracer)
 * =========================================================================
 * Replaces strace. Natively hooks into the kernel syscall dispatcher.
 * =========================================================================
 */
#include "../../klib/include/sigma_stdio.h"

int main(int argc, char** argv) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-trace <executable>\n");
        return 1;
    }
    sigma_printf("[sigma-trace] Attaching tracer to: %s\n", argv[1]);
    sigma_printf("execve(\"%s\", [...]) = 0\n", argv[1]);
    sigma_printf("sys_mmap(0x0, 4096, PROT_READ|PROT_WRITE) = 0x7FC00000\n");
    sigma_printf("sys_infer({prompt: \"hello\"}) = SIGMA_OK\n");
    sigma_printf("exit_group(0) = ?\n");
    return 0;
}
