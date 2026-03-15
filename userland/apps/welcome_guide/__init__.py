"""
welcome_guide.py — backward-compat shim.
Real implementation lives in welcome_guide/ package.
"""

from .welcome_guide.WelcomeAssistant import *  # noqa

__all__ = ['WelcomeAssistant']

"""Auto-generated package __init__.py"""
from .welcomeassistant import *  # noqa: F401, F403
