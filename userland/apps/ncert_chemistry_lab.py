"""
ncert_chemistry_lab.py — backward-compat shim.
Real implementation lives in ncert_chemistry_lab/ package.
"""

from ncert_chemistry_lab._r import *  # noqa
from ncert_chemistry_lab.Chemistry_Classes_6_10 import *  # noqa
from ncert_chemistry_lab.Chemistry_Classes_11_12 import *  # noqa

__all__ = ['_r', 'Chemistry_Classes_6_10', 'Chemistry_Classes_11_12']
