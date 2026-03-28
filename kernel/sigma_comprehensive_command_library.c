/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Comprehensive Command Library Implementation
 * ====================================================
 * Complete implementation of bash commands from all Linux distributions
 * and custom SigmaOS commands for all categories
 */

#include "../include/sigma_command_library.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Initialize Command Library
void sigma_command_library_initialize(void) {
    g_command_library = (SigmaCommandLibrary*)malloc(sizeof(SigmaCommandLibrary));
    if (!g_command_library) return;
    
    g_command_library->command_capacity = 2000;
    g_command_library->commands = (SigmaCommand*)malloc(
        g_command_library->command_capacity * sizeof(SigmaCommand));
    g_command_library->command_count = 0;
    g_command_library->distro_count = 0;
    strcpy(g_command_library->documentation, "");
    
    // Load all distro commands
    sigma_load_ubuntu_commands();
    sigma_load_fedora_commands();
    sigma_load_debian_commands();
    sigma_load_arch_commands();
    sigma_load_gentoo_commands();
    sigma_load_opensuse_commands();
    sigma_load_centos_commands();
    sigma_load_alpine_commands();
    sigma_load_void_commands();
    sigma_load_nixos_commands();
    
    // Load custom SigmaOS commands
    sigma_load_automation_commands();
    sigma_load_customization_commands();
    sigma_load_personalization_commands();
    sigma_load_data_science_commands();
    sigma_load_ml_commands();
    sigma_load_visualization_commands();
    sigma_load_camera_commands();
    sigma_load_setup_commands();
    sigma_load_security_commands();
    sigma_load_quantum_commands();
}

// ============================================
// UBUNTU/DEBIAN COMMANDS
// ============================================
void sigma_load_ubuntu_commands(void) {
    if (!g_command_library) return;
    
    // Package Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apt", "Advanced Package Tool for Debian/Ubuntu",
        "apt [options] [command] [package]",
        "apt update && apt upgrade -y",
        "Ubuntu/Debian", false, true, SIGMA_CMD_PACKAGE,
        "Most common package manager for Debian-based systems"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apt-get", "Advanced Package Tool (legacy)",
        "apt-get [options] [command] [package]",
        "apt-get install package-name",
        "Ubuntu/Debian", false, true, SIGMA_CMD_PACKAGE,
        "Legacy package manager, use apt instead"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "dpkg", "Debian Package Manager",
        "dpkg [options] [.deb file]",
        "dpkg -i package.deb",
        "Ubuntu/Debian", false, true, SIGMA_CMD_PACKAGE,
        "Low-level package manager for .deb files"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "snap", "Snap Package Manager",
        "snap [options] [command] [package]",
        "snap install code --classic",
        "Ubuntu", false, true, SIGMA_CMD_PACKAGE,
        "Universal package manager by Canonical"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "add-apt-repository", "Add APT Repository",
        "add-apt-repository [options] [repository]",
        "add-apt-repository ppa:user/ppa-name",
        "Ubuntu", false, true, SIGMA_CMD_PACKAGE,
        "Add PPA repositories to Ubuntu"
    };
    
    // System Administration
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "service", "System V Service Manager",
        "service [service] [start|stop|restart|status]",
        "service apache2 restart",
        "Ubuntu/Debian", false, true, SIGMA_CMD_SYSTEM,
        "Manage system services"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "systemctl", "Systemd Control",
        "systemctl [command] [service]",
        "systemctl enable nginx && systemctl start nginx",
        "Ubuntu", false, true, SIGMA_CMD_SYSTEM,
        "Modern service management tool"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "update-alternatives", "Manage Default Commands",
        "update-alternatives --config [command]",
        "update-alternatives --config editor",
        "Ubuntu/Debian", false, true, SIGMA_CMD_SYSTEM,
        "Set default applications for commands"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "update-grub", "Update GRUB Bootloader",
        "update-grub",
        "update-grub",
        "Ubuntu/Debian", false, true, SIGMA_CMD_SYSTEM,
        "Update GRUB bootloader configuration"
    };
    
    // Network Commands
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "netplan", "Network Configuration",
        "netplan [command]",
        "netplan apply",
        "Ubuntu", false, true, SIGMA_CMD_NETWORK,
        "Modern network configuration tool"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "ufw", "Uncomplicated Firewall",
        "ufw [command] [options]",
        "ufw enable && ufw allow 22/tcp",
        "Ubuntu", false, true, SIGMA_CMD_SECURITY,
        "Easy-to-use firewall frontend"
    };
    
    // User Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "adduser", "Add User (Interactive)",
        "adduser [username]",
        "adduser john",
        "Ubuntu/Debian", false, true, SIGMA_CMD_USER,
        "Interactive user creation tool"
    };
    
    // File Operations
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "locate", "Find Files by Name",
        "locate [filename]",
        "locate '*.conf'",
        "Ubuntu/Debian", false, false, SIGMA_CMD_FILE,
        "Fast file finder using database"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "updatedb", "Update File Database",
        "updatedb",
        "sudo updatedb",
        "Ubuntu/Debian", false, true, SIGMA_CMD_FILE,
        "Update database for locate command"
    };
    
    printf("[Command Library] Loaded %d Ubuntu/Debian commands\n", 
           g_command_library->command_count);
}

