# Generated method: ChromaticCrushUltra._init_state
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class ChromaticCrushUltra:
    def _init_state(self):
        self.board = [[random.randint(1, 6) for _ in range(8)] for _ in range(8)]