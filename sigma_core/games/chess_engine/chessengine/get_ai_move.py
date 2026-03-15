# Generated method: ChessEngine.get_ai_move
import random
from typing import Dict, Any, List, Optional, Tuple, cast

class ChessEngine:
    def get_ai_move(self) -> Optional[Tuple[int, int, int, int]]:
        """Greedy AI with depth-1 lookahead."""
        best_score = float('inf')
        best_move: Optional[Tuple[int, int, int, int]] = None
        b = self.board
        black_pieces: List[Tuple[int, int]] = []
        for r in range(8):
            for c in range(8):
                p = b[r][c]
                if p and cast(str, p).startswith('B'):
                    black_pieces.append((r, c))
        random.shuffle(black_pieces)
        for sr, sc in black_pieces:
            piece = b[sr][sc]
            if not piece:
                continue
            for dr, dc in [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)]:
                tr, tc = (sr + dr, sc + dc)
                if 0 <= tr < 8 and 0 <= tc < 8:
                    target = b[tr][tc]
                    if not target or cast(str, target).startswith('W'):
                        old_target = b[tr][tc]
                        b[tr][tc] = piece
                        b[sr][sc] = None
                        score = self.evaluate_board()
                        if score < best_score:
                            best_score = score
                            best_move = (sr, sc, tr, tc)
                        b[sr][sc] = piece
                        b[tr][tc] = old_target
        return best_move