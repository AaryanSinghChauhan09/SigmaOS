# Generated method: SigmaPrivacyShield.set_resource_usage
from __future__ import annotations
import os
import re
import time
import hashlib
from typing import Any, Dict, List, Optional

class SigmaPrivacyShield:
    def set_resource_usage(self, resource_name: str, in_use: bool) -> None:
        """Global Privacy Indicator: emits a bus event when a sensitive resource is accessed."""
        status = 'ACTIVE' if in_use else 'IDLE'
        bus = getattr(self.kernel, 'bus', None)
        if bus is not None:
            bus.emit('privacy.resource_usage', {'resource': resource_name, 'status': status})
        print(f"[PRIVACY] Resource '{resource_name}' → {status}")