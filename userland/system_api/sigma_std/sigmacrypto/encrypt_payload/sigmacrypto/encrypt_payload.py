# Generated method: SigmaCrypto.encrypt_payload
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
    def encrypt_payload(data: str, key_bytes: bytes) -> bytes:
        """USP: AES-256-GCM Authenticated Encryption."""
        try:
            from cryptography.hazmat.primitives.ciphers.aead import AESGCM
            aesgcm = AESGCM(key_bytes)
            nonce = os.urandom(12)
            return nonce + aesgcm.encrypt(nonce, data.encode(), None)
        except Exception as e:
            return f'ENC_ERR: {e}'.encode()