# Generated method: TypingSpeedTest._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class TypingSpeedTest:
    def _init_state(self):
        self.words = random.sample(self.WORDS, 5)
        self.word_idx = 0
        self.correct = 0
        self.start = time.time()