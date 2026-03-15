# Generated method: SovereignSerpent.add_player
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class SovereignSerpent:
    def add_player(self, name: str) -> str:
        self.players.append(name)
        self.positions[name] = 0
        return f"Player '{name}' joined."