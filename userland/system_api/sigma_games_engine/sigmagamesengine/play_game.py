# Generated method: SigmaGamesEngine.play_game
import os
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Type
from .games import SigmaGame, ALL_GAMES

class SigmaGamesEngine:
    def play_game(self, game_id: str) -> str:
        if game_id not in self.catalog:
            return 'Error: Game not found.'
        game = self.catalog[game_id]()
        game.hydrate()
        game._init_state()
        return f'PLAY_SESSION: {game.GAME_NAME} v{game.VERSION} logic active.'