# Generated method: LudoApex._init_state
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class LudoApex:
    def _init_state(self):
        self.tokens: Dict[str, List[int]] = {c: [-1, -1, -1, -1] for c in self.COLORS}
        self.current_player = 0
        self.dice = 0