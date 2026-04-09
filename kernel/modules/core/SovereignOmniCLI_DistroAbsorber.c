/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN OMNI-CLI DISTRO ABSORBER — IMPLEMENTATION (v2.0)
 * =========================================================================
 * Single definition of g_omnicli_absorption_table (moved from header to fix ODR).
 * =========================================================================
 */

#include "../../include/SovereignLibC.h"
#include "../../include/SovereignOmniCLI_DistroAbsorber.h"

/* -------------------------------------------------------------------------
 * Single definition of the translation table (was erroneously in the header)
 * ---------------------------------------------------------------------- */
OmniCLIPromptMapping_t g_omnicli_absorption_table[] = {
    /* Arch Linux family */
    {"pacman -Syu",    "Arch/Manjaro/Garuda/Endeavour", "sigma_sync_shard_all"},
    {"paru -S",        "Arch/AUR",                      "sigma_hotload_community_shard"},
    {"yay -S",         "Arch/AUR",                      "sigma_hotload_community_shard"},

    /* Debian / Ubuntu family */
    {"apt-get install","Debian/Ubuntu/Mint",             "sigma_hotload_shard"},
    {"apt install",    "Ubuntu/Pop!/Elementary",         "sigma_hotload_shard"},
    {"dpkg -i",        "Debian/Ubuntu",                  "sigma_mount_raw_shard"},

    /* SUSE / openSUSE */
    {"zypper install", "openSUSE/Leap/Tumbleweed",       "sigma_hotload_shard"},
    {"zypper dup",     "openSUSE/Tumbleweed",            "sigma_sync_shard_all"},

    /* RedHat / Fedora / CentOS */
    {"dnf install",    "Fedora/CentOS/RHEL/Nobara",      "sigma_hotload_shard"},
    {"dnf update",     "Fedora/CentOS/RHEL",             "sigma_sync_shard_all"},
    {"yum install",    "Legacy RHEL/CentOS",             "sigma_hotload_shard"},
    {"rpm -i",         "RedHat/Fedora",                  "sigma_mount_raw_shard"},

    /* Gentoo / source-based */
    {"emerge --ask",   "Gentoo",                         "sigma_compile_shard_source"},
    {"emerge -avuDN",  "Gentoo",                         "sigma_sync_shard_all"},

    /* Alpine */
    {"apk add",        "Alpine",                         "sigma_hotload_shard"},
    {"apk upgrade",    "Alpine",                         "sigma_sync_shard_all"},

    /* NixOS / declarative */
    {"nix-env -iA",    "NixOS",                          "sigma_hotload_declarative_shard"},
    {"nixos-rebuild",  "NixOS",                          "sigma_rebuild_system_shard"},

    /* Void Linux */
    {"xbps-install",   "Void",                           "sigma_hotload_shard"},
    {"xbps-remove",    "Void",                           "sigma_purge_shard"},

    /* Slackware */
    {"slackpkg update","Slackware",                      "sigma_sync_legacy"},
    {"installpkg",     "Slackware",                      "sigma_mount_raw_shard"},

    /* Bedrock */
    {"brl fetch",      "Bedrock",                        "sigma_hijack_subsystem"},
    {"brl enable",     "Bedrock",                        "sigma_ignite_service_shard"},

    /* Security / forensics distros */
    {"amnesia-wipe",   "Tails",                          "sigma_pqc_amnesic_purge"},
    {"qvm-create",     "Qubes",                          "sigma_spawn_secure_vfs_enclave"},

    /* Systemd service control */
    {"systemctl start","Systemd-based",                  "sigma_ignite_service_shard"},
    {"systemctl stop", "Systemd-based",                  "sigma_halt_service_shard"},
    {"systemctl enable","Systemd-based",                 "sigma_persist_service_shard"},

    /* macOS / Homebrew */
    {"brew install",   "macOS/Homebrew",                 "sigma_hotload_shard"},
    {"brew upgrade",   "macOS/Homebrew",                 "sigma_sync_shard_all"},

    /* Windows Package Managers */
    {"winget install", "Windows/WinGet",                 "sigma_hotload_shard"},
    {"choco install",  "Windows/Chocolatey",             "sigma_hotload_shard"},
    {"scoop install",  "Windows/Scoop",                  "sigma_hotload_shard"},

    /* Sentinel */
    {"EOF",            "NONE",                           "EOF"}
};

/* -------------------------------------------------------------------------
 * sigma_omnicli_map_command — returns shard name or NULL
 * ---------------------------------------------------------------------- */
const char *sigma_omnicli_map_command(const char *legacy_input) {
    for (int i = 0; ; i++) {
        if (sigma_streq(g_omnicli_absorption_table[i].legacy_command, "EOF"))
            return SIGMA_NULL;
        if (sigma_strstr(legacy_input,
                         g_omnicli_absorption_table[i].legacy_command))
            return g_omnicli_absorption_table[i].target_sigma_shard;
    }
}

/* -------------------------------------------------------------------------
 * sigma_omnicli_absorb_command — logs the mapping to the console
 * ---------------------------------------------------------------------- */
void sigma_omnicli_absorb_command(const char *legacy_input) {
    for (int i = 0; ; i++) {
        if (sigma_streq(g_omnicli_absorption_table[i].legacy_command, "EOF"))
            break;
        if (sigma_strstr(legacy_input,
                         g_omnicli_absorption_table[i].legacy_command)) {
            sigma_printf("Σ [OMNI-CLI]: Absorbed [%s] command from '%s'.\n",
                         legacy_input,
                         g_omnicli_absorption_table[i].legacy_distro_origin);
            sigma_printf("Σ [OMNI-CLI]: -> Native Shard: %s\n",
                         g_omnicli_absorption_table[i].target_sigma_shard);
            return;
        }
    }
    sigma_printf("Σ [OMNI-CLI]: Unknown command '%s'. Trying native dispatch.\n",
                 legacy_input);
}
