/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

// ============================================
// OPENSUSE ADVANCED COMMANDS
// ============================================
void sigma_load_opensuse_advanced(void) {
    if (!g_command_library) return;
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper repos -d", "Detailed Repo List",
        "zypper repos -d", "zypper repos -d",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "List repos with details"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper se -s", "Search with Source",
        "zypper se -s [package]", "zypper se -s nginx",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "Search with source package"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper if --requires", "Show Requires",
        "zypper if --requires [package]", "zypper if --requires firefox",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "Show package requirements"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper pt", "List Patterns",
        "zypper pt", "zypper pt",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "List available patterns"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper shell", "Zypper Shell",
        "zypper shell", "zypper sh",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "Interactive zypper shell"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper ps", "Show Processes",
        "zypper ps", "zypper ps -s",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "Show processes using deleted files"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper patch", "Apply Patches",
        "zypper patch", "zypper patch",
        "openSUSE", false, true, SIGMA_CMD_PACKAGE,
        "Apply maintenance patches"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper source-download", "Download Sources",
        "zypper source-download", "zypper source-download",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "Download all source packages"
    };
    
    // OBS
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "osc", "Open Build Service",
        "osc [command]", "osc checkout project/package",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "OBS command line tool"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "osc build", "OSC Build",
        "osc build [repo] [arch] [spec]", "osc build openSUSE_Tumbleweed x86_64",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "Build package in chroot"
    };
    
    printf("[Extended Commands] Loaded %d openSUSE advanced commands\n", 10);
}

// ============================================
// ALPINE/VOID/NIXOS ADVANCED
// ============================================
void sigma_load_alpine_advanced(void) {
    if (!g_command_library) return;
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apk fix", "Fix Packages",
        "apk fix [package]", "apk fix",
        "Alpine", false, true, SIGMA_CMD_PACKAGE,
        "Repair or upgrade packages"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apk audit", "Audit System",
        "apk audit [system]", "apk audit --system",
        "Alpine", false, false, SIGMA_CMD_PACKAGE,
        "Audit installed packages"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apk verify", "Verify Packages",
        "apk verify [package]", "apk verify",
        "Alpine", false, false, SIGMA_CMD_PACKAGE,
        "Verify package integrity"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "lbu status", "Backup Status",
        "lbu status", "lbu status",
        "Alpine", false, false, SIGMA_CMD_SYSTEM,
        "Show backup status"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "lbu package", "Create Apkovl",
        "lbu package", "lbu package",
        "Alpine", false, false, SIGMA_CMD_SYSTEM,
        "Create apkovl backup"
    };
    
    printf("[Extended Commands] Loaded %d Alpine advanced commands\n", 5);
}

void sigma_load_void_advanced(void) {
    if (!g_command_library) return;
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "xbps-query -l", "List Installed",
        "xbps-query -l", "xbps-query -l",
        "Void", false, false, SIGMA_CMD_PACKAGE,
        "List installed packages"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "xbps-query -o", "Owned By",
        "xbps-query -o [file]", "xbps-query -o /bin/bash",
        "Void", false, false, SIGMA_CMD_PACKAGE,
        "Find package owning file"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "xbps-query -H", "Hold Package",
        "xbps-query -H [package]", "xbps-query -H linux",
        "Void", false, true, SIGMA_CMD_PACKAGE,
        "Put package on hold"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "xbps-dgraph", "Dependency Graph",
        "xbps-dgraph [package]", "xbps-dgraph xbps",
        "Void", false, false, SIGMA_CMD_PACKAGE,
        "Output dependency graph"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "xbps-uchroot", "User Chroot",
        "xbps-uchroot [dir] [cmd]", "xbps-uchroot /chroot /bin/sh",
        "Void", false, true, SIGMA_CMD_SYSTEM,
        "Unprivileged chroot"
    };
    
    printf("[Extended Commands] Loaded %d Void advanced commands\n", 5);
}

void sigma_load_nixos_advanced(void) {
    if (!g_command_library) return;
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nix-channel", "Channel Manager",
        "nix-channel [command]", "nix-channel --list",
        "NixOS", false, false, SIGMA_CMD_PACKAGE,
        "Manage Nix channels"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nix-channel --add", "Add Channel",
        "nix-channel --add [url] [name]", "nix-channel --add https://nixos.org/channels/nixos-unstable nixos",
        "NixOS", false, true, SIGMA_CMD_PACKAGE,
        "Add Nix channel"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nix-copy-closure", "Copy Closure",
        "nix-copy-closure [options] [host]", "nix-copy-closure --to user@host /nix/store/...",
        "NixOS", false, false, SIGMA_CMD_PACKAGE,
        "Copy store paths to remote"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nix-prefetch-url", "Prefetch URL",
        "nix-prefetch-url [url]", "nix-prefetch-url https://example.com/file.tar.gz",
        "NixOS", false, false, SIGMA_CMD_PACKAGE,
        "Prefetch file and get hash"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nix-prefetch-git", "Prefetch Git",
        "nix-prefetch-git [url]", "nix-prefetch-git https://github.com/user/repo",
        "NixOS", false, false, SIGMA_CMD_PACKAGE,
        "Prefetch git repo and get hash"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nixos-container", "NixOS Container",
        "nixos-container [command]", "nixos-container create test",
        "NixOS", false, true, SIGMA_CMD_CONTAINER,
        "Manage NixOS containers"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nixos-enter", "Enter Container",
        "nixos-enter [container]", "nixos-enter test",
        "NixOS", false, true, SIGMA_CMD_CONTAINER,
        "Enter NixOS container"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nixos-option", "Query Option",
        "nixos-option [option]", "nixos-option services.nginx.enable",
        "NixOS", false, false, SIGMA_CMD_SYSTEM,
        "Query NixOS option value"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nixos-version", "NixOS Version",
        "nixos-version", "nixos-version --json",
        "NixOS", false, false, SIGMA_CMD_SYSTEM,
        "Show NixOS version"
    };
    
    printf("[Extended Commands] Loaded %d NixOS advanced commands\n", 9);
}

// ============================================
// CENTOS/ROCKY/ALMA ADVANCED
// ============================================
void sigma_load_centos_advanced(void) {
    if (!g_command_library) return;
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "subscription-manager list", "List Subscriptions",
        "subscription-manager list", "subscription-manager list --available",
        "RHEL", false, true, SIGMA_CMD_SYSTEM,
        "List available subscriptions"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "subscription-manager repos", "List Repos",
        "subscription-manager repos", "subscription-manager repos --list-enabled",
        "RHEL", false, true, SIGMA_CMD_PACKAGE,
        "Manage subscription repos"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "insights-client --checkin", "Insights Checkin",
        "insights-client --checkin", "insights-client --checkin",
        "RHEL", false, true, SIGMA_CMD_SYSTEM,
        "Check in with Red Hat Insights"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "rpm-ostree status", "OSTree Status",
        "rpm-ostree status", "rpm-ostree status -v",
        "RHEL CoreOS", false, false, SIGMA_CMD_PACKAGE,
        "Show OSTree status"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "rpm-ostree rebase", "Rebase OSTree",
        "rpm-ostree rebase [ref]", "rpm-ostree rebase fedora:fedora/40/x86_64/silverblue",
        "RHEL CoreOS", false, true, SIGMA_CMD_PACKAGE,
        "Rebase to different ref"
    };
    
    printf("[Extended Commands] Loaded %d CentOS/RHEL advanced commands\n", 5);
}

