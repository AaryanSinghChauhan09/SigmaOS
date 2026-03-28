/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS System Administration Commands
 * =======================================
 * Extended system administration commands:
 * - User and group management
 * - Process management
 * - Service management (systemd, init.d)
 * - Disk and filesystem management
 * - System monitoring and logging
 * - Hardware information
 * - Kernel management
 */

#ifndef SIGMA_SYSADMIN_COMMANDS_H
#define SIGMA_SYSADMIN_COMMANDS_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

// ==================== USER AND GROUP MANAGEMENT ====================

typedef struct {
    char* username;
    char* password; // Usually 'x' (in /etc/shadow)
    uid_t uid;
    gid_t gid;
    char* gecos; // Full name, room, work phone, home phone
    char* home_directory;
    char* shell;
    bool is_locked;
    char* last_login;
    char* password_last_changed;
    char* password_expires;
    char** groups;
    uint32_t n_groups;
} SigmaUserInfo;

typedef struct {
    char* groupname;
    char* password;
    gid_t gid;
    char** members;
    uint32_t n_members;
} SigmaGroupInfo;

// User management
int sigma_useradd(const char* username, uid_t uid, gid_t gid, 
                 const char* home, const char* shell, const char* comment);
int sigma_userdel(const char* username, bool remove_home);
int sigma_usermod(const char* username, const char* option, const char* value);
int sigma_passwd(const char* username);
int sigma_passwd_lock(const char* username);
int sigma_passwd_unlock(const char* username);
int sigma_passwd_delete(const char* username);
int sigma_passwd_status(const char* username);
int sigma_chage(const char* username, const char* option);
int sigma_chage_list(const char* username);
int sigma_chfn(const char* username, const char* full_name, 
               const char* room, const char* work_phone, const char* home_phone);
int sigma_chsh(const char* username, const char* shell);
int sigma_finger(const char* username);
int sigma_last(void);
int sigma_lastlog(void);
int sigma_lastlog_user(const char* username);
int sigma_who(void);
int sigma_w(void);
int sigma_users(void);
int sigma_logname(void);
int sigma_groups(const char* username);
int sigma_id(const char* username);
int sigma_su(const char* username);
int sigma_sudo(const char* command);
int sigma_sudo_l(void); // List sudo privileges
int sigma_sudo_u(const char* user, const char* command);
int sigma_sudo_g(const char* group, const char* command);
int sigma_sudo_k(void); // Reset timestamp
int sigma_sudo_K(void); // Remove timestamp
int sigma_visudo(void);
int sigma_newgrp(const char* groupname);
int sigma_sg(const char* groupname, const char* command);

// Group management
int sigma_groupadd(const char* groupname, gid_t gid);
int sigma_groupdel(const char* groupname);
int sigma_groupmod(const char* groupname, const char* option, const char* value);
int sigma_gpasswd(const char* groupname);
int sigma_gpasswd_a(const char* username, const char* groupname);
int sigma_gpasswd_d(const char* username, const char* groupname);
int sigma_gpasswd_A(const char* admin, const char* groupname);
int sigma_gpasswd_r(const char* groupname);
int sigma_gpasswd_R(const char* groupname);
int sigma_grpck(void);
int sigma_pwck(void);
int sigma_pwconv(void);
int sigma_pwunconv(void);
int sigma_grpconv(void);
int sigma_grpunconv(void);

// ==================== PROCESS MANAGEMENT ====================

typedef struct {
    pid_t pid;
    pid_t ppid;
    uid_t uid;
    gid_t gid;
    char* user;
    char* command;
    char* state;
    int priority;
    int nice;
    uint32_t virt;
    uint32_t res;
    uint32_t shr;
    double cpu_percent;
    double mem_percent;
    uint32_t time;
    char* tty;
    char* start_time;
} SigmaProcessInfo;

// Process listing and monitoring
int sigma_ps(void);
int sigma_ps_a(void);
int sigma_ps_u(const char* username);
int sigma_ps_x(void);
int sigma_ps_aux(void);
int sigma_ps_e(void);
int sigma_ps_f(void);
int sigma_ps_l(void);
int sigma_ps_j(void);
int sigma_ps_s(void);
int sigma_ps_v(void);
int sigma_ps_m(void);
int sigma_ps_p(pid_t pid);
int sigma_ps_t(const char* tty);
int sigma_ps_o(const char* format);
int sigma_ps_sort(const char* key);
int sigma_ps_tree(void);
int sigma_pgrep(const char* pattern);
int sigma_pgrep_u(const char* username);
int sigma_pgrep_l(const char* pattern);
int sigma_pkill(const char* pattern);
int sigma_pkill_signal(const char* pattern, int signal);
int sigma_pidof(const char* process_name);
int sigma_pidof_x(const char* process_name);
int sigma_pidof_s(const char* process_name);
int sigma_pidwait(const char* process_name);

// Process control
int sigma_kill(pid_t pid, int signal);
int sigma_killall(const char* process_name);
int sigma_killall_signal(const char* process_name, int signal);
int sigma_killall_u(const char* username);
int sigma_killall_w(const char* process_name);
int sigma_pkill_f(const char* full_pattern);
int sigma_skill(const char* pattern);
int sigma_snice(const char* pattern, int priority);

