"""
puzzle.py — backward-compat shim.
Real implementation lives in puzzle/ package.
"""

from .puzzle.JigsawPuzzleGame import *  # noqa
from .puzzle.SpotItGame import *  # noqa
from .puzzle.ShellGame import *  # noqa
from .puzzle.SlidingTilePuzzle import *  # noqa
from .puzzle.LightsOut import *  # noqa
from .puzzle.TowerOfHanoi import *  # noqa
from .puzzle.MemoryMatch import *  # noqa
from .puzzle.MathSprint import *  # noqa
from .puzzle.ConnectFour import *  # noqa
from .puzzle.Minesweeper import *  # noqa
from .puzzle.ReversiOthello import *  # noqa
from .puzzle.Battleship import *  # noqa
from .puzzle.CrosswordLite import *  # noqa
from .puzzle.Nonogram import *  # noqa
from .puzzle.LogicGridPuzzle import *  # noqa

__all__ = ['JigsawPuzzleGame', 'SpotItGame', 'ShellGame', 'SlidingTilePuzzle', 'LightsOut', 'TowerOfHanoi', 'MemoryMatch', 'MathSprint', 'ConnectFour', 'Minesweeper', 'ReversiOthello', 'Battleship', 'CrosswordLite', 'Nonogram', 'LogicGridPuzzle']

"""Auto-generated package __init__.py"""
from .jigsawpuzzlegame import *  # noqa: F401, F403
from .spotitgame import *  # noqa: F401, F403
from .shellgame import *  # noqa: F401, F403
from .slidingtilepuzzle import *  # noqa: F401, F403
from .lightsout import *  # noqa: F401, F403
from .towerofhanoi import *  # noqa: F401, F403
from .memorymatch import *  # noqa: F401, F403
from .mathsprint import *  # noqa: F401, F403
from .connectfour import *  # noqa: F401, F403
from .minesweeper import *  # noqa: F401, F403
from .reversiothello import *  # noqa: F401, F403
from .battleship import *  # noqa: F401, F403
from .crosswordlite import *  # noqa: F401, F403
from .nonogram import *  # noqa: F401, F403
from .logicgridpuzzle import *  # noqa: F401, F403
