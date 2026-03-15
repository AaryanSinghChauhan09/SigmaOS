# Generated method: MazeChasePacStyle._init_state
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class MazeChasePacStyle:
    def _init_state(self):
        self.maze = ['#########', '#...#...#', '#########']
        self.pos = (1, 1)
        self.ghosts = [(1, 7)]