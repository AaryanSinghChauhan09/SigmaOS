# Generated method: FindTheWord._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class FindTheWord:
    def _init_state(self):
        self.grid = [['A'] * 8 for _ in range(8)]
        self.matched = set()