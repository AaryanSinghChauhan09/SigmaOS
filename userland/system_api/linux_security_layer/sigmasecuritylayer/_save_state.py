# Generated method: SigmaSecurityLayer._save_state
import time
import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaSecurityLayer:
    def _save_state(self):
        with open(self.state_file, 'w', encoding='utf-8') as f:
            json.dump(self.state, f, indent=4)