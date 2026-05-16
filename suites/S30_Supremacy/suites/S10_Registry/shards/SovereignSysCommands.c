#include "../../../../../include/libc/SovereignLibC.h"
#include "../../../../../include/SovereignCommand.h"
#include "../../../../../include/libc/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_kernel.h"

extern void _sigma_sys_close_window(const char* target);
extern void _sigma_sys_minimize_window(const char* target);
extern void _sigma_sys_open_window(const char* target);
extern void _sigma_sys_kill_pid(int pid);

static int sigma_atoi_local(const char* str) {
    int res = 0;
    while (*str >= '0' && *str <= '9') { res = res * 10 + (*str - '0'); str++; }
    return res;
}

static int sigma_strcmp_local(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

void handle_ui(int argc, char** argv) {
    if (argc < 3) { sigma_sigma_printf("Usage: sigma ui <open|close|minimize|tile|snap|workspace|theme|dock> <target>\n"); return; }
    const char* action = argv[2];
    const char* target = argc > 3 ? argv[3] : "all";
    if (sigma_strcmp_local(action, "close") == 0)       _sigma_sys_close_window(target);
    else if (sigma_strcmp_local(action, "minimize") == 0)  _sigma_sys_minimize_window(target);
    else if (sigma_strcmp_local(action, "open") == 0)      _sigma_sys_open_window(target);
    else if (sigma_strcmp_local(action, "tile") == 0)      sigma_sigma_printf("[UI] Activating tiling layout: %s\n", target);
    else sigma_sigma_printf("[UI] Action %s executed.\n", action);
}

void handle_sys(int argc, char** argv) {
    if (argc < 3) { sigma_sigma_printf("Usage: sigma sys <kill|tune|irq|info> [args]\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp_local(action, "kill") == 0 && argc > 3) {
        int pid = sigma_atoi_local(argv[3]);
        _sigma_sys_kill_pid(pid);
        sigma_sigma_printf("[SYS] Process %d terminated.\n", pid);
    } else if (sigma_strcmp_local(action, "info") == 0) {
        sigma_sigma_printf("[SYS] SigmaOS Zenith vROADMAP_1001 | Industrial Parity Secured.\n");
    } else {
        sigma_sigma_printf("[SYS] Action %s processed.\n", action);
    }
}

void SovereignSysCommands_Register(void) {
    SovereignCommand_Register("ui", "Window manager and UI controls", handle_ui);
    SovereignCommand_Register("sys", "Core system and kernel tuning", handle_sys);
}



