# Generated method: SigmaAuraProject.stop_all_projections
from typing import Dict, List, Any

class SigmaAuraProject:
    def stop_all_projections(self) -> str:
        count = len(self._active_streams)
        self._active_streams = []
        return f'AuraProject: {count} streams terminated. Radios idle.'