"""
Auto-split from userland\system_api\games\classic.py — VidyaQuest._init_state
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class VidyaQuest:
    def _init_state(self):
        self.word_idx = 0
        self.correct = 0
