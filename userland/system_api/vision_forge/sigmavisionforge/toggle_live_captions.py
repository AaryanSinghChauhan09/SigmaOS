# Generated method: SigmaVisionForge.toggle_live_captions
from typing import Dict, List, Any
import random

class SigmaVisionForge:
    def toggle_live_captions(self, enabled: bool) -> str:
        """USP: Low-latency, multi-lingual audio-to-text live relay."""
        self._active_captions = enabled
        status = 'ENABLED' if enabled else 'DISABLED'
        return f'VisionForge: Sovereign Live Captions {status}.'