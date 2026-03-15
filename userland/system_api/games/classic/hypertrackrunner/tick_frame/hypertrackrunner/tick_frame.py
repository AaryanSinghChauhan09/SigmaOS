# Generated method: HyperTrackRunner.tick_frame
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class HyperTrackRunner:
    def tick_frame(self):
        self.distance = float(self.distance) + float(self.speed) / 10.0
        self.score = int(self.distance)