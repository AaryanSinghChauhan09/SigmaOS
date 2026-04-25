"""
SigmaOS Agentic Process Control Block (APCB)
FFI Wrapper. Core logic has been moved to bare-metal C++ for extreme performance.
"""
import ctypes
import os
from typing import Dict

# Load the native C++ library (assuming it's compiled as sigma_core.dll/so)
lib_path = os.path.join(os.path.dirname(__file__), "..", "core", "build", "sigma_core.so")
if os.name == 'nt':
    lib_path = os.path.join(os.path.dirname(__file__), "..", "core", "build", "sigma_core.dll")

try:
    _native_core = ctypes.CDLL(lib_path)
    
    # Define C-ABI signatures
    _native_core.apcb_create.restype = ctypes.c_void_p
    _native_core.apcb_create.argtypes = [ctypes.c_int, ctypes.c_char_p]
    
    _native_core.apcb_handle_crash.restype = None
    _native_core.apcb_handle_crash.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
    
    _native_core.apcb_destroy.restype = None
    _native_core.apcb_destroy.argtypes = [ctypes.c_void_p]

    NATIVE_AVAILABLE = True
except OSError:
    print("[Warning] Native C++ Core library not found. Falling back to Python stubs.")
    NATIVE_AVAILABLE = False

class APCB:
    def __init__(self, pid: int, intent: str):
        self.pid = pid
        self.intent = intent
        self.state = "READY"
        self._ptr = None
        
        if NATIVE_AVAILABLE:
            self._ptr = _native_core.apcb_create(pid, intent.encode('utf-8'))

    def handle_crash(self, traceback: str) -> None:
        self.state = "PAUSED_FOR_AI_FIX"
        if NATIVE_AVAILABLE and self._ptr:
            _native_core.apcb_handle_crash(self._ptr, traceback.encode('utf-8'))
        else:
            print(f"[APCB-Stub] Process {self.pid} crashed with intent '{self.intent}'.")

    def __del__(self):
        if NATIVE_AVAILABLE and self._ptr:
            _native_core.apcb_destroy(self._ptr)

class ProcessManager:
    def __init__(self):
        self.processes: Dict[int, APCB] = {}

    def spawn(self, pid: int, intent: str) -> APCB:
        process = APCB(pid, intent)
        self.processes[pid] = process
        return process
