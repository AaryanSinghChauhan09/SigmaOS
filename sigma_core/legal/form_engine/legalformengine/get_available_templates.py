# Generated method: LegalFormEngine.get_available_templates
import json
import os
import time
from typing import Dict, Any, List, Optional
from .statutory_data import GRAND_LIBRARY

class LegalFormEngine:
    def get_available_templates(self) -> List[Dict[str, str]]:
        """List all available legal form templates."""
        templates = []
        if not os.path.exists(self.templates_path):
            return []
        for f in os.listdir(self.templates_path):
            if f.endswith('.json'):
                try:
                    with open(os.path.join(self.templates_path, f), 'r') as tf:
                        data = json.load(tf)
                        templates.append({'id': f.replace('.json', ''), 'title': data.get('title', 'Unknown Form'), 'act': data.get('act', 'Generic Act')})
                except:
                    pass
        return templates