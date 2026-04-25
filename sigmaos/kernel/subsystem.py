"""
SigmaOS Subsystem Manager
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
    _native_core.subsystem_load.argtypes = [ctypes.c_char_p]
    _native_core.subsystem_unload.argtypes = [ctypes.c_char_p]
    _native_core.subsystem_is_active.restype = ctypes.c_int
    _native_core.subsystem_is_active.argtypes = [ctypes.c_char_p]
    NATIVE_AVAILABLE = True
except OSError:
    NATIVE_AVAILABLE = False

class SubsystemManager:
    def load_subsystem(self, name: str):
        if NATIVE_AVAILABLE:
            _native_core.subsystem_load(name.encode('utf-8'))
        else:
            print(f"[Subsystem-Stub] Loading {name}...")

    def unload_subsystem(self, name: str):
        if NATIVE_AVAILABLE:
            _native_core.subsystem_unload(name.encode('utf-8'))
        else:
            print(f"[Subsystem-Stub] Unloading {name}...")

    def is_active(self, name: str) -> bool:
        if NATIVE_AVAILABLE:
            return _native_core.subsystem_is_active(name.encode('utf-8')) == 1
        return False

manager = SubsystemManager()
