# Generated method: MemoryMatch.flip
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class MemoryMatch:
    def flip(self, idx: int) -> str:
        if idx in self.matched or self.revealed[idx]:
            return 'Already revealed.'
        self.revealed[idx] = True
        self.moves = int(self.moves) + 1
        s = self.cards[idx]
        p = next((i for i, r in enumerate(self.revealed) if r and i != idx and (self.cards[i] == s) and (i not in self.matched)), None)
        if p is not None:
            self.matched.add(idx)
            self.matched.add(p)
            self.score = int(self.score) + 50
            if len(self.matched) == 16:
                return 'ALL MATCHED!'
        return f'Flipped card {idx}'