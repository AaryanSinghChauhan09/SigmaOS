# Generated method: SigmaGamesEngine.health_check
import os
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Type
from .games import SigmaGame, ALL_GAMES

class SigmaGamesEngine:
    def health_check(self) -> str:
        cats = self.get_games_by_category()
        cat_summary = ' | '.join((f'{k}:{len(v)}' for k, v in cats.items()))
        return f'OK — SigmaGames Engine: {len(ALL_GAMES)} games registered | Categories: {cat_summary} | Performance: APEX READY.'