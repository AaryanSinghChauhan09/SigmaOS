/*
 * Σ SigmaOS — sigma_profile_selector: Sovereign Distro Flavor Selector
 * Zero-Dependency.
 * Selects which services, drivers, and GUI to load at boot based on profile.
 *
 * Profiles:
 *   MINIMAL   — kernel + shell only
 *   DEVELOPER — compiler toolchain + FS tools + pkg mgr
 *   DESKTOP   — full Zenith GUI + apps + display server
 *   CLOUD     — networking stack + orchestration + event bus
 *   MOBILE    — ARM drivers + touch input + lightweight GUI
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sigma_service_parse_and_start(const char* service_name);
extern "C" void sigma_meta_boot_for_profile(sigma_u32 profile_id);

#define PROFILE_MINIMAL   0
#define PROFILE_DEVELOPER 1
#define PROFILE_DESKTOP   2
#define PROFILE_CLOUD     3
#define PROFILE_MOBILE    4

static const char* profile_names[] = {
    "Minimal", "Developer", "Desktop", "Cloud", "Mobile"
};

extern "C" int sigma_boot_profile(int profile) {
    sigma_vga_printf("[PROFILE] Booting SigmaOS in '%s' mode.\n", profile_names[profile]);

    sigma_meta_boot_for_profile((sigma_u32)profile);

    // Common services for all profiles
    sigma_service_parse_and_start("sigma_init");

    switch (profile) {
        case PROFILE_MINIMAL:
            sigma_vga_printf("[PROFILE] Minimal: Shell only.\n");
            sigma_service_parse_and_start("sigma_sh");
            break;

        case PROFILE_DEVELOPER:
            sigma_vga_printf("[PROFILE] Developer: Compiler + FS + PKG.\n");
            sigma_service_parse_and_start("sigma_sh");
            sigma_service_parse_and_start("sigma_pkg_daemon");
            sigma_service_parse_and_start("sigma_cron_daemon");
            break;

        case PROFILE_DESKTOP:
            sigma_vga_printf("[PROFILE] Desktop: Full Zenith GUI + Display Server.\n");
            sigma_service_parse_and_start("sigma_display_server");
            sigma_service_parse_and_start("sigma_zenith_desktop");
            sigma_service_parse_and_start("sigma_audio_daemon");
            sigma_service_parse_and_start("sigma_network_manager");
            sigma_service_parse_and_start("sigma_pkg_daemon");
            sigma_service_parse_and_start("sigma_cron_daemon");
            break;

        case PROFILE_CLOUD:
            sigma_vga_printf("[PROFILE] Cloud: Network + Orchestration + Event Bus.\n");
            sigma_service_parse_and_start("sigma_network_manager");
            sigma_service_parse_and_start("sigma_kube_orchestrator");
            sigma_service_parse_and_start("sigma_event_bus");
            sigma_service_parse_and_start("sigma_mcp_bridge");
            sigma_service_parse_and_start("sigma_cron_daemon");
            break;

        case PROFILE_MOBILE:
            sigma_vga_printf("[PROFILE] Mobile: Touch + Lightweight GUI.\n");
            sigma_service_parse_and_start("sigma_display_server");
            sigma_service_parse_and_start("sigma_touch_input");
            sigma_service_parse_and_start("sigma_zenith_mobile");
            sigma_service_parse_and_start("sigma_network_manager");
            break;
    }

    sigma_vga_printf("[PROFILE] Boot sequence complete for '%s'.\n", profile_names[profile]);
    return 0;
}
