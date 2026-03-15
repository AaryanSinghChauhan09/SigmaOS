"""
SigmaOS Sovereign Protocol Module
=================================
Standardizes packet construction and cryptographic signaling.
"""
from typing import Dict
from userland.system_api.sigma_std import SigmaCrypto

class SecurePacket:
    """Standardizes the Sovereign Comm-Packet structure."""
    @staticmethod
    def construct(p_type: str, sender_sid: str, payload: bytes, shared_secret: bytes) -> Dict:
        nonce = SigmaCrypto.generate_pow(sender_sid + p_type, difficulty=3)
        encrypted = SigmaCrypto.encrypt_payload(payload.decode('utf-8', errors='ignore') if isinstance(payload, bytes) else str(payload), shared_secret)
        return {
            "type": p_type,
            "from": sender_sid,
            "payload": encrypted.hex(),
            "nonce": nonce,
            "sig": SigmaCrypto.sign(encrypted.hex(), sender_sid)
        }
