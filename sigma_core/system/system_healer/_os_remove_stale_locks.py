# Generated file: _os_remove_stale_locks
import os
import sys
import threading
import time
import ctypes
import platform
import subprocess
from typing import Dict, Any, List, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

def _os_remove_stale_locks(root_dir: str) -> int:
    """Remove all *.lock files under root_dir. Returns number removed."""
    removed_count: int = 0
    try:
        if not os.path.exists(root_dir):
            return 0
        for fname in os.listdir(root_dir):
            if fname.endswith('.lock'):
                try:
                    os.remove(os.path.join(root_dir, fname))
                    removed_count = removed_count + 1
                except (OSError, PermissionError):
                    pass
    except OSError:
        pass
    return removed_count