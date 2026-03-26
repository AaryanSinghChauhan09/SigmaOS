"""
energy_core.py — backward-compat shim.
Real implementation lives in energy_core/ package.
"""

from .energy_core.EnergyCore import *  # noqa

__all__ = ['EnergyCore']

"""Auto-generated package __init__.py"""
from .energycore import *  # noqa: F401, F403
