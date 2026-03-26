"""
terminal_page.py — backward-compat shim.
Real implementation lives in terminal_page/ package.
"""

from .terminal_page.TerminalPage import *  # noqa

__all__ = ['TerminalPage']

"""Auto-generated package __init__.py"""
from .terminalpage import *  # noqa: F401, F403
