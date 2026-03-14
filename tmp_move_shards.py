import os
import shutil

root = r"c:\Users\Aaryan\Downloads\SigmaOS"
moves = [
    ("userland/system_api/package_manager.py", "sigma_core/system/package_manager.py"),
    ("userland/system_api/process_manager.py", "sigma_core/system/process_manager.py"),
    ("userland/system_api/network_guardian.py", "sigma_core/security/network_guardian.py"),
    ("userland/system_api/predictive_scheduler.py", "sigma_core/system/predictive_scheduler.py"),
    ("userland/system_api/energy_hub.py", "sigma_core/system/energy_hub.py"),
    ("userland/system_api/neural_fabric.py", "sigma_core/system/neural_fabric.py"),
    ("userland/system_api/app_sandbox.py", "sigma_core/system/app_sandbox.py"),
    ("userland/system_api/shadow_state.py", "sigma_core/system/shadow_state.py"),
    ("userland/system_api/stability_watchdog.py", "sigma_core/system/stability_watchdog.py"),
    ("userland/system_api/pulse_engine.py", "sigma_core/system/pulse_engine.py"),
    ("userland/system_api/omni_search.py", "sigma_core/system/omni_search.py"),
    ("userland/system_api/offline_guard.py", "sigma_core/security/offline_guard.py"),
    ("userland/system_api/app_prewarmer.py", "sigma_core/system/app_prewarmer.py"),
    ("userland/system_api/update_manager.py", "sigma_core/system/update_manager.py"),
    ("userland/system_api/crash_reporter.py", "sigma_core/system/crash_reporter.py"),
    ("userland/system_api/anomaly_detector.py", "sigma_core/system/anomaly_detector.py"),
    ("userland/system_api/mode_manager.py", "sigma_core/system/mode_manager.py")
]

for src, dst in moves:
    src_p = os.path.join(root, src)
    dst_p = os.path.join(root, dst)
    if os.path.exists(src_p):
        os.makedirs(os.path.dirname(dst_p), exist_ok=True)
        shutil.copy2(src_p, dst_p)
        print(f"Migrated: {src} -> {dst}")
    else:
        print(f"Skipped (missing): {src}")
