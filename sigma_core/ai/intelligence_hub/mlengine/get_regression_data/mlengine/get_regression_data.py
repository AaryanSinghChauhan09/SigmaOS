# Generated method: MLEngine.get_regression_data
import math
import random
import time
from typing import List, Dict, Any, Optional

class MLEngine:
    def get_regression_data(self, n=50):
        """Generates mock data for Linear Graphs and Scatter Plots."""
        x = [i for i in range(n)]
        y = [2 * i + 5 + random.uniform(-5, 5) for i in x]
        return (x, y)