// Process priority and scheduling
int sigma_nice(int increment, const char* command);
int sigma_renice(int priority, pid_t pid);
int sigma_renice_g(int priority, gid_t gid);
int sigma_renice_u(int priority, const char* username);
int sigma_renice_p(int priority, pid_t pid);
int sigma_chrt(const char* policy, int priority, const char* command);
int sigma_chrt_p(const char* policy, int priority, pid_t pid);
int sigma_chrt_r(pid_t pid);
int sigma_taskset(int cpu_mask, const char* command);
int sigma_taskset_p(int cpu_mask, pid_t pid);
int sigma_taskset_pc(pid_t pid);
int sigma_numactl(const char* policy, const char* command);
int sigma_numactl_show(void);
int sigma_numastat(void);

// Process synchronization
int sigma_wait(void);
int sigma_waitpid(pid_t pid);
int sigma_nohup(const char* command);
int sigma_disown(void);
int sigma_disown_job(int job_id);
int sigma_disown_a(void);
int sigma_disown_h(void);
int sigma_disown_r(void);

// ==================== SERVICE MANAGEMENT (SYSTEMD) ====================

typedef struct {
    char* name;
    char* description;
    char* loaded;
    char* active;
    char* sub;
    char* status;
    char* unit_file_state;
    char* vendor_preset;
} SigmaServiceInfo;

// systemctl commands
int sigma_systemctl_start(const char* service);
int sigma_systemctl_stop(const char* service);
int sigma_systemctl_restart(const char* service);
int sigma_systemctl_reload(const char* service);
int sigma_systemctl_reload_or_restart(const char* service);
int sigma_systemctl_try_restart(const char* service);
int sigma_systemctl_status(const char* service, SigmaServiceInfo* info);
int sigma_systemctl_is_active(const char* service);
int sigma_systemctl_is_enabled(const char* service);
int sigma_systemctl_is_failed(const char* service);
int sigma_systemctl_enable(const char* service);
int sigma_systemctl_enable_now(const char* service);
int sigma_systemctl_disable(const char* service);
int sigma_systemctl_disable_now(const char* service);
int sigma_systemctl_mask(const char* service);
int sigma_systemctl_unmask(const char* service);
int sigma_systemctl_list_units(SigmaServiceInfo** services, uint32_t* n_services);
int sigma_systemctl_list_unit_files(SigmaServiceInfo** services, uint32_t* n_services);
int sigma_systemctl_list_failed(SigmaServiceInfo** services, uint32_t* n_services);
int sigma_systemctl_list_dependencies(const char* service);
int sigma_systemctl_show(const char* service);
int sigma_systemctl_show_environment(const char* service);
int sigma_systemctl_set_environment(const char* variable, const char* value);
int sigma_systemctl_unset_environment(const char* variable);
int sigma_systemctl_edit(const char* service);
int sigma_systemctl_edit_full(const char* service);
int sigma_systemctl_cat(const char* service);
int sigma_systemctl_daemon_reload(void);
int sigma_systemctl_daemon_reexec(void);
int sigma_systemctl_reset_failed(void);
int sigma_systemctl_reset_failed_service(const char* service);
int sigma_systemctl_preset(const char* service);
int sigma_systemctl_preset_all(void);
int sigma_systemctl_revert(const char* service);
int sigma_systemctl_add_wants(const char* target, const char* service);
int sigma_systemctl_add_requires(const char* target, const char* service);

// Target management
int sigma_systemctl_get_default(void);
int sigma_systemctl_set_default(const char* target);
int sigma_systemctl_isolate(const char* target);
int sigma_systemctl_rescue(void);
int sigma_systemctl_emergency(void);
int sigma_systemctl_halt(void);
int sigma_systemctl_poweroff(void);
int sigma_systemctl_reboot(void);
int sigma_systemctl_suspend(void);
int sigma_systemctl_hibernate(void);
int sigma_systemctl_hybrid_sleep(void);

// Timer management
int sigma_systemctl_list_timers(SigmaServiceInfo** timers, uint32_t* n_timers);
int sigma_systemctl_trigger(const char* timer);

// Socket management
int sigma_systemctl_list_sockets(SigmaServiceInfo** sockets, uint32_t* n_sockets);

// Machine management
int sigma_systemctl_machines(void);
int sigma_systemctl_machine_status(const char* machine);

// Logind
int sigma_loginctl_show_user(const char* username);
int sigma_loginctl_show_session(const char* session);
int sigma_loginctl_show_seat(const char* seat);
int sigma_loginctl_user_status(const char* username);
int sigma_loginctl_session_status(const char* session);
int sigma_loginctl_seat_status(const char* seat);
int sigma_loginctl_list_users(void);
int sigma_loginctl_list_sessions(void);
int sigma_loginctl_list_seats(void);
int sigma_loginctl_attach(const char* device, const char* seat);
int sigma_loginctl_flush_devices(void);
int sigma_loginctl_lock_session(const char* session);
int sigma_loginctl_unlock_session(const char* session);
int sigma_loginctl_terminate_user(const char* username);
int sigma_loginctl_kill_user(const char* username, int signal);
int sigma_loginctl_terminate_session(const char* session);
int sigma_loginctl_kill_session(const char* session, int signal);
int sigma_loginctl_terminate_seat(const char* seat);

// Networkd
int sigma_networkctl_status(void);
int sigma_networkctl_status_interface(const char* interface);
int sigma_networkctl_list(void);
int sigma_networkctl_lldp(void);
int sigma_networkctl_label(const char* label);
int sigma_networkctl_reconfigure(const char* interface);
int sigma_networkctl_reload(void);

