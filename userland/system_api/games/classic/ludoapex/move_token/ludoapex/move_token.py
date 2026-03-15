# Generated method: LudoApex.move_token
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class LudoApex:
    def move_token(self, player_color: str, token_idx: int) -> str:
        if self.dice == 0:
            return 'Roll dice first.'
        pos = self.tokens[player_color][token_idx]
        if pos == -1 and self.dice == 6:
            self.tokens[player_color][token_idx] = 0
            self.moves = int(self.moves) + 1
            return f'{player_color} Token {token_idx + 1}: Entered board!'
        elif pos == -1:
            return 'Need 6 to enter.'
        new_pos = min(pos + self.dice, 57)
        self.tokens[player_color][token_idx] = new_pos
        self.moves = int(self.moves) + 1
        if new_pos == 57:
            self.score = int(self.score) + 100
        return f'{player_color} Token {token_idx + 1}: {pos} → {new_pos}'