"""
_launch_app.py — backward-compat shim.
Real implementation lives in _launch_app/ package.
"""

from ._launch_app.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
