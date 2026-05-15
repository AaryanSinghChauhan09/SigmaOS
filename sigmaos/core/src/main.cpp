#include "../../../include/sigma_core.h"
#include "../../../include/libc/sigma_libc.h"

extern "C" int main(int argc, char** argv) {
    sigma_kprint("========================================\n");
    sigma_kprint("   SigmaOS Sovereign Native Shell       \n");
    sigma_kprint("========================================\n");

    if (argc < 2) {
        sigma_kprint("Usage: s-os <command> [args]\n");
        return 1;
    }

    const char* cmd = argv[1];

    if (sigma_strcmp(cmd, "boot") == 0) {
        sigma_kprint("[NativeOS] Initializing atomic subsystems...\n");
        ui_init();
        sec_audit();
    } else if (sigma_strcmp(cmd, "net-secure") == 0) {
        net_secure_connect();
    } else if (sigma_strcmp(cmd, "media-load") == 0) {
        if (argc > 2) media_load_codec(argv[2]);
    } else {
        sigma_kprint("[NativeOS] Unknown atomic command.\n");
    }

    return 0;
}

} // extern "C"
