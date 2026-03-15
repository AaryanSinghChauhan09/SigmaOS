"""
_notify.py — backward-compat shim.
Real implementation lives in _notify/ package.
"""

from _notify.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']
