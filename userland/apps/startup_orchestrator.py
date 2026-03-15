"""
startup_orchestrator.py — backward-compat shim.
Real implementation lives in startup_orchestrator/ package.
"""

from startup_orchestrator.StartupOrchestrator import *  # noqa

__all__ = ['StartupOrchestrator']
