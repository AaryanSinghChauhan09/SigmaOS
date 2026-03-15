# Generated method: SnakeGame._init_state
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class SnakeGame:
    def _init_state(self):
        self.W = 20
        self.H = 20
        self.snake = [(10, 10), (10, 9), (10, 8)]
        self.dir = (0, 1)
        self.food = (random.randint(0, 19), random.randint(0, 19))
        self.alive = True