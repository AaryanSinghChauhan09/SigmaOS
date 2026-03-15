# Generated method: ChessEngine.execute_move
import random
from typing import Dict, Any, List, Optional, Tuple, cast

class ChessEngine:
    def execute_move(self, sr, sc, tr, tc) -> bool:
        if self.is_valid_move(sr, sc, tr, tc):
            self.history.append(f'{self.turn}: {chr(97 + sc)}{8 - sr}->{chr(97 + tc)}{8 - tr}')
            self.last_move = ((sr, sc), (tr, tc))
            self.board[tr][tc] = self.board[sr][sc]
            self.board[sr][sc] = None
            self.turn = 'B' if self.turn == 'W' else 'W'
            return True
        return False