"""
Auto-split from sigma_core\system\sigma_fs.py — SigmaFS.rename
"""

import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath



class SigmaFS:
    def rename(self, src: str, dst: str) -> dict:
        if src not in self._inodes:
            return {'error': f"SigmaFS: '{src}' not found."}
        node = self._inodes.pop(src)
        node.path = dst
        node.modified_at = time.strftime('%Y-%m-%dT%H:%M:%S')
        self._inodes[dst] = node
        self._log_event(FSEvent.RENAME, src, f'→ {dst}')
        return {'status': 'Renamed', 'from': src, 'to': dst, 'message': f"SigmaFS: Renamed '{src}' → '{dst}' atomically."}
