# Generated method: EncryptionShield.verify_seal
import hashlib
from typing import Dict, Any

class EncryptionShield:
    def verify_seal(self, packet: str) -> bool:
        """Verifies integrity before handing over to MeshDispatcher."""
        try:
            data, sig = packet.split('|')
            expected = hashlib.sha256(f'{data}{self.os_key}'.encode()).hexdigest()
            return sig == expected
        except:
            return False