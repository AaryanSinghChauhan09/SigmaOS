"""
SigmaOS Morphic Layout Engine (v1.0 Apex)
==========================================
USP: Dynamic Tiling, Grid Snapping, and Workspace Architecture.
Absorbs USP of: i3 (tiling), FancyZones (grid), and Stage Manager (clustering).
"""
import math
from typing import List, Dict, Any, Tuple
from sigma_core.system.interfaces import SigmaModuleBase

class MorphicLayout(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.screen_res = (1920, 1080)
        self.active_layout = "FLOATING" # FLOATING, TILING, GRID, STAGE, QUARTERS, PILLAR, SIDEBAR
        self.padding = 10

    def calculate_positions(self, windows: List[Any]) -> List[Tuple[int, int, int, int]]:
        """USP: Adaptive Geometric Tiling."""
        if not windows: return []
        
        layout = self.active_layout.upper()
        if layout == "TILING":
            return self._calc_tiling(windows)
        elif layout == "GRID":
            return self._calc_grid(windows)
        elif layout == "QUARTERS":
            return self._calc_quarters(windows)
        elif layout == "PILLAR":
            return self._calc_pillar(windows)
        elif layout == "SIDEBAR":
            return self._calc_sidebar(windows)
        else: # Default Floating with Snapping
            return [(100, 100, 800, 600) for _ in windows]

    def _calc_tiling(self, windows: List[Any]) -> List[Tuple[int, int, int, int]]:
        count = len(windows)
        if count == 1:
            return [(self.padding, self.padding, self.screen_res[0]-2*self.padding, self.screen_res[1]-2*self.padding)]
        
        # Binary Splitting Logic (Simplified)
        results = []
        width = self.screen_res[0] // 2
        height = self.screen_res[1]
        results.append((self.padding, self.padding, width-2*self.padding, height-2*self.padding)) # Master
        
        stack_height = self.screen_res[1] // (count - 1)
        for i in range(count - 1):
            results.append((width + self.padding, i * stack_height + self.padding, width - 2*self.padding, stack_height - 2*self.padding))
        return results

    def _calc_grid(self, windows: List[Any]) -> List[Tuple[int, int, int, int]]:
        count = len(windows)
        cols = math.ceil(math.sqrt(count))
        rows = math.ceil(count / cols)
        
        w = self.screen_res[0] // cols
        h = self.screen_res[1] // rows
        
        results = []
        for i in range(count):
            r, c = divmod(i, cols)
            results.append((c * w + self.padding, r * h + self.padding, w - 2*self.padding, h - 2*self.padding))
        return results

    def _calc_quarters(self, windows: List[Any]) -> List[Tuple[int, int, int, int]]:
        """USP: Windows 11 2x2 Snap Layout."""
        w = self.screen_res[0] // 2
        h = self.screen_res[1] // 2
        results = [
            (self.padding, self.padding, w - 2*self.padding, h - 2*self.padding),
            (w + self.padding, self.padding, w - 2*self.padding, h - 2*self.padding),
            (self.padding, h + self.padding, w - 2*self.padding, h - 2*self.padding),
            (w + self.padding, h + self.padding, w - 2*self.padding, h - 2*self.padding)
        ]
        return results[:len(windows)]

    def _calc_pillar(self, windows: List[Any]) -> List[Tuple[int, int, int, int]]:
        """USP: Three-column pillar layout for research (Competitor UX)."""
        w = self.screen_res[0] // 3
        h = self.screen_res[1]
        results = [
            (self.padding, self.padding, w - 2*self.padding, h - 2*self.padding),
            (w + self.padding, self.padding, w - 2*self.padding, h - 2*self.padding),
            (2*w + self.padding, self.padding, w - 2*self.padding, h - 2*self.padding)
        ]
        return results[:len(windows)]

    def _calc_sidebar(self, windows: List[Any]) -> List[Tuple[int, int, int, int]]:
        """USP: Side-car productivity layout (70/30 split)."""
        w_main = int(self.screen_res[0] * 0.7)
        w_side = self.screen_res[0] - w_main
        h = self.screen_res[1]
        results = [
            (self.padding, self.padding, w_main - 2*self.padding, h - 2*self.padding),
            (w_main + self.padding, self.padding, w_side - 2*self.padding, h - 2*self.padding)
        ]
        return results[:len(windows)]

    def switch_layout(self, layout_type: str):
        self.active_layout = layout_type.upper()
        self.log_event("layout_switch", {"type": self.active_layout})
