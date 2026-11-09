// SPDX-License-Identifier: MIT
// Test file for all new Arch-inspired components
// This file tests the compilation and basic functionality of all new modules

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    // Test service manager
    #[test]
    fn test_service_manager_basic() {
        let manager = crate::etc::service_manager::ServiceManager::new();
        assert_eq!(manager.services.len(), 0);
    }

    #[test]
    fn test_service_config_builder() {
        let config = crate::etc::service_manager::ServiceConfig::new(
            "test".to_string(),
            "Test service".to_string(),
            "/usr/bin/test".to_string(),
        )
        .set_type(crate::etc::service_manager::ServiceType::Notify)
        .depends_on("network.target".to_string())
        .wanted_by("multi-user.target".to_string());

        assert_eq!(config.name, "test");
        assert_eq!(config.service_type, crate::etc::service_manager::ServiceType::Notify);
    }

    // Test mirror selector
    #[test]
    fn test_mirror_selector_basic() {
        let selector = crate::etc::mirror_selector::MirrorSelector::new();
        assert_eq!(selector.mirrors.len(), 0);
        assert_eq!(selector.max_mirrors, 10);
    }

    #[test]
    fn test_mirror_creation() {
        let mirror = crate::etc::mirror_selector::Mirror::new(
            "https://mirror.example.com".to_string(),
            "US".to_string(),
            "New York".to_string(),
        );

        assert_eq!(mirror.url, "https://mirror.example.com");
        assert_eq!(mirror.country, "US");
        assert_eq!(mirror.city, "New York");
        assert!(mirror.is_active);
    }

    // Test package hooks
    #[test]
    fn test_hook_manager_basic() {
        let manager = crate::etc::package_hooks::HookManager::new();
        assert_eq!(manager.hooks.len(), 0);
        assert_eq!(manager.default_timeout, 300);
    }

    #[test]
    fn test_hook_creation() {
        let hook = crate::etc::package_hooks::Hook::new(
            "test-hook".to_string(),
            "Test hook".to_string(),
            crate::etc::package_hooks::HookType::PostInstall,
            "/usr/bin/test".to_string(),
        );

        assert_eq!(hook.name, "test-hook");
        assert_eq!(hook.hook_type, crate::etc::package_hooks::HookType::PostInstall);
    }

    // Test hardware detection
    #[test]
    fn test_hardware_detector_basic() {
        let detector = crate::etc::hardware_detection::HardwareDetector::new();
        assert_eq!(detector.cpus.len(), 0);
        assert_eq!(detector.storage_devices.len(), 0);
    }

    #[test]
    fn test_cpu_info() {
        let cpu = crate::etc::hardware_detection::CpuInfo::new(
            "Intel".to_string(),
            "0".to_string(),
            "Intel Core i7".to_string(),
        );

        assert_eq!(cpu.vendor, "Intel");
        assert_eq!(cpu.model_name, "Intel Core i7");
    }

    // Test boot manager
    #[test]
    fn test_boot_manager_basic() {
        let manager = crate::etc::boot_manager::BootManager::new();
        assert_eq!(manager.presets.len(), 0);
        assert_eq!(manager.current_preset, None);
    }

    #[test]
    fn test_boot_config_builder() {
        let config = crate::etc::boot_manager::BootConfig::new()
            .add_hook(crate::etc::boot_manager::InitcpioHook::Base)
            .add_module("ext4".to_string())
            .add_binary("fsck".to_string())
            .root_device("/dev/sda1".to_string());

        assert!(config.hooks.contains(&crate::etc::boot_manager::InitcpioHook::Base));
        assert!(config.modules.contains(&"ext4".to_string()));
        assert!(config.binaries.contains(&"fsck".to_string()));
        assert_eq!(config.root_device, Some("/dev/sda1".to_string()));
    }

    // Test filesystem manager
    #[test]
    fn test_filesystem_manager_basic() {
        let manager = crate::etc::filesystem::FilesystemManager::new();
        assert_eq!(manager.mounts.len(), 0);
        assert_eq!(manager.fstab_path, PathBuf::from("/etc/fstab"));
    }

    #[test]
    fn test_mount_entry_creation() {
        let entry = crate::etc::filesystem::MountEntry::new(
            "/dev/sda1",
            "/",
            crate::etc::filesystem::FilesystemType::Ext4,
        )
        .uuid("12345678-1234-1234-1234-123456789012".to_string())
        .options(crate::etc::filesystem::MountOptions::new().rw().relatime());

        assert_eq!(entry.device, "/dev/sda1");
        assert_eq!(entry.mount_point, PathBuf::from("/"));
        assert_eq!(entry.filesystem, crate::etc::filesystem::FilesystemType::Ext4);
        assert_eq!(entry.uuid, Some("12345678-1234-1234-1234-123456789012".to_string()));
    }

    // Test kernel module manager
    #[test]
    fn test_module_manager_basic() {
        let manager = crate::etc::kernel_modules::ModuleManager::new();
        assert_eq!(manager.modules.len(), 0);
        assert_eq!(manager.loaded_modules.len(), 0);
    }

    #[test]
    fn test_module_info_creation() {
        let module = crate::etc::kernel_modules::ModuleInfo::new(
            "test_module".to_string(),
            PathBuf::from("/lib/modules/test.ko"),
        )
        .size(1024)
        .dependency("dep1".to_string())
        .description("Test module".to_string());

        assert_eq!(module.name, "test_module");
        assert_eq!(module.size, 1024);
        assert_eq!(module.dependencies.len(), 1);
        assert_eq!(module.get_description(), "Test module");
    }

    #[test]
    fn test_module_blacklist() {
        let mut blacklist = crate::etc::kernel_modules::ModuleBlacklist::new("/tmp/test_blacklist.conf");
        blacklist.add("bad_module".to_string());

        assert!(blacklist.is_blacklisted("bad_module"));
        assert!(!blacklist.is_blacklisted("good_module"));
    }

    // Test PKGBUILD system
    #[test]
    fn test_pkg_source() {
        let url_source = crate::etc::pkgbuild::PkgSource::url("https://example.com/package.tar.gz");
        assert_eq!(url_source.get_source_string(), "https://example.com/package.tar.gz");
        assert!(!url_source.is_local());
        assert!(!url_source.is_vcs());

        let git_source = crate::etc::pkgbuild::PkgSource::git("https://github.com/user/repo.git", Some("main"));
        assert_eq!(git_source.get_source_string(), "https://github.com/user/repo.git#branch=main");
        assert!(git_source.is_vcs());

        let local_source = crate::etc::pkgbuild::PkgSource::local("/path/to/local");
        assert!(local_source.is_local());
    }

    #[test]
    fn test_pkg_info_builder() {
        let pkginfo = crate::etc::pkgbuild::PkgInfo::new("test-package", "1.0.0")
            .pkgdesc("Test package description")
            .url("https://example.com")
            .license("MIT")
            .depends("dep1")
            .depends("dep2")
            .source_url("https://example.com/package.tar.gz")
            .sha256sum("abc123");

        assert_eq!(pkginfo.pkgname, "test-package");
        assert_eq!(pkginfo.pkgver, "1.0.0");
        assert_eq!(pkginfo.pkgdesc, "Test package description");
        assert_eq!(pkginfo.license.len(), 1);
        assert_eq!(pkginfo.depends.len(), 2);
        assert_eq!(pkginfo.source.len(), 1);
        assert_eq!(pkginfo.get_full_version(), "1.0.0-1.test-package");
    }

    #[test]
    fn test_pkgbuild_parser() {
        let content = r#"
pkgname=test-package
pkgver=1.0.0
pkgrel=1
pkgdesc="Test package"
arch=("x86_64")
url="https://example.com"
license=("MIT")
depends=("dep1" "dep2")
source=("https://example.com/package.tar.gz")
sha256sums=("abc123")

build() {
    cd package-1.0.0
    make
}

package() {
    cd package-1.0.0
    make install DESTDIR=$pkgdir
}
"#;

        let mut parser = crate::etc::pkgbuild::PkgbuildParser::new(PathBuf::from("PKGBUILD"));
        parser.parse_content(content).unwrap();

        let pkginfo = parser.get_pkginfo().unwrap();
        assert_eq!(pkginfo.pkgname, "test-package");
        assert_eq!(pkginfo.pkgver, "1.0.0");
        assert_eq!(pkginfo.pkgdesc, "Test package");
        assert_eq!(pkginfo.depends.len(), 2);

        assert!(parser.get_build_function("build").is_some());
        assert!(parser.get_build_function("package").is_some());
    }

    // Test network manager
    #[test]
    fn test_connection_type() {
        assert_eq!(crate::etc::network_manager::ConnectionType::Ethernet.as_str(), "ethernet");
        assert_eq!(crate::etc::network_manager::ConnectionType::from_str("wifi"), Some(crate::etc::network_manager::ConnectionType::Wifi));
    }

    #[test]
    fn test_ip_method() {
        assert_eq!(crate::etc::network_manager::IpMethod::Dhcp.as_str(), "dhcp");
        assert_eq!(crate::etc::network_manager::IpMethod::from_str("static"), Some(crate::etc::network_manager::IpMethod::Static));
    }

    #[test]
    fn test_connection_profile_builder() {
        let profile = crate::etc::network_manager::ConnectionProfile::new(
            "home-wifi",
            crate::etc::network_manager::ConnectionType::Wifi,
            "wlan0",
        )
        .description("Home WiFi network")
        .ipv4_method(crate::etc::network_manager::IpMethod::Dhcp)
        .ipv6_method(crate::etc::network_manager::IpMethod::Auto6)
        .ssid("HomeNetwork")
        .password("password123")
        .priority(10)
        .autoconnect(true);

        assert_eq!(profile.name, "home-wifi");
        assert_eq!(profile.connection_type, crate::etc::network_manager::ConnectionType::Wifi);
        assert_eq!(profile.ssid, Some("HomeNetwork".to_string()));
        assert_eq!(profile.password, Some("password123".to_string()));
        assert_eq!(profile.priority, 10);
        assert!(profile.autoconnect);
    }

    #[test]
    fn test_network_interface() {
        let interface = crate::etc::network_manager::NetworkInterface::new(
            "eth0".to_string(),
            "00:11:22:33:44:55".to_string(),
        )
        .add_ip("192.168.1.100".parse().unwrap())
        .gateway("192.168.1.1".parse().unwrap())
        .mtu(1500)
        .speed(1000)
        .duplex("full")
        .operstate("up")
        .state(true, true, true)
        .stats(1000000, 500000, 1000, 500);

        assert_eq!(interface.name, "eth0");
        assert_eq!(interface.mac_address, "00:11:22:33:44:55");
        assert_eq!(interface.ip_addresses.len(), 1);
        assert!(interface.has_ip());
        assert!(interface.is_up());
        assert_eq!(interface.get_total_bytes(), 1500000);
    }

    // Test system installer
    #[test]
    fn test_install_target() {
        assert_eq!(crate::etc::system_installer::InstallTarget::Disk.as_str(), "disk");
        assert_eq!(crate::etc::system_installer::InstallTarget::from_str("usb"), Some(crate::etc::system_installer::InstallTarget::Usb));
    }

    #[test]
    fn test_partition_scheme() {
        assert_eq!(crate::etc::system_installer::PartitionScheme::Gpt.as_str(), "gpt");
        assert_eq!(crate::etc::system_installer::PartitionScheme::from_str("mbr"), Some(crate::etc::system_installer::PartitionScheme::Mbr));
    }

    #[test]
    fn test_install_filesystem() {
        assert_eq!(crate::etc::system_installer::InstallFilesystem::Ext4.as_str(), "ext4");
        assert_eq!(crate::etc::system_installer::InstallFilesystem::from_str("btrfs"), Some(crate::etc::system_installer::InstallFilesystem::Btrfs));
        assert_eq!(crate::etc::system_installer::InstallFilesystem::Ext4.get_mkfs_command(), "mkfs.ext4");
    }

    #[test]
    fn test_boot_loader() {
        assert_eq!(crate::etc::system_installer::BootLoader::SystemdBoot.as_str(), "systemd-boot");
        assert_eq!(crate::etc::system_installer::BootLoader::from_str("grub"), Some(crate::etc::system_installer::BootLoader::Grub));
        assert!(crate::etc::system_installer::BootLoader::SystemdBoot.get_install_command().is_some());
    }

    #[test]
    fn test_install_step() {
        assert_eq!(crate::etc::system_installer::InstallStep::PartitionDisk.as_str(), "partition-disk");
        assert_eq!(crate::etc::system_installer::InstallStep::PartitionDisk.get_order(), 2);
        assert!(crate::etc::system_installer::InstallStep::DetectHardware.get_order() < crate::etc::system_installer::InstallStep::Finalize.get_order());
    }

    #[test]
    fn test_partition_config() {
        let partition = crate::etc::system_installer::PartitionConfig::new(
            "/dev/sda",
            1,
            "512M",
            crate::etc::system_installer::InstallFilesystem::Vfat,
        )
        .esp()
        .boot();

        assert_eq!(partition.device, "/dev/sda");
        assert_eq!(partition.partition_number, 1);
        assert_eq!(partition.size, "512M");
        assert!(partition.is_esp);
        assert!(partition.is_boot);
    }

    #[test]
    fn test_install_config_builder() {
        let config = crate::etc::system_installer::InstallConfig::new()
            .target(crate::etc::system_installer::InstallTarget::Disk)
            .target_device("/dev/sda")
            .partition_scheme(crate::etc::system_installer::PartitionScheme::Gpt)
            .bootloader(crate::etc::system_installer::BootLoader::SystemdBoot)
            .kernel("linux")
            .hostname("myhost")
            .timezone("America/New_York")
            .locale("en_US.UTF-8")
            .keyboard_layout("us")
            .desktop_environment("gnome")
            .display_manager("gdm")
            .user("user", "password")
            .package("base")
            .package("linux")
            .mirror("https://mirror.example.com")
            .enable_firewall(true)
            .enable_sshd(true);

        assert_eq!(config.target, crate::etc::system_installer::InstallTarget::Disk);
        assert_eq!(config.target_device, Some("/dev/sda".to_string()));
        assert_eq!(config.bootloader, crate::etc::system_installer::BootLoader::SystemdBoot);
        assert_eq!(config.kernel, "linux");
        assert_eq!(config.hostname, "myhost");
        assert_eq!(config.desktop_environment, Some("gnome".to_string()));
        assert_eq!(config.user_name, Some("user".to_string()));
        assert_eq!(config.packages.len(), 2);
        assert!(config.enable_firewall);
        assert!(config.enable_sshd);
    }

    // Test boot loader
    #[test]
    fn test_boot_loader_type() {
        assert_eq!(crate::etc::boot_loader::BootLoaderType::SystemdBoot.as_str(), "systemd-boot");
        assert_eq!(crate::etc::boot_loader::BootLoaderType::from_str("grub"), Some(crate::etc::boot_loader::BootLoaderType::Grub));
        assert_eq!(crate::etc::boot_loader::BootLoaderType::SystemdBoot.get_config_dir(), PathBuf::from("/boot/EFI/Linux"));
    }

    #[test]
    fn test_kernel_entry() {
        let kernel = crate::etc::boot_loader::KernelEntry::new("Linux 6.1", "/boot/vmlinuz-linux")
            .initrd("/boot/initramfs-linux.img")
            .cmdline("root=/dev/sda1 quiet")
            .description("Linux kernel")
            .default();

        assert_eq!(kernel.title, "Linux 6.1");
        assert_eq!(kernel.kernel_path, PathBuf::from("/boot/vmlinuz-linux"));
        assert!(kernel.is_default);
        assert_eq!(kernel.get_kernel_filename(), "vmlinuz-linux");
    }

    #[test]
    fn test_boot_entry() {
        let kernel = crate::etc::boot_loader::KernelEntry::new("Linux", "/boot/vmlinuz-linux");
        let entry = crate::etc::boot_loader::BootEntry::new("Linux 6.1", kernel)
            .description("Linux kernel 6.1")
            .icon("linux.png")
            .sort_order(10)
            .default();

        assert_eq!(entry.title, "Linux 6.1");
        assert_eq!(entry.description, Some("Linux kernel 6.1".to_string()));
        assert_eq!(entry.icon, Some("linux.png".to_string()));
        assert_eq!(entry.sort_order, 10);
        assert!(entry.is_default);
    }

    #[test]
    fn test_boot_loader_config_builder() {
        let config = crate::etc::boot_loader::BootLoaderConfig::new(crate::etc::boot_loader::BootLoaderType::SystemdBoot)
            .default_entry("sigmaos")
            .timeout(10)
            .theme("sigmaos-theme")
            .resolution("1920x1080")
            .kernel_param("quiet")
            .kernel_param("loglevel=3")
            .secure_boot(true)
            .quiet_boot(true);

        assert_eq!(config.boot_loader_type, crate::etc::boot_loader::BootLoaderType::SystemdBoot);
        assert_eq!(config.default_entry, Some("sigmaos".to_string()));
        assert_eq!(config.timeout, 10);
        assert_eq!(config.theme, Some("sigmaos-theme".to_string()));
        assert!(config.secure_boot);
        assert!(config.quiet_boot);
        assert_eq!(config.kernel_params.len(), 2);
    }

    // Test user manager
    #[test]
    fn test_user_info() {
        let user = crate::etc::user_manager::UserInfo::new("testuser", 1000, 1000)
            .gecos("Test User")
            .shell("/bin/bash")
            .password_hash("$6$hash")
            .add_group("wheel")
            .add_group("users");

        assert_eq!(user.username, "testuser");
        assert_eq!(user.uid, 1000);
        assert_eq!(user.gid, 1000);
        assert_eq!(user.gecos, "Test User");
        assert_eq!(user.shell, "/bin/bash");
        assert_eq!(user.groups.len(), 2);
        assert!(!user.is_locked);
        assert!(user.can_login());
    }

    #[test]
    fn test_group_info() {
        let group = crate::etc::user_manager::GroupInfo::new("testgroup", 1000)
            .add_member("user1")
            .add_member("user2")
            .system_group(false);

        assert_eq!(group.groupname, "testgroup");
        assert_eq!(group.gid, 1000);
        assert_eq!(group.members.len(), 2);
        assert!(!group.is_system_group);
        assert!(group.has_member("user1"));
        assert!(!group.has_member("user3"));
    }

    #[test]
    fn test_user_manager() {
        let mut manager = crate::etc::user_manager::UserManager::new();

        let user = crate::etc::user_manager::UserInfo::new("testuser", 1000, 1000)
            .gecos("Test User")
            .shell("/bin/bash");

        assert!(manager.add_user(user).is_ok());
        assert!(manager.get_user("testuser").is_some());
        assert_eq!(manager.get_all_users().len(), 1);
    }

    #[test]
    fn test_group_management() {
        let mut manager = crate::etc::user_manager::UserManager::new();

        let group = crate::etc::user_manager::GroupInfo::new("testgroup", 1000);
        assert!(manager.add_group(group).is_ok());
        assert!(manager.get_group("testgroup").is_some());
        assert_eq!(manager.get_all_groups().len(), 1);
    }

    #[test]
    fn test_user_group_membership() {
        let mut manager = crate::etc::user_manager::UserManager::new();

        let user = crate::etc::user_manager::UserInfo::new("testuser", 1000, 1000);
        let group = crate::etc::user_manager::GroupInfo::new("testgroup", 1000);

        manager.add_user(user).unwrap();
        manager.add_group(group).unwrap();

        assert!(manager.add_user_to_group("testuser", "testgroup").is_ok());
        assert!(manager.get_user_groups("testuser").contains(&"testgroup".to_string()));
        assert!(manager.get_group_members("testgroup").contains(&"testuser".to_string()));
    }

    #[test]
    fn test_password_management() {
        let mut manager = crate::etc::user_manager::UserManager::new();

        let user = crate::etc::user_manager::UserInfo::new("testuser", 1000, 1000);
        manager.add_user(user).unwrap();

        assert!(manager.change_password("testuser", "newpassword").is_ok());
        let user = manager.get_user("testuser").unwrap();
        assert!(user.password_hash.is_some());
        assert!(user.password_last_changed.is_some());
    }

    #[test]
    fn test_user_locking() {
        let mut manager = crate::etc::user_manager::UserManager::new();

        let user = crate::etc::user_manager::UserInfo::new("testuser", 1000, 1000);
        manager.add_user(user).unwrap();

        assert!(manager.lock_user("testuser").is_ok());
        assert!(manager.get_user("testuser").unwrap().is_locked);

        assert!(manager.unlock_user("testuser").is_ok());
        assert!(!manager.get_user("testuser").unwrap().is_locked);
    }

    #[test]
    fn test_create_user() {
        let mut manager = crate::etc::user_manager::UserManager::new();

        let result = manager.create_user("newuser", "password", "New User", &["wheel", "users"]);
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.username, "newuser");
        assert_eq!(user.uid, 1000);
        assert_eq!(user.gid, 1000);
        assert!(user.groups.contains(&"wheel".to_string()));
        assert!(user.groups.contains(&"users".to_string()));
    }

    #[test]
    fn test_user_export_import() {
        let mut manager = crate::etc::user_manager::UserManager::new();

        let user = crate::etc::user_manager::UserInfo::new("testuser", 1000, 1000)
            .gecos("Test User")
            .shell("/bin/bash")
            .add_group("wheel");

        manager.add_user(user).unwrap();

        let group = crate::etc::user_manager::GroupInfo::new("wheel", 10)
            .add_member("testuser");
        manager.add_group(group).unwrap();

        let export = manager.export_users();
        assert!(export.contains("testuser"));
        assert!(export.contains("wheel"));

        let mut manager2 = crate::etc::user_manager::UserManager::new();
        assert!(manager2.import_users(&export).is_ok());
        assert!(manager2.get_user("testuser").is_some());
    }

    #[test]
    fn test_user_statistics() {
        let mut manager = crate::etc::user_manager::UserManager::new();

        let user1 = crate::etc::user_manager::UserInfo::new("user1", 1000, 1000).system_user(false);
        let user2 = crate::etc::user_manager::UserInfo::new("user2", 1001, 1001).system_user(false).locked(true);
        let user3 = crate::etc::user_manager::UserInfo::new("system", 0, 0).system_user(true);

        manager.add_user(user1).unwrap();
        manager.add_user(user2).unwrap();
        manager.add_user(user3).unwrap();

        let group = crate::etc::user_manager::GroupInfo::new("users", 100);
        manager.add_group(group).unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_users, 3);
        assert_eq!(stats.system_users, 1);
        assert_eq!(stats.regular_users, 2);
        assert_eq!(stats.locked_users, 1);
        assert_eq!(stats.total_groups, 1);
    }
}
