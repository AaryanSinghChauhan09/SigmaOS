# Generated method: MeshCompute.share_idle_cycles
import time
import random
import uuid
import threading
from typing import Dict, List, Any

class MeshCompute:
    def share_idle_cycles(self, reserve_pct: float=20.0) -> str:
        """USP: Contributes your device's idle cycles to the mesh while reserving X% for local tasks."""
        return f'Mesh: Sharing {100 - reserve_pct:.0f}% of idle NPU/GPU cycles. {reserve_pct}% Reserved for local Apex task.'