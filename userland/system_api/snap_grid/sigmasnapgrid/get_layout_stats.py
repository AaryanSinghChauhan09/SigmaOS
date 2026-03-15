# Generated method: SigmaSnapGrid.get_layout_stats
from typing import Dict, List, Any

class SigmaSnapGrid:
    def get_layout_stats(self) -> Dict:
        return {'Active': self._current_layout, 'Workspace': self._active_workspace, 'Total_Cells': len(self._active_cells), 'Snap_Groups': len(self._snap_groups), 'Profile': self._layouts.get(self._current_layout, 'Custom')}