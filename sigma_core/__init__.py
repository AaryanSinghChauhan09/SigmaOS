"""
SigmaOS Core Package
Exports main kernel classes and utilities
"""
from .kernel import SigmaKernel
from .config import SigmaConfig
from .event_bus import EventBus
from .registry import ModuleRegistry

__all__ = ["SigmaKernel", "SigmaConfig", "EventBus", "ModuleRegistry"]
__version__ = "2.0.0"
