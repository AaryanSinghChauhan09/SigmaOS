# Generated method: SigmaSnapGrid.create_snap_group
from typing import Dict, List, Any

class SigmaSnapGrid:
    def create_snap_group(self, group_name: str, window_ids: List[str]) -> str:
        """Windows 11 USP Parity: Groups snapped windows to minimize/restore them together."""
        self._snap_groups[group_name] = window_ids
        return f"SnapGrid: Created Snap Group '{group_name}' with {len(window_ids)} apps."