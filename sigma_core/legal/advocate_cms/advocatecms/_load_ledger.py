# Generated method: AdvocateCMS._load_ledger
import os
import json
import time
from typing import Dict, Any, List, Optional

class AdvocateCMS:
    def _load_ledger(self):
        path = os.path.join(self.cms_path, 'ledger.json')
        if os.path.exists(path):
            try:
                with open(path, 'r') as f:
                    self.case_ledger = json.load(f)
            except:
                self.case_ledger = {}