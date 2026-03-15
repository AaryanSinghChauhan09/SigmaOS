"""
Auto-split from userland\system_api\network_stack.py — SigmaNetworkStack.bring_down
"""

import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaNetworkStack:
    def bring_down(self, iface_name: str) -> dict:
        iface = self._interfaces.get(iface_name)
        if iface is None:
            return {'error': f"Interface '{iface_name}' not found."}
        iface.up = False
        self._audit_event('iface_down', iface_name)
        return {'status': 'down', 'iface': iface_name, 'message': f"NetStack: Interface '{iface_name}' brought down."}
