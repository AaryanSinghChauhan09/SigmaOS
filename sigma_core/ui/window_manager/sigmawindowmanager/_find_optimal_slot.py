# Generated method: SigmaWindowManager._find_optimal_slot
from dataclasses import dataclass, field
import uuid
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaWindowManager:
    def _find_optimal_slot(self, w: int, h: int) -> tuple[int, int]:
        """Calculates geometry to minimize overlap and maximize visibility."""
        if not self._windows:
            return (100, 100)
        rows, cols = (4, 4)
        cell_w, cell_h = (self._screen_w // cols, self._screen_h // rows)
        occupancy = [0] * (rows * cols)
        for win in self._windows.values():
            if win.minimized:
                continue
            c1, r1 = (win.x // cell_w, win.y // cell_h)
            c2, r2 = ((win.x + win.w) // cell_w, (win.y + win.h) // cell_h)
            for r in range(max(0, r1), min(rows, r2 + 1)):
                for c in range(max(0, c1), min(cols, c2 + 1)):
                    occupancy[r * cols + c] = occupancy[r * cols + c] + 1
        best_cell = occupancy.index(min(occupancy))
        r, c = (best_cell // cols, best_cell % cols)
        return (c * cell_w + 30, r * cell_h + 30)