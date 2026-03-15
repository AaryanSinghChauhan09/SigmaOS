"""
Auto-split from userland\system_api\games\pro.py — ApexTetris._init_state
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class ApexTetris:
    def _init_state(self):
        self.grid = [[0] * 10 for _ in range(20)]
