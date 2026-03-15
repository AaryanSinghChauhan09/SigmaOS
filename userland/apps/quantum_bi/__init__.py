"""
quantum_bi.py — backward-compat shim.
Real implementation lives in quantum_bi/ package.
"""

from .quantum_bi.QuantumBIEngine import *  # noqa

__all__ = ['QuantumBIEngine']

"""Auto-generated package __init__.py"""
from .quantumbiengine import *  # noqa: F401, F403
