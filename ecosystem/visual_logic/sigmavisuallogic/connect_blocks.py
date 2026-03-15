# Generated method: SigmaVisualLogic.connect_blocks
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import json

class SigmaVisualLogic:
    def connect_blocks(self, source_id: str, target_id: str):
        if source_id in self.active_canvas and target_id in self.active_canvas:
            self.active_canvas[source_id].next_block_id = target_id
            return f'Connected: {source_id} -> {target_id}'
        return 'Error: Block(s) not found.'