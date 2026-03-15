"""
mode_manager_core.py — SigmaModeManager facade
================================================
The class SigmaModeManager is preserved here for backward compatibility.
All individual routines live in the mode_manager/routines/ sub-package;
each routine is its own file (one function = one file).
"""

from typing import Dict, List, Any, Callable, Optional
import time

# ── Import every routine from its own dedicated file ──────────────────────────
from userland.system_api.mode_manager.routines.log_mode_change import log_mode_change
from userland.system_api.mode_manager.routines.notifications import (
    disable_notifications, enable_notifications
)
from userland.system_api.mode_manager.routines.ui_theme import (
    set_gaming_ui_theme, reset_ui_theme
)
from userland.system_api.mode_manager.routines.creative import (
    launch_creative_suite, optimize_disk_cache, flush_disk_cache
)
from userland.system_api.mode_manager.routines.automation import (
    start_automation_agent, isolate_network_traffic,
    stop_automation_agent, restore_network_traffic
)
from userland.system_api.mode_manager.routines.mesh import (
    forge_global_mesh, spawn_hyper_swarm, build_cognitive_dag, cooldown_swarm
)
from userland.system_api.mode_manager.routines.ads import (
    activate_ad_blocker, open_shopping_browser, deactivate_ad_blocker
)
from userland.system_api.mode_manager.routines.display import (
    monitor_cpu_temp, dim_display, restore_display,
    calibrate_display, reset_display_calibration
)
from userland.system_api.mode_manager.routines.animations import (
    disable_animations, enable_animations
)
from userland.system_api.mode_manager.routines.console import (
    switch_to_text_console, kill_gui_processes,
    start_gui_processes, switch_to_graphical_console
)
from userland.system_api.mode_manager.routines.ai_frameworks import (
    load_ai_frameworks, allocate_vram, unload_ai_frameworks,
    deallocate_vram, activate_intelligence_suite
)
from userland.system_api.mode_manager.routines.data_lakes import (
    mount_data_lakes, start_jupyter_lab, unmount_data_lakes
)
from userland.system_api.mode_manager.routines.vpn import activate_vpn, deactivate_vpn
from userland.system_api.mode_manager.routines.legal import launch_legal_suite
from userland.system_api.mode_manager.routines.dev_environment import (
    start_dev_environment, enable_code_completion, stop_dev_environment
)
from userland.system_api.mode_manager.routines.presentation import (
    mute_system_sounds, start_presentation_software, unmute_system_sounds
)
from userland.system_api.mode_manager.routines.wifi import (
    disconnect_wifi, enable_offline_sync, connect_wifi
)
from userland.system_api.mode_manager.routines.diagnostics import (
    run_system_diagnostics, isolate_network, reboot_system
)
from userland.system_api.mode_manager.routines.hyper_drive import (
    engage_hyper_drive, activate_zen_latency, disengage_hyper_drive
)
from userland.system_api.mode_manager.routines.security import (
    run_compliance_audit, seal_all_vaults, activate_ghost_mask,
    scrub_recent_media, unseal_standard_vaults
)
from userland.system_api.mode_manager.routines.focus import (
    start_focus_timer, stop_focus_timer
)
from userland.system_api.mode_manager.routines.design import launch_design_software
from userland.system_api.mode_manager.routines.bi_dashboard import launch_bi_dashboard


