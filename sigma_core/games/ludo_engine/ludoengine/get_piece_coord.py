# Generated method: LudoEngine.get_piece_coord
import random
from typing import Dict, Any, List, Optional, Tuple

class LudoEngine:
    def get_piece_coord(self, color: str, idx: int) -> Tuple[int, int]:
        pos = self.piece_states[color][idx]
        if pos == 0:
            return self.yard_coords[color][idx]
        offsets = {'RED': 0, 'GREEN': 13, 'BLUE': 26, 'YELLOW': 39}
        base = 300
        angle = (pos + offsets[color]) * (360 / 52)
        import math
        r = 200
        x = base + r * math.cos(math.radians(angle))
        y = base + r * math.sin(math.radians(angle))
        return (int(x), int(y))