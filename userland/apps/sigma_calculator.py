"""
sigma_calculator.py — backward-compat shim.
Real implementation lives in sigma_calculator/ package.
"""

from sigma_calculator.SigmaCalculator import *  # noqa
from sigma_calculator.launch import *  # noqa

__all__ = ['SigmaCalculator', 'launch']
