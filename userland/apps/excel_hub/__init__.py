"""
excel_hub.py — backward-compat shim.
Real implementation lives in excel_hub/ package.
"""

from .excel_hub.ExcelHub import *  # noqa

__all__ = ['ExcelHub']

"""Auto-generated package __init__.py"""
from .excelhub import *  # noqa: F401, F403
