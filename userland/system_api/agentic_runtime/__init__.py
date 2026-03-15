"""
agentic_runtime.py — backward-compat shim.
Real implementation lives in agentic_runtime/ package.
"""

from .agentic_runtime.SigmaAgenticRuntime import *  # noqa

__all__ = ['SigmaAgenticRuntime']

"""Auto-generated package __init__.py"""
from .sigmaagenticruntime import *  # noqa: F401, F403
