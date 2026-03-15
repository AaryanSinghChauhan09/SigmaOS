# Generated method: MazeChasePacStyle.move
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class MazeChasePacStyle:
    def move(self, d):
        dr, dc = {'up': (-1, 0), 'down': (1, 0), 'left': (0, -1), 'right': (0, 1)}.get(d, (0, 0))
        h, w = (len(self.maze), len(self.maze[0]))
        nr, nc = (self.pos[0] + dr, self.pos[1] + dc)
        if 0 <= nr < h and 0 <= nc < w and (self.maze[nr][nc] != '#'):
            self.pos = (nr, nc)
            self.moves = int(self.moves) + 1