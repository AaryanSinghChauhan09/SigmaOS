# Generated method: SensoryAssets.get_glyphs
from typing import Dict, Any, Optional

class SensoryAssets:
    def get_glyphs(self, set_name: str='Sovereign') -> Dict[str, str]:
        return self.GLYPH_SETS.get(set_name, self.GLYPH_SETS['Sovereign'])