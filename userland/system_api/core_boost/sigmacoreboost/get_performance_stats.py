# Generated method: SigmaCoreBoost.get_performance_stats
from typing import Dict, List, Any

class SigmaCoreBoost:
    def get_performance_stats(self) -> Dict:
        return {'Input_Latency': f'{self._latency_ms} ms', 'Fenced_Apps': self._active_optimizations, 'DirectStorage_Active': True}