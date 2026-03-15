# Generated method: CrowdFlowLegends.tick
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class CrowdFlowLegends:
    def tick(self):
        arrived = 0
        for a in self.agents:
            if a['x'] < self.goal_x:
                a['x'] = int(a['x']) + 1
            if a['x'] == self.goal_x:
                arrived = int(arrived) + 1
                self.score = int(self.score) + 50
        self.moves = int(self.moves) + 1
        return arrived