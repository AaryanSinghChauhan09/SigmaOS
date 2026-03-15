"""
Auto-split from userland\system_api\games\classic.py — SovereignSudoku._init_state
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class SovereignSudoku:
    def _init_state(self):
        self.board = [[0] * 9 for _ in range(9)]
