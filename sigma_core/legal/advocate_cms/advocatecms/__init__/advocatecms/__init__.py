# Generated method: AdvocateCMS.__init__
import os
import json
import time
from typing import Dict, Any, List, Optional

class AdvocateCMS:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.kernel = kernel
        self.cms_path = 'userland/legal_data/cms/'
        os.makedirs(self.cms_path, exist_ok=True)
        self.case_ledger: Dict[str, Dict[str, Any]] = {}
        self._load_ledger()