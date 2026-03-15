# Generated method: SigmaNeuralLab.simulate_backprop
import time
import random
from typing import List, Dict, Any, Optional

class SigmaNeuralLab:
    def simulate_backprop(self, layers: int=5) -> List[Dict[str, float]]:
        """Simulates gradients across a deep network."""
        grads = []
        for i in range(layers):
            grad_val = 0.5 * 0.8 ** i
            grads.append({'layer_idx': i, 'grad_magnitude': float(int(grad_val * 10000)) / 10000.0})
        return grads