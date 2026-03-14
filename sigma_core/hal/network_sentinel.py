"""
SigmaOS Network Sentinel (v2.0 Apex)
=====================================
USP: Multi-layer Mesh security via Inspection and Encryption.
Modular Architecture: Delegating to TrafficInspector and EncryptionShield.
"""
from typing import Dict, Any
from .traffic_inspector import TrafficInspector
from .encryption_shield import EncryptionShield

class NetworkSentinel:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.inspector = TrafficInspector(kernel)
        self.shield = EncryptionShield(kernel)

    def process_egress(self, data: str) -> str:
        """Securely prepares data for mesh broadcast."""
        sealed = self.shield.seal_packet(data)
        self.inspector.inspect_flow({"size": len(sealed), "origin": "LOCAL"})
        return sealed

    def process_ingress(self, packet: str) -> bool:
        """Verifies and inspects incoming mesh packets."""
        if not self.shield.verify_seal(packet):
             return False
        return self.inspector.inspect_flow({"size": len(packet), "origin": "MESH"})

    def health_check(self) -> str:
        return f"OK — Sentinel Modularized (Inspector + Shield)"
