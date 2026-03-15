# Generated method: SigmaThemeEngine.apply_aura
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaThemeEngine:
    def apply_aura(self, aura_name: str) -> bool:
        if aura_name not in self.AURAS:
            return False
        self.current_aura = aura_name
        if self.kernel and hasattr(self.kernel, 'bus'):
            self.kernel.bus.emit('aura.applied', self.AURAS[aura_name])
        print(f'[AURA] Switched to {aura_name} style.')
        return True