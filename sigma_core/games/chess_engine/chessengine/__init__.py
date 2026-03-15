# Generated method: ChessEngine.__init__
import random
from typing import Dict, Any, List, Optional, Tuple, cast

class ChessEngine:
    def __init__(self):
        self.board: List[List[Optional[str]]] = self.init_board()
        self.turn: str = 'W'
        self.history: List[str] = []
        self.last_move: Optional[Tuple[Tuple[int, int], Tuple[int, int]]] = None