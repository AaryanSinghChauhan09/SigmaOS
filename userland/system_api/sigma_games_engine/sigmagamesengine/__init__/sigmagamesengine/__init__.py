# Generated method: SigmaGamesEngine.__init__
import os
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Type
from .games import SigmaGame, ALL_GAMES

class SigmaGamesEngine:
    def __init__(self, kernel):
        self.kernel = kernel
        self.catalog = {cls.GAME_ID: cls for cls in ALL_GAMES}