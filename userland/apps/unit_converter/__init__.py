"""
unit_converter.py — backward-compat shim.
Real implementation lives in unit_converter/ package.
"""

from .unit_converter.convert import *  # noqa
from .unit_converter.UnitConverter import *  # noqa
from .unit_converter.launch import *  # noqa

__all__ = ['convert', 'UnitConverter', 'launch']

"""Auto-generated package __init__.py"""
from .convert import *  # noqa: F401, F403
from .unitconverter import *  # noqa: F401, F403
from .launch import *  # noqa: F401, F403
