# Generated method: ChessEngine.evaluate_board
import random
from typing import Dict, Any, List, Optional, Tuple, cast

class ChessEngine:
    def evaluate_board(self) -> float:
        """USP: Neural-parity board evaluation."""
        weights = {'P': 1, 'N': 3, 'B': 3, 'R': 5, 'Q': 9, 'K': 99}
        score = 0
        for r in range(8):
            for c in range(8):
                p = self.board[r][c]
                if p:
                    val = weights.get(p[2], 0)
                    if p.startswith('W'):
                        score += val
                    else:
                        score -= val
        return float(score)