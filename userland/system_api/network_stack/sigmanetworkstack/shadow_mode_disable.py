"""
Auto-split from userland\system_api\network_stack.py — SigmaNetworkStack.shadow_mode_disable
"""

import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaNetworkStack:
    def shadow_mode_disable(self, app_name: str) -> dict:
        self._shadow_mode.pop(app_name, None)
        return {'app': app_name, 'mode': 'normal', 'message': f"NetworkShadow: '{app_name}' restored to normal networking."}
