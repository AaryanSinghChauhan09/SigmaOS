# Generated method: SigmaSovereignClipboard.copy
import threading
import time
import hashlib
from typing import Dict, Optional, Any

class SigmaSovereignClipboard:
    def copy(self, text: str, is_sensitive: bool=False) -> str:
        """USP: Sovereign Copy with Scrubbing."""
        clean_text = self._sigma_scrub(text)
        with self._lock:
            self._local_item = {'content': clean_text, 'timestamp': time.time(), 'node_id': 'SIGMA-NODE-PRO', 'sensitive': is_sensitive}
            self._history.append(self._local_item)
            if len(self._history) > 20:
                self._history.pop(0)
        if self._mesh_active and (not is_sensitive) and self.kernel:
            self.kernel.bus.emit('mesh.clipboard.sync', self._local_item)
        return f'Clipboard: Copied artifact (Scrubbed: {text != clean_text}). Shared across mesh.'