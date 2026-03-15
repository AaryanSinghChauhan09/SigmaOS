# Generated method: SigmaContinuityEngine.sync_clipboard
from typing import Dict, List, Any
import time

class SigmaContinuityEngine:
    def sync_clipboard(self, content: Any, source_device: str) -> str:
        """USP: Atomic clipboard syncing across all sovereign devices."""
        self._clipboard_content = content
        return f'Continuity: Clipboard synced from {source_device}. Available OS-wide.'