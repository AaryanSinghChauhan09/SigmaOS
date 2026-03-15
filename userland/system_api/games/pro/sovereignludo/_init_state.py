"""
Auto-split from userland\system_api\games\pro.py — SovereignLudo._init_state
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class SovereignLudo:
    def _init_state(self):
        self.turn = 0
