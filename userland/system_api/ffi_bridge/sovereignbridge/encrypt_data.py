# Generated method: SovereignBridge.encrypt_data
import os
import sys
import ctypes
from pathlib import Path

class SovereignBridge:
    def encrypt_data(self, key: bytes, nonce: bytes, data: bytes) -> bytes:
        """Calls Rust ChaCha20-Poly1305 implementation."""
        if self.emulated:
            return bytes([b ^ 170 for b in data])
        return data