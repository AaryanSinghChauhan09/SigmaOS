# Generated method: ZBufferEngine.is_occluded
from typing import List, Dict, Any

class ZBufferEngine:
    def is_occluded(self, win_id: str, x: int, y: int) -> bool:
        """Determines if a pixel at (x,y) is covered by a window above win_id."""
        if win_id not in self.z_map:
            return False
        idx = self.z_map.index(win_id)
        return False