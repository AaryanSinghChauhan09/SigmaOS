# Generated method: SensoryAssets.get_soundscape
from typing import Dict, Any, Optional

class SensoryAssets:
    def get_soundscape(self, set_name: str='Zen') -> Dict[str, Optional[str]]:
        return self.SOUNDSCAPES.get(set_name, self.SOUNDSCAPES['Zen'])