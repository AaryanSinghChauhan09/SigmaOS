"""
sigma_calculator.py — backward-compat shim.
Real implementation lives in sigma_calculator/ package.
"""

from .sigma_calculator.SigmaCalculator import *  # noqa
from .sigma_calculator.launch import *  # noqa

__all__ = ['SigmaCalculator', 'launch']

"""Auto-generated package __init__.py"""
from .sigmacalculator import *  # noqa: F401, F403
from .launch import *  # noqa: F401, F403
