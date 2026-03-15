# Generated method: MorphicLayout._calc_sidebar
import math
from typing import List, Dict, Any, Tuple
from sigma_core.system.interfaces import SigmaModuleBase

class MorphicLayout:
    def _calc_sidebar(self, windows: List[Any]) -> List[Tuple[int, int, int, int]]:
        """USP: Side-car productivity layout (70/30 split)."""
        w_main = int(self.screen_res[0] * 0.7)
        w_side = self.screen_res[0] - w_main
        h = self.screen_res[1]
        results = [(self.padding, self.padding, w_main - 2 * self.padding, h - 2 * self.padding), (w_main + self.padding, self.padding, w_side - 2 * self.padding, h - 2 * self.padding)]
        return results[:len(windows)]