// Resolved
int sigma_resolvectl_status(void);
int sigma_resolvectl_query(const char* hostname);
int sigma_resolvectl_service(const char* service, const char* protocol);
int sigma_resolvectl_openpgp(const char* email);
int sigma_resolvectl_tlsa(const char* service);
int sigma_resolvectl_status_interface(const char* interface);
int sigma_resolvectl_statistics(void);
int sigma_resolvectl_reset_statistics(void);
int sigma_resolvectl_flush_caches(void);
int sigma_resolvectl_reset_server_features(void);
int sigma_resolvectl_dns(const char* interface, const char** servers, uint32_t n_servers);
int sigma_resolvectl_domain(const char* interface, const char** domains, uint32_t n_domains);
int sigma_resolvectl_llmnr(const char* interface, const char* value);
int sigma_resolvectl_mdns(const char* interface, const char* value);
int sigma_resolvectl_dnssec(const char* interface, const char* value);
int sigma_resolvectl_dnsovertls(const char* interface, const char* value);
int sigma_resolvectl_nta(const char* interface, const char* domain);
int sigma_resolvectl_revert(const char* interface);

// Timesyncd
int sigma_timedatectl_status(void);
int sigma_timedatectl_set_time(const char* time);
int sigma_timedatectl_set_ntp(bool enabled);
int sigma_timedatectl_list_timezones(void);
int sigma_timedatectl_set_timezone(const char* timezone);
int sigma_timedatectl_set_local_rtc(bool local);
int sigma_timedatectl_show(void);

// Hostnamed
int sigma_hostnamectl_status(void);
int sigma_hostnamectl_set_hostname(const char* hostname);
int sigma_hostnamectl_set_icon_name(const char* icon_name);
int sigma_hostnamectl_set_chassis(const char* chassis);
int sigma_hostnamectl_set_deployment(const char* deployment);
int sigma_hostnamectl_set_location(const char* location);
int sigma_hostnamectl_pretty_hostname(const char* pretty_hostname);

// Init.d service management (for non-systemd systems)
int sigma_service_start(const char* service);
int sigma_service_stop(const char* service);
int sigma_service_restart(const char* service);
int sigma_service_reload(const char* service);
int sigma_service_force_reload(const char* service);
int sigma_service_status(const char* service);
int sigma_service_enable(const char* service);
int sigma_service_disable(const char* service);
int sigma_service_list(void);

// ==================== DISK AND FILESYSTEM MANAGEMENT ====================

typedef struct {
    char* filesystem;
    uint64_t size;
    uint64_t used;
    uint64_t available;
    uint64_t use_percent;
    char* mounted_on;
} SigmaDiskUsage;

typedef struct {
    char* device;
    char* boot;
    char* start;
    char* end;
    char* sectors;
    char* size;
    char* id;
    char* type;
} SigmaPartitionInfo;

// Disk usage
int sigma_df(void);
int sigma_df_h(void); // Human readable
int sigma_df_H(void); // SI units
int sigma_df_i(void); // Inodes
int sigma_df_P(void); // POSIX output
int sigma_df_T(void); // Print file type
int sigma_df_t(const char* type);
int sigma_df_x(const char* type);
int sigma_df_filesystem(const char* filesystem);

// Directory usage
int sigma_du(const char* path);
int sigma_du_a(const char* path);
int sigma_du_h(const char* path);
int sigma_du_s(const char* path);
int sigma_du_c(const char* path);
int sigma_du_d(uint32_t depth, const char* path);
int sigma_du_L(const char* path);
int sigma_du_l(const char* path);
int sigma_du_x(const char* pattern, const char* path);
int sigma_du_exclude(const char* pattern, const char* path);

// Filesystem management
int sigma_fdisk_l(void);
int sigma_fdisk_list(void);
int sigma_fdisk_create_partition(const char* device);
int sigma_fdisk_delete_partition(const char* device, int partition);
int sigma_fdisk_change_type(const char* device, int partition, const char* type);
int sigma_fdisk_write(const char* device);
int sigma_fdisk_verify(const char* device);
int sigma_fdisk_print(const char* device);
int sigma_parted(const char* device, const char* command);
int sigma_parted_mklabel(const char* device, const char* label_type);
int sigma_parted_mkpart(const char* device, const char* part_type, 
                        const char* fs_type, const char* start, const char* end);
int sigma_parted_rm(const char* device, int partition);
int sigma_parted_resize(const char* device, int partition, 
                        const char* start, const char* end);
int sigma_parted_move(const char* device, int partition, const char* start);
int sigma_parted_set(const char* device, int partition, const char* flag, bool on);
int sigma_parted_toggle(const char* device, int partition, const char* flag);
int sigma_parted_print(const char* device);
int sigma_parted_align_check(const char* device, const char* type, int partition);
int sigma_parted_resizepart(const char* device, int partition, const char* end);

// Filesystem creation
int sigma_mkfs_ext2(const char* device);
int sigma_mkfs_ext3(const char* device);
int sigma_mkfs_ext4(const char* device);
int sigma_mkfs_xfs(const char* device);
int sigma_mkfs_btrfs(const char* device);
int sigma_mkfs_vfat(const char* device);
int sigma_mkfs_ntfs(const char* device);
int sigma_mkfs_swap(const char* device);

