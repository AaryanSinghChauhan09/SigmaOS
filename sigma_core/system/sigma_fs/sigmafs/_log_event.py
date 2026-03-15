"""
Auto-split from sigma_core\system\sigma_fs.py — SigmaFS._log_event
"""

import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath



class SigmaFS:
    def _log_event(self, event: FSEvent, path: str, detail: str):
        """Append-only forensic log; each entry hash-chains to the previous."""
        entry = {'seq': len(self._ledger), 'ts': time.strftime('%Y-%m-%dT%H:%M:%S'), 'event': event.value, 'path': path, 'detail': detail}
        chain_input = f"{str(self._ledger_chain_hash)}{entry['event']}{entry['path']}{entry['ts']}"
        entry['chain_hash'] = hashlib.sha256(chain_input.encode()).hexdigest()
        self._ledger_chain_hash = str(entry['chain_hash'])
        self._journal.append(entry)
        if len(self._journal) > self.JOURNAL_RING_SIZE:
            self._journal.pop(0)
        self._ledger.append(entry)
