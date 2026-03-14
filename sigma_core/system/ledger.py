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
        Includes a Merkle Root every 10 entries for block-level integrity.
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

        # Block-Level Merkle Root (Every 10 entries)
        entry_count = 0
        if os.path.exists(self.path):
            with open(self.path, "r") as f:
                entry_count = sum(1 for _ in f)
        
        if (entry_count + 1) % 10 == 0:
             entry["merkle_root"] = self._calculate_epoch_merkle(entry_count)

        with open(self.path, "a") as f:
            f.write(json.dumps(entry) + "\n")
        
        return this_hash

    def _calculate_epoch_merkle(self, entry_count):
        """Calculates a Merkle Root for the current epoch (last 10 entries)."""
        hashes = []
        try:
            with open(self.path, "r") as f:
                lines = f.readlines()
                for line in lines[-9:]:
                    hashes.append(json.loads(line)["this_hash"])
            # Hash the collected hashes together
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
                    epoch_hashes = [e["h"] for e in entries[-9:]]
                    epoch_hashes.append(computed_hash)
                    if hashlib.sha256("".join(epoch_hashes).encode()).hexdigest() != m_root:
                        return False # Epoch tampered
                
                entries.append({"h": computed_hash})
                running_hash = computed_hash
        return True
