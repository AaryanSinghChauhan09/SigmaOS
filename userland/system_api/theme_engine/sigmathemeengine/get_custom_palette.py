# Generated method: SigmaThemeEngine.get_custom_palette
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaThemeEngine:
    def get_custom_palette(self) -> Dict[str, str]:
        return self.AURAS.get(self.current_aura, self.AURAS['DeepSpace'])