# ─────────────────────────────────────────────────────────────────────────────
# Mode definitions (data only — no logic here)
# ─────────────────────────────────────────────────────────────────────────────
_MODES: Dict[str, Dict] = {
    "Standard": {
        "CPU_Priority": "Balanced", "GPU_Profile": "Adaptive",
        "RAM_Focus": "System_Stability", "Storage_IO": "Balanced",
        "Network_Bandwidth": "Adaptive_QoS", "Background_Task_Limit": 10,
        "Description": "General purpose sovereign computing.",
        "AI_Config": {"max_depth": 3, "max_tokens": 1024, "style": "Helpful", "tool_budget": "Medium"},
        "Kernel_Flags": [], "Routines_On_Exit": [],
    },
    "Stability": {
        "CPU_Priority": "Strict_Balanced", "GPU_Profile": "Locked_Stable",
        "RAM_Focus": "Redundant_Checks / Parity", "Background_Task_Limit": 5,
        "Description": "Maximum system reliability and error checking.",
        "AI_Config": {"max_depth": 3, "max_tokens": 512, "style": "Reliable", "tool_budget": "Low"},
        "Kernel_Flags": ["paranoiac-fs", "watchdog-high-freq"],
        "Routines_On_Enter": ["enable_full_auditing", "start_stability_watchdog"],
        "Routines_On_Exit": ["stop_stability_watchdog"],
    },
    "Gaming": {
        "CPU_Priority": "High (Max Frequency)", "GPU_Profile": "High-Performance / Unlocked",
        "RAM_Focus": "Direct_Access / Zero_Swap", "Storage_IO": "NVMe_Bypass_DirectStorage",
        "Network_Bandwidth": "Aggressive_Ping_Prioritization", "Background_Task_Limit": 2,
        "Description": "Maximum throughput for immersive gaming.",
        "AI_Config": {"max_depth": 2, "max_tokens": 512, "style": "Playful", "tool_budget": "Low"},
        "Kernel_Flags": ["game-mode-boost", "disable-telemetry"],
        "Routines_On_Enter": ["disable_notifications", "set_gaming_ui_theme"],
        "Routines_On_Exit": ["enable_notifications", "reset_ui_theme"],
    },
    "Editing": {
        "CPU_Priority": "Balanced-High", "GPU_Profile": "Compute_Accelerated",
        "RAM_Focus": "Disk_Cache / High_Buffer", "Background_Task_Limit": 5,
        "Description": "Optimized for 8K video and media rendering.",
        "AI_Config": {"max_depth": 5, "max_tokens": 2048, "style": "Strict", "tool_budget": "Minimal"},
        "Kernel_Flags": ["io-priority-boost", "vram-reservation"],
        "Routines_On_Enter": ["launch_creative_suite", "optimize_disk_cache"],
        "Routines_On_Exit": ["flush_disk_cache"],
    },
    "Automation": {
        "CPU_Priority": "High (Multi-Core Focus)", "GPU_Profile": "Low",
        "RAM_Focus": "Process_Isolation", "Background_Task_Limit": 100,
        "Description": "Massive background task orchestration.",
        "AI_Config": {"max_depth": 20, "max_tokens": 8192, "style": "Agentic", "tool_budget": "High"},
        "Kernel_Flags": ["container-isolation", "network-qos-high"],
        "Routines_On_Enter": ["start_automation_agent", "isolate_network_traffic", "forge_global_mesh"],
        "Routines_On_Exit": ["stop_automation_agent", "restore_network_traffic"],
    },
    "Focus": {
        "CPU_Priority": "Balanced", "GPU_Profile": "Low",
        "RAM_Focus": "Single_App_Priority", "Background_Task_Limit": 2,
        "Description": "Deep work mode. Eliminates distractions.",
        "AI_Config": {"max_depth": 3, "max_tokens": 1024, "style": "Concise", "tool_budget": "Low"},
        "Kernel_Flags": ["disable-notifications", "block-social-media", "pomodoro-active"],
        "Routines_On_Enter": ["disable_notifications", "dim_display", "start_focus_timer"],
        "Routines_On_Exit": ["enable_notifications", "restore_display", "stop_focus_timer"],
    },
    "Performance": {
        "CPU_Priority": "High", "GPU_Profile": "Performance",
        "RAM_Focus": "Low_Latency", "Background_Task_Limit": 20,
        "Description": "Maximum throughput for compute-heavy workloads.",
        "AI_Config": {"max_depth": 5, "max_tokens": 1024, "style": "Direct", "tool_budget": "Efficient"},
        "Kernel_Flags": ["turbo-boost-max", "disable-power-saving"],
        "Routines_On_Enter": ["monitor_cpu_temp"], "Routines_On_Exit": [],
    },
    "Resource_Saving": {
        "CPU_Priority": "Low", "GPU_Profile": "Power_Saving",
        "RAM_Focus": "Minimal_Footprint", "Background_Task_Limit": 5,
        "Description": "Minimize power consumption and heat.",
        "AI_Config": {"max_depth": 1, "max_tokens": 256, "style": "Brief", "tool_budget": "Minimal"},
        "Kernel_Flags": ["power-save-aggressive", "cpu-throttle"],
        "Routines_On_Enter": ["dim_display", "disable_animations"],
        "Routines_On_Exit": ["restore_display", "enable_animations"],
    },
    "Hardened": {
        "CPU_Priority": "Strict", "GPU_Profile": "Stable",
        "RAM_Focus": "Encrypted_Pages", "Background_Task_Limit": 3,
        "Description": "Maximum security and compliance mode.",
        "AI_Config": {"max_depth": 3, "max_tokens": 512, "style": "Formal", "tool_budget": "Minimal"},
        "Kernel_Flags": ["zero-trust-max", "memory-encryption", "disk-seal"],
        "Routines_On_Enter": ["run_compliance_audit", "seal_all_vaults", "activate_ghost_mask"],
        "Routines_On_Exit": ["unseal_standard_vaults"],
    },
    "Apex": {
        "CPU_Priority": "Supreme (Locked Max)", "GPU_Profile": "HyperDrive / Overclocked",
        "RAM_Focus": "Pre-cognitive Cache / Zero_Jitter", "Background_Task_Limit": 1,
        "Description": "Absolute performance supremacy. Zero-latency everything.",
        "AI_Config": {"max_depth": 100, "max_tokens": 32768, "style": "Elite", "tool_budget": "Supreme+"},
        "Kernel_Flags": ["hyper-drive", "zen-latency", "zero-jit", "quantum-sched"],
        "Routines_On_Enter": ["engage_hyper_drive", "activate_zen_latency", "monitor_cpu_temp"],
        "Routines_On_Exit": ["disengage_hyper_drive"],
    },
    "Programmer": {
        "CPU_Priority": "High (Multi-Core Compilation)", "GPU_Profile": "Low (Unless CUDA active)",
        "RAM_Focus": "Docker/VM Cache Bias", "Background_Task_Limit": 50,
        "Description": "Maximum compilation speed. DevForge IDE integration.",
        "AI_Config": {"max_depth": 10, "max_tokens": 8192, "style": "Technical", "tool_budget": "Code_Execution"},
        "Kernel_Flags": ["container-isolation", "dev-mode-active"],
        "Routines_On_Enter": ["start_dev_environment", "enable_code_completion"],
        "Routines_On_Exit": ["stop_dev_environment"],
    },
    "Lawyer": {
        "CPU_Priority": "Balanced", "GPU_Profile": "Low",
        "RAM_Focus": "Text_Cache / High_Concurrency (Search)", "Background_Task_Limit": 20,
        "Description": "Optimized for legal research and contract analysis.",
        "AI_Config": {"max_depth": 5, "max_tokens": 8192, "style": "Formal", "tool_budget": "Optimized_NLP"},
        "Kernel_Flags": ["secure-boot-strict", "network-vpn-forced"],
        "Routines_On_Enter": ["activate_vpn", "launch_legal_suite"],
        "Routines_On_Exit": ["deactivate_vpn"],
    },
    "Business_Analyst": {
        "CPU_Priority": "Balanced", "GPU_Profile": "Low",
        "RAM_Focus": "Index_Cache / Office_Bloom", "Background_Task_Limit": 15,
        "Description": "Strategic analysis and BI reporting suite.",
        "AI_Config": {"max_depth": 5, "max_tokens": 2048, "style": "Strategic", "tool_budget": "Medium"},
        "Kernel_Flags": ["font-smoothing-high", "high-dpi-scaling"],
        "Routines_On_Enter": ["activate_intelligence_suite", "launch_bi_dashboard"],
        "Routines_On_Exit": [],
    },
    "Data_Scientist": {
        "CPU_Priority": "Vector_Optimized (SIMD)", "GPU_Profile": "Balanced_Compute",
        "RAM_Focus": "Large_Pages / High_Buffer", "Background_Task_Limit": 10,
        "Description": "Optimized for multi-terabyte data manipulation.",
        "AI_Config": {"max_depth": 10, "max_tokens": 4096, "style": "Analytical", "tool_budget": "High"},
        "Kernel_Flags": ["huge-pages-enabled", "vector-extensions-active"],
        "Routines_On_Enter": ["mount_data_lakes", "start_jupyter_lab", "activate_intelligence_suite"],
        "Routines_On_Exit": ["unmount_data_lakes"],
    },
    "AI_Engineer": {
        "CPU_Priority": "Max_Throughput", "GPU_Profile": "Compute_Exclusive",
        "RAM_Focus": "VRAM_Pinning", "Background_Task_Limit": 15,
        "Description": "Optimized for model training and local LLM prototyping.",
        "AI_Config": {"max_depth": 50, "max_tokens": 16384, "style": "Technical", "tool_budget": "Supreme"},
        "Kernel_Flags": ["gpu-exclusive-mode", "pci-passthrough-enabled"],
        "Routines_On_Enter": ["load_ai_frameworks", "allocate_vram", "activate_intelligence_suite"],
        "Routines_On_Exit": ["unload_ai_frameworks", "deallocate_vram"],
    },
    "Designer": {
        "CPU_Priority": "Real-Time / Low-Latency", "GPU_Profile": "Display_First / Render_Accelerated",
        "RAM_Focus": "GPU_Bias / Creative_Buffer", "Background_Task_Limit": 8,
        "Description": "Optimized for 8K video, 3D modeling, and generative art.",
        "AI_Config": {"max_depth": 3, "max_tokens": 1024, "style": "Creative", "tool_budget": "GPU_Heavy"},
        "Kernel_Flags": ["display-priority-high", "gpu-render-boost"],
        "Routines_On_Enter": ["calibrate_display", "launch_design_software"],
        "Routines_On_Exit": ["reset_display_calibration"],
    },
    "Presenter": {
        "CPU_Priority": "Balanced", "GPU_Profile": "Optimized_Display",
        "RAM_Focus": "Presentation_Buffer", "Background_Task_Limit": 3,
        "Description": "Optimized for smooth presentations and screen sharing.",
        "AI_Config": {"max_depth": 2, "max_tokens": 768, "style": "Concise", "tool_budget": "Low"},
        "Kernel_Flags": ["disable-notifications", "mirror-display-mode"],
        "Routines_On_Enter": ["mute_system_sounds", "start_presentation_software"],
        "Routines_On_Exit": ["unmute_system_sounds"],
    },
    "Travel": {
        "CPU_Priority": "Low", "GPU_Profile": "Power_Saving",
        "RAM_Focus": "Minimal_Footprint", "Background_Task_Limit": 5,
        "Description": "Extended battery life, offline capabilities, secure network.",
        "AI_Config": {"max_depth": 1, "max_tokens": 256, "style": "Brief", "tool_budget": "Minimal"},
        "Kernel_Flags": ["airplane-mode", "disk-encryption-active"],
        "Routines_On_Enter": ["disconnect_wifi", "enable_offline_sync"],
        "Routines_On_Exit": ["connect_wifi"],
    },
    "Emergency": {
        "CPU_Priority": "Critical", "GPU_Profile": "Off_Headless",
        "RAM_Focus": "Diagnostic_Only", "Background_Task_Limit": 0,
        "Description": "System diagnostics and critical recovery.",
        "AI_Config": {"max_depth": 0, "max_tokens": 0, "style": "None", "tool_budget": "None"},
        "Kernel_Flags": ["safe-mode", "read-only-filesystem"],
        "Routines_On_Enter": ["run_system_diagnostics", "isolate_network"],
        "Routines_On_Exit": ["reboot_system"],
    },
    "Sovereign_Orchestrator": {
        "CPU_Priority": "Supreme (Agentic Lock)", "GPU_Profile": "Compute_Accelerated (LLM Routing)",
        "RAM_Focus": "DAG_Cache / Vector_DB", "Storage_IO": "Memory_Mapped_Datasets",
        "Network_Bandwidth": "Swarm_P2P_Unlocked", "Background_Task_Limit": 200,
        "Description": "HyperSwarm Intelligence Pipeline.",
        "AI_Config": {"max_depth": 100, "max_tokens": 32768, "style": "Orchestrator", "tool_budget": "Infinite"},
        "Kernel_Flags": ["zero-latency-ring", "quantum-routing"],
        "Routines_On_Enter": ["spawn_hyper_swarm", "build_cognitive_dag"],
        "Routines_On_Exit": ["cooldown_swarm"],
    },
    "Shopping": {
        "CPU_Priority": "Balanced", "GPU_Profile": "Low",
        "RAM_Focus": "Catalog_Cache", "Background_Task_Limit": 15,
        "Description": "Optimized for e-commerce, price tracking, and logistics.",
        "AI_Config": {"max_depth": 3, "max_tokens": 1024, "style": "Personalized", "tool_budget": "Optimized"},
        "Kernel_Flags": ["network-latency-low", "ad-block-strict"],
        "Routines_On_Enter": ["activate_ad_blocker", "open_shopping_browser"],
        "Routines_On_Exit": ["deactivate_ad_blocker"],
    },
    "Healthcare": {
        "CPU_Priority": "Balanced", "GPU_Profile": "Low",
        "RAM_Focus": "Encrypted_HIPAA", "Background_Task_Limit": 5,
        "Description": "HIPAA-compliant mode. Encrypted patient data.",
        "AI_Config": {"max_depth": 5, "max_tokens": 4096, "style": "Formal_Medical", "tool_budget": "Medical_NLP"},
        "Kernel_Flags": ["hipaa-mode", "zero-external-telemetry", "audit-logging-max"],
        "Routines_On_Enter": ["run_compliance_audit", "activate_vpn", "seal_all_vaults"],
        "Routines_On_Exit": ["unseal_standard_vaults", "deactivate_vpn"],
    },
    "Forensics": {
        "CPU_Priority": "High", "GPU_Profile": "Balanced_Compute",
        "RAM_Focus": "Read_Only_Evidence_Cache", "Background_Task_Limit": 5,
        "Description": "Digital forensics and incident response.",
        "AI_Config": {"max_depth": 10, "max_tokens": 8192, "style": "Analytical_Forensic", "tool_budget": "Forensic_Suite"},
        "Kernel_Flags": ["write-block-all", "forensic-audit-log", "chain-of-custody-active"],
        "Routines_On_Enter": ["run_system_diagnostics", "isolate_network", "seal_all_vaults"],
        "Routines_On_Exit": ["unseal_standard_vaults", "restore_network_traffic"],
    },
    "Bare_Minimum": {
        "CPU_Priority": "Idle_Only", "GPU_Profile": "Off_Headless",
        "RAM_Focus": "Essential_Only", "Background_Task_Limit": 0,
        "Description": "Bare minimum resources. UI simplified to text-only.",
        "AI_Config": {"max_depth": 0, "max_tokens": 0, "style": "None", "tool_budget": "None"},
        "Kernel_Flags": ["no-animations", "no-bloat", "headless-ready", "text-only-ui"],
        "Routines_On_Enter": ["switch_to_text_console", "kill_gui_processes"],
        "Routines_On_Exit": ["start_gui_processes", "switch_to_graphical_console"],
    },
}

