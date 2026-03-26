"""
chat_page.py — backward-compat shim.
Real implementation lives in chat_page/ package.
"""

from .chat_page.SigmaChatPage import *  # noqa

__all__ = ['SigmaChatPage']

"""Auto-generated package __init__.py"""
from .sigmachatpage import *  # noqa: F401, F403
