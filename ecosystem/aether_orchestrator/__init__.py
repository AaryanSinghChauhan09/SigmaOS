"""
aether_orchestrator.py — backward-compat shim.
Real implementation lives in aether_orchestrator/ package.
"""

from .aether_orchestrator.AetherOrchestrator import *  # noqa

__all__ = ['AetherOrchestrator']

"""Auto-generated package __init__.py"""
from .aetherorchestrator import *  # noqa: F401, F403
