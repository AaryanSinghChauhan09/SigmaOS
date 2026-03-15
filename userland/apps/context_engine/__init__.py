"""
context_engine.py — backward-compat shim.
Real implementation lives in context_engine/ package.
"""

from .context_engine.ContextEngine import *  # noqa

__all__ = ['ContextEngine']

"""Auto-generated package __init__.py"""
from .contextengine import *  # noqa: F401, F403
