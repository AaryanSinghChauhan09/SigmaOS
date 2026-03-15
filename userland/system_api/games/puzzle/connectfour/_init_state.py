"""
Auto-split from userland\system_api\games\puzzle.py — ConnectFour._init_state
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class ConnectFour:
    def _init_state(self):
        self.board = [[0] * 7 for _ in range(6)]
        self.turn = 1
