# Generated method: SigmaSovereignClipboardV2.clear_history
import os
import sys
import json
import time
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignClipboardV2:
    def clear_history(self, keep_pinned: bool=True) -> int:
        """Wipes clipboard history, optionally preserving pinned items."""
        count = len(self._history)
        if keep_pinned:
            self._history = [i for i in self._history if i.get('pinned')]
        else:
            self._history = []
            self._pinned = []
        self._save_to_disk()
        return count