"""
chemistry_lab.py — backward-compat shim.
Real implementation lives in chemistry_lab/ package.
"""

from .chemistry_lab.ChemistryLabPage import *  # noqa

__all__ = ['ChemistryLabPage']

"""Auto-generated package __init__.py"""
from .chemistrylabpage import *  # noqa: F401, F403