// Filesystem operations
int sigma_fsck(const char* filesystem);
int sigma_fsck_a(void);
int sigma_fsck_r(void);
int sigma_fsck_A(void);
int sigma_fsck_C(void);
int sigma_fsck_M(void);
int sigma_fsck_N(void);
int sigma_fsck_P(void);
int sigma_fsck_R(void);
int sigma_fsck_T(void);
int sigma_fsck_t(const char* type);
int sigma_tune2fs(const char* device, const char* option);
int sigma_dumpe2fs(const char* device);
int sigma_resize2fs(const char* device);
int sigma_resize2fs_size(const char* device, uint64_t size);
int sigma_e2fsck(const char* device);
int sigma_debugfs(const char* device);
int sigma_e2image(const char* device, const char* image_file);
int sigma_xfs_repair(const char* device);
int sigma_xfs_growfs(const char* mount_point);
int sigma_xfs_admin(const char* device);
int sigma_xfs_db(const char* device);
int sigma_btrfs_check(const char* device);
int sigma_btrfs_resize(const char* mount_point, const char* size);
int sigma_btrfs_balance(const char* mount_point);
int sigma_btrfs_scrub(const char* mount_point);
int sigma_btrfs_device_add(const char* device, const char* mount_point);
int sigma_btrfs_device_delete(const char* device, const char* mount_point);
int sigma_btrfs_subvolume_create(const char* path);
int sigma_btrfs_subvolume_delete(const char* path);
int sigma_btrfs_subvolume_list(const char* mount_point);
int sigma_btrfs_subvolume_snapshot(const char* source, const char* dest);
int sigma_btrfs_send(const char* subvolume);
int sigma_btrfs_receive(const char* mount_point);

// Mount operations
int sigma_mount(const char* device, const char* mount_point);
int sigma_mount_t(const char* type, const char* device, const char* mount_point);
int sigma_mount_o(const char* options, const char* device, const char* mount_point);
int sigma_mount_a(void);
int sigma_mount_f(void);
int sigma_mount_v(void);
int sigma_mount_r(const char* mount_point);
int sigma_umount(const char* mount_point);
int sigma_umount_l(const char* mount_point);
int sigma_umount_f(const char* mount_point);
int sigma_umount_a(void);
int sigma_umount_t(const char* type);
int sigma_umount_O(const char* option);
int sigma_findmnt(void);
int sigma_findmnt_m(const char* mount_point);
int sigma_findmnt_s(const char* source);
int sigma_findmnt_t(const char* type);
int sigma_findmnt_o(const char* options);
int sigma_findmnt_S(const char* fsroot);
int sigma_findmnt_T(const char* target);
int sigma_findmnt_fstab(void);
int sigma_findmnt_verify(void);
int sigma_findmnt_kernel(void);

// Swap management
int sigma_swapon(const char* device);
int sigma_swapon_a(void);
int sigma_swapon_p(const char* device, int priority);
int sigma_swapoff(const char* device);
int sigma_swapoff_a(void);
int sigma_swapon_s(void);
int sigma_swapon_show(void);
int sigma_mkswap(const char* device);
int sigma_mkswap_L(const char* label, const char* device);
int sigma_mkswap_U(const char* uuid, const char* device);

// LVM (Logical Volume Management)
int sigma_pvcreate(const char* device);
int sigma_pvremove(const char* device);
int sigma_pvdisplay(void);
int sigma_pvdisplay_v(const char* device);
int sigma_pvs(void);
int sigma_pvscan(void);
int sigma_pvresize(const char* device);
int sigma_pvchange(const char* option, const char* device);
int sigma_pvck(const char* device);
int sigma_pvmove(const char* source, const char* destination);
int sigma_vgcreate(const char* vg_name, const char** devices, uint32_t n_devices);
int sigma_vgremove(const char* vg_name);
int sigma_vgrename(const char* old_name, const char* new_name);
int sigma_vgextend(const char* vg_name, const char* device);
int sigma_vgreduce(const char* vg_name, const char* device);
int sigma_vgdisplay(void);
int sigma_vgdisplay_v(const char* vg_name);
int sigma_vgs(void);
int sigma_vgscan(void);
int sigma_vgchange(const char* option, const char* vg_name);
int sigma_vgck(const char* vg_name);
int sigma_vgmerge(const char* vg1, const char* vg2);
int sigma_vgsplit(const char* vg, const char* new_vg, const char* device);
int sigma_vgexport(const char* vg_name);
int sigma_vgimport(const char* vg_name);
int sigma_lvcreate(const char* lv_name, const char* vg_name, const char* size);
int sigma_lvcreate_snapshot(const char* lv_name, const char* origin, const char* size);
int sigma_lvremove(const char* lv_name);
int sigma_lvrename(const char* old_name, const char* new_name);
int sigma_lvextend(const char* lv_name, const char* size);
int sigma_lvreduce(const char* lv_name, const char* size);
int sigma_lvresize(const char* lv_name, const char* size);
int sigma_lvdisplay(void);
int sigma_lvdisplay_m(void);
int sigma_lvs(void);
int sigma_lvscan(void);
int sigma_lvchange(const char* option, const char* lv_name);
int sigma_lvconvert(const char* option, const char* lv_name);
int sigma_lvconvert_merge(const char* snapshot);
int sigma_lvconvert_split(const char* origin, const char* snapshot);
int sigma_lvmdiskscan(void);
int sigma_lvmconf(void);
int sigma_lvmsadc(void);
int sigma_lvmsar(void);

