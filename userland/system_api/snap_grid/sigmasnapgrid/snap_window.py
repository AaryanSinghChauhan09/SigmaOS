# Generated method: SigmaSnapGrid.snap_window
from typing import Dict, List, Any

class SigmaSnapGrid:
    def snap_window(self, window_id: str, cell_id: int) -> str:
        """Anchors a specific window to a grid cell."""
        if not self._active_cells:
            self._active_cells = self._calculate_cells(self._current_layout)
        if cell_id >= len(self._active_cells):
            return 'Error: Cell ID out of layout bounds.'
        cell = self._active_cells[cell_id]
        return f"Window '{window_id}' snapped to Region {cell_id} ({cell}) in {self._active_workspace}."