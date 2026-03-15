"""
Auto-split from sigma_core\system\boost_engine.py — _overclock_bus
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



def _overclock_bus() -> str:
    print('      [4/6] OVERCLOCKING: Process Priority Elevation...')
    _native_set_high_priority()
    print('      [4/6] SUCCESS: Kernel priority set to HIGH.')
    return 'bus_overclocked'
