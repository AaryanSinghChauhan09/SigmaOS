"""
Auto-split from userland\system_api\games\puzzle.py — MathSprint._gen_q
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class MathSprint:
    def _gen_q(self):
        a = random.randint(1, 10)
        b = random.randint(1, 10)
        self.q = f'{a} + {b}'
        self.ans = a + b
