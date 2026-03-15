"""
ncert_master_lab.py — backward-compat shim.
Real implementation lives in ncert_master_lab/ package.
"""

from .ncert_master_lab.NCERTMasterLab import *  # noqa

__all__ = ['NCERTMasterLab']

"""Auto-generated package __init__.py"""
from .ncertmasterlab import *  # noqa: F401, F403
