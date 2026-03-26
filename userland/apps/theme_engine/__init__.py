"""
theme_engine.py — backward-compat shim.
Real implementation lives in theme_engine/ package.
"""

from .theme_engine.SovereignThemeEngine import *  # noqa

__all__ = ['SovereignThemeEngine']

"""Auto-generated package __init__.py"""
from .sovereignthemeengine import *  # noqa: F401, F403
