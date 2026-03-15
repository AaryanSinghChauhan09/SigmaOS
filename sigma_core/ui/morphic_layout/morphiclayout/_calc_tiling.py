# Generated method: MorphicLayout._calc_tiling
import math
from typing import List, Dict, Any, Tuple
from sigma_core.system.interfaces import SigmaModuleBase

class MorphicLayout:
    def _calc_tiling(self, windows: List[Any]) -> List[Tuple[int, int, int, int]]:
        count = len(windows)
        if count == 1:
            return [(self.padding, self.padding, self.screen_res[0] - 2 * self.padding, self.screen_res[1] - 2 * self.padding)]
        results = []
        width = self.screen_res[0] // 2
        height = self.screen_res[1]
        results.append((self.padding, self.padding, width - 2 * self.padding, height - 2 * self.padding))
        stack_height = self.screen_res[1] // (count - 1)
        for i in range(count - 1):
            results.append((width + self.padding, i * stack_height + self.padding, width - 2 * self.padding, stack_height - 2 * self.padding))
        return results