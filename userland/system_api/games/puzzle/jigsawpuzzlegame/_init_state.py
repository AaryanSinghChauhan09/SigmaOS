"""
Auto-split from userland\system_api\games\puzzle.py — JigsawPuzzleGame._init_state
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class JigsawPuzzleGame:
    def _init_state(self):
        self.grid = 4
        self.moves = 0
        self.solved = False
