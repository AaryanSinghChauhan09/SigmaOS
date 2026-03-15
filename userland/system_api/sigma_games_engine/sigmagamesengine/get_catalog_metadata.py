# Generated method: SigmaGamesEngine.get_catalog_metadata
import os
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Type
from .games import SigmaGame, ALL_GAMES

class SigmaGamesEngine:
    def get_catalog_metadata(self) -> List[Dict]:
        return [cls().get_info() for cls in ALL_GAMES]