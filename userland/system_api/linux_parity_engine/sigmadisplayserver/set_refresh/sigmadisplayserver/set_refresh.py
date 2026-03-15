# Generated method: SigmaDisplayServer.set_refresh
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaDisplayServer:
    def set_refresh(self, hz: int) -> str:
        self._refresh_hz = hz
        return f'[display] Refresh rate set to {hz}Hz.'