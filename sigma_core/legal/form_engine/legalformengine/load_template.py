# Generated method: LegalFormEngine.load_template
import json
import os
import time
from typing import Dict, Any, List, Optional
from .statutory_data import GRAND_LIBRARY

class LegalFormEngine:
    def load_template(self, form_id: str) -> Optional[Dict[str, Any]]:
        path = os.path.join(self.templates_path, f'{form_id}.json')
        if os.path.exists(path):
            with open(path, 'r') as f:
                return json.load(f)
        return None