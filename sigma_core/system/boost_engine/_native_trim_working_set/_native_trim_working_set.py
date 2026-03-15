# Generated file: _native_trim_working_set
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

def _native_trim_working_set() -> None:
    """Trim the process working set via Win32 (zero external deps)."""
    try:
        if sys.platform == 'win32':
            windll = getattr(ctypes, 'windll', None)
            if windll:
                windll.kernel32.SetProcessWorkingSetSize(windll.kernel32.GetCurrentProcess(), -1, -1)
    except Exception:
        pass