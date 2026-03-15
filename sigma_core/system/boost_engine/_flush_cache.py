"""
Auto-split from sigma_core\system\boost_engine.py — _flush_cache
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



def _flush_cache() -> str:
    print('      [1/6] OPTIMIZING: SigmaCache & RAM Trimming...')
    gc.collect()
    _native_trim_working_set()
    print('      [1/6] SUCCESS: RAM footprints compacted.')
    return 'cache_flushed'
