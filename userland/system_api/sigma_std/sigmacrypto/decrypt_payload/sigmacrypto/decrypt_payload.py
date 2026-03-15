# Generated method: SigmaCrypto.decrypt_payload
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
    def decrypt_payload(encrypted_data: bytes, key_bytes: bytes) -> str:
        """USP: Authenticated Decryption with Integrity Check."""
        try:
            from cryptography.hazmat.primitives.ciphers.aead import AESGCM
            nonce = encrypted_data[:12]
            ciphertext = encrypted_data[12:]
            aesgcm = AESGCM(key_bytes)
            decrypted = aesgcm.decrypt(nonce, ciphertext, None)
            return decrypted.decode('utf-8', errors='ignore')
        except Exception as e:
            return f'DEC_ERR: {str(e)}'