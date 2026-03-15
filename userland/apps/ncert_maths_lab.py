"""
ncert_maths_lab.py — backward-compat shim.
Real implementation lives in ncert_maths_lab/ package.
"""

from ncert_maths_lab._r import *  # noqa
from ncert_maths_lab.Maths_Classes_1_5 import *  # noqa
from ncert_maths_lab.Maths_Classes_6_10 import *  # noqa
from ncert_maths_lab.Maths_Classes_11_12 import *  # noqa

__all__ = ['_r', 'Maths_Classes_1_5', 'Maths_Classes_6_10', 'Maths_Classes_11_12']
