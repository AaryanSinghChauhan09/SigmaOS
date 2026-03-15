# Generated class core: QuantumKeyBundle
import hashlib
import hmac
import os
import time
import secrets
from dataclasses import dataclass

@dataclass
class QuantumKeyBundle:
    key_id: str
    public_key: str
    private_key: str
    algorithm: str = 'Kyber-1024-Sovereign'