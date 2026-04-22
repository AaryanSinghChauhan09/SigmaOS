/**
 * SigmaOS Sovereign Kernel Loader
 * Modularizes the loading of 50+ functional shards.
 */

const SYSTEM_MODULES = [
    "scripts/modules/00_accessibility.js",
    "scripts/modules/00_aether_browser.js",
    "scripts/modules/00_anims.js",
    "scripts/modules/00_audio_engine.js",
    "scripts/modules/00_canvas_anims.js",
    "scripts/modules/00_clipboard.js",
    "scripts/modules/00_event_bus.js",
    "scripts/modules/00_installer_engine.js",
    "scripts/modules/00_lattice_sync.js",
    "scripts/modules/00_localization.js",
    "scripts/modules/00_log_processor.js",
    "scripts/modules/00_neural_interface.js",
    "scripts/modules/00_notifications.js",
    "scripts/modules/00_process_manager.js",
    "scripts/modules/00_recovery_hub.js",
    "scripts/modules/00_registry.js",
    "scripts/modules/00_search_engine.js",
    "scripts/modules/00_shard_orchestrator.js",
    "scripts/modules/00_silicon_primitives.js",
    "scripts/modules/00_sovereign_framework.js",
    "scripts/modules/00_string_engine.js",
    "scripts/modules/00_theme_engine.js",
    "scripts/modules/00_ui_utils.js",
    "scripts/modules/00_vitals_engine.js",
    "scripts/modules/00_vitals_service.js",
    "scripts/modules/00_zenith_desktop.js",
    "scripts/modules/01_config.js",
    "scripts/modules/01_settings_engine.js",
    "scripts/modules/02_browser_use__dom_sweep_protocol.js",
    "scripts/modules/02_cli_mode__sovereign_shell.js",
    "scripts/modules/02_dashboard_orchestrator.js",
    "scripts/modules/02_filesystem.js",
    "scripts/modules/02_globals.js",
    "scripts/modules/02_gui_mode__zenith_dashboard.js",
    "scripts/modules/02_lattice_config_shard.js",
    "scripts/modules/02_lattice_visualizer.js",
    "scripts/modules/02_neural_search_spotlight.js",
    "scripts/modules/02_omnibox__neural_search.js",
    "scripts/modules/02_paradigm_engine.js",
    "scripts/modules/02_paradigm_selection__boot_menu_.js",
    "scripts/modules/02_shell_engine.js",
    "scripts/modules/02_telemetry.js",
    "scripts/modules/03_nexus_app_store.js",
    "scripts/modules/03_nexus_engine.js",
    "scripts/modules/04_layout_engine.js",
    "scripts/modules/04_zenith_window_manager.js",
    "scripts/modules/05_analytics_engine.js",
    "scripts/modules/05_effects_engine.js",
    "scripts/modules/05_environment.js",
    "scripts/modules/05_mission_control.js",
    "scripts/modules/05_premium_vitals.js",
    "scripts/modules/05_sovereign_stress_tester.js",
    "scripts/modules/05_task_manager.js",
    "scripts/modules/06_zenith_orchestrator.js",
    "scripts/modules/07_lattice_visualizer_v2.js",
    "scripts/modules/08_sovereign_automations.js",
    "scripts/modules/09_aether_pulse.js",
    "scripts/modules/09_mica_flux.js",
    "scripts/modules/10_sovereign_taskbar.js",
    "scripts/modules/11_sovereign_explorer.js",
    "scripts/modules/12_sovereign_theme_engine.js",
    "scripts/modules/13_workspace_orchestrator.js",
    "scripts/modules/14_zenith_search_v2.js",
    "scripts/modules/15_local_inference_shard.js",
    "scripts/modules/16_ghost_sharding_sim.js",
    "scripts/modules/17_sovereign_terminal_core.js",
    "scripts/modules/18_sovereign_settings.js",
    "scripts/modules/19_apex_telemetry_viz.js",
    "scripts/modules/20_sigma_vault.js",
    "scripts/modules/21_lattice_snapshot_engine.js",
    "scripts/modules/22_tiling_engine.js",
    "scripts/modules/23_sovereign_handoff.js",
    "scripts/modules/24_neural_bridge_ai.js",
    "scripts/modules/25_sovereign_sandbox.js",
    "scripts/modules/26_multi_tenancy_engine.js",
    "scripts/modules/27_sovereign_profiler.js",
    "scripts/modules/28_shortcut_orchestrator.js",
    "scripts/modules/29_notification_center.js",
    "scripts/modules/30_sovereign_window_manager.js",
    "scripts/modules/31_sovereign_focus_mode.js",
    "scripts/modules/32_sovereign_quick_look.js",
    "scripts/modules/33_sentinel_neural_firewall.js",
    "scripts/modules/34_sovereign_adaptive_pager.js",
    "scripts/modules/35_plugin_loader.js",
    "scripts/modules/36_config_bridge.js",
    "scripts/modules/37_settings_panel.js",
    "scripts/modules/38_shard_dashboard.js",
    "scripts/modules/39_quantum_scheduler.js",
    "scripts/modules/40_memory_compactor.js",
    "scripts/modules/41_snap_layouts.js",
    "scripts/modules/42_time_machine_backup.js",
    "scripts/modules/43_declarative_state.js",
    "scripts/modules/44_dynamic_island.js",
    "scripts/modules/45_quantum_link_device.js",
    "scripts/modules/46_fuchsia_sandbox.js",
    "scripts/modules/47_universal_control.js",
    "scripts/modules/48_neural_search.js",
    "scripts/modules/49_quantum_theme_engine.js",
    "scripts/modules/50_system_vitals_hud.js",
    "scripts/modules/51_stage_manager.js",
    "scripts/modules/52_biometric_auth.js",
    "scripts/modules/53_aero_glass.js",
    "scripts/modules/54_smart_tab_workflows.js",
    "scripts/modules/55_task_linked_tabs.js",
    "scripts/modules/56_adaptive_tab_memory.js",
    "scripts/modules/57_learning_integration.js",
    "scripts/modules/58_workspace_collaboration.js",
    "scripts/modules/59_github_integration.js",
    "scripts/modules/60_code_snippet_manager.js",
    "scripts/modules/61_api_playground.js",
    "scripts/modules/62_academic_mode.js",
    "scripts/modules/63_offline_continuity.js",
    "scripts/modules/64_workspace_templates.js",
    "scripts/modules/65_privacy_layer.js",
    "scripts/modules/66_utility_sidebar.js",
    "scripts/modules/67_workflow_scripting.js",
    "scripts/modules/68_learning_mode.js",
    "scripts/modules/69_coding_companion.js",
    "scripts/modules/70_citation_collector.js",
    "scripts/modules/71_workspace_ai_assistant.js",
    "scripts/modules/72_live_co_browsing.js",
    "scripts/modules/73_lecture_mode.js",
    "scripts/modules/74_quiz_generator.js",
    "scripts/modules/75_study_group_mode.js",
    "scripts/modules/76_debug_mode.js",
    "scripts/modules/77_comment_layer.js",
    "scripts/modules/78_versioned_workspaces.js",
    "scripts/modules/79_workspace_chat.js",
    "scripts/modules/80_task_suggestions.js",
    "scripts/modules/81_offline_study_mode.js",
    "scripts/modules/82_gamification_engine.js",
    "scripts/audit.js"
];

async function loadSystem() {
    console.log("Σ://BOOT> Initiating Modular Lattice Loading...");
    
    for (const module of SYSTEM_MODULES) {
        try {
            await loadScript(module);
        } catch (e) {
            console.error(`Σ://BOOT_ERR> Failed to load shard: ${module}`, e);
        }
    }
    
    console.log("Σ://BOOT> All Shards Integrated. Launching Zenith...");
    if (window.dispatchEvent) {
        window.dispatchEvent(new Event('load'));
    }
}

function loadScript(src) {
    return new Promise((resolve, reject) => {
        const script = document.createElement('script');
        script.src = src;
        script.onload = resolve;
        script.onerror = reject;
        document.head.appendChild(script);
    });
}

// Initial Bootstrap
loadSystem();
