# Generated method: SigmaGamesEngine.list_games
import os
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Type
from .games import SigmaGame, ALL_GAMES

class SigmaGamesEngine:
    def list_games(self) -> List[str]:
        return [cls.GAME_NAME for cls in ALL_GAMES]