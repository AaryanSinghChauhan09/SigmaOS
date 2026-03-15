# Generated method: SigmaVirtualizationLayer.health_check
import time
import uuid
from dataclasses import dataclass
from enum import Enum, auto

class SigmaVirtualizationLayer:
    def health_check(self) -> str:
        active = sum((1 for c in self._containers.values() if c.state == ContainerState.RUNNING))
        return f'OK — {len(self._containers)} containers, {active} running.'