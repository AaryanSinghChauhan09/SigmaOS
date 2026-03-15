# Generated method: SnakeGame.tick
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class SnakeGame:
    def tick(self):
        if not self.alive:
            return 'Game over.'
        h = self.snake[0]
        nr, nc = (h[0] + self.dir[0], h[1] + self.dir[1])
        if not (0 <= nr < 20 and 0 <= nc < 20) or (nr, nc) in self.snake:
            self.alive = False
            return '💀 GAME OVER!'
        self.snake.insert(0, (nr, nc))
        self.moves = int(self.moves) + 1
        if (nr, nc) == self.food:
            self.score = int(self.score) + 10
            self.food = (random.randint(0, 19), random.randint(0, 19))
        else:
            self.snake.pop()