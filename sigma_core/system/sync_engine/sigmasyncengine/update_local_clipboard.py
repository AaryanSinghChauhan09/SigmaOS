# Generated method: SigmaSyncEngine.update_local_clipboard
import time
import json
import threading
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncEngine:
    def update_local_clipboard(self, text: str):
        """API for local tools to push into the sync fabric."""
        self.last_clipboard = text
        return 'Clipboard staged for Apex Handoff.'