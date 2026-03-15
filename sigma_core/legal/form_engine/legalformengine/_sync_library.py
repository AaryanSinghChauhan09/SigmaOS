# Generated method: LegalFormEngine._sync_library
import json
import os
import time
from typing import Dict, Any, List, Optional
from .statutory_data import GRAND_LIBRARY

class LegalFormEngine:
    def _sync_library(self):
        """Synchronizes the modular STATUTORY_DATA with the template storage."""
        for fid, data in GRAND_LIBRARY.items():
            path = os.path.join(self.templates_path, f'{fid}.json')
            with open(path, 'w') as f:
                json.dump(data, f, indent=4)