// ============================================
// FEDORA/RED HAT COMMANDS
// ============================================
void sigma_load_fedora_commands(void) {
    if (!g_command_library) return;
    
    // Package Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "dnf", "Dandified YUM Package Manager",
        "dnf [options] [command] [package]",
        "dnf update && dnf install package",
        "Fedora/RHEL", false, true, SIGMA_CMD_PACKAGE,
        "Modern package manager for RPM-based systems"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "yum", "Yellowdog Updater Modified",
        "yum [options] [command] [package]",
        "yum install package-name",
        "RHEL/CentOS", false, true, SIGMA_CMD_PACKAGE,
        "Legacy package manager, use dnf on newer systems"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "rpm", "RPM Package Manager",
        "rpm [options] [.rpm file]",
        "rpm -ivh package.rpm",
        "Fedora/RHEL", false, true, SIGMA_CMD_PACKAGE,
        "Low-level package manager for RPM files"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "dnf repoquery", "Query DNF Repositories",
        "dnf repoquery [options] [package]",
        "dnf repoquery -l package-name",
        "Fedora", false, false, SIGMA_CMD_PACKAGE,
        "Query package information in repositories"
    };
    
    // System Administration
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "firewall-cmd", "Firewall Control",
        "firewall-cmd [options]",
        "firewall-cmd --add-service=http --permanent",
        "Fedora/RHEL", false, true, SIGMA_CMD_SECURITY,
        "FirewallD command line client"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "semanage", "SELinux Policy Management",
        "semanage [object] [options]",
        "semanage fcontext -a -t httpd_sys_content_t '/web(/.*)?'",
        "Fedora/RHEL", false, true, SIGMA_CMD_SECURITY,
        "Manage SELinux policy elements"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "setsebool", "Set SELinux Boolean",
        "setsebool [boolean] [on|off]",
        "setsebool httpd_enable_homedirs on",
        "Fedora/RHEL", false, true, SIGMA_CMD_SECURITY,
        "Toggle SELinux booleans"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "grub2-mkconfig", "Generate GRUB2 Config",
        "grub2-mkconfig -o /boot/grub2/grub.cfg",
        "grub2-mkconfig -o /boot/grub2/grub.cfg",
        "Fedora/RHEL", false, true, SIGMA_CMD_SYSTEM,
        "Generate GRUB2 configuration"
    };
    
    // User Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "useradd", "Add User",
        "useradd [options] [username]",
        "useradd -m -s /bin/bash john",
        "Fedora/RHEL", false, true, SIGMA_CMD_USER,
        "Create new user account"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "usermod", "Modify User",
        "usermod [options] [username]",
        "usermod -aG wheel john",
        "Fedora/RHEL", false, true, SIGMA_CMD_USER,
        "Modify existing user account"
    };
    
    // Network
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nmcli", "NetworkManager CLI",
        "nmcli [options] [object] [command]",
        "nmcli device wifi list",
        "Fedora", false, false, SIGMA_CMD_NETWORK,
        "NetworkManager command line interface"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nmtui", "NetworkManager TUI",
        "nmtui",
        "nmtui",
        "Fedora", false, false, SIGMA_CMD_NETWORK,
        "NetworkManager text user interface"
    };
    
    printf("[Command Library] Loaded Fedora/RHEL commands\n");
}

