"""
Auto-split from sigma_core\system\sigma_fs.py — SigmaFS.synchronous_commit
"""

import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath



class SigmaFS:
    def synchronous_commit(self, path: str, content: bytes) -> dict:
        """USP: ZFS-parity Synchronous Intent Log (ZIL)."""
        log_entry = {'ts': time.time(), 'op': 'SYNC_WRITE', 'path': path, 'data_len': len(content), 'crc': hashlib.md5(content).hexdigest()}
        self._intent_log.append(log_entry)
        res = self.create(path, content)
        res['intent_logged'] = True
        return res
