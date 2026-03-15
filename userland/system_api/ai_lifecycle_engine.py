"""
ai_lifecycle_engine.py — backward-compat shim.
Real implementation lives in ai_lifecycle_engine/ package.
"""

from ai_lifecycle_engine.SigmaAILifecycle import *  # noqa

__all__ = ['SigmaAILifecycle']
