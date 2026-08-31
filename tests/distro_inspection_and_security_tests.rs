#![allow(clippy::all, warnings)]
// SPDX-License-Identifier: MIT
// SigmaOS Distro Inspection & Security Unit Test Suite
// Inspired by Linux and BSD distribution innovations:
// - Alpine APK checksum & package parsing
// - Void XBPS package metadata parsing & runit integration
// - NixOS atomic generation store & rollback tracking
// - Gentoo Portage USE-flags, slots, and ebuild specs

#[path = "../src/sigpkg/universal_engine.rs"]
mod universal_engine;

#[path = "../src/sigpkg/portage.rs"]
mod portage;

#[path = "../src/system/generation_manager.rs"]
mod generation_manager;

use generation_manager::*;
use portage::*;
use universal_engine::*;

#[test]
fn test_alpine_apk_package_adapter_inspection() {
    let adapter = ApkPackageAdapter;
    assert_eq!(adapter.format(), PackageFormat::Apk);

    let raw_payload = b"alpine package binary content";
    let context = adapter.parse_package(raw_payload).unwrap();
    assert_eq!(context.name, "apk-compat-pkg");
    assert_eq!(context.version, "3.18.0");
    assert_eq!(context.format, PackageFormat::Apk);

    assert!(adapter
        .extract_to_store(&context, "/store/test-apk-node")
        .is_ok());
}

#[test]
fn test_void_xbps_package_adapter_inspection() {
    let adapter = XbpsPackageAdapter::new(Some("runit-daemon".to_string()));
    assert_eq!(adapter.format(), PackageFormat::Xbps);

    let raw_xbps = b"XBPS_META_PKG=runit\nVERSION=2.1.2_5\n";
    let context = adapter.parse_package(raw_xbps).unwrap();
    assert_eq!(context.format, PackageFormat::Xbps);
    assert_eq!(context.name, "xbps-compat-pkg");

    assert!(adapter
        .extract_to_store(&context, "/store/test-xbps-node")
        .is_ok());
}

#[test]
fn test_nixos_generation_rollback_inspection() {
    let mut mgr = GenerationManager::new();

    // Create Generation 1 (Root Inode 0x1000, Timestamp 1718900000)
    let gen1_id = mgr.create_generation(0x1000, 1718900000).unwrap();
    assert_eq!(gen1_id, 1);

    // Create Generation 2 (Root Inode 0x2000, Timestamp 1718910000)
    let gen2_id = mgr.create_generation(0x2000, 1718910000).unwrap();
    assert_eq!(gen2_id, 2);

    // Swap active generation to Gen 2
    let active_inode = mgr.swap_active_generation(2).unwrap();
    assert_eq!(active_inode, 0x2000);
    assert_eq!(mgr.get_active_generation().unwrap().id, 2);

    // Rollback to Gen 1
    let rollback_inode = mgr.swap_active_generation(1).unwrap();
    assert_eq!(rollback_inode, 0x1000);
    assert_eq!(mgr.get_active_generation().unwrap().id, 1);
}

#[test]
fn test_gentoo_portage_resolver_inspection() {
    let mut resolver = PortageResolver::new("amd64".to_string());
    resolver.set_use_flag("wayland".to_string(), true);
    resolver.set_use_flag("systemd".to_string(), false);

    assert_eq!(resolver.get_use_flag("wayland"), Some(true));
    assert_eq!(resolver.get_use_flag("systemd"), Some(false));

    let lib_spec = EbuildSpec::new("libwayland".to_string(), Version::new(1, 22, 0))
        .with_description("Wayland compositor protocol library".to_string());
    resolver.add_package(lib_spec);

    let app_spec = EbuildSpec::new("weston".to_string(), Version::new(13, 0, 0)).with_dependencies(
        DependencyCondition::Package {
            name: "libwayland".to_string(),
            version_constraint: VersionConstraint::Any,
            slot: None,
            use_flags: Vec::new(),
        },
    );
    resolver.add_package(app_spec);

    let deps = resolver.resolve_dependencies("weston").unwrap();
    assert!(deps.contains(&"libwayland".to_string()));
}
