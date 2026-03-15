# Generated method: HyperTrackRunner.swipe
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class HyperTrackRunner:
    def swipe(self, d):
        if d == 'left' and self.lane > 0:
            self.lane = int(self.lane) - 1
        if d == 'right' and self.lane < 2:
            self.lane = int(self.lane) + 1