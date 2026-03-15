# Generated method: SigmaCryptGuard.verify_file_integrity
import os
import sys
import hashlib
import binascii
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaCryptGuard:
    def verify_file_integrity(self, file_path: str, expected_hash: str) -> bool:
        """Verifies a file against a SHA-256 hash."""
        if not os.path.exists(file_path):
            return False
        sha256_hash = hashlib.sha256()
        with open(file_path, 'rb') as f:
            for byte_block in iter(lambda: f.read(4096), b''):
                sha256_hash.update(byte_block)
        self.stats['ops'] += 1
        return sha256_hash.hexdigest() == expected_hash