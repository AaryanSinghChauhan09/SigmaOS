"""
Quick import sanity check for newly added SigmaOS utilities.
"""
import sys, os
sys.path.append(os.path.abspath('.'))

modules = [
    "userland.system_api.forensic_scanner",
    "userland.system_api.circuit_breaker",
    "userland.system_api.bio_lock",
    "userland.system_api.sovereign_watchdog",
    "userland.system_api.omni_search_v2",
    "userland.system_api.sovereign_clipboard_v2",
]

failed = []
for mod in modules:
    try:
        __import__(mod)
        print(f"[OK] {mod}")
    except Exception as e:
        print(f"[FAIL] {mod}: {e}")
        failed.append(mod)

if failed:
    sys.exit(1)
else:
    sys.exit(0)
