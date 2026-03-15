"""
meshtalk.py — backward-compat shim.
Real implementation lives in meshtalk/ package.
"""

from .meshtalk.MeshTalk import *  # noqa

__all__ = ['MeshTalk']

"""Auto-generated package __init__.py"""
from .meshtalk import *  # noqa: F401, F403
