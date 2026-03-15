"""
media_studio.py — backward-compat shim.
Real implementation lives in media_studio/ package.
"""

from .media_studio.SigmaMediaStudio import *  # noqa

__all__ = ['SigmaMediaStudio']

"""Auto-generated package __init__.py"""
from .sigmamediastudio import *  # noqa: F401, F403
