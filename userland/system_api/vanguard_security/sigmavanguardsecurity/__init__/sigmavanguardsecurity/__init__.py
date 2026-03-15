# Generated method: SigmaVanguardSecurity.__init__
import hashlib

class SigmaVanguardSecurity:
    def __init__(self, key_id='QUANTUM_PRIORITY'):
        self.vault_id = hashlib.sha256(key_id.encode()).hexdigest()[:8]