# Generated method: AutomationBrain.train_on_feedback
import json
import os
import random
import time
from typing import Dict, List, Any

class AutomationBrain:
    def train_on_feedback(self, correct_cat: str):
        """Self-evolution: Slightly shifts weights towards successful outcomes."""
        print(f'[BRAIN] Learning from successful automation: {correct_cat}')
        if correct_cat in self.weights:
            self.weights[correct_cat] = [w * 1.05 for w in self.weights[correct_cat]]
            self._save_weights()