# Generated method: DeepLearningEngine.get_example_1
import math
import random
import time
from typing import List, Dict, Any, Optional

class DeepLearningEngine:
    def get_example_1(self):
        """Ex1: Simple Linear Prediction."""
        return {'name': 'Linear Predictor', 'intro': 'Predicts y based on x (y = 2x + 1).', 'data': 'Pairs like (1, 3), (2, 5), (3, 7).', 'model': 'Single Dense Layer (1 unit).', 'training': 'SGD optimizer, MSE loss.'}