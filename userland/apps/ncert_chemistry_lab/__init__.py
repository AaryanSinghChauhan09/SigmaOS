"""
ncert_chemistry_lab.py — backward-compat shim.
Real implementation lives in ncert_chemistry_lab/ package.
"""

from .ncert_chemistry_lab._r import *  # noqa
from .ncert_chemistry_lab.Chemistry_Classes_6_10 import *  # noqa
from .ncert_chemistry_lab.Chemistry_Classes_11_12 import *  # noqa

__all__ = ['_r', 'Chemistry_Classes_6_10', 'Chemistry_Classes_11_12']

"""Auto-generated package __init__.py"""
from ._r import *  # noqa: F401, F403
from .chemistry_classes_6_10 import *  # noqa: F401, F403
from .chemistry_classes_11_12 import *  # noqa: F401, F403
