# Generated method: SigmaAntigravityEngine.gather_all
import time
import math
from typing import Dict, List, Any

class SigmaAntigravityEngine:
    def gather_all(self):
        """The Gather Command: Pull all drift entities back to center."""
        for eid in self.entities:
            self.entities[eid]['pos'] = [self.bounds[0] / 2, self.bounds[1] / 2]
            self.entities[eid]['prev_pos'] = [self.bounds[0] / 2, self.bounds[1] / 2]
            self.entities[eid]['acc'] = [0.0, 0.0]
        self.kernel.bus.emit('ag.gather', {'status': 'CENTERED'})