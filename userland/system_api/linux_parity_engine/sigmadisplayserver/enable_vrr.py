"""
Auto-split from userland\system_api\linux_parity_engine.py — SigmaDisplayServer.enable_vrr
"""

import time
import uuid
import random
from typing import Dict, List, Any



class SigmaDisplayServer:
    def enable_vrr(self) -> str:
        self._vrr_enabled = True
        return f'[display] VRR enabled (FreeSync/G-Sync compatible) at {self._refresh_hz}Hz.'
