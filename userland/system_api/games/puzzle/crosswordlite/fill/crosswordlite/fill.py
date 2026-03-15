# Generated method: CrosswordLite.fill
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class CrosswordLite:
    def fill(self, num, direction, answer):
        self.answers[num] = answer.upper()
        self.moves = int(self.moves) + 1
        return 'Filled.'