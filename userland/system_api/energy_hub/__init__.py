"""
energy_hub.py — backward-compat shim.
Real implementation lives in energy_hub/ package.
"""

from .energy_hub.AdaptiveEnergyController import *  # noqa

__all__ = ['AdaptiveEnergyController']

"""Auto-generated package __init__.py"""
from .adaptiveenergycontroller import *  # noqa: F401, F403
