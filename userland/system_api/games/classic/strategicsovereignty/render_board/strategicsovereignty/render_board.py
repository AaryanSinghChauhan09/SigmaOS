# Generated method: StrategicSovereignty.render_board
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class StrategicSovereignty:
    def render_board(self) -> str:
        if self.COMPRESSED:
            return '[Strategic Sovereignty] — COMPRESSED.'
        lines = ['  a b c d e f g h']
        for i, row in enumerate(self.board):
            r = [self.PIECES.get(p, '·') for p in row]
            lines.append(f"{8 - i} {' '.join(r)} {8 - i}")
        lines.append('  a b c d e f g h')
        return '\n'.join(lines)