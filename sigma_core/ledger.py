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
        self._initialize_ledger()

    def _initialize_ledger(self):
        if not os.path.exists(self.path):
            with open(self.path, "w") as f:
                f.write("") # Genesis
        else:
            # Recover last hash from the end of file
            try:
                with open(self.path, "r") as f:
                    lines = [line.strip() for line in f.readlines() if line.strip()]
                    if lines:
                        last_entry = json.loads(lines[-1])
                        self._last_hash = last_entry["this_hash"]
            except:
                pass

    def commit(self, module, action, context):
        """
        Commits an event to the ledger.
        Every entry is hashed with the previous entry's hash.
        """
        entry = {
            "timestamp": time.time(),
            "module": module,
            "action": action,
            "context": context,
            "prev_hash": self._last_hash
        }
        
        # Calculate current hash
        raw_payload = json.dumps(entry, sort_keys=True).encode()
        this_hash = hashlib.sha256(raw_payload).hexdigest()
        entry["this_hash"] = this_hash
        self._last_hash = this_hash

        with open(self.path, "a") as f:
            f.write(json.dumps(entry) + "\n")
        
        return this_hash

    def verify_integrity(self):
        """Validates the entire chain of custody."""
        if not os.path.exists(self.path): return True
        
        running_hash = "0" * 64
        with open(self.path, "r") as f:
            for line in f:
                entry = json.loads(line)
                actual_this_hash = entry.pop("this_hash")
                if entry["prev_hash"] != running_hash:
                    return False # Chain broken
                
                check_payload = json.dumps(entry, sort_keys=True).encode()
                if hashlib.sha256(check_payload).hexdigest() != actual_this_hash:
                    return False # Data tampered
                
                running_hash = actual_this_hash
        return True
