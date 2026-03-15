# Generated method: NativeAccelerator._load_native_binary
import ctypes
import os
import platform
from typing import Optional, Any

class NativeAccelerator:
    def _load_native_binary(self):
        """Attempts to load the sovereign C library for this platform."""
        lib_name = 'sigma_native.so' if platform.system() != 'Windows' else 'sigma_native.dll'
        lib_path = os.path.join(os.path.dirname(__file__), 'native', lib_name)
        if os.path.exists(lib_path):
            try:
                self.lib = ctypes.CDLL(lib_path)
                print(f'[HAL] Native Accelerator Bound: {lib_name}')
            except Exception as e:
                print(f'[HAL] Failed to bind native binary: {e}')
        else:
            print('[HAL] Native Extension not found. Falling back to Optimized Python JIT Simulation.')