"""
jigsaw_puzzle.py — backward-compat shim.
Real implementation lives in jigsaw_puzzle/ package.
"""

from .jigsaw_puzzle.JigsawPuzzle import *  # noqa

__all__ = ['JigsawPuzzle']

"""Auto-generated package __init__.py"""
from .jigsawpuzzle import *  # noqa: F401, F403
