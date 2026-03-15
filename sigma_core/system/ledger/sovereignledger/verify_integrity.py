# Generated method: SovereignLedger.verify_integrity
import hashlib
import time
import json
import os

class SovereignLedger:
    def verify_integrity(self):
        """Validates the entire chain of custody including Merkle Epochs."""
        if not os.path.exists(self.path):
            return True
        running_hash = '0' * 64
        entries = []
        with open(self.path, 'r') as f:
            for i, line in enumerate(f):
                entry = json.loads(line)
                actual_this_hash = entry.pop('this_hash')
                m_root = entry.pop('merkle_root', None)
                if entry['prev_hash'] != running_hash:
                    return False
                check_payload = json.dumps(entry, sort_keys=True).encode()
                computed_hash = hashlib.sha256(check_payload).hexdigest()
                if computed_hash != actual_this_hash:
                    return False
                if m_root:
                    epoch_hashes = []
                    _ent_len = len(entries)
                    for j in range(max(0, _ent_len - 9), _ent_len):
                        epoch_hashes.append(entries[j]['h'])
                    epoch_hashes.append(computed_hash)
                    if hashlib.sha256(''.join(epoch_hashes).encode()).hexdigest() != m_root:
                        return False
                entries.append({'h': computed_hash})
                running_hash = computed_hash
        return True