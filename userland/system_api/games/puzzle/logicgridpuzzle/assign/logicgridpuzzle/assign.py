# Generated method: LogicGridPuzzle.assign
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class LogicGridPuzzle:
    def assign(self, person, language):
        self.answers[person] = language
        self.moves = int(self.moves) + 1