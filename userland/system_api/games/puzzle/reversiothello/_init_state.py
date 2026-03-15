"""
Auto-split from userland\system_api\games\puzzle.py — ReversiOthello._init_state
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class ReversiOthello:
    def _init_state(self):
        self.board = [[0] * 8 for _ in range(8)]
        self.turn = 1
