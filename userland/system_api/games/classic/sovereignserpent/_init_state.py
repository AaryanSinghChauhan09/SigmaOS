"""
Auto-split from userland\system_api\games\classic.py — SovereignSerpent._init_state
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class SovereignSerpent:
    def _init_state(self):
        self.positions: Dict[str, int] = {}
        self.players: List[str] = []
