# Generated method: Riddler._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class Riddler:
    def _init_state(self):
        self.idx = 0
        self.hints_used = 0