"""
media_studio.py — backward-compat shim.
Real implementation lives in media_studio/ package.
"""

from media_studio.SigmaMediaStudio import *  # noqa

__all__ = ['SigmaMediaStudio']
