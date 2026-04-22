/**
 * SigmaOS Sovereign Kernel Loader
 * Modularizes the loading of 50+ functional shards.
 */

const SYSTEM_MODULES = [
    "scripts/modules/00_sovereign_framework.js",
    "scripts/modules/00_ui_utils.js",
    "scripts/modules/00_event_bus.js",
    "scripts/modules/00_zenith_desktop.js",
    "scripts/modules/00_shard_orchestrator.js",
    "scripts/modules/00_vitals_service.js",
    "scripts/modules/00_accessibility.js",
    "scripts/modules/00_log_processor.js",
    "scripts/modules/00_neural_interface.js",
    "scripts/modules/00_process_manager.js",
    "scripts/modules/00_recovery_hub.js",
    "scripts/modules/00_string_engine.js",
    "scripts/modules/00_audio_engine.js",
    "scripts/modules/00_vitals_engine.js",
    "scripts/modules/00_lattice_sync.js",
    "scripts/modules/00_localization.js",
    "scripts/modules/00_registry.js",
    "scripts/modules/00_aether_browser.js",
    "scripts/modules/02_neural_search_spotlight.js",
    "scripts/modules/02_lattice_config_shard.js",
    "scripts/modules/00_theme_engine.js",
    "scripts/modules/00_notifications.js",
    "scripts/modules/00_canvas_anims.js",
    "scripts/modules/00_clipboard.js",
    "scripts/modules/00_search_engine.js",
    "scripts/modules/01_settings_engine.js",
    "scripts/modules/01_config.js",
    "scripts/modules/02_globals.js",
    "scripts/modules/02_telemetry.js",
    "scripts/modules/02_filesystem.js",
    "scripts/modules/02_paradigm_engine.js",
    "scripts/modules/02_paradigm_selection__boot_menu_.js",
    "scripts/modules/02_dashboard_orchestrator.js",
    "scripts/modules/02_lattice_visualizer.js",
    "scripts/modules/02_gui_mode__zenith_dashboard.js",
    "scripts/modules/02_browser_use__dom_sweep_protocol.js",
    "scripts/modules/02_shell_engine.js",
    "scripts/modules/02_cli_mode__sovereign_shell.js",
    "scripts/modules/02_omnibox__neural_search.js",
    "scripts/modules/03_nexus_engine.js",
    "scripts/modules/03_nexus_app_store.js",
    "scripts/modules/04_layout_engine.js",
    "scripts/modules/04_zenith_window_manager.js",
    "scripts/modules/05_environment.js",
    "scripts/modules/05_effects_engine.js",
    "scripts/modules/05_analytics_engine.js",
    "scripts/modules/05_task_manager.js",
    "scripts/modules/05_mission_control.js",
    "scripts/modules/05_premium_vitals.js",
    "scripts/modules/05_sovereign_stress_tester.js",
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
