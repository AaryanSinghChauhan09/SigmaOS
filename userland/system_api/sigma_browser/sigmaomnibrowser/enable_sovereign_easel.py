"""
Auto-split from userland\system_api\sigma_browser.py — SigmaOmniBrowser.enable_sovereign_easel
"""

import random
from sigma_core.system.sovereign_app import SovereignApp



class SigmaOmniBrowser:
    def enable_sovereign_easel(self, easel_id: str):
        """USP: Arc Browser Parity. Creates a collaborative, persistent scratchpad/canvas."""
        self.active_easels.append(easel_id)
        if self.kernel:
            self.kernel.bus.emit('browser.easel.create', {'id': easel_id, 'mode': 'Collaborative'})
        return f"Sovereign Easel '{easel_id}' initialized. Collaborative focus mode ACTIVE."
