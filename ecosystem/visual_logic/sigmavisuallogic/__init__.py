# Generated method: SigmaVisualLogic.__init__
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import json

class SigmaVisualLogic:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.active_canvas: Dict[str, VisualBlock] = {}
        self._stats = {'blocks_placed': 0, 'chains_compiled': 0}