# Generated method: EncryptionShield.seal_packet
import hashlib
from typing import Dict, Any

class EncryptionShield:
    def seal_packet(self, data: str) -> str:
        """Sovereign HMAC sealing for packet integrity."""
        signature = hashlib.sha256(f'{data}{self.os_key}'.encode()).hexdigest()
        return f'{data}|{signature}'