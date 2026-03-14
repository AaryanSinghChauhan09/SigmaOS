"""
SigmaOS Hardware Abstraction Shard
===================================
Low-level silicon shims and polyglot loaders.
"""
from .hal import SigmaHAL
from .kernel_hal import SovereignHAL
from .polyglot_loader import PolyglotLoader

__all__ = ["SigmaHAL", "SovereignHAL", "PolyglotLoader"]
