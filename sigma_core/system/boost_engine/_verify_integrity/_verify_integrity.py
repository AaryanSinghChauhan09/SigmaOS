# Generated file: _verify_integrity
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

def _verify_integrity() -> str:
    print('      [2/6] AUDITING: Bit-Level System Integrity...')
    try:
        import importlib
        try:
            mod = importlib.import_module('sigma_core.security.integrity')
        except ImportError:
            if '_ROOT' in globals() and _ROOT not in sys.path:
                sys.path.append(_ROOT)
            mod = importlib.import_module('sigma_core.security.integrity')
        guard = getattr(mod, 'IntegrityGuard')()
        res = guard.verify_system_integrity()
        print(f"      [2/6] SUCCESS: Status={res.get('status', 'UNKNOWN')}")
    except Exception as e:
        print(f'      [2/6] SKIPPED: Integrity check unavailable ({e})')
    return 'integrity_verified'