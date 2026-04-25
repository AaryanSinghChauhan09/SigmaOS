"""
SigmaOS Performance Subsystem
FFI Wrapper. Core logic has been moved to bare-metal C++ for absolute performance.
"""
import ctypes
import os

# Load the native core library
lib_path = os.path.join(os.path.dirname(__file__), "..", "core", "build", "sigma_core.so")
if os.name == 'nt':
    lib_path = os.path.join(os.path.dirname(__file__), "..", "core", "build", "sigma_core.dll")

try:
    _native_core = ctypes.CDLL(lib_path)
    NATIVE_AVAILABLE = True
except OSError:
    NATIVE_AVAILABLE = False

class PerformanceManager:
    def balance(self):
        if NATIVE_AVAILABLE:
            _native_core.perf_balance()
        else:
            print("[Perf-Stub] Balancing system resources...")

    def cache_adaptive(self):
        if NATIVE_AVAILABLE:
            _native_core.perf_cache_adaptive()
        else:
            print("[Perf-Stub] Optimizing cache...")

    def isolate(self, pid: int):
        if NATIVE_AVAILABLE:
            _native_core.perf_isolate(pid)
        else:
            print(f"[Perf-Stub] Isolating process {pid}...")

# Canonical Global Performance Manager
sigma_perf = PerformanceManager()
