# Generated method: AutomationBrain._save_weights
import json
import os
import random
import time
from typing import Dict, List, Any

class AutomationBrain:
    def _save_weights(self):
        with open(self.model_path, 'w') as f:
            json.dump(self.weights, f)