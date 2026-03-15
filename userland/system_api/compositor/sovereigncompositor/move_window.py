# Generated method: SovereignCompositor.move_window
from dataclasses import dataclass, field
from typing import List

class SovereignCompositor:
    def move_window(self, win_id: str, dx: int, dy: int):
        win = next((w for w in self.windows if w.id == win_id), None)
        if win:
            win.x += dx
            win.y += dy
            self.dirty_rects.append((win.x, win.y, win.width, win.height))