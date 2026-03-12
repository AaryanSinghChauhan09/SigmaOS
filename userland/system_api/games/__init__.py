from .base import SigmaGame
from .classic import *
from .puzzle import *
from .arcade import *
from .brain import *
from .pro import *

import sys

# Collect all SigmaGame subclasses from the submodules
def get_all_games():
    all_games = []
    # This is a bit of a hack but ensures we get everything imported
    for name, obj in globals().items():
        if isinstance(obj, type) and issubclass(obj, SigmaGame) and obj != SigmaGame:
            all_games.append(obj)
    return all_games

ALL_GAMES = get_all_games()
