# Generated method: SigmaNeuralFabric.add_peer_to_pool
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import time

class SigmaNeuralFabric:
    def add_peer_to_pool(self, peer_id: str, cpu_contribution: float):
        """Adds peer CPU cycles to the local fabric pool."""
        self.active_pool[peer_id] = cpu_contribution
        self._stats['pool_reloads'] += 1
        return f'Mesh-Pool: Added {peer_id} (+{cpu_contribution}% CPU). Fabric re-balanced.'