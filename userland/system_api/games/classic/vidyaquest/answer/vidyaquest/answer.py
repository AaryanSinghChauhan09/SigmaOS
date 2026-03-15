# Generated method: VidyaQuest.answer
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class VidyaQuest:
    def answer(self, ans):
        if ans == self.QUESTIONS[0]['a']:
            self.correct = int(self.correct) + 1
            self.score = int(self.score) + 100