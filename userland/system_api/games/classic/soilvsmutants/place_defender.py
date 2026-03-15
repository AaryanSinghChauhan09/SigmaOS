"""
Auto-split from userland\system_api\games\classic.py — SoilVsMutants.place_defender
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class SoilVsMutants:
    def place_defender(self, r, c, t):
        if int(self.energy) >= 50:
            self.grid[r][c] = t
            self.energy = int(self.energy) - 50
            self.moves = int(self.moves) + 1
