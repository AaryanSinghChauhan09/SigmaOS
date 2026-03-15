"""
ncert_physics_lab.py — backward-compat shim.
Real implementation lives in ncert_physics_lab/ package.
"""

from .ncert_physics_lab._r import *  # noqa
from .ncert_physics_lab.Physics_Classes_6_10 import *  # noqa
from .ncert_physics_lab.Physics_Classes_11_12 import *  # noqa

__all__ = ['_r', 'Physics_Classes_6_10', 'Physics_Classes_11_12']

"""Auto-generated package __init__.py"""
from ._r import *  # noqa: F401, F403
from .physics_classes_6_10 import *  # noqa: F401, F403
from .physics_classes_11_12 import *  # noqa: F401, F403
