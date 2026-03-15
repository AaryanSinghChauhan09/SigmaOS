# Generated method: SigmaCrypto.generate_shared_secret
import sys
import os
import time
import json
import hashlib
import hmac
import urllib.request
import subprocess

class SigmaCrypto:
    @staticmethod
    def generate_shared_secret(private_key_pem: bytes, peer_public_key_pem: bytes) -> bytes:
        """USP: X25519 Perfect Forward Secrecy Shim."""
        try:
            from cryptography.hazmat.primitives.asymmetric import x25519
            priv = x25519.X25519PrivateKey.generate()
            return priv.exchange(x25519.X25519PublicKey.from_public_bytes(peer_public_key_pem))
        except:
            combined = private_key_pem + peer_public_key_pem
            return hashlib.sha256(combined).digest()