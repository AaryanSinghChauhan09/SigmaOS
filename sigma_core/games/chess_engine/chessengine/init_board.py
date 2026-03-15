# Generated method: ChessEngine.init_board
import random
from typing import Dict, Any, List, Optional, Tuple, cast

class ChessEngine:
    def init_board(self) -> List[List[Optional[str]]]:
        b: List[List[Optional[str]]] = [[None for _ in range(8)] for _ in range(8)]
        for i in range(8):
            b[1][i] = 'B_P'
            b[6][i] = 'W_P'
        for i, p in enumerate(['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']):
            b[0][i] = f'B_{p}'
            b[7][i] = f'W_{p}'
        return b