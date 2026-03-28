/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Additional Linux/Unix Commands Library
 * ==============================================
 * Extended command set inspired by all major Linux distributions:
 * - Ubuntu/Debian (apt, dpkg, snap)
 * - Fedora/RHEL/CentOS (dnf, yum, rpm)
 * - Arch Linux (pacman, yay, makepkg)
 * - openSUSE (zypper)
 * - Alpine (apk)
 * - Gentoo (emerge, portage)
 * - NixOS (nix)
 * - Void Linux (xbps)
 */

#ifndef SIGMA_LINUX_COMMANDS_EXTENDED_H
#define SIGMA_LINUX_COMMANDS_EXTENDED_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

// ==================== PACKAGE MANAGEMENT COMMANDS ====================

// Ubuntu/Debian (APT - Advanced Package Tool)
typedef struct {
    char* package_name;
    char* version;
    char* repository;
    bool is_installed;
    bool is_upgradeable;
    char* description;
    uint64_t installed_size;
    char** dependencies;
    uint32_t n_dependencies;
} SigmaAptPackage;

// APT Commands
int sigma_apt_update(void); // Update package lists
int sigma_apt_upgrade(bool full_upgrade); // Upgrade packages
int sigma_apt_install(const char* package_name, bool auto_confirm);
int sigma_apt_remove(const char* package_name, bool purge);
int sigma_apt_autoremove(void); // Remove unused packages
int sigma_apt_search(const char* keyword, SigmaAptPackage** results, uint32_t* n_results);
int sigma_apt_show(const char* package_name, SigmaAptPackage* info);
int sigma_apt_list(bool installed_only, SigmaAptPackage** packages, uint32_t* n_packages);
int sigma_apt_cache_search(const char* pattern);
int sigma_apt_cache_show(const char* package_name);
int sigma_apt_cache_stats(void);
int sigma_apt_cache_policy(const char* package_name);
int sigma_apt_add_repository(const char* repository);
int sigma_apt_remove_repository(const char* repository);
int sigma_apt_key_add(const char* key_url);
int sigma_apt_key_del(const char* key_id);
int sigma_apt_edit_sources(void);
int sigma_apt_mark_auto(const char* package_name);
int sigma_apt_mark_manual(const char* package_name);
int sigma_apt_hold(const char* package_name);
int sigma_apt_unhold(const char* package_name);
int sigma_apt_download(const char* package_name, const char* destination);
int sigma_apt_source(const char* package_name, const char* destination);
int sigma_apt_build_dep(const char* package_name);
int sigma_apt_fulfill_depends(const char* deb_file);
int sigma_apt_simulate(const char* command, char** output);

// DPKG (Debian Package)
typedef struct {
    char* package_name;
    char* version;
    char* architecture;
    char* maintainer;
    char* installed_size;
    char* description;
    char* status;
    char** files;
    uint32_t n_files;
} SigmaDpkgInfo;

int sigma_dpkg_install(const char* deb_file);
int sigma_dpkg_remove(const char* package_name);
int sigma_dpkg_purge(const char* package_name);
int sigma_dpkg_list(SigmaDpkgInfo** packages, uint32_t* n_packages);
int sigma_dpkg_info(const char* package_name, SigmaDpkgInfo* info);
int sigma_dpkg_status(const char* package_name);
int sigma_dpkg_search(const char* file_path, char** package_name);
int sigma_dpkg_contents(const char* deb_file, char*** files, uint32_t* n_files);
int sigma_dpkg_extract(const char* deb_file, const char* destination);
int sigma_dpkg_configure(const char* package_name);
int sigma_dpkg_reconfigure(const char* package_name);
int sigma_dpkg_trigger(const char* package_name);
int sigma_dpkg_divert(const char* file_path, const char* divert_to);
int sigma_dpkg_statoverride(const char* file_path, const char* owner, const char* group, uint32_t mode);
int sigma_dpkg_build(const char* directory, const char* output_deb);
int sigma_dpkg_deb(const char* directory, const char* deb_file);
int sigma_dpkg_split(const char* deb_file, const char* prefix, uint64_t max_size);
int sigma_dpkg_join(const char** parts, const char* output_deb);

// Snap (Ubuntu Snap Packages)
typedef struct {
    char* name;
    char* version;
    char* revision;
    char* tracking;
    char* publisher;
    char* notes;
    bool is_installed;
    uint64_t size;
} SigmaSnapInfo;

