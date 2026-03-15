# Generated method: AntigravityForensicCore.track_file_change
import os
import hashlib
import time

class AntigravityForensicCore:
    def track_file_change(self, file_path: str, action: str):
        """Cryptography signs all file modifications with a hash (Chain of Custody)."""
        content_hash = 'no_content'
        if os.path.exists(file_path):
            with open(file_path, 'rb') as f:
                content_hash = hashlib.sha256(f.read()).hexdigest()
        entry = {'timestamp': time.time(), 'file': file_path, 'action': action, 'hash': content_hash, 'signature': self.kernel.crypto.sign(content_hash) if hasattr(self.kernel, 'crypto') else 'unsigned'}
        self._custody_ledger.append(entry)
        return entry