# Generated file: _os_native_set_high_priority
import os
import sys
import threading
import time
import ctypes
import platform
import subprocess
from typing import Dict, Any, List, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

def _os_native_set_high_priority() -> None:
    """Sets current process to HIGH priority."""
    if platform.system() == 'Windows':
        try:
            kernel32 = getattr(ctypes, 'windll', None)
            if kernel32:
                handle = kernel32.kernel32.GetCurrentProcess()
                kernel32.kernel32.SetPriorityClass(handle, 128)
        except Exception:
            pass