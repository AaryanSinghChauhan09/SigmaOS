# Generated method: SigmaFS.set_xattr
import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath

class SigmaFS:
    def set_xattr(self, path: str, key: str, value: str) -> dict:
        """Set extended attribute on any file or directory."""
        node = self._inodes.get(path)
        if node is None:
            return {'error': f"'{path}' not found."}
        node.attrs[key] = value
        return {'status': 'OK', 'path': path, 'attr': {key: value}}