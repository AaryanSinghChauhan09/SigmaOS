# Generated method: ChessEngine.is_valid_move
import random
from typing import Dict, Any, List, Optional, Tuple, cast

class ChessEngine:
    def is_valid_move(self, sr, sc, tr, tc) -> bool:
        """Simplified move validation for the current shard."""
        if not (0 <= tr < 8 and 0 <= tc < 8):
            return False
        piece = self.board[sr][sc]
        if not piece:
            return False
        if not piece.startswith(self.turn):
            return False
        target = self.board[tr][tc]
        if target and target.startswith(self.turn):
            return False
        return True