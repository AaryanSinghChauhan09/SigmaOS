# Generated method: MathSprint._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class MathSprint:
    def _init_state(self):
        self.level = 1
        self._gen_q()