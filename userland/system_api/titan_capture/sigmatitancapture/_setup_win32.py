# Generated method: SigmaTitanCapture._setup_win32
import os
import sys
import time
import platform
import ctypes
from ctypes import wintypes
from typing import Dict, List, Any
from sigma_core.hal.graphics_driver import GDI32Driver, BMPEncoder

class SigmaTitanCapture:
    def _setup_win32(self):
        if platform.system() == 'Windows':
            self.user32 = ctypes.windll.user32
            self.gdi32 = ctypes.windll.gdi32