_APP_HEURISTICS: Dict[str, str] = {
    "steam.exe": "Gaming", "epicgames.exe": "Gaming", "valorant.exe": "Gaming", "csgo.exe": "Gaming",
    "photoshop.exe": "Editing", "premiere.exe": "Editing", "aftereffects.exe": "Editing", "davinci": "Editing",
    "docker": "Programmer",
    "jupyter": "Data_Scientist", "python": "Data_Scientist", "rstudio": "Data_Scientist",
    "excel.exe": "Business_Analyst", "powerbi.exe": "Business_Analyst", "tableau.exe": "Business_Analyst",
    "chrome.exe": "Standard", "firefox.exe": "Standard",
    "spotify.exe": "Standard", "discord.exe": "Gaming",
    "autoloader": "Automation", "script.py": "Automation",
}


class SigmaModeManager:
    """
    SigmaModeManager — Dynamic Operation Profiles.
    ===============================================
    Thin facade. All routine logic lives in mode_manager/routines/<category>.py
    (one function per file). Mode definitions live in _MODES dict above.
    """

    def __init__(self, kernel=None):
        self.kernel = kernel
        self._current_mode = "Standard"
        self._modes: Dict[str, Dict] = dict(_MODES)
        self._app_heuristics: Dict[str, str] = dict(_APP_HEURISTICS)

        # Build the routine dispatch table by binding kernel-aware closures
        k = kernel
        self._routines: Dict[str, Callable[..., str]] = {
            "log_mode_change":          lambda phase="": log_mode_change(self._current_mode, phase),
            "disable_notifications":    lambda phase="": disable_notifications(phase),
            "enable_notifications":     lambda phase="": enable_notifications(phase),
            "set_gaming_ui_theme":      lambda phase="": set_gaming_ui_theme(phase),
            "reset_ui_theme":           lambda phase="": reset_ui_theme(phase),
            "launch_creative_suite":    lambda phase="": launch_creative_suite(phase),
            "optimize_disk_cache":      lambda phase="": optimize_disk_cache(phase),
            "flush_disk_cache":         lambda phase="": flush_disk_cache(phase),
            "start_automation_agent":   lambda phase="": start_automation_agent(phase),
            "isolate_network_traffic":  lambda phase="": isolate_network_traffic(phase),
            "stop_automation_agent":    lambda phase="": stop_automation_agent(phase),
            "restore_network_traffic":  lambda phase="": restore_network_traffic(phase),
            "forge_global_mesh":        lambda phase="": forge_global_mesh(k, phase),
            "spawn_hyper_swarm":        lambda phase="": spawn_hyper_swarm(k, phase),
            "build_cognitive_dag":      lambda phase="": build_cognitive_dag(k, phase),
            "cooldown_swarm":           lambda phase="": cooldown_swarm(phase),
            "activate_ad_blocker":      lambda phase="": activate_ad_blocker(phase),
            "open_shopping_browser":    lambda phase="": open_shopping_browser(phase),
            "deactivate_ad_blocker":    lambda phase="": deactivate_ad_blocker(phase),
            "monitor_cpu_temp":         lambda phase="": monitor_cpu_temp(phase),
            "dim_display":              lambda phase="": dim_display(phase),
            "disable_animations":       lambda phase="": disable_animations(phase),
            "restore_display":          lambda phase="": restore_display(phase),
            "enable_animations":        lambda phase="": enable_animations(phase),
            "switch_to_text_console":   lambda phase="": switch_to_text_console(phase),
            "kill_gui_processes":       lambda phase="": kill_gui_processes(phase),
            "start_gui_processes":      lambda phase="": start_gui_processes(phase),
            "switch_to_graphical_console": lambda phase="": switch_to_graphical_console(phase),
            "load_ai_frameworks":       lambda phase="": load_ai_frameworks(phase),
            "allocate_vram":            lambda phase="": allocate_vram(phase),
            "unload_ai_frameworks":     lambda phase="": unload_ai_frameworks(phase),
            "deallocate_vram":          lambda phase="": deallocate_vram(phase),
            "mount_data_lakes":         lambda phase="": mount_data_lakes(phase),
            "start_jupyter_lab":        lambda phase="": start_jupyter_lab(phase),
            "unmount_data_lakes":       lambda phase="": unmount_data_lakes(phase),
            "activate_vpn":             lambda phase="": activate_vpn(phase),
            "launch_legal_suite":       lambda phase="": launch_legal_suite(phase),
            "deactivate_vpn":           lambda phase="": deactivate_vpn(phase),
            "calibrate_display":        lambda phase="": calibrate_display(phase),
            "launch_design_software":   lambda phase="": launch_design_software(phase),
            "reset_display_calibration": lambda phase="": reset_display_calibration(phase),
            "start_dev_environment":    lambda phase="": start_dev_environment(phase),
            "enable_code_completion":   lambda phase="": enable_code_completion(phase),
            "stop_dev_environment":     lambda phase="": stop_dev_environment(phase),
            "mute_system_sounds":       lambda phase="": mute_system_sounds(phase),
            "start_presentation_software": lambda phase="": start_presentation_software(phase),
            "unmute_system_sounds":     lambda phase="": unmute_system_sounds(phase),
            "disconnect_wifi":          lambda phase="": disconnect_wifi(phase),
            "enable_offline_sync":      lambda phase="": enable_offline_sync(phase),
            "connect_wifi":             lambda phase="": connect_wifi(phase),
            "run_system_diagnostics":   lambda phase="": run_system_diagnostics(phase),
            "isolate_network":          lambda phase="": isolate_network(phase),
            "reboot_system":            lambda phase="": reboot_system(phase),
            "engage_hyper_drive":       lambda phase="": engage_hyper_drive(k, phase),
            "activate_zen_latency":     lambda phase="": activate_zen_latency(k, phase),
            "disengage_hyper_drive":    lambda phase="": disengage_hyper_drive(phase),
            "run_compliance_audit":     lambda phase="": run_compliance_audit(k, phase),
            "seal_all_vaults":          lambda phase="": seal_all_vaults(k, phase),
            "activate_ghost_mask":      lambda phase="": activate_ghost_mask(k, phase),
            "scrub_recent_media":       lambda phase="": scrub_recent_media(k, phase),
            "unseal_standard_vaults":   lambda phase="": unseal_standard_vaults(k, phase),
            "activate_intelligence_suite": lambda phase="": activate_intelligence_suite(k, phase),
            "launch_bi_dashboard":      lambda phase="": launch_bi_dashboard(phase),
            "start_focus_timer":        lambda phase="": start_focus_timer(phase),
            "stop_focus_timer":         lambda phase="": stop_focus_timer(phase),
        }

    # ── Core mode-switching API ───────────────────────────────────────────────

    def trigger_auto_switch(self, app_name: str) -> Dict[str, str]:
        """USP: Automatically profiles an app launch and drops into the perfect Mode."""
        lower_app = app_name.lower()
        target_mode = "Standard"
        for key, mode in self._app_heuristics.items():
            if key in lower_app:
                target_mode = mode
                break
        if target_mode != self._current_mode:
            self.switch_mode(target_mode)
            return {"status": "Switched", "from": self._current_mode, "to": target_mode, "app": app_name}
        return {"status": "Unchanged", "mode": self._current_mode}

    def switch_mode(self, mode_name: str) -> Dict:
        """USP: Atomic mode switching with kernel re-profiling and routine execution."""
        if mode_name not in self._modes:
            return {"Error": f"Mode '{mode_name}' not recognized."}

        old_mode = self._current_mode
        old_profile = self._modes[old_mode]
        new_profile = self._modes[mode_name]

        exit_list = old_profile.get("Routines_On_Exit", [])
        if not isinstance(exit_list, list):
            exit_list = []
        exit_results = self._apply_routines(exit_list, "exit")

        if hasattr(self.kernel, "prewarmer") and self.kernel and self.kernel.prewarmer:
            self.kernel.prewarmer.purge_cold_apps()
        self._current_mode = mode_name

        if self.kernel and hasattr(self.kernel, "registry"):
            config = self.kernel.registry.get("config")
            if config and hasattr(config, "apply_mode"):
                config.apply_mode(mode_name.lower())

        tune_status = self._apply_tuning(new_profile)

        enter_list = new_profile.get("Routines_On_Enter", [])
        if not isinstance(enter_list, list):
            enter_list = []
        enter_results = self._apply_routines(enter_list, "enter")

        return {
            "From": old_mode, "To": mode_name,
            "Performance_Profile": new_profile,
            "Kernel_Tuning": tune_status,
            "Exit_Routines_Status": exit_results,
            "Enter_Routines_Status": enter_results,
            "Status": "READY",
        }

    def _apply_tuning(self, profile: Dict) -> str:
        flags = profile.get("Kernel_Flags", [])
        net = profile.get("Network_Bandwidth", "Default")
        io = profile.get("Storage_IO", "Default")
        return (
            f"Schedulers: {profile['CPU_Priority']} | Mem: {profile['RAM_Focus']} "
            f"| Net: {net} | I/O: {io} | Flags: {', '.join(flags) if flags else 'NONE'}"
        )

    def _apply_routines(self, routine_names: List[str], phase: str) -> Dict[str, str]:
        return {name: self._execute_routine(name, phase) for name in routine_names}

    def _execute_routine(self, routine_name: str, phase: str) -> str:
        if routine_name in self._routines:
            try:
                return self._routines[routine_name](phase=phase)
            except Exception as e:
                return f"Routine failed: {e}"
        return f"Routine '{routine_name}' not found."

    # ── Query API ─────────────────────────────────────────────────────────────

    def get_active_profile(self) -> Dict:
        return {"Mode": self._current_mode, "Config": self._modes[self._current_mode]}

    def get_all_modes(self) -> List[str]:
        return list(self._modes.keys())

    def get_mode_details(self, mode_name: str) -> Dict:
        return self._modes.get(mode_name, {"Error": f"Mode '{mode_name}' not found."})

    def get_mode_count(self) -> int:
        return len(self._modes)

    def health_check(self) -> str:
        return f"OK — Mode: {self._current_mode}."

    # ── Mutation API ──────────────────────────────────────────────────────────

    def add_mode(self, mode_name: str, config: Dict) -> Dict:
        if mode_name in self._modes:
            return {"Error": f"Mode '{mode_name}' already exists."}
        self._modes[mode_name] = config
        return {"Status": f"Mode '{mode_name}' added successfully."}

    def remove_mode(self, mode_name: str) -> Dict[str, str]:
        if mode_name not in self._modes:
            return {"Error": f"Mode '{mode_name}' not found."}
        if mode_name == self._current_mode:
            return {"Error": f"Cannot remove active mode '{mode_name}'."}
        self._modes.pop(mode_name, None)
        return {"Status": f"Mode '{mode_name}' removed successfully."}

    def update_mode(self, mode_name: str, config: Dict) -> Dict:
        if mode_name not in self._modes:
            return {"Error": f"Mode '{mode_name}' not found."}
        self._modes[mode_name].update(config)
        return {"Status": f"Mode '{mode_name}' updated successfully."}

    def add_routine(self, routine_name: str, routine_func: Callable[..., Any]) -> Dict:
        if routine_name in self._routines:
            return {"Error": f"Routine '{routine_name}' already exists."}
        self._routines[routine_name] = routine_func
        return {"Status": f"Routine '{routine_name}' added successfully."}

    def remove_routine(self, routine_name: str) -> Dict[str, str]:
        if routine_name not in self._routines:
            return {"Error": f"Routine '{routine_name}' not found."}
        self._routines.pop(routine_name, None)
        return {"Status": f"Routine '{routine_name}' removed successfully."}

    def get_all_routines(self) -> List[str]:
        return list(self._routines.keys())

    def get_routine_details(self, routine_name: str) -> str:
        if routine_name in self._routines:
            return f"Routine '{routine_name}': {self._routines[routine_name].__doc__ or 'No doc.'}"
        return f"Routine '{routine_name}' not found."

    def smart_suggest_mode(self, context: Dict) -> str:
        """USP: AI-powered mode recommendation based on time, battery, and active apps."""
        hour = context.get("hour", 12)
        battery = context.get("battery_pct", 100)
        active_apps = context.get("active_apps", [])

        for app in active_apps:
            for key, mode in self._app_heuristics.items():
                if key in app.lower():
                    return mode

        if battery < 20:
            return "Resource_Saving"
        if 22 <= hour or hour < 6:
            return "Focus"
        return "Standard"
