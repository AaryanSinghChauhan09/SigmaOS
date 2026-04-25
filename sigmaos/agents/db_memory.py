"""
SigmaOS Vector Memory Layer
FFI Wrapper. Core logic has been moved to bare-metal C++ for absolute performance.
"""
import ctypes
import os
import json

# Load the native core library
lib_path = os.path.join(os.path.dirname(__file__), "..", "core", "build", "sigma_core.so")
if os.name == 'nt':
    lib_path = os.path.join(os.path.dirname(__file__), "..", "core", "build", "sigma_core.dll")

try:
    _native_core = ctypes.CDLL(lib_path)
    _native_core.mem_store.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
    _native_core.mem_query.argtypes = [ctypes.c_char_p]
    NATIVE_AVAILABLE = True
except OSError:
    NATIVE_AVAILABLE = False

class DbMemoryLayer:
    def store(self, intent: str, vector_data: list):
        if NATIVE_AVAILABLE:
            _native_core.mem_store(intent.encode('utf-8'), json.dumps(vector_data).encode('utf-8'))
        else:
            print(f"[Mem-Stub] Storing {intent}")

    def query(self, intent_filter: str):
        if NATIVE_AVAILABLE:
            _native_core.mem_query(intent_filter.encode('utf-8'))
        else:
            print(f"[Mem-Stub] Querying {intent_filter}")

    def prune_stale(self, days_old: int = 30):
        if NATIVE_AVAILABLE:
            _native_core.mem_prune(days_old)
        else:
            print(f"[Mem-Stub] Pruning older than {days_old} days")
