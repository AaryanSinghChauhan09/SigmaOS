"""
SigmaOS Networking Subsystem
FFI Wrapper. Core logic has been moved to bare-metal C++ for absolute performance.
"""
import ctypes
import os

# Load the native core library
lib_path = os.path.join(os.path.dirname(__file__), "..", "..", "core", "build", "sigma_core.so")
if os.name == 'nt':
    lib_path = os.path.join(os.path.dirname(__file__), "..", "..", "core", "build", "sigma_core.dll")

try:
    _native_core = ctypes.CDLL(lib_path)
    NATIVE_AVAILABLE = True
except OSError:
    NATIVE_AVAILABLE = False

class NetworkingSubsystem:
    def secure_connect(self):
        if NATIVE_AVAILABLE:
            _native_core.net_secure_connect()
        else:
            print("[Net-Stub] Establishing secure connection...")

    def audit(self):
        if NATIVE_AVAILABLE:
            _native_core.net_audit()
        else:
            print("[Net-Stub] Auditing network traffic...")
