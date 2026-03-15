"""
sigma_gui.py — backward-compat shim.
Real implementation lives in sigma_gui/ package.
"""

from .sigma_gui.SigmaGUI import *  # noqa
from .sigma_gui.launch_gui import *  # noqa

__all__ = ['SigmaGUI', 'launch_gui']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
from .launch_gui import *  # noqa: F401, F403
