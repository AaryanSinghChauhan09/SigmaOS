# Generated method: MeshCompute.request_tflops
import time
import random
import uuid
import threading
from typing import Dict, List, Any

class MeshCompute:
    def request_tflops(self, amount: float, priority: str='NORMAL') -> bool:
        """General compute request from kernel modules."""
        if self._aggregate_tflops - self._requested_tflops >= amount:
            self._requested_tflops += amount
            return True
        return False