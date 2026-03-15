"""
_launch_app.py — backward-compat shim.
Real implementation lives in _launch_app/ package.
"""

from _launch_app.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']
