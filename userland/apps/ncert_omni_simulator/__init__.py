"""
ncert_omni_simulator.py — backward-compat shim.
Real implementation lives in ncert_omni_simulator/ package.
"""

from .ncert_omni_simulator.NCERTOmniSimulator import *  # noqa

__all__ = ['NCERTOmniSimulator']

"""Auto-generated package __init__.py"""
from .ncertomnisimulator import *  # noqa: F401, F403
