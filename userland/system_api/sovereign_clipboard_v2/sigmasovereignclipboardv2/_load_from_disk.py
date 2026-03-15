# Generated method: SigmaSovereignClipboardV2._load_from_disk
import os
import sys
import json
import time
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignClipboardV2:
    def _load_from_disk(self):
        """Loads clipboard state from disk if it exists."""
        if os.path.exists(self._persist_path):
            try:
                with open(self._persist_path, 'r', encoding='utf-8') as f:
                    payload = json.load(f)
                    hist = payload.get('history', [])
                    self._history = hist if isinstance(hist, list) else []
                    pinned = payload.get('pinned', [])
                    self._pinned = pinned if isinstance(pinned, list) else []
            except Exception:
                self._history = []
                self._pinned = []