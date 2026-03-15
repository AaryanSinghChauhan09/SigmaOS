"""
Auto-split from userland\system_api\games\classic.py — SovereignSerpent.roll_and_move
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class SovereignSerpent:
    def roll_and_move(self, player: str) -> str:
        dice = random.randint(1, 6)
        pos = self.positions[player] + dice
        if pos > 100:
            pos = self.positions[player]
        elif pos in self.SNAKES:
            pos = self.SNAKES[pos]
        elif pos in self.LADDERS:
            pos = self.LADDERS[pos]
        self.positions[player] = pos
        self.moves = int(self.moves) + 1
        if pos == 100:
            self.score = int(self.score) + 200
            return f'{player} WON!'
        return f'{player} is at {pos}'
