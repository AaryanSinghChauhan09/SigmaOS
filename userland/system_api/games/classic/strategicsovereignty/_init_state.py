"""
Auto-split from userland\system_api\games\classic.py — StrategicSovereignty._init_state
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class StrategicSovereignty:
    def _init_state(self):
        self.board = [['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'], ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'], ['.', '.', '.', '.', '.', '.', '.', '.'], ['.', '.', '.', '.', '.', '.', '.', '.'], ['.', '.', '.', '.', '.', '.', '.', '.'], ['.', '.', '.', '.', '.', '.', '.', '.'], ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'], ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']]
        self.turn = 'white'
