# Generated method: ShellGame._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class ShellGame:
    def _init_state(self):
        self.streak = 0
        self.score = 0