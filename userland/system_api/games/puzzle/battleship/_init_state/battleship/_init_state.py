# Generated method: Battleship._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class Battleship:
    def _init_state(self):
        self.grid = [[0] * 10 for _ in range(10)]
        self.hits = []
        self.misses = []