# Generated method: SigmaAutomationLayer._save_data
import time
import json
import uuid
import threading
from pathlib import Path
from typing import Dict, List, Any

class SigmaAutomationLayer:
    def _save_data(self, path: Path, data: Any):
        with open(path, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=4)