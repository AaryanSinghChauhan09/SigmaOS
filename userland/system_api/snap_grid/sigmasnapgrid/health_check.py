# Generated method: SigmaSnapGrid.health_check
from typing import Dict, List, Any

class SigmaSnapGrid:
    def health_check(self) -> str:
        return f'OK — Active Layout: {self._current_layout} | Groups: {len(self._snap_groups)} | Workspace: {self._active_workspace}.'