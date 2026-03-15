"""
Auto-split from userland\system_api\games\puzzle.py — MemoryMatch._init_state
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class MemoryMatch:
    def _init_state(self):
        n = 4
        syms = ['🍎', '🍊', '🍋', '🍇', '🍓', '🍒', '🌸', '🌺'] * 2
        random.shuffle(syms)
        self.cards = syms
        self.revealed = [False] * 16
        self.matched = set()
