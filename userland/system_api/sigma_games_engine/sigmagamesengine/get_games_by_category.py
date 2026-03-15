# Generated method: SigmaGamesEngine.get_games_by_category
import os
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Type
from .games import SigmaGame, ALL_GAMES

class SigmaGamesEngine:
    def get_games_by_category(self) -> Dict[str, List[str]]:
        cats: Dict[str, List[str]] = {}
        for cls in ALL_GAMES:
            cat = cls.CATEGORY.split('/')[0].strip()
            cats.setdefault(cat, []).append(cls.GAME_NAME)
        return cats