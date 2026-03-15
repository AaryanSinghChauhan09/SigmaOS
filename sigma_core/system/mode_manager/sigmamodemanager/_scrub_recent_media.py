"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager._scrub_recent_media
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def _scrub_recent_media(self, phase: str='') -> str:
        if self.kernel and self.kernel.media_forge:
            return 'MediaForge forensic scrub initiated on recent assets.'
        return 'MediaForge offline.'
