# Generated method: SovereignBridge.__init__
import os
import sys
import ctypes
from pathlib import Path

class SovereignBridge:
    def __init__(self, kernel):
        self.kernel = kernel
        self._c_lib = None
        self._rust_lib = None
        self._load_binaries()