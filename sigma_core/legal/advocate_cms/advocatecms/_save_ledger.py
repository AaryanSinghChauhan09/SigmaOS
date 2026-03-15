# Generated method: AdvocateCMS._save_ledger
import os
import json
import time
from typing import Dict, Any, List, Optional

class AdvocateCMS:
    def _save_ledger(self):
        path = os.path.join(self.cms_path, 'ledger.json')
        with open(path, 'w') as f:
            json.dump(self.case_ledger, f, indent=4)