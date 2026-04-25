"""
SigmaOS Automation FFI Bridge
Acts as a thin orchestrator calling the high-performance C++ Automation Engine.
Eliminates high-level language overhead for critical automation paths.
"""
import ctypes
import os

# Load Native Automation Core
lib_path = os.path.join(os.path.dirname(__file__), "..", "core", "build", "sigma_core.so")
if os.name == 'nt':
    lib_path = os.path.join(os.path.dirname(__file__), "..", "core", "build", "sigma_core.dll")

try:
    _native_auto = ctypes.CDLL(lib_path)
    _native_auto.auto_init.restype = ctypes.c_void_p
    _native_auto.auto_run_all.argtypes = [ctypes.c_void_p]
    _native_auto.auto_trigger_rollback.restype = None
    _native_auto.auto_watchdog_start.argtypes = [ctypes.c_char_p]
    _native_auto.auto_watchdog_status.restype = None
    _native_auto.auto_patch_nightly.restype = None
    NATIVE_AUTO_AVAILABLE = True
except OSError:
    print("[Warning] Native Automation Core not found. Falling back to Python stubs.")
    NATIVE_AUTO_AVAILABLE = False

class NativeAutomatorWrapper:
    def __init__(self):
        self._ptr = None
        if NATIVE_AUTO_AVAILABLE:
            self._ptr = _native_auto.auto_init()

    def run_all(self):
        if NATIVE_AUTO_AVAILABLE and self._ptr:
            _native_auto.auto_run_all(self._ptr)
        else:
            print("[Auto-Stub] Running high-level automation tasks...")

    def trigger_rollback(self):
        if NATIVE_AUTO_AVAILABLE:
            _native_auto.auto_trigger_rollback()
        else:
            print("[Auto-Stub] Triggering high-level rollback.")

    def monitor(self, shard: str):
        if NATIVE_AUTO_AVAILABLE:
            _native_auto.auto_watchdog_start(shard.encode('utf-8'))
        else:
            print(f"[Auto-Stub] Monitoring shard: {shard}")

    def status(self):
        if NATIVE_AUTO_AVAILABLE:
            _native_auto.auto_watchdog_status()
        else:
            print("[Auto-Stub] Checking automation status...")

    def patch(self):
        if NATIVE_AUTO_AVAILABLE:
            _native_auto.auto_patch_nightly()
        else:
            print("[Auto-Stub] Running nightly patch...")

# Canonical Native Automator
sigma_native_auto = NativeAutomatorWrapper()
