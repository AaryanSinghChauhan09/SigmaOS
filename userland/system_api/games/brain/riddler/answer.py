# Generated method: Riddler.answer
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class Riddler:
    def answer(self, ans: str):
        if ans.lower().strip() == self.RIDDLES[0]['a']:
            self.score = int(self.score) + 50
            self.idx = int(self.idx) + 1
            self.moves = int(self.moves) + 1