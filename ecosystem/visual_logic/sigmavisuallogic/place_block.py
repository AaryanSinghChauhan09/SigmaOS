# Generated method: SigmaVisualLogic.place_block
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import json

class SigmaVisualLogic:
    def place_block(self, block_type: str, name: str, **inputs) -> str:
        bid = f'block_{len(self.active_canvas)}'
        block = VisualBlock(bid, block_type, name, inputs)
        self.active_canvas[bid] = block
        self._stats['blocks_placed'] += 1
        return bid