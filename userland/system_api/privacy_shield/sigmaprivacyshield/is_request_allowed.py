"""
Auto-split from userland\system_api\privacy_shield.py — SigmaPrivacyShield.is_request_allowed
"""

from __future__ import annotations
import os
import re
import time
import hashlib
from typing import Any, Dict, List, Optional



class SigmaPrivacyShield:
    def is_request_allowed(self, domain: str) -> bool:
        """
            Returns False (block) if the domain is in the third-party blocklist.
            Call this from any network layer before making outbound connections.
            """
        for blocked in self._blocked_domains:
            if blocked in domain:
                self._stats['third_party_requests_blocked'] += 1
                return False
        return True
