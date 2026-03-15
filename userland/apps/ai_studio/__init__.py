"""
ai_studio.py — backward-compat shim.
Real implementation lives in ai_studio/ package.
"""

from .ai_studio.AIStudio import *  # noqa

__all__ = ['AIStudio']

"""Auto-generated package __init__.py"""
from .aistudio import *  # noqa: F401, F403
