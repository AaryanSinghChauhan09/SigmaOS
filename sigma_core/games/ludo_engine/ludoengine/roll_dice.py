# Generated method: LudoEngine.roll_dice
import random
from typing import Dict, Any, List, Optional, Tuple

class LudoEngine:
    def roll_dice(self) -> int:
        self.dice_val = random.randint(1, 6)
        return self.dice_val