# Generated method: SovereignLedger._calculate_epoch_merkle
import hashlib
import time
import json
import os

class SovereignLedger:
    def _calculate_epoch_merkle(self):
        """Calculates a Merkle Root for the current epoch (last 10 entries)."""
        hashes = []
        try:
            with open(self.path, 'r') as f:
                lines = f.readlines()
                _count = len(lines)
                for i in range(max(0, _count - 9), _count):
                    hashes.append(json.loads(lines[i])['this_hash'])
            combined = ''.join(hashes).encode()
            return hashlib.sha256(combined).hexdigest()
        except:
            return '0' * 64