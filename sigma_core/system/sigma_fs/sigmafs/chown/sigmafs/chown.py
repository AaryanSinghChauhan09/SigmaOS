# Generated method: SigmaFS.chown
import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath

class SigmaFS:
    def chown(self, path: str, uid: int, gid: int) -> dict:
        """Linux-parity ownership change."""
        node = self._inodes.get(path)
        if not node:
            return {'error': 'Not found'}
        node.uid, node.gid = (uid, gid)
        self._log_event(FSEvent.WRITE, path, f'chown -> {uid}:{gid}')
        return {'status': 'OK', 'path': path, 'owner': f'{uid}:{gid}'}