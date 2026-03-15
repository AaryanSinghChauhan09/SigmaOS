"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager._activate_zen_latency
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def _activate_zen_latency(self, phase: str='') -> str:
        """USP: Activates Zen Latency mode for instant UI feedback."""
        if self.kernel and hasattr(self.kernel, 'registry'):
            hd = self.kernel.registry.get('hyper_drive')
            if hd and hasattr(hd, 'engage_zen_latency_mode'):
                return hd.engage_zen_latency_mode()
        return 'Hyper-Drive module not available for Zen Latency.'
