# Generated method: SigmaGamesEngine.__init__
import os
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaEntropy as _ent
    class random:
        @staticmethod
        def random(): return _ent.randint(0, 10**9) / 10**9
        @staticmethod
        def randint(a, b): return _ent.randint(a, b)
        @staticmethod
        def uniform(a, b): return a + (b - a) * (random.random())
        @staticmethod
        def choice(seq): return seq[_ent.randint(0, len(seq)-1)] if seq else None
        @staticmethod
        def shuffle(lst):
            for i in range(len(lst)-1, 0, -1):
                j = _ent.randint(0, i); lst[i], lst[j] = lst[j], lst[i]
        @staticmethod
        def sample(pop, k): return [pop[_ent.randint(0,len(pop)-1)] for _ in range(k)]
except Exception:
    import random
import time
from typing import List, Tuple, Dict, Any, Optional, Type
from .games import SigmaGame, ALL_GAMES

class SigmaGamesEngine:
    def __init__(self, kernel):
        self.kernel = kernel
        self.catalog = {cls.GAME_ID: cls for cls in ALL_GAMES}