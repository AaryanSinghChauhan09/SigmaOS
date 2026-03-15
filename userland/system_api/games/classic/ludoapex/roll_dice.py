"""
Auto-split from userland\system_api\games\classic.py — LudoApex.roll_dice
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class LudoApex:
    def roll_dice(self) -> int:
        self.dice = random.randint(1, 6)
        return self.dice
