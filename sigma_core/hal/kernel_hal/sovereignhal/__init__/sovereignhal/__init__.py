# Generated method: SovereignHAL.__init__
import ctypes
import platform
import os
import sys
from typing import Any

class SovereignHAL:
    def __init__(self):
        self.os_type = platform.system()
        self.k32: Any = None
        self._init_win32() if self.os_type == 'Windows' else self._init_posix()