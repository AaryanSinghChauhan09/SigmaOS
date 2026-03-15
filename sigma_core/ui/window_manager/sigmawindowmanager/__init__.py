# Generated method: SigmaWindowManager.__init__
from dataclasses import dataclass, field
import uuid
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaWindowManager:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self._windows: dict[str, SigmaWindow] = {}
        self._stack: list[str] = []
        self._screen_w = 2560
        self._screen_h = 1440