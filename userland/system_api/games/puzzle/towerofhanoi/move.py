"""
Auto-split from userland\system_api\games\puzzle.py — TowerOfHanoi.move
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class TowerOfHanoi:
    def move(self, src: str, dst: str):
        if not self.pegs[src]:
            return 'Empty.'
        d = self.pegs[src][-1]
        if self.pegs[dst] and self.pegs[dst][-1] < d:
            return 'Invalid.'
        self.pegs[src].pop()
        self.pegs[dst].append(d)
        self.moves = int(self.moves) + 1
        if len(self.pegs['C']) == 5:
            self.score = int(self.score) + 1000
            return 'SOLVED!'
