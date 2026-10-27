// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Independent Complete OS Engine
// (`tests/test_sovereign_independent_complete_os_engine.rs`)

#[path = "../src/distro/sovereign_independent_complete_os_engine.rs"]
mod sovereign_independent_complete_os_engine;

use sovereign_independent_complete_os_engine::*;

#[test]
fn test_sovereign_craft_creative_matrix() {
    // Arrange
    let matrix = SovereignCraftCreativeMatrix::new();

    // Act
    let count = matrix.total_module_count();
    let res_3d = matrix.execute_creative_pipeline("3D", "CyberpunkCity.blend");
    let res_sound = matrix.execute_creative_pipeline("Sound", "SynthMaster.wav");

    // Assert
    assert_eq!(count, 20);
    assert!(res_3d.contains("3D"));
    assert!(res_3d.contains("CyberpunkCity.blend"));
    assert!(res_sound.contains("Sound"));
    assert!(matrix.spatial_render_pipeline_active);
}

#[test]
fn test_sovereign_office_productivity_suite() {
    // Arrange
    let office = SovereignOfficeEngine::new();

    // Act
    let writer_res = office.dispatch_office_task("Writer", "AnnualReport.odt");
    let calc_res = office.dispatch_office_task("Calc", "QuarterlyFinancials.ods");
    let impress_res = office.dispatch_office_task("Impress", "ProductKeynote.odp");
    let draw_res = office.dispatch_office_task("Draw", "SystemArchitecture.odg");
    let math_res = office.dispatch_office_task("Math", "SchrodingerEquation");
    let base_res = office.dispatch_office_task("Base", "EnterpriseLedger.odb");

    // Assert
    assert!(writer_res.contains("WordCraft"));
    assert!(calc_res.contains("SheetCraft"));
    assert!(impress_res.contains("DeckCraft"));
    assert!(draw_res.contains("DrawCraft"));
    assert!(math_res.contains("MathCraft"));
    assert!(base_res.contains("BaseCraft"));
}

#[test]
fn test_sovereign_unified_data_engine() {
    // Arrange
    let data_engine = SovereignUnifiedDataEngine::new();

    // Act
    let query_res =
        data_engine.execute_hybrid_query("SELECT user_id, count(*) FROM events GROUP BY user_id");
    let bi_res = data_engine.render_bi_dashboard("Executive Global Overview");

    // Assert
    assert!(query_res.contains("DuckDB SIMD"));
    assert!(query_res.contains("Arrow zero-copy"));
    assert!(bi_res.contains("Executive Global Overview"));
    assert_eq!(data_engine.postgres_wire_v3_port, 5432);
}

#[test]
fn test_sovereign_ai_developer_subsystem() {
    // Arrange
    let dev_engine = SovereignAiDeveloperEngine::new();

    // Act
    let blender_res = dev_engine.trigger_developer_service("Blender");
    let agents_res = dev_engine.trigger_developer_service("Agents");
    let terminal_res = dev_engine.trigger_developer_service("Terminal");
    let editor_res = dev_engine.trigger_developer_service("Editor");
    let containers_res = dev_engine.trigger_developer_service("Containers");
    let robotics_res = dev_engine.trigger_developer_service("Robotics");
    let vlc_res = dev_engine.trigger_developer_service("VLC");

    // Assert
    assert!(blender_res.contains("3D Studio"));
    assert!(agents_res.contains("Autonomous task planner"));
    assert!(terminal_res.contains("Warp Terminal"));
    assert!(editor_res.contains("Scintilla buffer"));
    assert!(containers_res.contains("namespaces"));
    assert!(robotics_res.contains("actuators"));
    assert!(vlc_res.contains("codecs"));
}

#[test]
fn test_sovereign_universal_kernel_gateway() {
    // Arrange
    let gateway = SovereignUniversalKernelGateway::new();

    // Act
    let bsd_call = gateway.resolve_abi_call("BSD", 20);
    let dos_call = gateway.resolve_abi_call("DOS", 0x21);
    let linux_call = gateway.resolve_abi_call("Linux", 1);
    let hurd_call = gateway.resolve_abi_call("Hurd", 100);
    let minix_call = gateway.resolve_abi_call("Minix", 5);
    let reactos_call = gateway.resolve_abi_call("ReactOS", 10);
    let xnu_call = gateway.resolve_abi_call("XNU", 33);

    // Assert
    assert!(bsd_call.contains("Capsicum rights"));
    assert!(dos_call.contains("NTVDM"));
    assert!(linux_call.contains("zero-copy VFS"));
    assert!(hurd_call.contains("Mach 3.0 IPC"));
    assert!(minix_call.contains("Reincarnation Server"));
    assert!(reactos_call.contains("IRP"));
    assert!(xnu_call.contains("Mach-trap"));
}

#[test]
fn test_sovereign_master_coordinator_full_audit() {
    // Arrange
    let coordinator = SovereignCompleteOsMasterCoordinator::new();

    // Act
    let audit = coordinator.audit_system_sovereignty();
    let manifest = coordinator.print_sovereignty_manifest();

    // Assert
    assert!(audit.get("sovereignty_score").unwrap().contains("98.5%"));
    assert_eq!(audit.get("craft_modules_count").unwrap(), "20");
    assert_eq!(
        audit.get("independence_level").unwrap(),
        "100% Pure Clean-Room Rust"
    );
    assert!(manifest.contains("Creative Suite: 20 Craft Modules"));
    assert!(manifest.contains("Office Suite: Native Writer, Calc, Impress"));
    assert!(manifest.contains("Universal ABI Translation"));
}