// ============================================
// ARCH LINUX COMMANDS
// ============================================
void sigma_load_arch_commands(void) {
    if (!g_command_library) return;
    
    // Package Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "pacman", "Package Manager",
        "pacman [options] [operation] [package(s)]",
        "pacman -Syu package-name",
        "Arch Linux", false, true, SIGMA_CMD_PACKAGE,
        "Primary package manager for Arch"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "yay", "Yet Another Yogurt (AUR Helper)",
        "yay [options] [operation] [package(s)]",
        "yay -S package-name",
        "Arch Linux", false, false, SIGMA_CMD_PACKAGE,
        "Popular AUR helper written in Go"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "paru", "Feature Packed AUR Helper",
        "paru [options] [operation] [package(s)]",
        "paru -S package-name",
        "Arch Linux", false, false, SIGMA_CMD_PACKAGE,
        "Rust-based AUR helper"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "makepkg", "Make Package",
        "makepkg [options]",
        "makepkg -si",
        "Arch Linux", false, false, SIGMA_CMD_PACKAGE,
        "Build packages from PKGBUILD"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "pacman-key", "Pacman Keyring Manager",
        "pacman-key [options]",
        "pacman-key --init && pacman-key --populate archlinux",
        "Arch Linux", false, true, SIGMA_CMD_PACKAGE,
        "Manage pacman keyring"
    };
    
    // System Administration
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "mkinitcpio", "Make Initial RAM Filesystem",
        "mkinitcpio [options]",
        "mkinitcpio -p linux",
        "Arch Linux", false, true, SIGMA_CMD_SYSTEM,
        "Create initial ramdisk environment"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "arch-chroot", "Arch Chroot",
        "arch-chroot [directory]",
        "arch-chroot /mnt",
        "Arch Linux", false, true, SIGMA_CMD_SYSTEM,
        "Chroot into Arch installation"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "genfstab", "Generate fstab File",
        "genfstab [options] [root]",
        "genfstab -U /mnt >> /mnt/etc/fstab",
        "Arch Linux", false, true, SIGMA_CMD_DISK,
        "Generate fstab file for installation"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "bootctl", "Boot Control",
        "bootctl [command]",
        "bootctl install",
        "Arch Linux", false, true, SIGMA_CMD_SYSTEM,
        "Control systemd-boot"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "timedatectl", "Control System Time",
        "timedatectl [command]",
        "timedatectl set-timezone America/New_York",
        "Arch Linux", false, false, SIGMA_CMD_SYSTEM,
        "Control system time and date"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "hostnamectl", "Control System Hostname",
        "hostnamectl [command]",
        "hostnamectl set-hostname mycomputer",
        "Arch Linux", false, false, SIGMA_CMD_SYSTEM,
        "Control system hostname"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "localectl", "Control System Locale",
        "localectl [command]",
        "localectl set-locale LANG=en_US.UTF-8",
        "Arch Linux", false, false, SIGMA_CMD_SYSTEM,
        "Control system locale and keyboard"
    };
    
    printf("[Command Library] Loaded Arch Linux commands\n");
}

