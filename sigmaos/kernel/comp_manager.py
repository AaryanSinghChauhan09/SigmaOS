"""
SigmaOS Component Subsystem
FFI Wrapper. Handles the decomposition of legacy shards into micro-modules.
"""
import ctypes
import os

# Load the native core library
lib_path = os.path.join(os.path.dirname(__file__), "..", "core", "build", "sigma_core.so")
if os.name == 'nt':
    lib_path = os.path.join(os.path.dirname(__file__), "..", "core", "build", "sigma_core.dll")

try:
    _native_core = ctypes.CDLL(lib_path)
    _native_core.comp_split.argtypes = [ctypes.c_char_p]
    _native_core.comp_optimize.argtypes = [ctypes.c_char_p]
    NATIVE_AVAILABLE = True
except OSError:
    NATIVE_AVAILABLE = False

class ComponentManager:
    def split(self, name: str):
        if NATIVE_AVAILABLE:
            _native_core.comp_split(name.encode('utf-8'))
        else:
            print(f"[Comp-Stub] Splitting {name}...")

    def audit(self):
        if NATIVE_AVAILABLE:
            _native_core.comp_audit()
        else:
            print("[Comp-Stub] Running modularization audit...")

    def optimize(self, name: str):
        if NATIVE_AVAILABLE:
            _native_core.comp_optimize(name.encode('utf-8'))
        else:
            print(f"[Comp-Stub] Optimizing {name}...")

# Canonical Global Component Manager
sigma_comp = ComponentManager()
