# Generated method: SigmaThemeEngine._auto_theme
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaThemeEngine:
    def _auto_theme(self, payload: dict):
        """Automatically shift aura based on OS Mode."""
        mode = payload.get('preset', 'Normal')
        if mode == 'Gaming_Apex':
            self.apply_aura('CyberPunk')
        elif mode == 'Nightly_Purge':
            self.apply_aura('DeepSpace')
        elif mode == 'Work_Symmetry':
            self.apply_aura('Zodiac')