// ============================================
// GENTOO COMMANDS
// ============================================
void sigma_load_gentoo_commands(void) {
    if (!g_command_library) return;
    
    // Package Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "emerge", "Portage Package Manager",
        "emerge [options] [package]",
        "emerge -av package-name",
        "Gentoo", false, true, SIGMA_CMD_PACKAGE,
        "Source-based package manager"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "equery", "Portage Query Tool",
        "equery [module] [options]",
        "equery f package-name",
        "Gentoo", false, false, SIGMA_CMD_PACKAGE,
        "Query Portage package information"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "eselect", "Gentoo Configuration Tool",
        "eselect [module] [command]",
        "eselect kernel list",
        "Gentoo", false, false, SIGMA_CMD_SYSTEM,
        "Manage Gentoo system configuration"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "dispatch-conf", "Manage Config Updates",
        "dispatch-conf",
        "dispatch-conf",
        "Gentoo", false, false, SIGMA_CMD_SYSTEM,
        "Merge configuration file updates"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "etc-update", "Update Config Files",
        "etc-update [options]",
        "etc-update",
        "Gentoo", false, false, SIGMA_CMD_SYSTEM,
        "Update configuration files after emerge"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "revdep-rebuild", "Reverse Dependency Rebuild",
        "revdep-rebuild [options]",
        "revdep-rebuild",
        "Gentoo", false, true, SIGMA_CMD_PACKAGE,
        "Rebuild broken reverse dependencies"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "layman", "Overlay Manager",
        "layman [options]",
        "layman -a repository",
        "Gentoo", false, true, SIGMA_CMD_PACKAGE,
        "Manage Portage overlays"
    };
    
    // System Administration
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "rc-update", "OpenRC Update",
        "rc-update [add|del] [service] [runlevel]",
        "rc-update add sshd default",
        "Gentoo", false, true, SIGMA_CMD_SYSTEM,
        "Add/remove services to runlevels"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "rc-service", "OpenRC Service Control",
        "rc-service [service] [start|stop|restart|status]",
        "rc-service sshd start",
        "Gentoo", false, true, SIGMA_CMD_SYSTEM,
        "Control OpenRC services"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "rc-status", "OpenRC Status",
        "rc-status [runlevel]",
        "rc-status",
        "Gentoo", false, false, SIGMA_CMD_SYSTEM,
        "Show status of OpenRC services"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "eselect news", "Read Gentoo News",
        "eselect news [command]",
        "eselect news read",
        "Gentoo", false, false, SIGMA_CMD_SYSTEM,
        "Read important Gentoo news items"
    };
    
    printf("[Command Library] Loaded Gentoo commands\n");
}

// ============================================
// OPENSUSE COMMANDS
// ============================================
void sigma_load_opensuse_commands(void) {
    if (!g_command_library) return;
    
    // Package Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper", "ZYpp Package Manager",
        "zypper [options] [command] [package]",
        "zypper install package-name",
        "openSUSE", false, true, SIGMA_CMD_PACKAGE,
        "Powerful package manager for openSUSE"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper repos", "List Repositories",
        "zypper repos",
        "zypper repos -u",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "List configured repositories"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper search", "Search Packages",
        "zypper search [package]",
        "zypper search -s package-name",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "Search for packages in repositories"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper info", "Package Information",
        "zypper info [package]",
        "zypper info package-name",
        "openSUSE", false, false, SIGMA_CMD_PACKAGE,
        "Show detailed package information"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "zypper dup", "Distribution Upgrade",
        "zypper dup",
        "zypper dup --allow-vendor-change",
        "openSUSE", false, true, SIGMA_CMD_PACKAGE,
        "Perform distribution upgrade"
    };
    
    // System Administration
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "YaST", "Yet another Setup Tool",
        "yast [module]",
        "yast2",
        "openSUSE", false, true, SIGMA_CMD_SYSTEM,
        "Comprehensive system administration tool"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "YaST GUI", "YaST Graphical",
        "yast2",
        "yast2",
        "openSUSE", false, false, SIGMA_CMD_SYSTEM,
        "Graphical system administration"
    };
    
    // Security
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "susefirewall2", "SuSE Firewall 2",
        "SuSEfirewall2 [command]",
        "SuSEfirewall2 start",
        "openSUSE", false, true, SIGMA_CMD_SECURITY,
        "Traditional SuSE firewall"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "suseconnect", "SUSE Connect",
        "SUSEConnect [options]",
        "SUSEConnect -r registration_code",
        "SUSE", false, true, SIGMA_CMD_SYSTEM,
        "Register SUSE system"
    };
    
    printf("[Command Library] Loaded openSUSE commands\n");
}

