"""
aether_assistant.py — backward-compat shim.
Real implementation lives in aether_assistant/ package.
"""

from .aether_assistant.AetherAssistant import *  # noqa

__all__ = ['AetherAssistant']

"""Auto-generated package __init__.py"""
from .aetherassistant import *  # noqa: F401, F403