// RAID
int sigma_mdadm_create(const char* md_device, int level, const char** devices, uint32_t n_devices);
int sigma_mdadm_assemble(const char* md_device, const char** devices, uint32_t n_devices);
int sigma_mdadm_manage(const char* md_device, const char* option);
int sigma_mdadm_detail(const char* md_device);
int sigma_mdadm_examine(const char* device);
int sigma_mdadm_stop(const char* md_device);
int sigma_mdadm_run(const char* md_device);
int sigma_mdadm_add(const char* md_device, const char* device);
int sigma_mdadm_remove(const char* md_device, const char* device);
int sigma_mdadm_fail(const char* md_device, const char* device);
int sigma_mdadm_replace(const char* md_device, const char* old_device, const char* new_device);
int sigma_mdadm_grow(const char* md_device, const char* option);
int sigma_mdadm_monitor(const char* config_file);

// ==================== SYSTEM MONITORING ====================

typedef struct {
    char* load_average_1min;
    char* load_average_5min;
    char* load_average_15min;
    uint32_t n_users;
    char* uptime;
} SigmaLoadAverage;

// System uptime and load
int sigma_uptime(void);
int sigma_uptime_p(void);
int sigma_uptime_s(void);
int sigma_loadavg(SigmaLoadAverage* load);

// Memory information
int sigma_free(void);
int sigma_free_h(void);
int sigma_free_b(void);
int sigma_free_k(void);
int sigma_free_m(void);
int sigma_free_g(void);
int sigma_free_l(void);
int sigma_free_t(void);
int sigma_free_s(void);
int sigma_free_w(void);
int sigma_vmstat(void);
int sigma_vmstat_a(void);
int sigma_vmstat_f(void);
int sigma_vmstat_m(void);
int sigma_vmstat_s(void);
int sigma_vmstat_d(void);
int sigma_vmstat_n(uint32_t count);
int sigma_vmstat_delay(uint32_t delay, uint32_t count);

// CPU information
int sigma_lscpu(void);
int sigma_lscpu_e(void);
int sigma_lscpu_p(void);
int sigma_lscpu_a(void);
int sigma_lscpu_x(void);
int sigma_lscpu_y(void);
int sigma_lscpu_J(void);
int sigma_nproc(void);
int sigma_nproc_all(void);
int sigma_nproc_ignore(void);

// I/O and block devices
int sigma_iostat(void);
int sigma_iostat_x(void);
int sigma_iostat_c(void);
int sigma_iostat_d(void);
int sigma_iostat_k(void);
int sigma_iostat_m(void);
int sigma_iostat_p(void);
int sigma_iostat_t(void);
int sigma_iostat_delay(uint32_t delay, uint32_t count);
int sigma_blkid(void);
int sigma_blkid_s(const char* device);
int sigma_blkid_o(const char* output_format);
int sigma_blkid_g(void);
int sigma_lsblk(void);
int sigma_lsblk_a(void);
int sigma_lsblk_b(void);
int sigma_lsblk_d(void);
int sigma_lsblk_e(void);
int sigma_lsblk_f(void);
int sigma_lsblk_h(void);
int sigma_lsblk_i(void);
int sigma_lsblk_J(void);
int sigma_lsblk_l(void);
int sigma_lsblk_m(void);
int sigma_lsblk_n(void);
int sigma_lsblk_o(const char* columns);
int sigma_lsblk_p(void);
int sigma_lsblk_s(void);
int sigma_lsblk_t(void);
int sigma_lsblk_x(void);
int sigma_lsblk_z(void);

// USB and PCI devices
int sigma_lsusb(void);
int sigma_lsusb_s(void);
int sigma_lsusb_d(const char* device);
int sigma_lsusb_t(void);
int sigma_lsusb_v(void);
int sigma_lspci(void);
int sigma_lspci_b(void);
int sigma_lspci_k(void);
int sigma_lspci_m(void);
int sigma_lspci_n(void);
int sigma_lspci_nn(void);
int sigma_lspci_q(void);
int sigma_lspci_s(void);
int sigma_lspci_t(void);
int sigma_lspci_v(void);
int sigma_lspci_vv(void);
int sigma_lspci_vvv(void);
int sigma_lspci_x(void);
int sigma_lspci_xx(void);
int sigma_lspci_xxx(void);

// SCSI/SATA devices
int sigma_lsscsi(void);
int sigma_lsscsi_c(void);
int sigma_lsscsi_d(void);
int sigma_lsscsi_g(void);
int sigma_lsscsi_h(void);
int sigma_lsscsi_k(void);
int sigma_lsscsi_l(void);
int sigma_lsscsi_L(void);
int sigma_lsscsi_p(void);
int sigma_lsscsi_s(void);
int sigma_lsscsi_t(void);
int sigma_lsscsi_u(void);
int sigma_lsscsi_v(void);
int sigma_lsscsi_w(void);
int sigma_lsscsi_x(void);

