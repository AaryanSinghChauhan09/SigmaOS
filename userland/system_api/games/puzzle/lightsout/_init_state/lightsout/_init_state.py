# Generated method: LightsOut._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class LightsOut:
    def _init_state(self):
        self.size = 5
        self.grid = [[random.choice([0, 1]) for _ in range(5)] for _ in range(5)]