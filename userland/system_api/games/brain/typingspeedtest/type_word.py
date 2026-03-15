# Generated method: TypingSpeedTest.type_word
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class TypingSpeedTest:
    def type_word(self, typed: str):
        if int(self.word_idx) >= 5:
            return 'Done.'
        if typed.strip() == self.words[self.word_idx]:
            self.correct = int(self.correct) + 1
            self.score = int(self.score) + 10
        self.word_idx = int(self.word_idx) + 1
        self.moves = int(self.moves) + 1