int sigma_snap_install(const char* snap_name, bool classic, const char* channel);
int sigma_snap_remove(const char* snap_name);
int sigma_snap_refresh(const char* snap_name);
int sigma_snap_revert(const char* snap_name);
int sigma_snap_list(SigmaSnapInfo** snaps, uint32_t* n_snaps);
int sigma_snap_find(const char* keyword, SigmaSnapInfo** results, uint32_t* n_results);
int sigma_snap_info(const char* snap_name, SigmaSnapInfo* info);
int sigma_snap_switch_channel(const char* snap_name, const char* channel);
int sigma_snap_enable(const char* snap_name);
int sigma_snap_disable(const char* snap_name);
int sigma_snap_alias(const char* snap_name, const char* app_name, const char* alias);
int sigma_snap_unalias(const char* alias);
int sigma_snap_set(const char* snap_name, const char* key, const char* value);
int sigma_snap_get(const char* snap_name, const char* key, char** value);
int sigma_snap_watch(const char* snap_name);
int sigma_snap_ack(const char* assertion_file);
int sigma_snap_known(const char* assertion_type, char*** assertions, uint32_t* n_assertions);
int sigma_snap_validate(const char* snap_file);
int sigma_snap_try(const char* directory);
int sigma_snap_restart(const char* snap_name);
int sigma_snap_services(const char* snap_name, char*** services, uint32_t* n_services);
int sigma_snap_start(const char* snap_name, const char* service);
int sigma_snap_stop(const char* snap_name, const char* service);

// Fedora/RHEL/CentOS (DNF/YUM)
typedef struct {
    char* name;
    char* version;
    char* release;
    char* architecture;
    char* repository;
    char* size;
    bool is_installed;
    char* summary;
    char* license;
} SigmaDnfPackage;

int sigma_dnf_check_update(void);
int sigma_dnf_upgrade(bool security_only);
int sigma_dnf_install(const char* package_name, bool auto_confirm);
int sigma_dnf_remove(const char* package_name);
int sigma_dnf_autoremove(void);
int sigma_dnf_search(const char* keyword, SigmaDnfPackage** results, uint32_t* n_results);
int sigma_dnf_info(const char* package_name, SigmaDnfPackage* info);
int sigma_dnf_list(bool installed_only, SigmaDnfPackage** packages, uint32_t* n_packages);
int sigma_dnf_provides(const char* file_or_capability);
int sigma_dnf_whatprovides(const char* capability);
int sigma_dnf_repoquery(const char* query);
int sigma_dnf_repository_list(void);
int sigma_dnf_repository_enable(const char* repo_id);
int sigma_dnf_repository_disable(const char* repo_id);
int sigma_dnf_repository_add(const char* name, const char* url);
int sigma_dnf_module_list(void);
int sigma_dnf_module_enable(const char* module_name);
int sigma_dnf_module_disable(const char* module_name);
int sigma_dnf_module_install(const char* module_name, const char* stream);
int sigma_dnf_module_remove(const char* module_name);
int sigma_dnf_history_list(void);
int sigma_dnf_history_undo(uint32_t transaction_id);
int sigma_dnf_history_redo(uint32_t transaction_id);
int sigma_dnf_history_rollback(uint32_t transaction_id);
int sigma_dnf_downgrade(const char* package_name);
int sigma_dnf_reinstall(const char* package_name);
int sigma_dnf_distro_sync(void);
int sigma_dnf_system_upgrade(const char* releasever);
int sigma_dnf_group_list(void);
int sigma_dnf_group_install(const char* group_name);
int sigma_dnf_group_remove(const char* group_name);
int sigma_dnf_mark_install(const char* package_name);
int sigma_dnf_mark_remove(const char* package_name);
int sigma_dnf_shell(const char* script);

// RPM (Red Hat Package Manager)
int sigma_rpm_install(const char* rpm_file);
int sigma_rpm_remove(const char* package_name);
int sigma_rpm_query(const char* package_name, char** info);
int sigma_rpm_query_all(char*** packages, uint32_t* n_packages);
int sigma_rpm_query_file(const char* file_path, char** package_name);
int sigma_rpm_verify(const char* package_name);
int sigma_rpm_checksig(const char* rpm_file);
int sigma_rpm_import_key(const char* key_file);
int sigma_rpm_extract(const char* rpm_file, const char* destination);
int sigma_rpm_build(const char* spec_file, const char* source_dir);
int sigma_rpm_rebuild(const char* srpm_file);
int sigma_rmd_k(string_t args[]);

// Arch Linux (Pacman)
typedef struct {
    char* name;
    char* version;
    char* description;
    char* architecture;
    char* url;
    char* licenses;
    char* groups;
    char** provides;
    char** depends;
    char** optdepends;
    bool is_installed;
    char* install_date;
    char* install_reason;
    char* install_script;
    char* validated_by;
} SigmaPacmanPackage;

