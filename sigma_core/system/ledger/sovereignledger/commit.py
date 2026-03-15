# Generated method: SovereignLedger.commit
import hashlib
import time
import json
import os

class SovereignLedger:
    def commit(self, module, action, context):
        """
            Commits an event to the ledger with optimized metadata tracking.
            """
        entry = {'timestamp': time.time(), 'module': module, 'action': action, 'context': context, 'prev_hash': self._last_hash}
        raw_payload = json.dumps(entry, sort_keys=True).encode()
        this_hash = hashlib.sha256(raw_payload).hexdigest()
        entry['this_hash'] = this_hash
        self._last_hash = this_hash
        if (self._entry_count + 1) % 10 == 0:
            entry['merkle_root'] = self._calculate_epoch_merkle()
        with open(self.path, 'a') as f:
            f.write(json.dumps(entry) + '\n')
        self._entry_count += 1
        return this_hash