"""
Auto-split from sigma_core\system\boost_engine.py — _native_cpu_usage
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



def _native_cpu_usage() -> float:
    """Pure-stdlib CPU load — no psutil needed."""
    try:
        if sys.platform == 'win32':
            out = subprocess.check_output(['wmic', 'cpu', 'get', 'loadpercentage'], stderr=subprocess.DEVNULL).decode()
            return float(out.split('\n')[1].strip())
    except Exception:
        pass
    return 15.0
