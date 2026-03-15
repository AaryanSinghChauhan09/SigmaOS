# Generated file: get_all_games
from .base import SigmaGame
from .classic import *
from .puzzle import *
from .arcade import *
from .brain import *
from .pro import *
import sys

def get_all_games():
    all_games = []
    for name, obj in globals().items():
        if isinstance(obj, type) and issubclass(obj, SigmaGame) and (obj != SigmaGame):
            all_games.append(obj)
    return all_games