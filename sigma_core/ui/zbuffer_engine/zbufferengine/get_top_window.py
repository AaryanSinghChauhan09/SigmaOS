# Generated method: ZBufferEngine.get_top_window
from typing import List, Dict, Any

class ZBufferEngine:
    def get_top_window(self) -> str:
        return self.z_map[-1] if self.z_map else 'NONE'