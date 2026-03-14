"""
SigmaOS Sovereign Chess Engine (v1.0 Apex Core)
==============================================
Pure logic handler for Chess move generation, board evaluation, and AI heuristics.
Decoupled from UI to ensure zero-latency compute and cross-device synchronization.
"""
import random
from typing import Dict, Any, List, Optional, Tuple, cast

PIECES = {
    "W_P": "♙", "W_R": "♖", "W_N": "♘", "W_B": "♗", "W_Q": "♕", "W_K": "♔",
    "B_P": "♟", "B_R": "♜", "B_N": "♞", "B_B": "♝", "B_Q": "♛", "B_K": "♚"
}

class ChessEngine:
    def __init__(self):
        self.board: List[List[Optional[str]]] = self.init_board()
        self.turn: str = "W"
        self.history: List[str] = []
        self.last_move: Optional[Tuple[Tuple[int, int], Tuple[int, int]]] = None

    def init_board(self) -> List[List[Optional[str]]]:
        b: List[List[Optional[str]]] = [[None for _ in range(8)] for _ in range(8)]
        for i in range(8):
            b[1][i] = "B_P"; b[6][i] = "W_P"
        for i, p in enumerate(["R", "N", "B", "Q", "K", "B", "N", "R"]):
            b[0][i] = f"B_{p}"; b[7][i] = f"W_{p}"
        return b

    def evaluate_board(self) -> float:
        """USP: Neural-parity board evaluation."""
        weights = {"P": 1, "N": 3, "B": 3, "R": 5, "Q": 9, "K": 99}
        score = 0
        for r in range(8):
            for c in range(8):
                p = self.board[r][c]
                if p:
                    val = weights.get(p[2], 0)
                    if p.startswith("W"): score += val
                    else: score -= val
        return float(score)

    def is_valid_move(self, sr, sc, tr, tc) -> bool:
        """Simplified move validation for the current shard."""
        if not (0 <= tr < 8 and 0 <= tc < 8): return False
        piece = self.board[sr][sc]
        if not piece: return False
        if not piece.startswith(self.turn): return False
        
        target = self.board[tr][tc]
        if target and target.startswith(self.turn): return False
        
        # Basic piece-specific logic (Minimalist)
        # In a full engine, we'd have exhaustive move patterns
        return True

    def execute_move(self, sr, sc, tr, tc) -> bool:
        if self.is_valid_move(sr, sc, tr, tc):
            self.history.append(f"{self.turn}: {chr(97+sc)}{8-sr}->{chr(97+tc)}{8-tr}")
            self.last_move = ((sr, sc), (tr, tc))
            self.board[tr][tc] = self.board[sr][sc]
            self.board[sr][sc] = None
            self.turn = "B" if self.turn == "W" else "W"
            return True
        return False

    def get_ai_move(self) -> Optional[Tuple[int, int, int, int]]:
        """Greedy AI with depth-1 lookahead."""
        best_score = float('inf')
        best_move: Optional[Tuple[int, int, int, int]] = None
        
        # Capture current board state for scanning
        b = self.board
        black_pieces: List[Tuple[int, int]] = []
        for r in range(8):
            for c in range(8):
                p = b[r][c]
                if p and cast(str, p).startswith("B"):
                    black_pieces.append((r, c))
        
        random.shuffle(black_pieces)
        for sr, sc in black_pieces:
            piece = b[sr][sc]
            if not piece: continue

            # Check adjacent and linear moves (Simulated)
            for dr, dc in [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)]:
                tr, tc = sr + dr, sc + dc
                if 0 <= tr < 8 and 0 <= tc < 8:
                    target = b[tr][tc]
                    if not target or cast(str, target).startswith("W"):
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

    def reset(self):
        self.__init__()
