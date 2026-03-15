# Generated method: DeepLearningEngine.add_layer
import math
import random
import time
from typing import List, Dict, Any, Optional

class DeepLearningEngine:
    def add_layer(self, units: int, activation: str='relu'):
        self.layers.append({'units': units, 'activation': activation})
        self.log_activity(f'Added layer: {units} units, {activation}')