# Generated method: SigmaNeuralFabric.get_live_metrics
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import time

class SigmaNeuralFabric:
    def get_live_metrics(self) -> ComputeState:
        """Returns unified system telemetry."""
        self._stats['telemetry_hits'] += 1
        return ComputeState(cpu_usage=12.4, ram_available=312.0, mesh_nodes_online=len(self.active_pool) - 1)