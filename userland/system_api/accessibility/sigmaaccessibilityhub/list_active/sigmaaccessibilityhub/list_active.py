# Generated method: SigmaAccessibilityHub.list_active
from dataclasses import dataclass
from enum import Enum, auto
import threading

class SigmaAccessibilityHub:
    def list_active(self) -> list[str]:
        return [f for f, active in self._active_features.items() if active]