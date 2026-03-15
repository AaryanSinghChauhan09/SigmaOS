"""
Auto-split from userland\system_api\network_stack.py — SigmaNetworkStack.calculate_ip_checksum
"""

import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaNetworkStack:
    def calculate_ip_checksum(self, data: bytes) -> int:
        """USP: 1's Complement Sum for Header Verification."""
        if len(data) % 2:
            data += b'\x00'
        res = sum((int.from_bytes(data[i:i + 2], 'big') for i in range(0, len(data), 2)))
        while res > 65535:
            res = (res & 65535) + (res >> 16)
        return ~res & 65535
