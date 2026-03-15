# Generated method: SigmaAntigravityEngine.apply_impulse
import time
import math
from typing import Dict, List, Any

class SigmaAntigravityEngine:
    def apply_impulse(self, eid: str, fx: float, fy: float):
        """Mouse impulse: Clicking provides a push."""
        if eid in self.entities:
            self.entities[eid]['pos'][0] += fx
            self.entities[eid]['pos'][1] += fy