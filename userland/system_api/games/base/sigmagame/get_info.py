# Generated method: SigmaGame.get_info
from typing import List, Tuple, Dict, Optional, Any, Type, Set

class SigmaGame:
    def get_info(self) -> Dict[str, Any]:
        return {'id': self.GAME_ID, 'name': self.GAME_NAME, 'category': self.CATEGORY, 'version': self.VERSION, 'size': f'{self.SIZE_KB}KB', 'icon': self.ICON, 'desc': self.DESC, 'age_rating': self.AGE_RATING}