"""
Auto-split from sigma_core\system\sigma_fs.py — SigmaFS.chmod
"""

import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath



class SigmaFS:
    def chmod(self, path: str, mode: int) -> dict:
        """Linux-parity permission change."""
        node = self._inodes.get(path)
        if not node:
            return {'error': 'Not found'}
        old = oct(node.mode)
        node.mode = mode
        self._log_event(FSEvent.WRITE, path, f'chmod {old} -> {oct(mode)}')
        return {'status': 'OK', 'path': path, 'mode': oct(mode)}
