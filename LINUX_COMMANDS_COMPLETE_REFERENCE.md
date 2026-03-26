# SigmaOS Complete Linux Commands Reference

## Comprehensive Documentation of All Linux/Unix Commands in SigmaOS

This document contains complete documentation of all Linux/Unix commands available in SigmaOS, organized by category with usage examples.

---

## Table of Contents

1. [Package Management Commands](#1-package-management-commands)
2. [System Administration Commands](#2-system-administration-commands)
3. [Network Commands](#3-network-commands)
4. [Process Management Commands](#4-process-management-commands)
5. [User Management Commands](#5-user-management-commands)
6. [Service Management Commands](#6-service-management-commands)
7. [Disk Management Commands](#7-disk-management-commands)
8. [Hardware Information Commands](#8-hardware-information-commands)
9. [Monitoring Commands](#9-monitoring-commands)
10. [Security Commands](#10-security-commands)

---

## 1. Package Management Commands

SigmaOS supports package management from all major Linux distributions.

### 1.1 APT (Debian/Ubuntu)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_apt_update` | Update package lists | `sigma_apt_update()` |
| `sigma_apt_upgrade` | Upgrade packages | `sigma_apt_upgrade(full_upgrade=true)` |
| `sigma_apt_install` | Install a package | `sigma_apt_install(package_name="nginx", auto_confirm=true)` |
| `sigma_apt_remove` | Remove a package | `sigma_apt_remove(package_name="nginx", purge=true)` |
| `sigma_apt_autoremove` | Remove unused packages | `sigma_apt_autoremove()` |
| `sigma_apt_search` | Search for packages | `sigma_apt_search(keyword="web")` |
| `sigma_apt_show` | Show package details | `sigma_apt_show(package_name="nginx")` |
| `sigma_apt_list` | List installed packages | `sigma_apt_list(installed_only=true)` |
| `sigma_apt_cache_search` | Search package cache | `sigma_apt_cache_search(pattern="python")` |
| `sigma_apt_add_repository` | Add PPA/repository | `sigma_apt_add_repository(repository="ppa:nginx/stable")` |
| `sigma_apt_key_add` | Add GPG key | `sigma_apt_key_add(key_url="https://key.url")` |
| `sigma_apt_download` | Download package | `sigma_apt_download(package_name="nginx", destination="/tmp")` |
| `sigma_apt_source` | Download source | `sigma_apt_source(package_name="nginx")` |
| `sigma_apt_build_dep` | Install build deps | `sigma_apt_build_dep(package_name="nginx")` |
| `sigma_apt_mark_auto` | Mark auto-installed | `sigma_apt_mark_auto(package_name="libssl")` |
| `sigma_apt_hold` | Hold package | `sigma_apt_hold(package_name="nginx")` |

**Example Workflow:**
```c
// Update and upgrade system
sigma_apt_update();
sigma_apt_upgrade(full_upgrade=true);

// Install nginx
sigma_apt_install(package_name="nginx", auto_confirm=true);

// Search for python packages
SigmaAptPackage* results;
uint32_t n_results;
sigma_apt_search("python3", &results, &n_results);

// Remove unused packages
sigma_apt_autoremove();
```

### 1.2 DPKG (Debian Package)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_dpkg_install` | Install .deb package | `sigma_dpkg_install(deb_file="package.deb")` |
| `sigma_dpkg_remove` | Remove package | `sigma_dpkg_remove(package_name="nginx")` |
| `sigma_dpkg_purge` | Purge package | `sigma_dpkg_purge(package_name="nginx")` |
| `sigma_dpkg_list` | List packages | `sigma_dpkg_list(packages, &n_packages)` |
| `sigma_dpkg_info` | Package info | `sigma_dpkg_info(package_name="nginx", &info)` |
| `sigma_dpkg_search` | Search for file | `sigma_dpkg_search(file_path="/usr/bin/nginx", &package_name)` |
| `sigma_dpkg_contents` | List package contents | `sigma_dpkg_contents(deb_file="package.deb", &files, &n_files)` |
| `sigma_dpkg_extract` | Extract package | `sigma_dpkg_extract(deb_file="package.deb", destination="/tmp")` |
| `sigma_dpkg_configure` | Configure package | `sigma_dpkg_configure(package_name="nginx")` |
| `sigma_dpkg_divert` | Divert file | `sigma_dpkg_divert(file_path="/bin/ls", divert_to="/bin/ls.distrib")` |

### 1.3 SNAP (Ubuntu Snap)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_snap_install` | Install snap | `sigma_snap_install(snap_name="code", classic=true)` |
| `sigma_snap_remove` | Remove snap | `sigma_snap_remove(snap_name="code")` |
| `sigma_snap_refresh` | Update snap | `sigma_snap_refresh(snap_name="code")` |
| `sigma_snap_revert` | Revert snap | `sigma_snap_revert(snap_name="code")` |
| `sigma_snap_list` | List snaps | `sigma_snap_list(snaps, &n_snaps)` |
| `sigma_snap_find` | Search snaps | `sigma_snap_find(keyword="editor", results, &n_results)` |
| `sigma_snap_info` | Snap info | `sigma_snap_info(snap_name="code", &info)` |
| `sigma_snap_switch_channel` | Change channel | `sigma_snap_switch_channel(snap_name="code", channel="beta")` |
| `sigma_snap_enable` | Enable snap | `sigma_snap_enable(snap_name="code")` |
| `sigma_snap_disable` | Disable snap | `sigma_snap_disable(snap_name="code")` |
| `sigma_snap_alias` | Create alias | `sigma_snap_alias(snap_name="code", app_name="bin/code", alias="code")` |
| `sigma_snap_services` | List services | `sigma_snap_services(snap_name="lxd", &services, &n_services)` |
| `sigma_snap_start` | Start service | `sigma_snap_start(snap_name="lxd", service="daemon")` |
| `sigma_snap_stop` | Stop service | `sigma_snap_stop(snap_name="lxd", service="daemon")` |

### 1.4 DNF (Fedora/RHEL/CentOS)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_dnf_check_update` | Check for updates | `sigma_dnf_check_update()` |
| `sigma_dnf_upgrade` | Upgrade packages | `sigma_dnf_upgrade(security_only=true)` |
| `sigma_dnf_install` | Install package | `sigma_dnf_install(package_name="nginx", auto_confirm=true)` |
| `sigma_dnf_remove` | Remove package | `sigma_dnf_remove(package_name="nginx")` |
| `sigma_dnf_autoremove` | Remove unused | `sigma_dnf_autoremove()` |
| `sigma_dnf_search` | Search packages | `sigma_dnf_search(keyword="web", results, &n_results)` |
| `sigma_dnf_info` | Package info | `sigma_dnf_info(package_name="nginx", &info)` |
| `sigma_dnf_list` | List packages | `sigma_dnf_list(installed_only=true, packages, &n_packages)` |
| `sigma_dnf_provides` | Find provider | `sigma_dnf_provides(file_or_capability="/usr/bin/nginx")` |
| `sigma_dnf_whatprovides` | What provides | `sigma_dnf_whatprovides(capability="libssl.so")` |
| `sigma_dnf_repoquery` | Query repos | `sigma_dnf_repoquery(query="nginx")` |
| `sigma_dnf_repository_enable` | Enable repo | `sigma_dnf_repository_enable(repo_id="epel")` |
| `sigma_dnf_repository_disable` | Disable repo | `sigma_dnf_repository_disable(repo_id="epel")` |
| `sigma_dnf_module_list` | List modules | `sigma_dnf_module_list()` |
| `sigma_dnf_module_enable` | Enable module | `sigma_dnf_module_enable(module_name="nodejs")` |
| `sigma_dnf_module_install` | Install module | `sigma_dnf_module_install(module_name="nodejs", stream="14")` |
| `sigma_dnf_history_list` | Show history | `sigma_dnf_history_list()` |
| `sigma_dnf_history_undo` | Undo transaction | `sigma_dnf_history_undo(transaction_id=10)` |
| `sigma_dnf_downgrade` | Downgrade | `sigma_dnf_downgrade(package_name="nginx")` |
| `sigma_dnf_group_install` | Install group | `sigma_dnf_group_install(group_name="Development Tools")` |

### 1.5 RPM (Red Hat Package Manager)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_rpm_install` | Install RPM | `sigma_rpm_install(rpm_file="package.rpm")` |
| `sigma_rpm_remove` | Remove package | `sigma_rpm_remove(package_name="nginx")` |
| `sigma_rpm_query` | Query package | `sigma_rpm_query(package_name="nginx", &info)` |
| `sigma_rpm_query_all` | List all | `sigma_rpm_query_all(packages, &n_packages)` |
| `sigma_rpm_query_file` | Find owner | `sigma_rpm_query_file(file_path="/usr/bin/nginx", &package_name)` |
| `sigma_rpm_verify` | Verify package | `sigma_rpm_verify(package_name="nginx")` |
| `sigma_rpm_checksig` | Check signature | `sigma_rpm_checksig(rpm_file="package.rpm")` |
| `sigma_rpm_import_key` | Import key | `sigma_rpm_import_key(key_file="key.gpg")` |
| `sigma_rpm_extract` | Extract RPM | `sigma_rpm_extract(rpm_file="package.rpm", destination="/tmp")` |
| `sigma_rpm_build` | Build RPM | `sigma_rpm_build(spec_file="package.spec", source_dir="sources/")` |

### 1.6 PACMAN (Arch Linux)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_pacman_sync` | Sync databases | `sigma_pacman_sync()` |
| `sigma_pacman_upgrade` | Upgrade system | `sigma_pacman_upgrade(force_refresh=true)` |
| `sigma_pacman_install` | Install package | `sigma_pacman_install(package_name="nginx", no_confirm=true)` |
| `sigma_pacman_remove` | Remove package | `sigma_pacman_remove(package_name="nginx", cascade=true, recursive=true)` |
| `sigma_pacman_search` | Search packages | `sigma_pacman_search(keyword="web", results, &n_results)` |
| `sigma_pacman_query` | Query package | `sigma_pacman_query(package_name="nginx", &info)` |
| `sigma_pacman_query_all` | List all | `sigma_pacman_query_all(packages, &n_packages)` |
| `sigma_pacman_query_explicit` | List explicit | `sigma_pacman_query_explicit(packages, &n_packages)` |
| `sigma_pacman_query_foreign` | List AUR | `sigma_pacman_query_foreign(packages, &n_packages)` |
| `sigma_pacman_query_unrequired` | List orphans | `sigma_pacman_query_unrequired(packages, &n_packages)` |
| `sigma_pacman_query_upgrades` | List upgrades | `sigma_pacman_query_upgrades(packages, &n_packages)` |
| `sigma_pacman_database` | Database ops | `sigma_pacman_database(check=true, update=true)` |
| `sigma_pacman_files` | List files | `sigma_pacman_files(package_name="nginx", &files, &n_files)` |
| `sigma_pacman_owns` | Find owner | `sigma_pacman_owns(file_path="/usr/bin/nginx", &package_name)` |
| `sigma_pacman_group_list` | List group | `sigma_pacman_group_list(group_name="base-devel", packages, &n_packages)` |
| `sigma_pacman_deptest` | Test deps | `sigma_pacman_deptest(packages, n_packages)` |
| `sigma_pacman_clean` | Clean cache | `sigma_pacman_clean()` |
| `sigma_pacman_key_init` | Init keyring | `sigma_pacman_key_init()` |
| `sigma_pacman_log` | View log | `sigma_pacman_log()` |

### 1.7 YAY (AUR Helper)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_yay_search` | Search AUR | `sigma_yay_search(keyword="visual-studio-code", results, &n_results)` |
| `sigma_yay_info` | AUR info | `sigma_yay_info(package_name="visual-studio-code", &info)` |
| `sigma_yay_clone` | Clone PKGBUILD | `sigma_yay_clone(package_name="visual-studio-code", destination="/tmp")` |
| `sigma_yay_build` | Build package | `sigma_yay_build(pkgbuild_dir="/tmp/visual-studio-code")` |
| `sigma_yay_install_aur` | Install from AUR | `sigma_yay_install_aur(package_name="visual-studio-code")` |
| `sigma_yay_update_devel` | Update devel | `sigma_yay_update_devel(rebuild=true)` |
| `sigma_yay_clean` | Clean build | `sigma_yay_clean()` |
| `sigma_yay_diff` | Show diffs | `sigma_yay_diff()` |
| `sigma_yay_edit` | Edit PKGBUILD | `sigma_yay_edit(package_name="visual-studio-code")` |
| `sigma_yay_merge` | Merge changes | `sigma_yay_merge(package_name="visual-studio-code")` |

### 1.8 ZYPPER (openSUSE)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_zypper_refresh` | Refresh repos | `sigma_zypper_refresh()` |
| `sigma_zypper_update` | Update packages | `sigma_zypper_update(full=true)` |
| `sigma_zypper_install` | Install package | `sigma_zypper_install(package_name="nginx", auto_agree=true)` |
| `sigma_zypper_remove` | Remove package | `sigma_zypper_remove(package_name="nginx")` |
| `sigma_zypper_search` | Search packages | `sigma_zypper_search(keyword="web", results, &n_results)` |
| `sigma_zypper_info` | Package info | `sigma_zypper_info(package_name="nginx", &info)` |
| `sigma_zypper_patches` | List patches | `sigma_zypper_patches()` |
| `sigma_zypper_patch` | Apply patches | `sigma_zypper_patch()` |
| `sigma_zypper_dist_upgrade` | Distribution upgrade | `sigma_zypper_dist_upgrade()` |
| `sigma_zypper_verify` | Verify deps | `sigma_zypper_verify()` |
| `sigma_zypper_source_install` | Install source | `sigma_zypper_source_install(package_name="nginx")` |
| `sigma_zypper_addrepo` | Add repo | `sigma_zypper_addrepo(name="nginx", url="https://nginx.org")` |
| `sigma_zypper_removerepo` | Remove repo | `sigma_zypper_removerepo(name="nginx")` |
| `sigma_zypper_renamerepo` | Rename repo | `sigma_zypper_renamerepo(old_name="nginx", new_name="nginx-mainline")` |
| `sigma_zypper_modifyrepo` | Modify repo | `sigma_zypper_modifyrepo(name="nginx", enable=true, refresh=true, gpgcheck=true)` |
| `sigma_zypper_lock` | Lock package | `sigma_zypper_lock(package_name="nginx")` |
| `sigma_zypper_unlock` | Unlock package | `sigma_zypper_unlock(package_name="nginx")` |

### 1.9 APK (Alpine Linux)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_apk_update` | Update index | `sigma_apk_update()` |
| `sigma_apk_upgrade` | Upgrade packages | `sigma_apk_upgrade(available=true)` |
| `sigma_apk_add` | Add package | `sigma_apk_add(package_name="nginx")` |
| `sigma_apk_del` | Delete package | `sigma_apk_del(package_name="nginx")` |
| `sigma_apk_search` | Search packages | `sigma_apk_search(keyword="web", results, &n_results)` |
| `sigma_apk_info` | Package info | `sigma_apk_info(package_name="nginx", &info)` |
| `sigma_apk_list` | List packages | `sigma_apk_list(packages, &n_packages)` |
| `sigma_apk_policy` | Show policy | `sigma_apk_policy(package_name="nginx")` |
| `sigma_apk_version` | Show versions | `sigma_apk_version(package_name="nginx")` |
| `sigma_apk_index` | Update index | `sigma_apk_index(repository_url="https://dl-cdn.alpinelinux.org")` |
| `sigma_apk_fetch` | Fetch package | `sigma_apk_fetch(package_name="nginx", destination="/tmp")` |
| `sigma_apk_cache_clean` | Clean cache | `sigma_apk_cache_clean()` |
| `sigma_apk_cache_download` | Download cache | `sigma_apk_cache_download()` |
| `sigma_apk_fix` | Fix packages | `sigma_apk_fix()` |
| `sigma_apk_audit` | Audit system | `sigma_apk_audit()` |
| `sigma_apk_verify` | Verify package | `sigma_apk_verify(package_name="nginx")` |

### 1.10 EMERGE (Gentoo)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_emerge_sync` | Sync Portage | `sigma_emerge_sync()` |
| `sigma_emerge_update` | Update world | `sigma_emerge_update(packages, n_packages, deep=true, newuse=true)` |
| `sigma_emerge_install` | Install package | `sigma_emerge_install(package_name="www-servers/nginx", auto_unmask=true)` |
| `sigma_emerge_unmerge` | Unmerge package | `sigma_emerge_unmerge(package_name="www-servers/nginx", selective=true)` |
| `sigma_emerge_search` | Search packages | `sigma_emerge_search(keyword="nginx", results, &n_results)` |
| `sigma_emerge_info` | Package info | `sigma_emerge_info(package_name="www-servers/nginx", &info)` |
| `sigma_emerge_pretend` | Pretend install | `sigma_emerge_pretend(package_name="www-servers/nginx")` |
| `sigma_emerge_fetch` | Fetch only | `sigma_emerge_fetch(package_name="www-servers/nginx")` |
| `sigma_emerge_clean` | Clean system | `sigma_emerge_clean(distfiles=true, packages=true)` |
| `sigma_emerge_depclean` | Clean deps | `sigma_emerge_depclean()` |
| `sigma_emerge_world` | Update world | `sigma_emerge_world()` |
| `sigma_emerge_system` | Update system | `sigma_emerge_system()` |
| `sigma_emerge_emptytree` | Empty tree | `sigma_emerge_emptytree()` |
| `sigma_emerge_oneshot` | One-shot | `sigma_emerge_oneshot(package_name="www-servers/nginx")` |
| `sigma_emerge_onlydeps` | Only deps | `sigma_emerge_onlydeps(package_name="www-servers/nginx")` |
| `sigma_emerge_usepkg` | Use binary | `sigma_emerge_usepkg(package_name="www-servers/nginx")` |
| `sigma_emerge_buildpkg` | Build binary | `sigma_emerge_buildpkg(package_name="www-servers/nginx")` |
| `sigma_emerge_reinstall` | Reinstall | `sigma_emerge_reinstall(package_name="www-servers/nginx")` |

### 1.11 NIX (NixOS)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_nix_search` | Search packages | `sigma_nix_search(keyword="nginx", results, &n_results)` |
| `sigma_nix_install` | Install package | `sigma_nix_install(package_name="nginx")` |
| `sigma_nix_uninstall` | Uninstall | `sigma_nix_uninstall(package_name="nginx")` |
| `sigma_nix_upgrade` | Upgrade package | `sigma_nix_upgrade(package_name="nginx")` |
| `sigma_nix_upgrade_all` | Upgrade all | `sigma_nix_upgrade_all()` |
| `sigma_nix_info` | Package info | `sigma_nix_info(package_name="nginx", &info)` |
| `sigma_nix_list` | List packages | `sigma_nix_list(packages, &n_packages)` |
| `sigma_nix_collect_garbage` | Garbage collect | `sigma_nix_collect_garbage()` |
| `sigma_nix_store_optimise` | Optimise store | `sigma_nix_store_optimise()` |
| `sigma_nix_build` | Build | `sigma_nix_build(nix_file="default.nix")` |
| `sigma_nix_shell` | Enter shell | `sigma_nix_shell(package_name="nginx")` |
| `sigma_nix_run` | Run package | `sigma_nix_run(package_name="nginx")` |
| `sigma_nix_env_install` | Env install | `sigma_nix_env_install(nix_file="shell.nix")` |
| `sigma_nix_env_list` | Env list | `sigma_nix_env_list()` |
| `sigma_nix_channel_add` | Add channel | `sigma_nix_channel_add(name="nixpkgs", url="https://nixos.org/channels/nixpkgs-unstable")` |
| `sigma_nix_channel_update` | Update channel | `sigma_nix_channel_update(name="nixpkgs")` |
| `sigma_nix_flake_init` | Init flake | `sigma_nix_flake_init(template_name="templates#rust")` |
| `sigma_nix_flake_lock` | Lock flake | `sigma_nix_flake_lock()` |
| `sigma_nix_flake_update` | Update flake | `sigma_nix_flake_update()` |
| `sigma_nix_develop` | Develop | `sigma_nix_develop(nix_file="flake.nix")` |

### 1.12 FLATPAK

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_flatpak_install` | Install app | `sigma_flatpak_install(remote="flathub", app_name="com.visualstudio.code")` |
| `sigma_flatpak_uninstall` | Uninstall | `sigma_flatpak_uninstall(app_name="com.visualstudio.code")` |
| `sigma_flatpak_update` | Update app | `sigma_flatpak_update(app_name="com.visualstudio.code")` |
| `sigma_flatpak_update_all` | Update all | `sigma_flatpak_update_all()` |
| `sigma_flatpak_list` | List apps | `sigma_flatpak_list(apps, &n_apps)` |
| `sigma_flatpak_search` | Search apps | `sigma_flatpak_search(keyword="code", results, &n_results)` |
| `sigma_flatpak_info` | App info | `sigma_flatpak_info(app_name="com.visualstudio.code", &info)` |
| `sigma_flatpak_run` | Run app | `sigma_flatpak_run(app_name="com.visualstudio.code")` |
| `sigma_flatpak_override` | Override | `sigma_flatpak_override(app_name="com.visualstudio.code")` |
| `sigma_flatpak_remote_add` | Add remote | `sigma_flatpak_remote_add(name="flathub", url="https://flathub.org/repo/flathub.flatpakrepo")` |
| `sigma_flatpak_remote_delete` | Delete remote | `sigma_flatpak_remote_delete(name="flathub")` |
| `sigma_flatpak_remote_list` | List remotes | `sigma_flatpak_remote_list()` |
| `sigma_flatpak_build` | Build | `sigma_flatpak_build(directory="build-dir")` |
| `sigma_flatpak_build_init` | Init build | `sigma_flatpak_build_init(directory="build-dir", app_name="com.app.Name")` |
| `sigma_flatpak_build_finish` | Finish build | `sigma_flatpak_build_finish(directory="build-dir")` |
| `sigma_flatpak_build_export` | Export | `sigma_flatpak_build_export(repository="repo", directory="build-dir")` |

---

## 2. System Administration Commands

### 2.1 User and Group Management

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_useradd` | Add user | `sigma_useradd(username="john", uid=1001, gid=1001, home="/home/john", shell="/bin/bash", comment="John Doe")` |
| `sigma_userdel` | Delete user | `sigma_userdel(username="john", remove_home=true)` |
| `sigma_usermod` | Modify user | `sigma_usermod(username="john", option="shell", value="/bin/zsh")` |
| `sigma_passwd` | Change password | `sigma_passwd(username="john")` |
| `sigma_passwd_lock` | Lock account | `sigma_passwd_lock(username="john")` |
| `sigma_passwd_unlock` | Unlock account | `sigma_passwd_unlock(username="john")` |
| `sigma_chage` | Password aging | `sigma_chage(username="john", option="M", value="30")` |
| `sigma_chfn` | Change finger info | `sigma_chfn(username="john", full_name="John Doe", room="101", work_phone="123-456", home_phone="789-012")` |
| `sigma_chsh` | Change shell | `sigma_chsh(username="john", shell="/bin/zsh")` |
| `sigma_finger` | User info | `sigma_finger(username="john")` |
| `sigma_last` | Login history | `sigma_last()` |
| `sigma_lastlog` | Last login | `sigma_lastlog()` |
| `sigma_who` | Who is logged in | `sigma_who()` |
| `sigma_w` | Who and what | `sigma_w()` |
| `sigma_users` | List users | `sigma_users()` |
| `sigma_groups` | List groups | `sigma_groups(username="john")` |
| `sigma_id` | Show ID | `sigma_id(username="john")` |
| `sigma_su` | Switch user | `sigma_su(username="root")` |
| `sigma_sudo` | Execute as root | `sigma_sudo(command="apt update")` |
| `sigma_visudo` | Edit sudoers | `sigma_visudo()` |
| `sigma_groupadd` | Add group | `sigma_groupadd(groupname="developers", gid=1001)` |
| `sigma_groupdel` | Delete group | `sigma_groupdel(groupname="developers")` |
| `sigma_groupmod` | Modify group | `sigma_groupmod(groupname="developers", option="gid", value=1002)` |
| `sigma_gpasswd` | Administer group | `sigma_gpasswd(groupname="developers")` |
| `sigma_gpasswd_a` | Add user to group | `sigma_gpasswd_a(username="john", groupname="developers")` |
| `sigma_gpasswd_d` | Remove from group | `sigma_gpasswd_d(username="john", groupname="developers")` |
| `sigma_grpck` | Verify groups | `sigma_grpck()` |
| `sigma_pwck` | Verify passwd | `sigma_pwck()` |

**Example Workflow:**
```c
// Add a new user
sigma_useradd("john", 1001, 1001, "/home/john", "/bin/bash", "John Doe");

// Set password
sigma_passwd("john");

// Add user to sudo group
sigma_gpasswd_a("john", "sudo");

// Verify
sigma_groups("john");
```

### 2.2 Process Management

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_ps` | List processes | `sigma_ps()` |
| `sigma_ps_a` | All processes | `sigma_ps_a()` |
| `sigma_ps_u` | User processes | `sigma_ps_u(username="john")` |
| `sigma_ps_x` | All processes | `sigma_ps_x()` |
| `sigma_ps_aux` | Detailed list | `sigma_ps_aux()` |
| `sigma_ps_e` | All processes | `sigma_ps_e()` |
| `sigma_ps_f` | Full format | `sigma_ps_f()` |
| `sigma_ps_l` | Long format | `sigma_ps_l()` |
| `sigma_ps_tree` | Tree view | `sigma_ps_tree()` |
| `sigma_pgrep` | Find process | `sigma_pgrep(pattern="nginx")` |
| `sigma_pgrep_u` | By user | `sigma_pgrep_u(username="john", pattern="bash")` |
| `sigma_pkill` | Kill by name | `sigma_pkill(pattern="nginx")` |
| `sigma_pkill_signal` | Send signal | `sigma_pkill_signal(pattern="nginx", signal=9)` |
| `sigma_pidof` | Find PID | `sigma_pidof(process_name="nginx")` |
| `sigma_kill` | Kill process | `sigma_kill(pid=1234, signal=9)` |
| `sigma_killall` | Kill by name | `sigma_killall(process_name="nginx")` |
| `sigma_nice` | Run with nice | `sigma_nice(increment=-10, command="./app")` |
| `sigma_renice` | Change priority | `sigma_renice(priority=10, pid=1234)` |
| `sigma_chrt` | Set scheduler | `sigma_chrt(policy="FIFO", priority=99, command="./realtime-app")` |
| `sigma_taskset` | CPU affinity | `sigma_taskset(cpu_mask=0x3, command="./app")` |
| `sigma_numactl` | NUMA control | `sigma_numactl(policy="interleave", command="./app")` |
| `sigma_nohup` | Immune to hangup | `sigma_nohup(command="./long-running-app")` |
| `sigma_disown` | Disown job | `sigma_disown(job_id=1)` |

### 2.3 Service Management (systemd)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_systemctl_start` | Start service | `sigma_systemctl_start(service="nginx")` |
| `sigma_systemctl_stop` | Stop service | `sigma_systemctl_stop(service="nginx")` |
| `sigma_systemctl_restart` | Restart | `sigma_systemctl_restart(service="nginx")` |
| `sigma_systemctl_reload` | Reload | `sigma_systemctl_reload(service="nginx")` |
| `sigma_systemctl_reload_or_restart` | Reload or restart | `sigma_systemctl_reload_or_restart(service="nginx")` |
| `sigma_systemctl_try_restart` | Try restart | `sigma_systemctl_try_restart(service="nginx")` |
| `sigma_systemctl_status` | Status | `sigma_systemctl_status(service="nginx", &info)` |
| `sigma_systemctl_is_active` | Check active | `sigma_systemctl_is_active(service="nginx")` |
| `sigma_systemctl_is_enabled` | Check enabled | `sigma_systemctl_is_enabled(service="nginx")` |
| `sigma_systemctl_is_failed` | Check failed | `sigma_systemctl_is_failed(service="nginx")` |
| `sigma_systemctl_enable` | Enable service | `sigma_systemctl_enable(service="nginx")` |
| `sigma_systemctl_enable_now` | Enable and start | `sigma_systemctl_enable_now(service="nginx")` |
| `sigma_systemctl_disable` | Disable | `sigma_systemctl_disable(service="nginx")` |
| `sigma_systemctl_disable_now` | Disable and stop | `sigma_systemctl_disable_now(service="nginx")` |
| `sigma_systemctl_mask` | Mask service | `sigma_systemctl_mask(service="nginx")` |
| `sigma_systemctl_unmask` | Unmask | `sigma_systemctl_unmask(service="nginx")` |
| `sigma_systemctl_list_units` | List units | `sigma_systemctl_list_units(services, &n_services)` |
| `sigma_systemctl_list_unit_files` | List files | `sigma_systemctl_list_unit_files(services, &n_services)` |
| `sigma_systemctl_list_failed` | List failed | `sigma_systemctl_list_failed(services, &n_services)` |
| `sigma_systemctl_list_dependencies` | List deps | `sigma_systemctl_list_dependencies(service="nginx")` |
| `sigma_systemctl_show` | Show config | `sigma_systemctl_show(service="nginx")` |
| `sigma_systemctl_daemon_reload` | Reload daemon | `sigma_systemctl_daemon_reload()` |
| `sigma_systemctl_get_default` | Get default target | `sigma_systemctl_get_default()` |
| `sigma_systemctl_set_default` | Set default target | `sigma_systemctl_set_default(target="graphical.target")` |
| `sigma_systemctl_isolate` | Isolate target | `sigma_systemctl_isolate(target="rescue.target")` |
| `sigma_systemctl_reboot` | Reboot | `sigma_systemctl_reboot()` |
| `sigma_systemctl_poweroff` | Power off | `sigma_systemctl_poweroff()` |

### 2.4 Disk Management

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_df` | Disk free | `sigma_df()` |
| `sigma_df_h` | Human readable | `sigma_df_h()` |
| `sigma_df_i` | Inodes | `sigma_df_i()` |
| `sigma_du` | Disk usage | `sigma_du(path="/var/log")` |
| `sigma_du_h` | Human readable | `sigma_du_h(path="/var/log")` |
| `sigma_du_s` | Summary | `sigma_du_s(path="/var/log")` |
| `sigma_fdisk_l` | List partitions | `sigma_fdisk_l()` |
| `sigma_fdisk_create_partition` | Create partition | `sigma_fdisk_create_partition(device="/dev/sdb")` |
| `sigma_fdisk_delete_partition` | Delete partition | `sigma_fdisk_delete_partition(device="/dev/sdb", partition=1)` |
| `sigma_parted` | Partition editor | `sigma_parted(device="/dev/sdb", command="print")` |
| `sigma_parted_mklabel` | Create label | `sigma_parted_mklabel(device="/dev/sdb", label_type="gpt")` |
| `sigma_parted_mkpart` | Create partition | `sigma_parted_mkpart(device="/dev/sdb", part_type="primary", fs_type="ext4", start="0%", end="100%")` |
| `sigma_mkfs_ext4` | Make ext4 fs | `sigma_mkfs_ext4(device="/dev/sdb1")` |
| `sigma_mkfs_xfs` | Make XFS fs | `sigma_mkfs_xfs(device="/dev/sdb1")` |
| `sigma_mkfs_btrfs` | Make btrfs fs | `sigma_mkfs_btrfs(device="/dev/sdb1")` |
| `sigma_fsck` | Check filesystem | `sigma_fsck(filesystem="/dev/sdb1")` |
| `sigma_mount` | Mount | `sigma_mount(device="/dev/sdb1", mount_point="/mnt")` |
| `sigma_umount` | Unmount | `sigma_umount(mount_point="/mnt")` |
| `sigma_swapon` | Enable swap | `sigma_swapon(device="/dev/sdb2")` |
| `sigma_swapoff` | Disable swap | `sigma_swapoff(device="/dev/sdb2")` |
| `sigma_lvextend` | Extend LV | `sigma_lvextend(lv_name="vg0/root", size="+10G")` |

---

## 3. Network Commands

### 3.1 IP Command (Modern)

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_ip_link_show` | Show interfaces | `sigma_ip_link_show()` |
| `sigma_ip_link_set_up` | Bring up | `sigma_ip_link_set_up(interface="eth0")` |
| `sigma_ip_link_set_down` | Bring down | `sigma_ip_link_set_down(interface="eth0")` |
| `sigma_ip_link_set_mtu` | Set MTU | `sigma_ip_link_set_mtu(interface="eth0", mtu=9000)` |
| `sigma_ip_addr_show` | Show addresses | `sigma_ip_addr_show()` |
| `sigma_ip_addr_add` | Add address | `sigma_ip_addr_add(address="192.168.1.10/24", interface="eth0")` |
| `sigma_ip_addr_del` | Delete address | `sigma_ip_addr_del(address="192.168.1.10/24", interface="eth0")` |
| `sigma_ip_route_show` | Show routes | `sigma_ip_route_show()` |
| `sigma_ip_route_add` | Add route | `sigma_ip_route_add(destination="10.0.0.0/8", gateway="192.168.1.1")` |
| `sigma_ip_route_del` | Delete route | `sigma_ip_route_del(destination="10.0.0.0/8")` |
| `sigma_ip_neigh_show` | Show neighbors | `sigma_ip_neigh_show()` |
| `sigma_ip_neigh_add` | Add neighbor | `sigma_ip_neigh_add(address="192.168.1.1", mac="00:11:22:33:44:55", interface="eth0")` |
| `sigma_ip_netns_list` | List namespaces | `sigma_ip_netns_list()` |
| `sigma_ip_netns_add` | Add namespace | `sigma_ip_netns_add(name="netns1")` |

### 3.2 DNS Tools

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_dig` | DNS lookup | `sigma_dig(domain="example.com")` |
| `sigma_dig_a` | A records | `sigma_dig_a(domain="example.com")` |
| `sigma_dig_mx` | MX records | `sigma_dig_mx(domain="example.com")` |
| `sigma_dig_ns` | NS records | `sigma_dig_ns(domain="example.com")` |
| `sigma_dig_txt` | TXT records | `sigma_dig_txt(domain="example.com")` |
| `sigma_nslookup` | DNS query | `sigma_nslookup(domain="example.com")` |
| `sigma_host` | DNS lookup | `sigma_host(domain="example.com")` |
| `sigma_whois` | WHOIS lookup | `sigma_whois(domain="example.com")` |

### 3.3 Connectivity Testing

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_ping` | Ping host | `sigma_ping(host="google.com")` |
| `sigma_ping_c` | Ping count | `sigma_ping_c(count=4, host="google.com")` |
| `sigma_traceroute` | Trace route | `sigma_traceroute(host="google.com")` |
| `sigma_tracepath` | Trace path | `sigma_tracepath(host="google.com")` |
| `sigma_mtr` | My traceroute | `sigma_mtr(host="google.com")` |
| `sigma_arping` | ARP ping | `sigma_arping(host="192.168.1.1")` |
| `sigma_nmap` | Network scan | `sigma_nmap(target="192.168.1.0/24")` |

### 3.4 Network Monitoring

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_netstat` | Network stats | `sigma_netstat()` |
| `sigma_ss` | Socket stats | `sigma_ss()` |
| `sigma_nstat` | Network stats | `sigma_nstat()` |

---

## 4. Process Management Commands

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_ps` | List processes | `sigma_ps()` |
| `sigma_ps_aux` | Detailed list | `sigma_ps_aux()` |
| `sigma_ps_tree` | Tree view | `sigma_ps_tree()` |
| `sigma_pgrep` | Find process | `sigma_pgrep(pattern="nginx")` |
| `sigma_pkill` | Kill by name | `sigma_pkill(pattern="nginx")` |
| `sigma_pidof` | Find PID | `sigma_pidof(process_name="nginx")` |
| `sigma_kill` | Kill process | `sigma_kill(pid=1234, signal=9)` |
| `sigma_killall` | Kill by name | `sigma_killall(process_name="nginx")` |
| `sigma_nice` | Run with nice | `sigma_nice(increment=-10, command="./app")` |
| `sigma_renice` | Change priority | `sigma_renice(priority=10, pid=1234)` |
| `sigma_top` | Top processes | `sigma_top()` |
| `sigma_htop` | Interactive top | `sigma_htop()` |

---

## 5. User Management Commands

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_useradd` | Add user | `sigma_useradd(username="john", uid=1001, gid=1001, home="/home/john", shell="/bin/bash", comment="John Doe")` |
| `sigma_userdel` | Delete user | `sigma_userdel(username="john", remove_home=true)` |
| `sigma_usermod` | Modify user | `sigma_usermod(username="john", option="shell", value="/bin/zsh")` |
| `sigma_passwd` | Change password | `sigma_passwd(username="john")` |
| `sigma_groupadd` | Add group | `sigma_groupadd(groupname="developers", gid=1001)` |
| `sigma_groupdel` | Delete group | `sigma_groupdel(groupname="developers")` |
| `sigma_groups` | List groups | `sigma_groups(username="john")` |
| `sigma_id` | Show ID | `sigma_id(username="john")` |
| `sigma_su` | Switch user | `sigma_su(username="root")` |
| `sigma_sudo` | Execute as root | `sigma_sudo(command="apt update")` |

---

## 6. Service Management Commands

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_systemctl_start` | Start service | `sigma_systemctl_start(service="nginx")` |
| `sigma_systemctl_stop` | Stop service | `sigma_systemctl_stop(service="nginx")` |
| `sigma_systemctl_restart` | Restart | `sigma_systemctl_restart(service="nginx")` |
| `sigma_systemctl_reload` | Reload | `sigma_systemctl_reload(service="nginx")` |
| `sigma_systemctl_status` | Status | `sigma_systemctl_status(service="nginx", &info)` |
| `sigma_systemctl_enable` | Enable service | `sigma_systemctl_enable(service="nginx")` |
| `sigma_systemctl_disable` | Disable | `sigma_systemctl_disable(service="nginx")` |
| `sigma_systemctl_list_units` | List units | `sigma_systemctl_list_units(services, &n_services)` |
| `sigma_service_start` | Start (init.d) | `sigma_service_start(service="nginx")` |
| `sigma_service_stop` | Stop (init.d) | `sigma_service_stop(service="nginx")` |
| `sigma_service_restart` | Restart (init.d) | `sigma_service_restart(service="nginx")` |
| `sigma_service_status` | Status (init.d) | `sigma_service_status(service="nginx")` |

---

## 7. Disk Management Commands

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_df` | Disk free | `sigma_df()` |
| `sigma_df_h` | Human readable | `sigma_df_h()` |
| `sigma_du` | Disk usage | `sigma_du(path="/var/log")` |
| `sigma_du_h` | Human readable | `sigma_du_h(path="/var/log")` |
| `sigma_fdisk_l` | List partitions | `sigma_fdisk_l()` |
| `sigma_parted` | Partition editor | `sigma_parted(device="/dev/sdb", command="print")` |
| `sigma_mkfs_ext4` | Make ext4 fs | `sigma_mkfs_ext4(device="/dev/sdb1")` |
| `sigma_mkfs_xfs` | Make XFS fs | `sigma_mkfs_xfs(device="/dev/sdb1")` |
| `sigma_fsck` | Check filesystem | `sigma_fsck(filesystem="/dev/sdb1")` |
| `sigma_mount` | Mount | `sigma_mount(device="/dev/sdb1", mount_point="/mnt")` |
| `sigma_umount` | Unmount | `sigma_umount(mount_point="/mnt")` |
| `sigma_lvextend` | Extend LV | `sigma_lvextend(lv_name="vg0/root", size="+10G")` |

---

## 8. Hardware Information Commands

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_lscpu` | CPU info | `sigma_lscpu()` |
| `sigma_free` | Memory info | `sigma_free()` |
| `sigma_free_h` | Human readable | `sigma_free_h()` |
| `sigma_lsusb` | USB devices | `sigma_lsusb()` |
| `sigma_lspci` | PCI devices | `sigma_lspci()` |
| `sigma_lsscsi` | SCSI devices | `sigma_lsscsi()` |
| `sigma_lsblk` | Block devices | `sigma_lsblk()` |
| `sigma_lshw` | Hardware info | `sigma_lshw()` |
| `sigma_dmidecode` | DMI info | `sigma_dmidecode()` |
| `sigma_inxi` | System info | `sigma_inxi()` |
| `sigma_uptime` | System uptime | `sigma_uptime()` |

---

## 9. Monitoring Commands

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_top` | Top processes | `sigma_top()` |
| `sigma_htop` | Interactive top | `sigma_htop()` |
| `sigma_iostat` | I/O stats | `sigma_iostat()` |
| `sigma_vmstat` | Virtual memory | `sigma_vmstat()` |
| `sigma_sar` | System activity | `sigma_sar()` |
| `sigma_dstat` | System stats | `sigma_dstat()` |
| `sigma_nmon` | Performance monitor | `sigma_nmon()` |
| `sigma_glances` | System monitor | `sigma_glances()` |

---

## 10. Security Commands

| Command | Description | Usage Example |
|---------|-------------|---------------|
| `sigma_getenforce` | SELinux status | `sigma_getenforce()` |
| `sigma_setenforce` | Set SELinux | `sigma_setenforce(mode=1)` |
| `sigma_chcon` | Change context | `sigma_chcon(context="httpd_sys_content_t", file="/var/www/html")` |
| `sigma_restorecon` | Restore context | `sigma_restorecon(path="/var/www/html")` |
| `sigma_chmod` | Change mode | `sigma_chmod(mode="755", file="/path")` |
| `sigma_chown` | Change owner | `sigma_chown(owner="root", file="/path")` |
| `sigma_getfacl` | Get ACL | `sigma_getfacl(file="/path")` |
| `sigma_setfacl_m` | Set ACL | `sigma_setfacl_m(acl="u:john:rwx", file="/path")` |

---

## Command Count Summary

| Category | Number of Commands |
|----------|-------------------|
| Package Management (APT) | 16 |
| Package Management (DPKG) | 10 |
| Package Management (SNAP) | 14 |
| Package Management (DNF) | 19 |
| Package Management (RPM) | 10 |
| Package Management (PACMAN) | 19 |
| Package Management (YAY) | 9 |
| Package Management (ZYPPER) | 17 |
| Package Management (APK) | 16 |
| Package Management (EMERGE) | 17 |
| Package Management (NIX) | 21 |
| Package Management (FLATPAK) | 15 |
| **Total Package Management** | **183** |
| User & Group Management | 27 |
| Process Management | 22 |
| Service Management | 23 |
| Disk Management | 12 |
| Network Commands | 28 |
| Hardware Info | 11 |
| Monitoring | 8 |
| Security | 8 |
| **Total System Admin** | **139** |
| **GRAND TOTAL** | **322+** |

---

*End of SigmaOS Complete Linux Commands Reference*

*This documentation covers 322+ Linux/Unix commands available in SigmaOS*
