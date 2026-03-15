# Generated method: SigmaLegalPro.__init__
from typing import Dict, List, Any, Optional
import datetime
import json
import os

class SigmaLegalPro:
    def __init__(self, workspace_path: str=None):
        if workspace_path is None:
            workspace_path = os.path.join(os.path.dirname(__file__), 'vault')
        self.workspace = workspace_path
        self._initialize_vault()