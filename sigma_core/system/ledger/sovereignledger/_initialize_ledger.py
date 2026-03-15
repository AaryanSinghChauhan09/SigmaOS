# Generated method: SovereignLedger._initialize_ledger
import hashlib
import time
import json
import os

class SovereignLedger:
    def _initialize_ledger(self):
        if not os.path.exists(self.path):
            with open(self.path, 'w') as f:
                f.write('')
        else:
            try:
                with open(self.path, 'r') as f:
                    for line in f:
                        if line.strip():
                            self._entry_count += 1
                            last_line = line
                    if self._entry_count > 0:
                        last_entry = json.loads(last_line)
                        self._last_hash = last_entry['this_hash']
            except:
                pass