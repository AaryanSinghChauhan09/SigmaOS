# Generated method: NativeAccelerator.__init__
import ctypes
import os
import platform
from typing import Optional, Any

class NativeAccelerator:
    def __init__(self, kernel):
        self.kernel = kernel
        self.lib: Optional[Any] = None
        self._load_native_binary()