"""
SigmaOS Multimedia Subsystem
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
    _native_core.media_load_codec.argtypes = [ctypes.c_char_p]
    NATIVE_AVAILABLE = True
except OSError:
    NATIVE_AVAILABLE = False

class MultimediaSubsystem:
    def load_codec(self, codec: str):
        if NATIVE_AVAILABLE:
            _native_core.media_load_codec(codec.encode('utf-8'))
        else:
            print(f"[Media-Stub] Loading codec: {codec}")

    def list_codecs(self):
        if NATIVE_AVAILABLE:
            _native_core.media_list_codecs()
        else:
            print("[Media-Stub] Listing available codecs...")
