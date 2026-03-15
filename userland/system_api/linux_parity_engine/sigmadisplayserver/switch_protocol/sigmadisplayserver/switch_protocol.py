# Generated method: SigmaDisplayServer.switch_protocol
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaDisplayServer:
    def switch_protocol(self, proto: str) -> str:
        if proto not in ('Wayland', 'X11', 'XWayland'):
            return f"[display] Unknown protocol '{proto}'."
        self._protocol = proto
        return f'[display] Switched to {proto}. XWayland={self._xwayland_active}.'