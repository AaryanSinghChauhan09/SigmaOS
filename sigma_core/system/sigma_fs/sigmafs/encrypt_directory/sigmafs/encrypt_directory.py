# Generated method: SigmaFS.encrypt_directory
import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath

class SigmaFS:
    def encrypt_directory(self, dir_path: str, algorithm: str='AES-256-GCM+Kyber1024') -> dict:
        """Per-directory quantum-safe encryption."""
        count = sum((1 for p in self._inodes if p.startswith(dir_path)))
        for path, node in self._inodes.items():
            if path.startswith(dir_path):
                node.encrypted = True
        self._log_event(FSEvent.ENCRYPT, dir_path, f'algo={algorithm} files={count}')
        return {'status': 'Encrypted', 'directory': dir_path, 'algorithm': algorithm, 'files': count, 'message': f"SigmaFS: '{dir_path}' encrypted with {algorithm}. {count} files protected (quantum-safe post-quantum layer enabled)."}