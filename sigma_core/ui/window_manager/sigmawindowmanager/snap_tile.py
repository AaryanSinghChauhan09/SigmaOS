# Generated method: SigmaWindowManager.snap_tile
from dataclasses import dataclass, field
import uuid
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaWindowManager:
    def snap_tile(self, win_id: str, direction: str):
        """USP: Dynamic Snap-Grid via Sigma-Layout-Atoms."""
        win = self._windows.get(win_id)
        if not win:
            return
        if direction == 'left':
            win.x, win.y = (10, 10)
            win.w, win.h = (self._screen_w // 2 - 15, self._screen_h - 20)
        elif direction == 'right':
            win.x, win.y = (self._screen_w // 2 + 5, 10)
            win.w, win.h = (self._screen_w // 2 - 15, self._screen_h - 20)
        elif direction == 'maximize':
            win.x, win.y = (0, 0)
            win.w, win.h = (self._screen_w, self._screen_h)
            win.maximized = True
        print(f'[WMS] Tiled: {win.title} -> {direction} grid.')