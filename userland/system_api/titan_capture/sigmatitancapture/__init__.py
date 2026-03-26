# Generated method: SigmaTitanCapture.__init__
import os
import sys
import time
import platform
import ctypes
from ctypes import wintypes
from typing import Dict, List, Any
from sigma_core.hal.graphics_driver import GDI32Driver, BMPEncoder

class SigmaTitanCapture:
    def __init__(self, kernel):
        self.kernel = kernel
        self.stats = {'captures': 0, 'total_frames': 0}
        self.user32: Any = None
        self.gdi32: Any = None
        self._setup_win32()