int sigma_pacman_sync(void); // -Sy
int sigma_pacman_upgrade(bool force_refresh); // -Syu
int sigma_pacman_install(const char* package_name, bool no_confirm);
int sigma_pacman_remove(const char* package_name, bool cascade, bool recursive);
int sigma_pacman_search(const char* keyword, SigmaPacmanPackage** results, uint32_t* n_results);
int sigma_pacman_query(const char* package_name, SigmaPacmanPackage* info);
int sigma_pacman_query_all(SigmaPacmanPackage** packages, uint32_t* n_packages);
int sigma_pacman_query_explicit(SigmaPacmanPackage** packages, uint32_t* n_packages);
int sigma_pacman_query_foreign(SigmaPacmanPackage** packages, uint32_t* n_packages);
int sigma_pacman_query_unrequired(SigmaPacmanPackage** packages, uint32_t* n_packages);
int sigma_pacman_query_upgrades(SigmaPacmanPackage** packages, uint32_t* n_packages);
int sigma_pacman_database(bool check, bool update);
int sigma_pacman_files(const char* package_name, char*** files, uint32_t* n_files);
int sigma_pacman_owns(const char* file_path, char** package_name);
int sigma_pacman_group_list(const char* group_name, SigmaPacmanPackage** packages, uint32_t* n_packages);
int sigma_pacman_deptest(const char** packages, uint32_t n_packages);
int sigma_pacman_clean(void);
int sigma_pacman_clean_uninstalled(void);
int sigma_pacman_key_init(void);
int sigma_pacman_key_populate(void);
int sigma_pacman_key_refresh(void);
int sigma_pacman_log(void);

// AUR Helper (Yay)
int sigma_yay_search(const char* keyword, SigmaPacmanPackage** results, uint32_t* n_results);
int sigma_yay_info(const char* package_name, SigmaPacmanPackage* info);
int sigma_yay_clone(const char* package_name, const char* destination);
int sigma_yay_build(const char* pkgbuild_dir);
int sigma_yay_install_aur(const char* package_name);
int sigma_yay_update_devel(bool rebuild);
int sigma_yay_clean(void);
int sigma_yay_diff(void);
int sigma_yay_edit(const char* package_name);
int sigma_yay_merge(const char* package_name);

// Makepkg
int sigma_makepkg_sync(void);
int sigma_makepkg_install(const char* pkgbuild_dir);
int sigma_makepkg_build(const char* pkgbuild_dir);
int sigma_makepkg_clean(void);
int sigma_makepkg_geninteg(const char* pkgbuild_dir);
int sigma_makepkg_checksum(const char* pkgbuild_dir);

// openSUSE (Zypper)
typedef struct {
    char* name;
    char* version;
    char* arch;
    char* repository;
    char* installed;
    char* status;
    char* summary;
} SigmaZypperPackage;

int sigma_zypper_refresh(void);
int sigma_zypper_update(bool full);
int sigma_zypper_install(const char* package_name, bool auto_agree);
int sigma_zypper_remove(const char* package_name);
int sigma_zypper_search(const char* keyword, SigmaZypperPackage** results, uint32_t* n_results);
int sigma_zypper_info(const char* package_name, SigmaZypperPackage* info);
int sigma_zypper_patches(void);
int sigma_zypper_patch(void);
int sigma_zypper_dist_upgrade(void);
int sigma_zypper_verify(void);
int sigma_zypper_source_install(const char* package_name);
int sigma_zypper_addrepo(const char* name, const char* url);
int sigma_zypper_removerepo(const char* name);
int sigma_zypper_renamerepo(const char* old_name, const char* new_name);
int sigma_zypper_modifyrepo(const char* name, bool enable, bool refresh, bool gpgcheck);
int sigma_zypper_lock(const char* package_name);
int sigma_zypper_unlock(const char* package_name);
int sigma_zypper_ps(void); // List processes using deleted files
int sigma_zypper_pa(void); // List packages affected by running processes
int sigma_zypper_al(void); // Add a lock
int sigma_zypper_ll(void); // List locks
int sigma_zypper_rl(void); // Remove lock
int sigma_zypper_in(const char* package_name, const char* version);
int sigma_zypper_rm(const char* package_name);

// Alpine (APK)
typedef struct {
    char* name;
    char* version;
    char* description;
    char* license;
    char* origin;
    char* maintainer;
    char* build_date;
    char* commit;
    char* installed_size;
    bool is_installed;
} SigmaApkPackage;

