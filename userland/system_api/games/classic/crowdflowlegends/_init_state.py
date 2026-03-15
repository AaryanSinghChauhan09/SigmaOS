"""
Auto-split from userland\system_api\games\classic.py — CrowdFlowLegends._init_state
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class CrowdFlowLegends:
    def _init_state(self):
        self.agents = [{'id': i, 'x': 0, 'y': random.randint(0, 9)} for i in range(10)]
        self.obstacles = [(3, i) for i in range(3, 8)]
        self.goal_x = 9
