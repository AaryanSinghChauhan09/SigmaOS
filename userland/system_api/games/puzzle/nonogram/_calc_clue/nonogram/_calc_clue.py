# Generated method: Nonogram._calc_clue
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class Nonogram:
    def _calc_clue(self, line):
        clues: List[int] = []
        current_run: int = 0
        for v in line:
            if v:
                current_run = int(current_run) + 1
            elif current_run:
                clues.append(current_run)
                current_run = 0
        if current_run:
            clues.append(current_run)
        return clues or [0]