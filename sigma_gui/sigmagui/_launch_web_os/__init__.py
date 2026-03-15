"""
_launch_web_os.py — backward-compat shim.
Real implementation lives in _launch_web_os/ package.
"""

from ._launch_web_os.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