// ============================================
// CENTOS/ROCKY/ALMA COMMANDS
// ============================================
void sigma_load_centos_commands(void) {
    if (!g_command_library) return;
    
    // Additional RHEL-specific commands not covered by Fedora
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "subscription-manager", "Red Hat Subscription",
        "subscription-manager [options]",
        "subscription-manager register --username user",
        "RHEL", false, true, SIGMA_CMD_SYSTEM,
        "Manage Red Hat subscriptions"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "rhsm-debug", "Subscription Debug",
        "rhsm-debug [options]",
        "rhsm-debug system",
        "RHEL", false, false, SIGMA_CMD_SYSTEM,
        "Debug subscription issues"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "insights-client", "Red Hat Insights",
        "insights-client [options]",
        "insights-client --register",
        "RHEL", false, true, SIGMA_CMD_SYSTEM,
        "Red Hat Insights client"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "rpm-ostree", "RPM OSTree",
        "rpm-ostree [command]",
        "rpm-ostree upgrade",
        "RHEL CoreOS", false, true, SIGMA_CMD_PACKAGE,
        "Hybrid image/package system"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "toolbox", "Fedora Toolbox",
        "toolbox [command]",
        "toolbox create && toolbox enter",
        "Fedora/RHEL", false, false, SIGMA_CMD_SYSTEM,
        "Containerized development environment"
    };
    
    printf("[Command Library] Loaded CentOS/RHEL commands\n");
}

// ============================================
// ALPINE LINUX COMMANDS
// ============================================
void sigma_load_alpine_commands(void) {
    if (!g_command_library) return;
    
    // Package Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apk", "Alpine Package Keeper",
        "apk [options] [command] [package]",
        "apk add package-name",
        "Alpine", false, true, SIGMA_CMD_PACKAGE,
        "Lightweight package manager for Alpine"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apk search", "Search Alpine Packages",
        "apk search [package]",
        "apk search -v package-name",
        "Alpine", false, false, SIGMA_CMD_PACKAGE,
        "Search Alpine repositories"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apk info", "Package Information",
        "apk info [package]",
        "apk info -a package-name",
        "Alpine", false, false, SIGMA_CMD_PACKAGE,
        "Show package information"
    };
    
    // System Administration
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "rc-service", "OpenRC Service (Alpine)",
        "rc-service [service] [command]",
        "rc-service sshd start",
        "Alpine", false, true, SIGMA_CMD_SYSTEM,
        "Control OpenRC services on Alpine"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "rc-update", "OpenRC Runlevel (Alpine)",
        "rc-update [add|del] [service] [runlevel]",
        "rc-update add sshd",
        "Alpine", false, true, SIGMA_CMD_SYSTEM,
        "Add services to runlevels"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "setup-alpine", "Alpine Setup",
        "setup-alpine",
        "setup-alpine",
        "Alpine", false, true, SIGMA_CMD_SETUP,
        "Initial Alpine setup script"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "lbu", "Local Backup Utility",
        "lbu [command]",
        "lbu commit",
        "Alpine", false, true, SIGMA_CMD_SYSTEM,
        "Backup Alpine configuration"
    };
    
    printf("[Command Library] Loaded Alpine Linux commands\n");
}

// ============================================
// VOID LINUX COMMANDS
// ============================================
void sigma_load_void_commands(void) {
    if (!g_command_library) return;
    
    // Package Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "xbps-install", "XBPS Install",
        "xbps-install [options] [package]",
        "xbps-install -S package-name",
        "Void", false, true, SIGMA_CMD_PACKAGE,
        "Install packages on Void Linux"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "xbps-query", "XBPS Query",
        "xbps-query [options] [package]",
        "xbps-query -Rs package-name",
        "Void", false, false, SIGMA_CMD_PACKAGE,
        "Query XBPS repositories"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "xbps-remove", "XBPS Remove",
        "xbps-remove [options] [package]",
        "xbps-remove -o package-name",
        "Void", false, true, SIGMA_CMD_PACKAGE,
        "Remove packages on Void Linux"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "xbps-reconfigure", "XBPS Reconfigure",
        "xbps-reconfigure [package]",
        "xbps-reconfigure -f package-name",
        "Void", false, true, SIGMA_CMD_PACKAGE,
        "Reconfigure installed packages"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "xbps-src", "XBPS Source Packages",
        "xbps-src [command]",
        "xbps-src pkg package-name",
        "Void", false, false, SIGMA_CMD_PACKAGE,
        "Build packages from source"
    };
    
    // System Administration
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sv", "Runit Service Manager",
        "sv [command] [service]",
        "sv start sshd",
        "Void", false, true, SIGMA_CMD_SYSTEM,
        "Control runit services"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "vsv", "Void Service Viewer",
        "vsv [options]",
        "vsv",
        "Void", false, false, SIGMA_CMD_SYSTEM,
        "Visual service status viewer"
    };
    
    printf("[Command Library] Loaded Void Linux commands\n");
}

