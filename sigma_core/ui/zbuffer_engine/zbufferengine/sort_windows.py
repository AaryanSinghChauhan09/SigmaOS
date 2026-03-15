# Generated method: ZBufferEngine.sort_windows
from typing import List, Dict, Any

class ZBufferEngine:
    def sort_windows(self, windows: Dict[str, Any]) -> List[str]:
        """USP: Depth-aware occlusion calculation."""
        sorted_ids = sorted(windows.keys(), key=lambda x: windows[x].get('z_index', 0))
        self.z_map = sorted_ids
        return sorted_ids