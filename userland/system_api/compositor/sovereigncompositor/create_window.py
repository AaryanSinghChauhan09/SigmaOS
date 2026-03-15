# Generated method: SovereignCompositor.create_window
from dataclasses import dataclass, field
from typing import List

class SovereignCompositor:
    def create_window(self, pid: int, x: int, y: int, w: int, h: int) -> str:
        import uuid
        win_id = str(uuid.uuid4())[:8]
        new_win = Window(win_id, pid, x, y, w, h, z_order=len(self.windows))
        self.windows.append(new_win)
        return win_id