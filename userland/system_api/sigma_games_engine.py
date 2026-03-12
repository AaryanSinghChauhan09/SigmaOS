import os
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Type

from .games import SigmaGame, ALL_GAMES

class SigmaGamesEngine:
    """Master games registry and orchestration engine — 100+ games, 12 categories."""

    def __init__(self, kernel):
        self.kernel  = kernel
        self.catalog = {cls.GAME_ID: cls for cls in ALL_GAMES}

    def get_catalog_metadata(self) -> List[Dict]:
        return [cls().get_info() for cls in ALL_GAMES]

    def list_games(self) -> List[str]:
        return [cls.GAME_NAME for cls in ALL_GAMES]

    def get_games_by_category(self) -> Dict[str, List[str]]:
        cats: Dict[str, List[str]] = {}
        for cls in ALL_GAMES:
            cat = cls.CATEGORY.split("/")[0].strip()
            cats.setdefault(cat, []).append(cls.GAME_NAME)
        return cats

    def search(self, query: str) -> List[str]:
        """Search games by name or category."""
        q = query.lower()
        return [cls.GAME_NAME for cls in ALL_GAMES
                if q in cls.GAME_NAME.lower() or q in cls.CATEGORY.lower() or q in cls.DESC.lower()]

    def install_game(self, game_id: str) -> Dict:
        if game_id not in self.catalog:
            return {"status": "error", "message": f"Game '{game_id}' not found."}
        game   = self.catalog[game_id]()
        result = game.hydrate()
        return {"status": "success", "message": result, "game_id": game_id, "name": game.GAME_NAME}

    def play_game(self, game_id: str) -> str:
        if game_id not in self.catalog:
            return "Error: Game not found."
        game = self.catalog[game_id]()
        game.hydrate()
        game._init_state()
        return f"PLAY_SESSION: {game.GAME_NAME} v{game.VERSION} logic active."

    def health_check(self) -> str:
        cats = self.get_games_by_category()
        cat_summary = " | ".join(f"{k}:{len(v)}" for k, v in cats.items())
        return (f"OK — SigmaGames Engine: {len(ALL_GAMES)} games registered "
                f"| Categories: {cat_summary} | Performance: APEX READY.")

if __name__ == "__main__":
    engine = SigmaGamesEngine(None)
    print(engine.health_check())
    print(f"Total Games Loaded: {len(ALL_GAMES)}")
