"""
ncert_biology_lab.py — backward-compat shim.
Real implementation lives in ncert_biology_lab/ package.
"""

from ncert_biology_lab._r import *  # noqa
from ncert_biology_lab.Biology_Classes_6_10 import *  # noqa
from ncert_biology_lab.Biology_Classes_11_12 import *  # noqa

__all__ = ['_r', 'Biology_Classes_6_10', 'Biology_Classes_11_12']
