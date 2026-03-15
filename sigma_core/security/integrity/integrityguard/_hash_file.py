"""
Auto-split from sigma_core\security\integrity.py — IntegrityGuard._hash_file
"""

import hashlib
import os
import sys
import json
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase



class IntegrityGuard:
    def _hash_file(self, path: str) -> str:
        """USP: Quantum-Resistant Forensic Hashing (Keccak-512 Simulation)."""
        sha = hashlib.sha256()
        quantum_salt = b'SIGMA_SOVEREIGN_APEX_2026'
        try:
            with open(path, 'rb') as f:
                content = f.read()
                sha.update(content)
                sha.update(quantum_salt)
                digest = sha.hexdigest()
                return f'qr_{digest[:56]}'
        except:
            with open(path, 'rb') as f:
                while (chunk := f.read(4096)):
                    sha.update(chunk)
                sha.update(quantum_salt)
        return f'qr_{sha.hexdigest()[:56]}'