// Hardware info
int sigma_lshw(void);
int sigma_lshw_businfo(void);
int sigma_lshw_class(const char* class_name);
int sigma_lshw_disable(const char* test);
int sigma_lshw_enable(const char* test);
int sigma_lshw_html(const char* output_file);
int sigma_lshw_json(const char* output_file);
int sigma_lshw_notime(void);
int sigma_lshw_quiet(void);
int sigma_lshw_short(void);
int sigma_lshw_sanitize(void);
int sigma_lshw_xml(const char* output_file);
int sigma_lshw_numeric(void);
int sigma_dmidecode(void);
int sigma_dmidecode_d(uint32_t type);
int sigma_dmidecode_q(void);
int sigma_dmidecode_s(const char* keyword);
int sigma_dmidecode_u(void);
int sigma_dmidecode_handle(const char* handle);
int sigma_dmidump(const char* file);
int sigma_inxi(void);
int sigma_inxi_A(void);
int sigma_inxi_B(void);
int sigma_inxi_C(void);
int sigma_inxi_d(void);
int sigma_inxi_D(void);
int sigma_inxi_f(void);
int sigma_inxi_F(void);
int sigma_inxi_G(void);
int sigma_inxi_i(void);
int sigma_inxi_I(void);
int sigma_inxi_J(void);
int sigma_inxi_l(void);
int sigma_inxi_m(void);
int sigma_inxi_M(void);
int sigma_inxi_n(void);
int sigma_inxi_N(void);
int sigma_inxi_o(void);
int sigma_inxi_p(void);
int sigma_inxi_P(void);
int sigma_inxi_r(void);
int sigma_inxi_R(void);
int sigma_inxi_s(void);
int sigma_inxi_S(void);
int sigma_inxi_t(void);
int sigma_inxi_u(void);
int sigma_inxi_U(void);
int sigma_inxi_v(void);
int sigma_inxi_V(void);
int sigma_inxi_w(void);
int sigma_inxi_W(void);
int sigma_inxi_z(void);

// System activity
int sigma_sar(void);
int sigma_sar_A(void);
int sigma_sar_b(void);
int sigma_sar_B(void);
int sigma_sar_c(void);
int sigma_sar_d(void);
int sigma_sar_f(const char* filename);
int sigma_sar_h(void);
int sigma_sar_H(void);
int sigma_sar_i(void);
int sigma_sar_m(void);
int sigma_sar_n(void);
int sigma_sar_p(void);
int sigma_sar_q(void);
int sigma_sar_r(void);
int sigma_sar_R(void);
int sigma_sar_s(void);
int sigma_sar_S(void);
int sigma_sar_u(void);
int sigma_sar_v(void);
int sigma_sar_V(void);
int sigma_sar_w(void);
int sigma_sar_W(void);
int sigma_sar_y(void);
int sigma_sar_start(uint32_t interval);
int sigma_sar_stop(void);

// ==================== LOGGING AND JOURNAL ====================

// Traditional logging
int sigma_syslog(const char* message, int priority);
int sigma_logger(const char* message);
int sigma_logger_t(const char* tag);
int sigma_logger_p(const char* priority);
int sigma_logger_s(const char* size);
int sigma_logger_f(const char* file);
int sigma_logger_i(void);
int sigma_logrotate(const char* config_file);
int sigma_logrotate_d(void);
int sigma_logrotate_f(void);
int sigma_logrotate_force(void);
int sigma_logrotate_m(void);
int sigma_logrotate_s(void);
int sigma_logrotate_v(void);

// Systemd journal
int sigma_journalctl(void);
int sigma_journalctl_all(void);
int sigma_journalctl_boot(uint32_t boot_id);
int sigma_journalctl_catalog(void);
int sigma_journalctl_dmesg(void);
int sigma_journalctl_directory(const char* path);
int sigma_journalctl_file(const char* file);
int sigma_journalctl_flush(void);
int sigma_journalctl_follow(void);
int sigma_journalctl_forward(void);
int sigma_journalctl_header(void);
int sigma_journalctl_identifier(const char* identifier);
int sigma_journalctl_image(const char* image);
int sigma_journalctl_json(void);
int sigma_journalctl_json_pretty(void);
int sigma_journalctl_json_sse(void);
int sigma_journalctl_lines(uint32_t n);
int sigma_journalctl_machine(const char* machine);
int sigma_journalctl_merge(void);
int sigma_journalctl_no_hostname(void);
int sigma_journalctl_no_pager(void);
int sigma_journalctl_output(const char* format);
int sigma_journalctl_output_fields(const char* fields);
int sigma_journalctl_pager_end(void);
int sigma_journalctl_priority(int priority);
int sigma_journalctl_quiet(void);
int sigma_journalctl_reverse(void);
int sigma_journalctl_rotate(void);
int sigma_journalctl_setup_keys(void);
int sigma_journalctl_since(const char* time);
int sigma_journalctl_sync(void);
int sigma_journalctl_system(void);
int sigma_journalctl_t(const char* identifier);
int sigma_journalctl_tll(const char* transport);
int sigma_journalctl_unit(const char* unit);
int sigma_journalctl_user_unit(const char* unit);
int sigma_journalctl_until(const char* time);
int sigma_journalctl_update_catalog(void);
int sigma_journalctl_verify(void);
int sigma_journalctl_verify_key(const char* key);
int sigma_journalctl_disk_usage(void);
int sigma_journalctl_vacuum_files(uint32_t n);
int sigma_journalctl_vacuum_size(uint64_t size);
int sigma_journalctl_vacuum_time(const char* time);

// ==================== KERNEL MANAGEMENT ====================

