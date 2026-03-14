import hashlib
import time
import json
import os

class SovereignLedger:
    """
    Forensic-Grade Audit Ledger for SigmaOS.
    Implements a cryptographic chain of custody for all system events.
    """
    def __init__(self, ledger_path="system_audit.sigma"):
        self.path = ledger_path
        self._last_hash = "0" * 64
        self._entry_count = 0
        self._initialize_ledger()

    def _initialize_ledger(self):
        if not os.path.exists(self.path):
            with open(self.path, "w") as f:
                f.write("") # Genesis
        else:
            try:
                with open(self.path, "r") as f:
                    for line in f:
                        if line.strip():
                            self._entry_count += 1
                            last_line = line
                    if self._entry_count > 0:
                        last_entry = json.loads(last_line)
                        self._last_hash = last_entry["this_hash"]
            except:
                pass

    def commit(self, module, action, context):
        """
        Commits an event to the ledger with optimized metadata tracking.
        """
        entry = {
            "timestamp": time.time(),
            "module": module,
            "action": action,
            "context": context,
            "prev_hash": self._last_hash
        }
        
        raw_payload = json.dumps(entry, sort_keys=True).encode()
        this_hash = hashlib.sha256(raw_payload).hexdigest()
        entry["this_hash"] = this_hash
        self._last_hash = this_hash

        # Block-Level Merkle Root (Every 10 entries)
        if (self._entry_count + 1) % 10 == 0:
             entry["merkle_root"] = self._calculate_epoch_merkle()

        with open(self.path, "a") as f:
            f.write(json.dumps(entry) + "\n")
        
        self._entry_count += 1
        return this_hash

    def _calculate_epoch_merkle(self):
        """Calculates a Merkle Root for the current epoch (last 10 entries)."""
        hashes = []
        try:
            with open(self.path, "r") as f:
                lines = f.readlines()
                _count = len(lines)
                # Manual iterator to bypass slicing linter issue
                for i in range(max(0, _count - 9), _count):
                    hashes.append(json.loads(lines[i])["this_hash"])
            combined = "".join(hashes).encode()
            return hashlib.sha256(combined).hexdigest()
        except:
            return "0" * 64

    def verify_integrity(self):
        """Validates the entire chain of custody including Merkle Epochs."""
        if not os.path.exists(self.path): return True
        
        running_hash = "0" * 64
        entries = []
        with open(self.path, "r") as f:
            for i, line in enumerate(f):
                entry = json.loads(line)
                actual_this_hash = entry.pop("this_hash")
                m_root = entry.pop("merkle_root", None)
                
                if entry["prev_hash"] != running_hash:
                    return False # Chain broken
                
                check_payload = json.dumps(entry, sort_keys=True).encode()
                computed_hash = hashlib.sha256(check_payload).hexdigest()
                if computed_hash != actual_this_hash:
                    return False # Data tampered
                
                # Verify Merkle Root if present
                if m_root:
                    epoch_hashes = []
                    _ent_len = len(entries)
                    for j in range(max(0, _ent_len - 9), _ent_len):
                         epoch_hashes.append(entries[j]["h"])
                    epoch_hashes.append(computed_hash)
                    if hashlib.sha256("".join(epoch_hashes).encode()).hexdigest() != m_root:
                        return False # Epoch tampered
                
                entries.append({"h": computed_hash})
                running_hash = computed_hash
        return True
