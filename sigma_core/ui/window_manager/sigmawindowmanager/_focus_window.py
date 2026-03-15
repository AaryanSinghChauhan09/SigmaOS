# Generated method: SigmaWindowManager._focus_window
from dataclasses import dataclass, field
import uuid
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaWindowManager:
    def _focus_window(self, win_id: str):
        if win_id not in self._windows:
            return
        if win_id in self._stack:
            self._stack.remove(win_id)
        self._stack.append(win_id)
        for i, sid in enumerate(self._stack):
            self._windows[sid].z_index = i
            self._windows[sid].is_active = sid == win_id