// Module management
int sigma_modprobe(const char* module_name);
int sigma_modprobe_r(const char* module_name);
int sigma_modprobe_a(void);
int sigma_modprobe_b(void);
int sigma_modprobe_c(void);
int sigma_modprobe_d(const char* directory);
int sigma_modprobe_force(void);
int sigma_modprobe_ignore_install(void);
int sigma_modprobe_ignore_remove(void);
int sigma_modprobe_show(void);
int sigma_modprobe_first_time(void);
int sigma_modprobe_use_blacklist(void);
int sigma_modprobe_verbose(void);
int sigma_modprobe_syslog(void);
int sigma_insmod(const char* module_file, const char* params);
int sigma_rmmod(const char* module_name);
int sigma_rmmod_f(const char* module_name);
int sigma_rmmod_w(const char* module_name);
int sigma_rmmod_s(const char* module_name);
int sigma_rmmod_v(const char* module_name);
int sigma_lsmod(void);
int sigma_lsmod_j(void);
int sigma_lsmod_p(void);
int sigma_lsmod_S(void);
int sigma_lsmod_s(void);
int sigma_modinfo(const char* module_name);
int sigma_modinfo_0(void);
int sigma_modinfo_F(const char* field);
int sigma_modinfo_k(const char* kernel);
int sigma_modinfo_n(void);
int sigma_modinfo_V(void);
int sigma_depmod(void);
int sigma_depmod_a(void);
int sigma_depmod_A(void);
int sigma_depmod_b(const char* boot_dir);
int sigma_depmod_e(void);
int sigma_depmod_F(const char* system_map);
int sigma_depmod_n(void);
int sigma_depmod_v(void);

// Kernel parameters
int sigma_sysctl(void);
int sigma_sysctl_a(void);
int sigma_sysctl_n(const char* variable);
int sigma_sysctl_p(const char* file);
int sigma_sysctl_w(const char* variable, const char* value);
int sigma_sysctl_q(void);
int sigma_sysctl_e(void);
int sigma_sysctl_N(void);
int sigma_sysctl_b(void);
int sigma_sysctl_o(void);
int sigma_sysctl_x(void);

// Kernel information
int sigma_uname(void);
int sigma_uname_a(void);
int sigma_uname_s(void);
int sigma_uname_n(void);
int sigma_uname_r(void);
int sigma_uname_v(void);
int sigma_uname_m(void);
int sigma_uname_p(void);
int sigma_uname_i(void);
int sigma_uname_o(void);
int sigma_hostname(void);
int sigma_hostname_f(void);
int sigma_hostname_a(void);
int sigma_hostname_d(void);
int sigma_hostname_i(void);
int sigma_hostname_I(void);
int sigma_hostname_s(const char* hostname);
int sigma_domainname(void);
int sigma_domainname_y(void);

// Kernel upgrade
int sigma_kernel_install(const char* kernel_version);
int sigma_kernel_remove(const char* kernel_version);
int sigma_kernel_list(void);
int sigma_kernel_update(void);
int sigma_dracut(const char* initramfs, const char* kernel_version);
int sigma_mkinitcpio(const char* preset);
int sigma_update_initramfs(void);
int sigma_update_initramfs_u(void);
int sigma_update_initramfs_c(void);
int sigma_update_initramfs_k(const char* version);
int sigma_update_grub(void);
int sigma_grub_mkconfig(void);
int sigma_grub_install(const char* device);
int sigma_grub_probe(void);
int sigma_grub_set_default(const char* entry);
int sigma_grub_reboot(const char* entry);

// ==================== CONTAINER AND VIRTUALIZATION ====================

// Docker
int sigma_docker_ps(void);
int sigma_docker_ps_a(void);
int sigma_docker_ps_q(void);
int sigma_docker_ps_s(void);
int sigma_docker_images(void);
int sigma_docker_images_a(void);
int sigma_docker_images_q(void);
int sigma_docker_pull(const char* image);
int sigma_docker_push(const char* image);
int sigma_docker_build(const char* path);
int sigma_docker_run(const char* image);
int sigma_docker_stop(const char* container);
int sigma_docker_start(const char* container);
int sigma_docker_restart(const char* container);
int sigma_docker_rm(const char* container);
int sigma_docker_rmi(const char* image);
int sigma_docker_exec(const char* container, const char* command);
int sigma_docker_logs(const char* container);
int sigma_docker_inspect(const char* object);
int sigma_docker_network_ls(void);
int sigma_docker_volume_ls(void);
int sigma_docker_compose_up(void);
int sigma_docker_compose_down(void);
int sigma_docker_compose_build(void);
int sigma_docker_system_df(void);
int sigma_docker_system_prune(void);

// Podman
int sigma_podman_ps(void);
int sigma_podman_images(void);
int sigma_podman_pull(const char* image);
int sigma_podman_push(const char* image);
int sigma_podman_build(const char* path);
int sigma_podman_run(const char* image);
int sigma_podman_stop(const char* container);
int sigma_podman_start(const char* container);
int sigma_podman_rm(const char* container);
int sigma_podman_rmi(const char* image);
int sigma_podman_exec(const char* container, const char* command);
int sigma_podman_logs(const char* container);

// LXC/LXD
int sigma_lxc_ls(void);
int sigma_lxc_info(const char* container);
int sigma_lxc_create(const char* container, const char* template);
int sigma_lxc_start(const char* container);
int sigma_lxc_stop(const char* container);
int sigma_lxc_destroy(const char* container);
int sigma_lxc_attach(const char* container, const char* command);
int sigma_lxd_init(void);
int sigma_lxc_image_list(void);

