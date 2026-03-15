# Generated method: FastFive._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class FastFive:
    def _init_state(self):
        self.pool = self.QUESTIONS
        self.idx = 0
        self.start = time.time()