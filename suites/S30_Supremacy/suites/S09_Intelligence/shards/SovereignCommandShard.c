#include "libc/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN CLI ENGINE (v1.0)
 * =========================================================================
 * Mission: High-performance Command Line Shell (Kernel Level).
 * Principles: Argv Tokenization, Command Routing, Help-System Integration.
 *
 * Implements a real command-line parser for the Sovereign Shell.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

#define MAX_ARGS 16

typedef void (*SigmaCmdFunc_t)(int argc, char** argv);

typedef struct {
    char            name[16];
    SigmaCmdFunc_t  func;
    char            help[48];
} SovereignCommand_t;

/* --- Registry --- */
static SovereignCommand_t s_cmd_registry[32];
static int s_cmd_count = 0;

void sigma_cli_register(const char* name, SigmaCmdFunc_t func, const char* help) {
    if (s_cmd_count >= 32) return;
    sigma_strncpy(s_cmd_registry[s_cmd_count].name, name, 16);
    s_cmd_registry[s_cmd_count].func = func;
    sigma_strncpy(s_cmd_registry[s_cmd_count].help, help, 48);
    s_cmd_count++;
}

/**
 * sigma_cli_execute: Parses a raw string into argc/argv and dispatches.
 */
void sigma_cli_execute(char* raw_cmd) {
    char* argv[MAX_ARGS];
    int argc = 0;
    
    /* Simplified space-split tokenizer */
    char* token = sigma_strtok(raw_cmd, " ");
    while (token && argc < MAX_ARGS) {
        argv[argc++] = token;
        token = sigma_strtok(SIGMA_NULL, " ");
    }
    
    if (argc == 0) return;
    
    /* Dispatch */
    for (int i = 0; i < s_cmd_count; i++) {
        if (sigma_streq(argv[0], s_cmd_registry[i].name)) {
            s_cmd_registry[i].func(argc, argv);
            return;
        }
    }
    sigma_sigma_printf("[CLI]: Unknown command '%s'.\n", argv[0]);
}

/* --- Module Factory --- */

void SovereignCommand_Register(void) {
    sigma_sigma_printf("[ZENITH]: Sovereign CLI Engine (Kernel Shell) active.\n");
}



