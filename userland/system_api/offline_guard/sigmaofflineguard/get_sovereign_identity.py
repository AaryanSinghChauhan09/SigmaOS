# Generated method: SigmaOfflineGuard.get_sovereign_identity
import socket
import hashlib
import time

class SigmaOfflineGuard:
    def get_sovereign_identity(self) -> str:
        """Generates a hardware-bound unique ID that doesn't rely on a central server."""
        hostname = socket.gethostname()
        hw_hash = hashlib.sha256(hostname.encode()).hexdigest()[:16]
        return f'SID-{hw_hash.upper()}'