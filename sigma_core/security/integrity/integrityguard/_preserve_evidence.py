"""
Auto-split from sigma_core\security\integrity.py — IntegrityGuard._preserve_evidence
"""

import hashlib
import os
import sys
import json
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase



class IntegrityGuard:
    def _preserve_evidence(self, path: str, current_hash: str):
        """USP: Forensic Evidence Locking. Copies tampered file to the vault."""
        import shutil
        import time
        timestamp = int(time.time())
        filename = os.path.basename(path)
        vault_name = f'evidence_{timestamp}_{current_hash}_{filename}'
        target = os.path.join(self.vault_path, vault_name)
        try:
            shutil.copy2(path, target)
            self.stats['evidence_locked'] += 1
            if self.kernel:
                self.kernel.bus.emit('forensic.evidence_locked', {'file': filename, 'vault': vault_name})
        except Exception as e:
            print(f'Forensic Vault Failure: {e}')
