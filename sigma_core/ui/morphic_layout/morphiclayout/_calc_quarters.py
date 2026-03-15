# Generated method: MorphicLayout._calc_quarters
import math
from typing import List, Dict, Any, Tuple
from sigma_core.system.interfaces import SigmaModuleBase

class MorphicLayout:
    def _calc_quarters(self, windows: List[Any]) -> List[Tuple[int, int, int, int]]:
        """USP: Windows 11 2x2 Snap Layout."""
        w = self.screen_res[0] // 2
        h = self.screen_res[1] // 2
        results = [(self.padding, self.padding, w - 2 * self.padding, h - 2 * self.padding), (w + self.padding, self.padding, w - 2 * self.padding, h - 2 * self.padding), (self.padding, h + self.padding, w - 2 * self.padding, h - 2 * self.padding), (w + self.padding, h + self.padding, w - 2 * self.padding, h - 2 * self.padding)]
        return results[:len(windows)]