int sigma_apk_update(void);
int sigma_apk_upgrade(bool available);
int sigma_apk_add(const char* package_name);
int sigma_apk_del(const char* package_name);
int sigma_apk_search(const char* keyword, SigmaApkPackage** results, uint32_t* n_results);
int sigma_apk_info(const char* package_name, SigmaApkPackage* info);
int sigma_apk_list(SigmaApkPackage** packages, uint32_t* n_packages);
int sigma_apk_policy(const char* package_name);
int sigma_apk_version(const char* package_name);
int sigma_apk_index(const char* repository_url);
int sigma_apk_fetch(const char* package_name, const char* destination);
int sigma_apk_cache_clean(void);
int sigma_apk_cache_download(void);
int sigma_apk_dot(const char* command);
int sigma_apk_fix(void);
int sigma_apk_audit(void);
int sigma_apk_verify(const char* package_name);

// Gentoo (Portage/Emerge)
typedef struct {
    char* category;
    char* name;
    char* version;
    char* slot;
    char* repo;
    char* description;
    char* homepage;
    char* license;
    char* iuse;
    char** use_flags;
    uint32_t n_use_flags;
    bool is_installed;
    char* installed_version;
} SigmaEmergePackage;

int sigma_emerge_sync(void);
int sigma_emerge_update(const char** packages, uint32_t n_packages, bool deep, bool newuse);
int sigma_emerge_install(const char* package_name, bool auto_unmask);
int sigma_emerge_unmerge(const char* package_name, bool selective);
int sigma_emerge_search(const char* keyword, SigmaEmergePackage** results, uint32_t* n_results);
int sigma_emerge_info(const char* package_name, SigmaEmergePackage* info);
int sigma_emerge_pretend(const char* package_name);
int sigma_emerge_fetch(const char* package_name);
int sigma_emerge_clean(bool distfiles, bool packages);
int sigma_emerge_depclean(void);
int sigma_emerge_world(void);
int sigma_emerge_system(void);
int sigma_emerge_emptytree(void);
int sigma_emerge_noconfmem(void);
int sigma_emerge_oneshot(const char* package_name);
int sigma_emerge_onlydeps(const char* package_name);
int sigma_emerge_usepkg(const char* package_name);
int sigma_emerge_buildpkg(const char* package_name);
int sigma_emerge_getbinpkg(const char* package_name);
int sigma_emerge_backtrack(uint32_t attempts);
int sigma_emerge_verbose(bool verbose);
int sigma_emerge_quiet(bool quiet);
int sigma_emerge_ask(bool ask);
int sigma_emerge_tree(void);
int sigma_emerge_verbose_conflicts(void);
int sigma_emerge_resume(void);
int sigma_emerge_skipfirst(void);
int sigma_emerge_reinstall(const char* package_name);

// Equery (Gentoo Query Tool)
int sigma_equery_belongs(const char* file_path);
int sigma_equery_changes(const char* package_name);
int sigma_equery_check(const char* package_name);
int sigma_equery_depends(const char* package_name);
int sigma_equery_dups(void);
int sigma_equery_files(const char* package_name);
int sigma_equery_graph(const char* package_name);
int sigma_equery_has(const char* use_flag);
int sigma_equery_hasuse(const char* package_name);
int sigma_equery_list(const char* package_name);
int sigma_equery_meta(const char* package_name);
int sigma_equery_size(const char* package_name);
int sigma_equery_uses(const char* package_name);
int sigma_equery_which(const char* file_path);

// NixOS (Nix)
typedef struct {
    char* name;
    char* version;
    char* description;
    char* license;
    char* homepage;
    char* maintainers;
    char* platforms;
    char* system;
    char* drv_path;
    char* out_path;
    char** dependencies;
    uint32_t n_dependencies;
} SigmaNixPackage;

