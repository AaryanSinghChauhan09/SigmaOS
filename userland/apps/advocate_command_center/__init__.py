"""
advocate_command_center.py — backward-compat shim.
Real implementation lives in advocate_command_center/ package.
"""

from .advocate_command_center.AdvocateCommandCenter import *  # noqa

__all__ = ['AdvocateCommandCenter']

"""Auto-generated package __init__.py"""
from .advocatecommandcenter import *  # noqa: F401, F403