// systemd-nspawn
int sigma_systemd_nspawn(const char* directory);
int sigma_systemd_nspawn_D(const char* directory);
int sigma_systemd_nspawn_b(const char* boot);
int sigma_systemd_nspawn_u(const char* user);
int sigma_systemd_nspawn_m(const char* machine);
int sigma_systemd_nspawn_network_veth(void);
int sigma_systemd_nspawn_network_bridge(const char* bridge);
int sigma_systemd_nspawn_network_zone(const char* zone);
int sigma_machinectl(void);
int sigma_machinectl_list(void);
int sigma_machinectl_status(const char* machine);
int sigma_machinectl_login(const char* machine);
int sigma_machinectl_shell(const char* machine);

// KVM/QEMU
int sigma_virsh_list(void);
int sigma_virsh_list_all(void);
int sigma_virsh_start(const char* domain);
int sigma_virsh_shutdown(const char* domain);
int sigma_virsh_destroy(const char* domain);
int sigma_virsh_define(const char* xml_file);
int sigma_virsh_undefine(const char* domain);
int sigma_virsh_edit(const char* domain);
int sigma_virsh_console(const char* domain);
int sigma_virsh_dumpxml(const char* domain);
int sigma_virsh_attach_disk(const char* domain, const char* path);
int sigma_virsh_detach_disk(const char* domain, const char* target);
int sigma_virsh_snapshot_create(const char* domain, const char* name);
int sigma_virsh_snapshot_revert(const char* domain, const char* name);
int sigma_virsh_snapshot_delete(const char* domain, const char* name);
int sigma_virsh_pool_list(void);
int sigma_virsh_pool_define(const char* xml_file);
int sigma_virsh_pool_start(const char* pool);
int sigma_virsh_pool_destroy(const char* pool);
int sigma_virsh_vol_list(const char* pool);
int sigma_virsh_vol_create(const char* pool, const char* xml_file);
int sigma_virsh_vol_delete(const char* pool, const char* vol);

// ==================== SECURITY AND AUDITING ====================

// SELinux
int sigma_getenforce(void);
int sigma_setenforce(int mode);
int sigma_sestatus(void);
int sigma_sestatus_v(void);
int sigma_sestatus_b(void);
int sigma_chcon(const char* context, const char* file);
int sigma_chcon_u(const char* user);
int sigma_chcon_r(const char* role);
int sigma_chcon_t(const char* type);
int sigma_chcon_l(const char* range);
int sigma_chcon_R(void);
int sigma_restorecon(const char* path);
int sigma_restorecon_R(const char* path);
int sigma_restorecon_F(void);
int sigma_restorecon_v(void);
int sigma_restorecon_n(void);
int sigma_restorecon_D(const char* directory);
int sigma_semanage_login(void);
int sigma_semanage_user(void);
int sigma_semanage_port(void);
int sigma_semanage_fcontext(void);
int sigma_semanage_module(void);
int sigma_semanage_boolean(void);
int sigma_semanage_dontaudit(void);
int sigma_semanage_export(void);
int sigma_semanage_import(void);

// AppArmor
int sigma_aa_status(void);
int sigma_aa_enabled(void);
int sigma_aa_enforce(const char* profile);
int sigma_aa_complain(const char* profile);
int sigma_aa_disable(const char* profile);
int sigma_aa_genprof(const char* profile);
int sigma_aa_logprof(void);
int sigma_aa_autodep(const char* profile);
int sigma_aagenprof(const char* profile);
int sigma_aalogprof(void);
int sigma_aa_cleanprof(const char* profile);
int sigma_aa_mergeprof(const char* profile);
int sigma_aa_unconfined(void);

// Permissions and ACLs
int sigma_chmod(const char* mode, const char* file);
int sigma_chmod_R(const char* mode, const char* file);
int sigma_chown(const char* owner, const char* file);
int sigma_chown_R(const char* owner, const char* file);
int sigma_chgrp(const char* group, const char* file);
int sigma_chgrp_R(const char* group, const char* file);
int sigma_getfacl(const char* file);
int sigma_setfacl_m(const char* acl, const char* file);
int sigma_setfacl_x(const char* acl, const char* file);
int sigma_setfacl_b(const char* file);
int sigma_setfacl_R(const char* option, const char* file);
int sigma_getfattr(const char* file);
int sigma_setfattr_n(const char* name, const char* value, const char* file);
int sigma_setfattr_x(const char* name, const char* file);
int sigma_setfattr_d(const char* name, const char* file);

// Audit
int sigma_auditctl_l(void);
int sigma_auditctl_s(void);
int sigma_auditctl_a(const char* rule);
int sigma_auditctl_d(const char* rule);
int sigma_auditctl_D(void);
int sigma_auditctl_r(uint32_t backlog);
int sigma_auditctl_e(uint32_t enabled);
int sigma_ausearch(void);
int sigma_ausearch_m(const char* message_type);
int sigma_ausearch_ts(const char* start_time);
int sigma_ausearch_te(const char* end_time);
int sigma_ausearch_ui(uint32_t uid);
int sigma_ausearch_k(const char* key);
int sigma_ausearch_f(const char* file);
int sigma_aureport(void);
int sigma_aureport_a(void);
int sigma_aureport_au(void);
int sigma_aureport_f(void);
int sigma_aureport_h(void);
int sigma_aureport_i(void);
int sigma_aureport_l(void);
int sigma_aureport_m(void);
int sigma_aureport_p(void);
int sigma_aureport_r(void);
int sigma_aureport_s(void);
int sigma_aureport_t(void);
int sigma_aureport_u(void);
int sigma_aureport_x(void);

#endif // SIGMA_SYSADMIN_COMMANDS_H

