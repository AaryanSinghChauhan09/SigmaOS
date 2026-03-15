# Generated method: SigmaGamesEngine.search
import os
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Type
from .games import SigmaGame, ALL_GAMES

class SigmaGamesEngine:
    def search(self, query: str) -> List[str]:
        """Search games by name or category."""
        q = query.lower()
        return [cls.GAME_NAME for cls in ALL_GAMES if q in cls.GAME_NAME.lower() or q in cls.CATEGORY.lower() or q in cls.DESC.lower()]