int sigma_nix_search(const char* keyword, SigmaNixPackage** results, uint32_t* n_results);
int sigma_nix_install(const char* package_name);
int sigma_nix_uninstall(const char* package_name);
int sigma_nix_upgrade(const char* package_name);
int sigma_nix_upgrade_all(void);
int sigma_nix_info(const char* package_name, SigmaNixPackage* info);
int sigma_nix_list(SigmaNixPackage** packages, uint32_t* n_packages);
int sigma_nix_collect_garbage(void);
int sigma_nix_collect_garbage_delete_older_than(const char* time);
int sigma_nix_store_optimise(void);
int sigma_nix_store_verify(void);
int sigma_nix_store_repair(const char* path);
int sigma_nix_build(const char* nix_file);
int sigma_nix_shell(const char* package_name);
int sigma_nix_run(const char* package_name);
int sigma_nix_edit(const char* package_name);
int sigma_nix_derivation_show(const char* drv_file);
int sigma_nix_copy_closure(const char* source, const char* destination);
int sigma_nix_copy_log(const char* drv_path);
int sigma_nix_env_install(const char* nix_file);
int sigma_nix_env_uninstall(const char* package_name);
int sigma_nix_env_query(const char* package_name);
int sigma_nix_env_list(void);
int sigma_nix_env_switch(const char* profile);
int sigma_nix_env_rollback(void);
int sigma_nix_env_delete_generations(const char* pattern);
int sigma_nix_env_list_generations(void);
int sigma_nix_channel_list(void);
int sigma_nix_channel_add(const char* name, const char* url);
int sigma_nix_channel_remove(const char* name);
int sigma_nix_channel_update(const char* name);
int sigma_nix_flake_init(const char* template_name);
int sigma_nix_flake_lock(void);
int sigma_nix_flake_update(void);
int sigma_nix_flake_check(void);
int sigma_nix_flake_metadata(void);
int sigma_nix_flake_clone(const char* url);
int sigma_nix_develop(const char* nix_file);
int sigma_nix_print_dev_env(const char* nix_file);

// Void Linux (XBPS)
typedef struct {
    char* name;
    char* version;
    char* revision;
    char* arch;
    char* repository;
    char* license;
    char* short_desc;
    char* long_desc;
    char** dependencies;
    uint32_t n_dependencies;
    bool is_installed;
} SigmaXbpsPackage;

int sigma_xbps_install(const char* package_name);
int sigma_xbps_remove(const char* package_name);
int sigma_xbps_query(const char* package_name, SigmaXbpsPackage* info);
int sigma_xbps_query_all(SigmaXbpsPackage** packages, uint32_t* n_packages);
int sigma_xbps_search(const char* keyword, SigmaXbpsPackage** results, uint32_t* n_results);
int sigma_xbps_update(void);
int sigma_xbps_upgrade(bool distfiles);
int sigma_xbps_reconfigure(const char* package_name);
int sigma_xbps_alternatives(const char* package_name);
int sigma_xbps_check(void);
int sigma_xbps_dgraph(const char* package_name);
int sigma_xbps_fetch(const char* package_name);
int sigma_xbps_list(void);
int sigma_xbps_pkgdb(void);
int sigma_xbps_rindex(const char* repository_path);

// Flatpak
typedef struct {
    char* name;
    char* version;
    char* branch;
    char* origin;
    char* installation;
    char* runtime;
    char* sdk;
    bool is_installed;
} SigmaFlatpakApp;

int sigma_flatpak_install(const char* remote, const char* app_name);
int sigma_flatpak_uninstall(const char* app_name);
int sigma_flatpak_update(const char* app_name);
int sigma_flatpak_update_all(void);
int sigma_flatpak_list(SigmaFlatpakApp** apps, uint32_t* n_apps);
int sigma_flatpak_search(const char* keyword, SigmaFlatpakApp** results, uint32_t* n_results);
int sigma_flatpak_info(const char* app_name, SigmaFlatpakApp* info);
int sigma_flatpak_run(const char* app_name);
int sigma_flatpak_override(const char* app_name);
int sigma_flatpak_remote_add(const char* name, const char* url);
int sigma_flatpak_remote_delete(const char* name);
int sigma_flatpak_remote_list(void);
int sigma_flatpak_remote_modify(const char* name);
int sigma_flatpak_build(const char* directory);
int sigma_flatpak_build_init(const char* directory, const char* app_name);
int sigma_flatpak_build_finish(const char* directory);
int sigma_flatpak_build_export(const char* repository, const char* directory);
int sigma_flatpak_repo(void);
int sigma_flatpak_create_usb(const char* mount_point);

// AppImage
int sigma_appimage_extract(const char* appimage_file, const char* destination);
int sigma_appimage_mount(const char* appimage_file, const char* mount_point);
int sigma_appimage_unmount(const char* mount_point);
int sigma_appimage_update(const char* appimage_file);
int sigma_appimage_update_info(const char* appimage_file);
int sigma_appimage_sign(const char* appimage_file, const char* key);
int sigma_appimage_verify(const char* appimage_file);
int sigma_appimage_integrate(const char* appimage_file);
int sigma_appimage_integrate_list(void);
int sigma_appimage_integrate_remove(const char* appimage_file);

// AppStream
int sigma_appstream_search(const char* keyword);
int sigma_appstream_info(const char* app_id);
int sigma_appstream_get(const char* app_id);

#endif // SIGMA_LINUX_COMMANDS_EXTENDED_H

