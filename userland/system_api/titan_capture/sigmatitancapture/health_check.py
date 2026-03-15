# Generated method: SigmaTitanCapture.health_check
import os
import sys
import time
import platform
import ctypes
from ctypes import wintypes
from typing import Dict, List, Any
from sigma_core.hal.graphics_driver import GDI32Driver, BMPEncoder

class SigmaTitanCapture:
    def health_check(self) -> str:
        return f'OK - Native_Driver: {platform.system()}'