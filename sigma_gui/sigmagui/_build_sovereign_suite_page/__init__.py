"""
_build_sovereign_suite_page.py — backward-compat shim.
Real implementation lives in _build_sovereign_suite_page/ package.
"""

from ._build_sovereign_suite_page.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
