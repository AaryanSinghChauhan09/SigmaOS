"""
katbook_reader.py — backward-compat shim.
Real implementation lives in katbook_reader/ package.
"""

from .katbook_reader.KatbookReaderPage import *  # noqa

__all__ = ['KatbookReaderPage']

"""Auto-generated package __init__.py"""
from .katbookreaderpage import *  # noqa: F401, F403
