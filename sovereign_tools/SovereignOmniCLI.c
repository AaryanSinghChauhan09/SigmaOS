/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN OMNI-CLI DISPATCHER (PURE C11 & ASSEMBLY DELEGATES)
 * =========================================================================
 * Description: The Central Dispatch Authority. Replaces standard GNU tools
 * (bash, bash-builtins). Contains hardcoded low-level routing for all OS
 * actions (Network, AI, ML, Law, UI, Systools).
 * =========================================================================
 */

#include "../SigmaC11.h"
#include "../libc/SovereignLibC.h"
#include "../kernel/SovereignOmniShard.h"

/* Externs matching our SOLID-structured kernel modules */
extern void _sigma_sys_close_window(const char* target);
extern void _sigma_sys_minimize_window(const char* target);
extern void _sigma_sys_open_window(const char* target);
extern void _sigma_sys_kill_pid(int pid);
extern void SovereignAIKernel_ExecutePrompt(const char* prompt);
extern void SovereignML_RunInference(const char* data);
extern void SovereignIndianLaw_Query(const char* section);
extern void SovereignDataScience_RunAnalysis(const char* dataset);
extern void SovereignNet_ZeroTrustHandshake(void);

static int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

static int sigma_atoi(const char* str) {
    int res = 0;
    while (*str) {
        if (*str >= '0' && *str <= '9') {
            res = res * 10 + (*str - '0');
        }
        str++;
    }
    return res;
}

/* =========================================================================
 * SUB-ROUTERS FOR SPECIFIC CLI DOMAINS
 * ========================================================================= */

void SovereignCLI_HandleUI(int argc, char** argv) {
    if (argc < 3) return;
    const char* action = argv[2]; 
    const char* target = argc > 3 ? argv[3] : "all";

    if (sigma_strcmp(action, "close") == 0) _sigma_sys_close_window(target);
    else if (sigma_strcmp(action, "minimize") == 0) _sigma_sys_minimize_window(target);
    else if (sigma_strcmp(action, "open") == 0) _sigma_sys_open_window(target);
}

void SovereignCLI_HandleSystem(int argc, char** argv) {
    if (argc < 3) return;
    const char* action = argv[2];
    if (sigma_strcmp(action, "kill") == 0 && argc > 3) {
        int pid = sigma_atoi(argv[3]);
        _sigma_sys_kill_pid(pid);
        sigma_printf("[OMNI-CLI] Process %d terminated securely via C syscall.\n", pid);
    }
}

void SovereignCLI_HandleAI(int argc, char** argv) {
    if (argc < 3) return;
    SovereignAIKernel_ExecutePrompt(argv[2]);
}

void SovereignCLI_HandleML(int argc, char** argv) {
    if (argc < 3) return;
    SovereignML_RunInference(argv[2]);
}

void SovereignCLI_HandleLaw(int argc, char** argv) {
    if (argc < 3) return;
    SovereignIndianLaw_Query(argv[2]);
}

void SovereignCLI_HandleNetwork(int argc, char** argv) {
    SovereignNet_ZeroTrustHandshake();
    sigma_printf("[OMNI-CLI] Zero-Trust Aether Mesh connected.\n");
}

/* =========================================================================
 * MAIN OMNI-CLI DISPATCHER
 * ========================================================================= */

int main(int argc, char** argv) {
    if (argc < 2) {
        sigma_printf("Usage: sigma <module> <action> [args...]\n");
        sigma_printf("Modules: ui, sys, ai, ml, law, net, ds\n");
        return 1;
    }

    const char* module = argv[1];
    
    // Command Matrix 
    if (sigma_strcmp(module, "ui") == 0) SovereignCLI_HandleUI(argc, argv);
    else if (sigma_strcmp(module, "sys") == 0) SovereignCLI_HandleSystem(argc, argv);
    else if (sigma_strcmp(module, "ai") == 0) SovereignCLI_HandleAI(argc, argv);
    else if (sigma_strcmp(module, "ml") == 0) SovereignCLI_HandleML(argc, argv);
    else if (sigma_strcmp(module, "law") == 0) SovereignCLI_HandleLaw(argc, argv);
    else if (sigma_strcmp(module, "net") == 0) SovereignCLI_HandleNetwork(argc, argv);
    else if (sigma_strcmp(module, "ds") == 0 && argc > 2) SovereignDataScience_RunAnalysis(argv[2]);
    else {
        sigma_printf("[OMNI-CLI] Invalid Shard mapping. Purge or correct command.\n");
    }
    
    return 0;
}

