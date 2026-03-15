# Generated method: LudoEngine.move_piece
import random
from typing import Dict, Any, List, Optional, Tuple

class LudoEngine:
    def move_piece(self, color: str, piece_idx: int) -> bool:
        """USP: Atomic piece movement via path-indexed quantization."""
        if color != self.turn:
            return False
        pos = self.piece_states[color][piece_idx]
        v = self.dice_val
        if pos == 0:
            if v == 6:
                self.piece_states[color][piece_idx] = 1
                self.history.append(f'{color} #{piece_idx}: DEPLOYED TO GRID')
                return True
            return False
        new_pos = pos + v
        if new_pos > 56:
            return False
        self.piece_states[color][piece_idx] = new_pos
        self.history.append(f'{color} #{piece_idx}: ADVANCED TO {new_pos}')
        for other_color, states in self.piece_states.items():
            if other_color == color:
                continue
            for i, other_pos in enumerate(states):
                if other_pos == new_pos and new_pos != 0:
                    self.piece_states[other_color][i] = 0
                    self.history.append(f'CRITICAL: {color} ELIMINATED {other_color} AT {new_pos}')
        if v != 6:
            colors = ['RED', 'GREEN', 'BLUE', 'YELLOW']
            idx = colors.index(self.turn)
            self.turn = colors[(idx + 1) % 4]
        return True