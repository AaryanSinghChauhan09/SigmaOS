# Generated method: SigmaSovereignClipboardV2.get_pinned
import os
import sys
import json
import time
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignClipboardV2:
    def get_pinned(self) -> List[Dict[str, Any]]:
        return list(self._pinned)