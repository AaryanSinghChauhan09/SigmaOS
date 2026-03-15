# Generated method: NutsAndNodes._init_state
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class NutsAndNodes:
    def _init_state(self):
        self.grid_size = 6
        self.nodes = [{'id': i, 'x': random.randint(0, 5), 'y': random.randint(0, 5), 'rotation': 0, 'connected': []} for i in range(5)]