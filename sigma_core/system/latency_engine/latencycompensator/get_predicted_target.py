# Generated method: LatencyCompensator.get_predicted_target
import time
from typing import List, Tuple

class LatencyCompensator:
    def get_predicted_target(self) -> Tuple[float, float]:
        """Returns the predicted (x, y) coordinates of the next interaction."""
        if not self.input_history:
            return (0, 0)
        last_entry = self.input_history[-1]
        return (last_entry[1] + 10, last_entry[2] + 10)