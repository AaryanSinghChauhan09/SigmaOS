# Generated method: MathSprint.answer
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class MathSprint:
    def answer(self, val: int):
        self.moves = int(self.moves) + 1
        if val == self.ans:
            self.score = int(self.score) + 10
            self._gen_q()
            return '✅ Correct!'
        return '❌ Wrong.'