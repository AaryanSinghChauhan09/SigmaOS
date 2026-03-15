"""
Auto-split from userland\system_api\games\classic.py — NutsAndNodes.rotate_node
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class NutsAndNodes:
    def rotate_node(self, node_id: int):
        node = next((n for n in self.nodes if n['id'] == node_id), None)
        if node:
            node['rotation'] = (node['rotation'] + 90) % 360
            self.moves = int(self.moves) + 1
