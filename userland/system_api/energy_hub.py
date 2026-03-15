"""
energy_hub.py — backward-compat shim.
Real implementation lives in energy_hub/ package.
"""

from energy_hub.AdaptiveEnergyController import *  # noqa

__all__ = ['AdaptiveEnergyController']
