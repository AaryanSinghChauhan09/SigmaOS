"""
puzzle.py — backward-compat shim.
Real implementation lives in puzzle/ package.
"""

from puzzle.JigsawPuzzleGame import *  # noqa
from puzzle.SpotItGame import *  # noqa
from puzzle.ShellGame import *  # noqa
from puzzle.SlidingTilePuzzle import *  # noqa
from puzzle.LightsOut import *  # noqa
from puzzle.TowerOfHanoi import *  # noqa
from puzzle.MemoryMatch import *  # noqa
from puzzle.MathSprint import *  # noqa
from puzzle.ConnectFour import *  # noqa
from puzzle.Minesweeper import *  # noqa
from puzzle.ReversiOthello import *  # noqa
from puzzle.Battleship import *  # noqa
from puzzle.CrosswordLite import *  # noqa
from puzzle.Nonogram import *  # noqa
from puzzle.LogicGridPuzzle import *  # noqa

__all__ = ['JigsawPuzzleGame', 'SpotItGame', 'ShellGame', 'SlidingTilePuzzle', 'LightsOut', 'TowerOfHanoi', 'MemoryMatch', 'MathSprint', 'ConnectFour', 'Minesweeper', 'ReversiOthello', 'Battleship', 'CrosswordLite', 'Nonogram', 'LogicGridPuzzle']
