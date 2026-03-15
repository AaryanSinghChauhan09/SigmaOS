# Generated method: SigmaGamesEngine.install_game
import os
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Type
from .games import SigmaGame, ALL_GAMES

class SigmaGamesEngine:
    def install_game(self, game_id: str) -> Dict:
        if game_id not in self.catalog:
            return {'status': 'error', 'message': f"Game '{game_id}' not found."}
        game = self.catalog[game_id]()
        result = game.hydrate()
        return {'status': 'success', 'message': result, 'game_id': game_id, 'name': game.GAME_NAME}