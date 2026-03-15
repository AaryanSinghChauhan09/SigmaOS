# Generated method: SigmaAppStore._compute_checksum
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json

class SigmaAppStore:
    def _compute_checksum(self, app_id: str, version: str) -> str:
        """Sovereign HMAC-style checksum (deterministic, no external deps)."""
        raw = f'SIGMA_OS|{app_id}|{version}|SOVEREIGN_SEAL'
        return hashlib.sha256(raw.encode()).hexdigest()