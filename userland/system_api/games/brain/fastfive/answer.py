# Generated method: FastFive.answer
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class FastFive:
    def answer(self, ans: str):
        if ans.strip() == self.QUESTIONS[0]['a']:
            self.score = int(self.score) + 50
            self.idx = int(self.idx) + 1
            self.moves = int(self.moves) + 1