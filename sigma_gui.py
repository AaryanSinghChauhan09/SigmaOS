"""
sigma_gui.py — backward-compat shim.
Real implementation lives in sigma_gui/ package.
"""

from sigma_gui.SigmaGUI import *  # noqa
from sigma_gui.launch_gui import *  # noqa

__all__ = ['SigmaGUI', 'launch_gui']
