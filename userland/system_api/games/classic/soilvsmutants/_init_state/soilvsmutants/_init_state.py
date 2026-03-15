# Generated method: SoilVsMutants._init_state
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class SoilVsMutants:
    def _init_state(self):
        self.grid = [[None] * 9 for _ in range(5)]
        self.energy = 200
        self.wave = 0
        self.lives = 5