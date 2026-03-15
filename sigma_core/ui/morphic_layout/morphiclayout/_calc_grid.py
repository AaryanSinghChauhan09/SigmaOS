# Generated method: MorphicLayout._calc_grid
import math
from typing import List, Dict, Any, Tuple
from sigma_core.system.interfaces import SigmaModuleBase

class MorphicLayout:
    def _calc_grid(self, windows: List[Any]) -> List[Tuple[int, int, int, int]]:
        count = len(windows)
        cols = math.ceil(math.sqrt(count))
        rows = math.ceil(count / cols)
        w = self.screen_res[0] // cols
        h = self.screen_res[1] // rows
        results = []
        for i in range(count):
            r, c = divmod(i, cols)
            results.append((c * w + self.padding, r * h + self.padding, w - 2 * self.padding, h - 2 * self.padding))
        return results