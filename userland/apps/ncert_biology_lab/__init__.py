"""
ncert_biology_lab.py — backward-compat shim.
Real implementation lives in ncert_biology_lab/ package.
"""

from .ncert_biology_lab._r import *  # noqa
from .ncert_biology_lab.Biology_Classes_6_10 import *  # noqa
from .ncert_biology_lab.Biology_Classes_11_12 import *  # noqa

__all__ = ['_r', 'Biology_Classes_6_10', 'Biology_Classes_11_12']

"""Auto-generated package __init__.py"""
from ._r import *  # noqa: F401, F403
from .biology_classes_6_10 import *  # noqa: F401, F403
from .biology_classes_11_12 import *  # noqa: F401, F403
