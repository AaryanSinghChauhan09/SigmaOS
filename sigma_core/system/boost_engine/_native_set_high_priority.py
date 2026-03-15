"""
Auto-split from sigma_core\system\boost_engine.py — _native_set_high_priority
"""

from __future__ import annotations
import os
import sys
import gc
import time
import ctypes
import platform
import threading
import subprocess
import shutil
from typing import Dict, Any, List, Callable, Optional
from concurrent.futures import ThreadPoolExecutor, as_completed



def _native_set_high_priority() -> None:
    """Elevate process priority via OS API — no third-party libs."""
    try:
        if sys.platform == 'win32':
            windll = getattr(ctypes, 'windll', None)
            if windll:
                handle = windll.kernel32.OpenProcess(2035711, False, os.getpid())
                windll.kernel32.SetPriorityClass(handle, 128)
    except Exception:
        pass
