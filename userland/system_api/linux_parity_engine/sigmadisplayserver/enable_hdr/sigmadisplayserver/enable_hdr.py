# Generated method: SigmaDisplayServer.enable_hdr
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaDisplayServer:
    def enable_hdr(self) -> str:
        self._hdr_enabled = True
        return '[display] HDR enabled (10-bit, BT.2020 color space).'