"""
startup_orchestrator.py — backward-compat shim.
Real implementation lives in startup_orchestrator/ package.
"""

from .startup_orchestrator.StartupOrchestrator import *  # noqa

__all__ = ['StartupOrchestrator']

"""Auto-generated package __init__.py"""
from .startuporchestrator import *  # noqa: F401, F403
