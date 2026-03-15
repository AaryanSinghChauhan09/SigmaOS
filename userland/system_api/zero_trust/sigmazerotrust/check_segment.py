"""
Auto-split from userland\system_api\zero_trust.py — SigmaZeroTrust.check_segment
"""

import time
import uuid
import hashlib
import hmac
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaZeroTrust:
    def check_segment(self, src: str, dst: str) -> dict:
        allowed = self._segments.get(src, set())
        permitted = dst in allowed or not self._segments.get(src)
        return {'src': src, 'dst': dst, 'permitted': permitted, 'message': f"MicroSegment: '{src}' → '{dst}' {('PERMITTED' if permitted else 'BLOCKED (lateral movement prevented)')}."}
