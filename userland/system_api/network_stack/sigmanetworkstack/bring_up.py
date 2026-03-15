"""
Auto-split from userland\system_api\network_stack.py — SigmaNetworkStack.bring_up
"""

import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaNetworkStack:
    def bring_up(self, iface_name: str) -> dict:
        iface = self._interfaces.get(iface_name)
        if iface is None:
            return {'error': f"Interface '{iface_name}' not found."}
        iface.up = True
        self._audit_event('iface_up', iface_name)
        return {'status': 'up', 'iface': iface_name, 'message': f"NetStack: Interface '{iface_name}' brought up ({iface.speed_mbps:.0f}Mbps)."}
