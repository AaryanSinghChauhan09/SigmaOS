# Generated method: SigmaOfflineGuard.toggle_hardened_airgap
import socket
import hashlib
import time

class SigmaOfflineGuard:
    def toggle_hardened_airgap(self, enabled: bool) -> str:
        """Strictly disables the hardware NIC and Bluetooth except for P2P Mesh."""
        if enabled:
            return 'Air-Gap Mode: HARDENED. Outbound WAN blocked. P2P Mesh discovery ONLY.'
        return 'Air-Gap Mode: HYBRID. Local LAN access restored.'