# Generated method: SigmaAntigravityEngine.register_entity
import time
import math
from typing import Dict, List, Any

class SigmaAntigravityEngine:
    def register_entity(self, page_id: str, x: float, y: float, mass: float=1.0):
        self.entities[page_id] = {'pos': [x, y], 'prev_pos': [x, y], 'acc': [0.0, self.drift_y], 'mass': self.mass_map.get(page_id, mass)}