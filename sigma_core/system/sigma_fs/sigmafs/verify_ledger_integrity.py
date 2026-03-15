"""
Auto-split from sigma_core\system\sigma_fs.py — SigmaFS.verify_ledger_integrity
"""

import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath



class SigmaFS:
    def verify_ledger_integrity(self) -> dict:
        """Walks the full ledger chain to detect any tampering."""
        prev_hash = '0' * 64
        for entry in self._ledger:
            input_str = f"{prev_hash}{entry['event']}{entry['path']}{entry['ts']}"
            expected = hashlib.sha256(input_str.encode()).hexdigest()
            if expected != entry['chain_hash']:
                return {'status': 'TAMPERED', 'seq': entry['seq'], 'message': 'Ledger integrity violation!'}
            prev_hash = entry['chain_hash']
        return {'status': 'VERIFIED', 'entries': len(self._ledger), 'message': f'SigmaFS Ledger: All {len(self._ledger)} entries verified. No tampering detected.'}
