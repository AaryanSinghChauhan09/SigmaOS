"""
Auto-split from userland\system_api\games\puzzle.py — SpotItGame._init_state
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class SpotItGame:
    def _init_state(self):
        self.idx = 0
        self.score = 0
