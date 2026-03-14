"""
SigmaOS Core Package (Modular Apex)
====================================
Exports main kernel classes and modular shards.
"""
import os
import sys

# Robust Root Level Injection
_p = os.path.abspath(__file__)
_root = os.path.dirname(os.path.dirname(_p))
if _root not in sys.path:
    sys.path.insert(0, _root)

from .kernel import SigmaKernel
from .system.config import SigmaConfig
from .system.event_bus import EventBus
from .system.registry import ModuleRegistry
from .system.interfaces import SigmaModuleBase

__all__ = ["SigmaKernel", "SigmaConfig", "EventBus", "ModuleRegistry", "SigmaModuleBase"]
__version__ = "5.2.0"