// ============================================
// NIXOS COMMANDS
// ============================================
void sigma_load_nixos_commands(void) {
    if (!g_command_library) return;
    
    // Package Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nix-env", "Nix Environment",
        "nix-env [options] [operation]",
        "nix-env -iA nixpkgs.package-name",
        "NixOS", false, false, SIGMA_CMD_PACKAGE,
        "Nix package manager"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nix-shell", "Nix Shell",
        "nix-shell [options] [packages]",
        "nix-shell -p python39 git",
        "NixOS", false, false, SIGMA_CMD_PACKAGE,
        "Spawn shell with packages"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nix-build", "Nix Build",
        "nix-build [options] [derivation]",
        "nix-build '<nixpkgs>' -A package-name",
        "NixOS", false, false, SIGMA_CMD_PACKAGE,
        "Build Nix derivation"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nix-collect-garbage", "Collect Garbage",
        "nix-collect-garbage [options]",
        "nix-collect-garbage -d",
        "NixOS", false, true, SIGMA_CMD_SYSTEM,
        "Remove old generations and free space"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nix-store", "Nix Store",
        "nix-store [command] [options]",
        "nix-store --query --roots",
        "NixOS", false, false, SIGMA_CMD_SYSTEM,
        "Manipulate Nix store"
    };
    
    // System Administration
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nixos-rebuild", "NixOS Rebuild",
        "nixos-rebuild [command]",
        "nixos-rebuild switch",
        "NixOS", false, true, SIGMA_CMD_SYSTEM,
        "Rebuild NixOS configuration"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nixos-generate-config", "Generate NixOS Config",
        "nixos-generate-config",
        "nixos-generate-config --root /mnt",
        "NixOS", false, true, SIGMA_CMD_SYSTEM,
        "Generate hardware configuration"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "nixos-install", "NixOS Install",
        "nixos-install [options]",
        "nixos-install",
        "NixOS", false, true, SIGMA_CMD_SETUP,
        "Install NixOS"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "home-manager", "Home Manager",
        "home-manager [command]",
        "home-manager switch",
        "NixOS", false, false, SIGMA_CMD_SYSTEM,
        "Manage user environment"
    };
    
    printf("[Command Library] Loaded NixOS commands\n");
}

// ============================================
// DEBIAN-SPECIFIC COMMANDS
// ============================================
void sigma_load_debian_commands(void) {
    if (!g_command_library) return;
    
    // Additional Debian-specific commands
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "aptitude", "Advanced Package Manager",
        "aptitude [options] [command]",
        "aptitude search package-name",
        "Debian", false, false, SIGMA_CMD_PACKAGE,
        "Advanced package manager with TUI"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "tasksel", "Task Selector",
        "tasksel [options]",
        "tasksel install desktop",
        "Debian", false, true, SIGMA_CMD_PACKAGE,
        "Install groups of related packages"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "debconf", "Debian Configuration",
        "debconf [options]",
        "debconf-show package-name",
        "Debian", false, false, SIGMA_CMD_PACKAGE,
        "View and change package configuration"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apt-file", "APT File Search",
        "apt-file [command]",
        "apt-file search filename",
        "Debian", false, false, SIGMA_CMD_PACKAGE,
        "Search for files in packages"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apt-mark", "APT Mark",
        "apt-mark [command] [package]",
        "apt-mark auto package-name",
        "Debian", false, true, SIGMA_CMD_PACKAGE,
        "Set package markings"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "apt-cache", "APT Cache Query",
        "apt-cache [command] [package]",
        "apt-cache search package-name",
        "Debian", false, false, SIGMA_CMD_PACKAGE,
        "Query APT cache"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "update-rc.d", "Update RC Scripts",
        "update-rc.d [service] [defaults|remove]",
        "update-rc.d sshd defaults",
        "Debian", false, true, SIGMA_CMD_SYSTEM,
        "Manage System V init scripts"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "invoke-rc.d", "Invoke RC Script",
        "invoke-rc.d [service] [command]",
        "invoke-rc.d sshd restart",
        "Debian", false, true, SIGMA_CMD_SYSTEM,
        "Execute init script with policy"
    };
    
    printf("[Command Library] Loaded Debian-specific commands\n");
}

// Continue with custom SigmaOS commands...

