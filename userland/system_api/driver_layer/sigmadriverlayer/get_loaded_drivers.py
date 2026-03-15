"""
Auto-split from userland\system_api\driver_layer.py — SigmaDriverLayer.get_loaded_drivers
"""

import time
import uuid
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaDriverLayer:
    def get_loaded_drivers(self) -> list[dict]:
        return [{'hw_id': hw_id, 'name': r.name, 'class': r.cls.name, 'version': r.version, 'status': r.status.value} for hw_id, r in self._loaded.items()]
