"""
text_cleaner.py — backward-compat shim.
Real implementation lives in text_cleaner/ package.
"""

from .text_cleaner.TextCleaner import *  # noqa

__all__ = ['TextCleaner']

"""Auto-generated package __init__.py"""
from .textcleaner import *  # noqa: F401, F403
